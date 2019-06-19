use crate::error::*;
#[allow(unused_imports)]
pub(crate) use cadence::prelude::*;
use cadence::{NopMetricSink, StatsdClient, UdpMetricSink};
use log::error;
use log::info;
use std::fmt::Display;
use std::net::{ToSocketAddrs, UdpSocket};
use std::sync::Arc;

static STATSD_ROOT: &str = "geoproxy";

pub(crate) type MetricsClient = Arc<StatsdClient>;

pub(crate) fn setup_metrics(host: Option<impl ToSocketAddrs + Display>) -> Result<MetricsClient> {
    let client = if let Some(host) = host {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_nonblocking(true)?;

        info!("Configuring statsd output on {}", host);

        StatsdClient::builder(STATSD_ROOT, UdpMetricSink::from(host, socket)?)
    } else {
        info!("Statsd output disabled");

        StatsdClient::builder(STATSD_ROOT, NopMetricSink)
    }
    .with_error_handler(|error| error!("Metric error: {}", error))
    .build();

    Ok(Arc::new(client))
}
