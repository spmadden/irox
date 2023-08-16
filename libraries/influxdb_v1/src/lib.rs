// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use error::{Error, ErrorType};
use irox_networking::http::HttpProtocol;

pub mod error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConnectionParams {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) protocol: HttpProtocol,
}

impl Default for ConnectionParams {
    fn default() -> Self {
        ConnectionParams {
            host: String::from("localhost"),
            port: 8086,
            protocol: HttpProtocol::HTTP,
        }
    }
}

impl ConnectionParams {
    pub fn open(&self) -> Result<InfluxDB, error::Error> {
        let agent = ureq::AgentBuilder::new()
            .max_idle_connections(100)
            .max_idle_connections_per_host(200)
            .redirect_auth_headers(ureq::RedirectAuthHeaders::SameHost)
            .no_delay(true)
            .build();
        let base_url_str = format!("{}://{}:{}", self.protocol.name(), self.host, self.port);
        let base_url = url::Url::parse(&base_url_str)?;
        Ok(InfluxDB {
            agent, base_url_str, base_url
        })
    }
}

#[derive(Clone)]
pub struct InfluxDB {
    agent: ureq::Agent,
    base_url_str: String,
    base_url: url::Url
}

impl InfluxDB {
    pub fn open(params: &ConnectionParams) -> Result<InfluxDB, error::Error> {
        params.open()
    }

    pub fn open_default() -> Result<InfluxDB, error::Error> {
        ConnectionParams::default().open()
    }

    pub fn ping(&self) -> Result<(), error::Error>{
        let mut url = self.base_url.clone();
        url.set_path("ping");
        let req = self.agent.request_url("GET", &url);

        let resp = req.call()?;
        let status = resp.status();
        match status {
            200|204 => Ok(()),
            _ => Error::err(ErrorType::RequestErrorCode(status), "Bad Ping Response")
        }
    }
}
