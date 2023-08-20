// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::io::Read;

use log::{debug, error};
use url::Url;

use error::{Error, ErrorType};
use irox_csv::Row;
use irox_networking::http::HttpProtocol;
use types::RetentionPolicy;

use crate::types::MeasurementDescriptor;

pub mod error;
pub mod types;

#[derive(Debug, Copy, Clone, Default)]
pub enum EncodingType {
    #[default]
    JSON,

    CSV,
}
impl EncodingType {
    pub const fn accept_header(&self) -> &'static str {
        match self {
            EncodingType::JSON => "application/json",
            EncodingType::CSV => "application/csv",
        }
    }
}

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

pub type OwnedReader = Box<dyn Read + Send + Sync + 'static>;

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

    pub fn query_json(
        &self,
        query: impl AsRef<str>,
        db: Option<String>,
    ) -> Result<OwnedReader, Error> {
        self.query(query, EncodingType::JSON, db)
    }

    pub fn query_csv(
        &self,
        query: impl AsRef<str>,
        db: Option<String>,
    ) -> Result<OwnedReader, Error> {
        self.query(query, EncodingType::CSV, db)
    }

    pub fn query_data(
        &self,
        query: impl AsRef<str>,
        encoding: EncodingType,
        db: Option<String>,
    ) -> Result<Vec<u8>, Error> {
        let mut reader = self.query(query, encoding, db)?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn query_string(
        &self,
        query: impl AsRef<str>,
        encoding: EncodingType,
        db: Option<String>,
    ) -> Result<String, Error> {
        let data = self.query_data(query, encoding, db)?;
        Ok(String::from_utf8_lossy(&data).to_string())
    }

    pub fn query(
        &self,
        query: impl AsRef<str>,
        encoding: EncodingType,
        db: Option<String>,
    ) -> Result<OwnedReader, Error> {
        let mut url = self.base_url.clone();
        url.set_path("query");
        if let Some(db) = db {
            url.set_query(Some(format!("db={db}").as_str()));
        }
        let resp = self
            .agent
            .request_url("POST", &url)
            .set("Accept", encoding.accept_header())
            .send_form(&[("q", query.as_ref())])?;

        let status = resp.status();
        if status != 200 {
            return Error::err(ErrorType::RequestErrorCode(status), "Query error");
        }
        Ok(resp.into_reader())
    }

    pub fn list_databases(&self) -> Result<Vec<String>, Error> {
        let res = self.query_csv("SHOW DATABASES", None)?;
        let mut out: Vec<String> = Vec::new();
        irox_csv::CSVMapReader::new(res)?.for_each(|row| {
            let row = row.as_map_lossy();
            if let Some(name) = row.get("name") {
                out.push(name.clone());
            }
        })?;
        Ok(out)
    }

    pub fn show_retention_policites(
        &self,
        db: Option<String>,
    ) -> Result<Vec<RetentionPolicy>, Error> {
        let res = match db {
            Some(db) => self.query_csv(format!("SHOW RETENTION POLICIES ON {}", db), None),
            None => self.query_csv("SHOW RETENTION POLICIES", None),
        }?;
        let mut out: Vec<RetentionPolicy> = Vec::new();
        irox_csv::CSVMapReader::new(res)?.for_each(|row| {
            match TryInto::<RetentionPolicy>::try_into(row.as_map_lossy()) {
                Ok(r) => out.push(r),
                Err(e) => error!("Error converting map into Retention: {e:?}"),
            };
        })?;

        Ok(out)
    }

    pub fn show_tag_keys(&self, db: Option<String>) -> Result<(), Error> {
        let res = match db {
            Some(db) => self.query_csv(format!("SHOW TAG KEYS ON {}", db), None),
            None => self.query_csv("SHOW TAG KEYS", None),
        }?;
        irox_csv::CSVMapReader::new(res)?.for_each(|row| {
            debug!("{:?}", row.as_map_lossy());
        })?;
        Ok(())
    }

    fn update_descriptor_map<
        T: FnOnce(&mut MeasurementDescriptor, &BTreeMap<String, String>) -> Result<(), Error>,
    >(
        data: &mut BTreeMap<String, MeasurementDescriptor>,
        row: Row,
        func: T,
    ) -> Result<(), Error> {
        let row_map = row.as_map_lossy();
        let Some(name) = row_map.get("name") else {
            return Error::err(ErrorType::MissingKeyError("name".to_string()), "Missing key name");
        };
        if !data.contains_key(name) {
            data.insert(
                name.to_string(),
                MeasurementDescriptor::new(name.to_string()),
            );
        }
        let Some(meas) = data.get_mut(name) else {
            return Error::err(ErrorType::NameKeyMismatch, "Missing name in map?");
        };
        func(meas, &row_map)
    }

    pub fn get_descriptors(&self, db: Option<String>) -> Result<Vec<MeasurementDescriptor>, Error> {
        let mut data: BTreeMap<String, MeasurementDescriptor> = BTreeMap::new();

        let res = match &db {
            Some(db) => self.query_csv(format!("SHOW TAG KEYS ON {db}"), None),
            None => self.query_csv("SHOW TAG KEYS", None),
        }?;
        let mut reader = irox_csv::CSVMapReader::new(res)?;
        while let Some(row) = reader.next_row()? {
            Self::update_descriptor_map(&mut data, row, |meas, row_map| {
                meas.merge_tag_key_map(row_map)
            })?;
        }

        let res = match &db {
            Some(db) => self.query_csv(format!("SHOW FIELD KEYS ON {db}"), None),
            None => self.query_csv("SHOW FIELD KEYS", None),
        }?;
        let mut reader = irox_csv::CSVMapReader::new(res)?;
        while let Some(row) = reader.next_row()? {
            Self::update_descriptor_map(&mut data, row, |meas, row_map| {
                meas.merge_field_key_map(row_map)
            })?;
        }

        Ok(data.into_values().collect())
    }
}
