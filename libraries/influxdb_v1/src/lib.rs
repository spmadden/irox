// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

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
        todo!()
    }
}

pub struct InfluxDB {}

impl InfluxDB {
    pub fn open(params: &ConnectionParams) -> Result<InfluxDB, error::Error> {
        params.open()
    }

    pub fn open_default() -> Result<InfluxDB, error::Error> {
        ConnectionParams::default().open()
    }
}
