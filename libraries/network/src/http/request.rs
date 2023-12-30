// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use crate::http::headers::HttpHeaders;
use crate::http::{HttpBody, HttpMethod, HttpVersion};
use crate::url::URL;
use log::debug;
use std::io::Write;

pub struct HttpRequest {
    pub(crate) url: URL,
    pub(crate) method: HttpMethod,
    pub(crate) version: HttpVersion,
    pub(crate) headers: HttpHeaders,
    pub(crate) body: HttpBody,
}

impl HttpRequest {
    pub fn new(url: URL) -> Self {
        Self {
            method: HttpMethod::Get,
            url,
            version: HttpVersion::Http1_1,
            headers: HttpHeaders::new_request(),
            body: HttpBody::Empty,
        }
    }

    pub fn set_method(&mut self, method: HttpMethod) {
        self.method = method
    }
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn set_url(&mut self, url: URL) {
        self.url = url;
    }
    pub fn url(&self) -> &URL {
        &self.url
    }

    pub fn write_to<T: Write>(mut self, mut out: &mut T) -> Result<(), std::io::Error> {
        if !self.headers.contains_header("Host") {
            let port = if let Some(port) = self.url.port {
                format!(":{port}")
            } else {
                String::new()
            };
            let host = format!("{}{port}", self.url.host);
            self.headers.maybe_add("Host", &host);
        }
        self.headers
            .maybe_add("User-Agent", crate::http::DEFAULT_USER_AGENT);
        match &self.body {
            HttpBody::Empty => {
                self.headers.maybe_add("Content-Length", "0");
            }
            HttpBody::String(s) => {
                self.headers
                    .maybe_add("Content-Length", format!("{}", s.len()));
            }
            HttpBody::Bytes(b) => {
                self.headers
                    .maybe_add("Content-Length", format!("{}", b.len()));
            }
            _ => {}
        }

        let hdr = format!(
            "{} {} {}",
            self.method,
            self.url.get_path_query_fragment(),
            self.version
        );
        debug!("{hdr}");
        write!(out, "{hdr}\r\n")?;
        self.headers.write_to(&mut out)?;

        self.body.write_to(&mut out)?;

        out.flush()?;

        Ok(())
    }
}
