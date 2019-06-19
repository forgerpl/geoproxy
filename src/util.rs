use log::*;

use hyper::{
    rt::{lazy, Future},
    Body, Error as HyperError, Method, Response, StatusCode,
};
use std::fmt::Debug;
use std::time::Instant;

use geoindex::GeoIndex;

use crate::config::{Backend, BackendDefinition, ProxyConfig};
use crate::metrics::*;

pub(crate) fn setup_index(config: ProxyConfig) -> GeoIndex<Backend> {
    let ProxyConfig {
        backends,
        default_backend,
    } = config;

    let defs = backends
        .into_iter()
        .map(|BackendDefinition { areas, backend }| (areas, backend))
        .collect();

    GeoIndex::new(defs, default_backend)
}

pub(crate) fn error_result(
    code: StatusCode,
    method: Method,
    path: impl Debug,
    metrics: MetricsClient,
    span: Instant,
    metric: &'static str,
) -> Box<dyn Future<Item = Response<Body>, Error = HyperError> + Send> {
    let status = format!("{} {} {:?}", code.as_str(), method, path);

    Box::new(lazy(move || {
        error!("{} {:?}", status, span.elapsed());
        let _ = metrics.incr(metric);

        Ok(Response::builder()
            .status(code)
            .body(Body::empty())
            .unwrap())
    }))
}
