use crate::error::*;
use failure::format_err;
use geo_types::Polygon;
use http::uri::Uri;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fmt::{self, Display};
use std::fs::File;
use std::path::Path;
use url::Url;
use url_serde;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Backend {
    #[serde(with = "url_serde")]
    base_url: Url,
}

impl Backend {
    pub(crate) fn map_url(&self, uri: &Uri) -> Uri {
        let mut new = self.base_url.clone();

        // clear preexisting settings
        new.set_path(uri.path());
        new.set_query(uri.query());

        new.as_str().parse().unwrap()
    }

    fn validate(&self) -> Result<()> {
        if self.base_url.cannot_be_a_base()
            || (self.base_url.scheme() != "http" && self.base_url.scheme() != "https")
        {
            Err(format_err!(
                "Backend URL scheme has to be http(s), {}",
                self.base_url
            ))
        } else if !self.base_url.has_host() {
            Err(format_err!(
                "Backend URL needs to have a host specified, {}",
                self.base_url
            ))
        } else {
            Ok(())
        }
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.base_url)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BackendDefinition {
    pub(crate) areas: Vec<Polygon<f32>>,
    pub(crate) backend: Backend,
}

impl BackendDefinition {
    fn validate(&self) -> Result<()> {
        if self.areas.is_empty() {
            Err(format_err!(
                "Backend definition has to have at least one area declared: {:?}",
                self.backend
            ))
        } else {
            self.backend.validate()
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ProxyConfig {
    pub(crate) backends: Vec<BackendDefinition>,
    pub(crate) default_backend: Backend,
}

impl ProxyConfig {
    fn validate(&self) -> Result<()> {
        self.backends
            .iter()
            .map(|backend| backend.validate())
            .collect::<Result<()>>()
    }
}

pub(crate) fn read_config(source: impl AsRef<Path>) -> Result<ProxyConfig> {
    let file = File::open(source)?;
    let config: ProxyConfig = serde_json::from_reader(file)?;
    config.validate()?;

    Ok(config)
}
