use crate::error::*;
use log::*;

use hyper::{
    rt::{self, Future},
    service::service_fn,
    Client, Method, Server, StatusCode,
};
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Instant;

use crate::cli::setup_cli;
use crate::config::read_config;
use crate::logger::init_logger;
use crate::metrics::*;
use crate::util::{error_result, setup_index};

mod cli;
mod config;
mod error;
mod logger;
mod metrics;
mod util;

fn main() -> Result<()> {
    let args = setup_cli().get_matches();

    let bind_addr = args
        .value_of("address")
        .unwrap()
        .to_socket_addrs()?
        .next()
        .unwrap();
    let metrics_addr = args
        .value_of("statsd")
        .map(|value| value.to_socket_addrs().unwrap().next().unwrap());
    let config = args.value_of("config").unwrap();

    init_logger();

    // setup metrics
    let metrics = setup_metrics(metrics_addr)?;

    let config = read_config(config)?;
    let index = Arc::new(setup_index(config));

    let client = Client::new();

    let proxy_service = move || {
        let index = index.clone();
        let client = client.clone();
        let metrics = metrics.clone();

        service_fn(
            move |mut req| -> Box<dyn Future<Item = _, Error = _> + Send> {
                // request time span measure
                let span = Instant::now();

                match req.method() {
                    &Method::GET => {
                        // Geolocation header
                        let location = req
                            .headers()
                            .get("Geolocation")
                            .map(|value| value.to_str().ok())
                            .and_then(|value| value)
                            .map(|value| serde_json::from_str(value).ok())
                            .and_then(|value| value);

                        // backend by provided geolocation
                        let backend = index.lookup_coords(location.as_ref());

                        // rewrite url
                        let mapped_uri = backend.map_url(req.uri());
                        let orig_uri = std::mem::replace(req.uri_mut(), mapped_uri);

                        let backend = format!("{}", backend);

                        Box::new(
                            client
                                .request(req)
                                .and_then({
                                    let metrics = metrics.clone();
                                    let orig_uri = orig_uri.clone();

                                    move |resp| {
                                        let elapsed = span.elapsed();

                                        info!(
                                            "{} GET {} [via: {}, loc: {:?}] {:?}",
                                            resp.status().as_str(),
                                            orig_uri,
                                            backend,
                                            location,
                                            elapsed
                                        );

                                        let _ = metrics.incr("requests.proxied");
                                        let _ = metrics.time_duration("request.duration", elapsed);

                                        Ok(resp)
                                    }
                                })
                                .or_else({
                                    let metrics = metrics.clone();
                                    move |_error| {
                                        error_result(
                                            StatusCode::BAD_GATEWAY,
                                            Method::GET,
                                            orig_uri,
                                            metrics.clone(),
                                            span,
                                            "requests.failed",
                                        )
                                    }
                                }),
                        )
                    }
                    method => error_result(
                        StatusCode::METHOD_NOT_ALLOWED,
                        method.clone(),
                        req.uri().path_and_query(),
                        metrics.clone(),
                        span,
                        "requests.rejected",
                    ),
                }
            },
        )
    };

    let server = Server::bind(&bind_addr)
        .serve(proxy_service)
        .map_err(|e| error!("server error: {}", e));

    info!("Listening on {}", bind_addr);

    rt::run(server);

    Ok(())
}
