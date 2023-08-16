// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use log::debug;
use url::Url;

use error::{Error, ErrorType};
use irox_networking::http::HttpProtocol;

pub mod error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InfluxDBConnectionParams {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) scheme: HttpProtocol,
}

impl Default for InfluxDBConnectionParams {
    fn default() -> Self {
        InfluxDBConnectionParams {
            host: String::from("localhost"),
            port: 8086,
            scheme: HttpProtocol::HTTP,
        }
    }
}

impl InfluxDBConnectionParams {
    pub fn open(&self) -> Result<InfluxDB, Error> {
        let base_url_str = format!("{}://{}:{}", self.scheme.name(), self.host, self.port);
        let base_url = Url::parse(&base_url_str)?;
        Self::open_url(base_url)
    }

    pub fn open_url(base_url_str: impl AsRef<str>) -> Result<InfluxDB, Error> {
        let base_url = Url::parse(base_url_str.as_ref())?;
        let agent = ureq::AgentBuilder::new()
            .max_idle_connections(100)
            .max_idle_connections_per_host(200)
            .redirect_auth_headers(ureq::RedirectAuthHeaders::SameHost)
            .no_delay(true)
            .build();
        Ok(InfluxDB { agent, base_url })
    }
}

#[derive(Default)]
pub struct InfluxConnectionBuilder {
    host: Option<String>,
    port: Option<u16>,
    scheme: Option<HttpProtocol>,
}

impl InfluxConnectionBuilder {
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    pub fn maybe_host(mut self, host: Option<String>) -> Self {
        self.host = host;
        self
    }

    pub fn with_port(mut self, port: impl Into<u16>) -> Self {
        self.port = Some(port.into());
        self
    }

    pub fn maybe_port(mut self, port: Option<u16>) -> Self {
        self.port = port;
        self
    }

    pub fn with_scheme(mut self, scheme: HttpProtocol) -> Self {
        self.scheme = Some(scheme);
        self
    }

    pub fn maybe_scheme(mut self, scheme: Option<HttpProtocol>) -> Self {
        self.scheme = scheme;
        self
    }

    pub fn build(self) -> Result<InfluxDB, Error> {
        let mut params = InfluxDBConnectionParams::default();
        if let Some(host) = self.host {
            params.host = host;
        }
        if let Some(port) = self.port {
            params.port = port;
        }
        if let Some(scheme) = self.scheme {
            params.scheme = scheme;
        }

        params.open()
    }
}

#[derive(Clone)]
pub struct InfluxDB {
    agent: ureq::Agent,
    base_url: Url,
}

impl InfluxDB {
    pub fn open(params: &InfluxDBConnectionParams) -> Result<InfluxDB, Error> {
        params.open()
    }

    pub fn open_default() -> Result<InfluxDB, Error> {
        InfluxDBConnectionParams::default().open()
    }

    pub fn ping(&self) -> Result<(), Error> {
        let mut url = self.base_url.clone();
        url.set_path("ping");
        let req = self.agent.request_url("GET", &url);

        let resp = req.call()?;
        let status = resp.status();
        match status {
            200 | 204 => Ok(()),
            _ => Error::err(ErrorType::RequestErrorCode(status), "Bad Ping Response"),
        }
    }

    pub fn query(&self, query: impl AsRef<str>) -> Result<String, Error> {
        let mut url = self.base_url.clone();
        url.set_path("query");
        let req = self
            .agent
            .request_url("POST", &url)
            .send_form(&[("q", query.as_ref())])?;

        let status = req.status();
        if status != 200 {
            return Error::err(ErrorType::RequestErrorCode(status), "Query error");
        }
        Ok(req.into_string()?)
    }

    pub fn query_csv(&self, query: impl AsRef<str>) -> Result<String, Error> {
        let mut url = self.base_url.clone();
        url.set_path("query");
        let req = self
            .agent
            .request_url("POST", &url)
            .set("Accept", "application/csv")
            .send_form(&[("q", query.as_ref())])?;

        let status = req.status();
        if status != 200 {
            return Error::err(ErrorType::RequestErrorCode(status), "Query error");
        }
        Ok(req.into_string()?)
    }

    pub fn list_databases(&self) -> Result<Vec<String>, Error> {
        let res = self.query_csv("SHOW DATABASES")?;
        debug!("{}", res);
        Ok(vec![])
    }
}
