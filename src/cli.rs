use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, Arg};
use std::net::ToSocketAddrs;

fn validate_sockaddr(value: String) -> Result<(), String> {
    value
        .to_socket_addrs()
        .map(|_| ())
        .map_err(|_| "invalid socket address".to_owned())
}

pub(super) fn setup_cli<'a, 'b>() -> App<'a, 'b> {
    const DEFAULT_SERVER_BIND: &str = "localhost:8000";
    const DEFAULT_CONFIG_NAME: &str = "config.json";

    app_from_crate!()
        .arg(
            Arg::with_name("address")
                .takes_value(true)
                .help("Address to bind to (e.g. 'localhost:8888')")
                .required(false)
                .default_value(DEFAULT_SERVER_BIND)
                .validator(validate_sockaddr)
                .short("a")
                .long("address"),
        )
        .arg(
            Arg::with_name("statsd")
                .takes_value(true)
                .help("Statsd address to send metrics to (e.g. 'localhost:8125')")
                .required(false)
                .validator(validate_sockaddr)
                .short("s")
                .long("statsd"),
        )
        .arg(
            Arg::with_name("config")
                .takes_value(true)
                .help("Location of the backend config file")
                .required(true)
                .default_value(DEFAULT_CONFIG_NAME)
                .short("c")
                .long("config"),
        )
}
