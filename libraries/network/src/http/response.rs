// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use crate::http::headers::HttpHeaders;
use crate::http::{HeaderResponse, HttpBody};
use irox_tools::options::MaybeMap;
use irox_tools::read::{ReadAny, ReadEmpty};
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::str::FromStr;

pub struct HttpResponse {
    pub(crate) version: String,
    pub(crate) code: String,
    pub(crate) status: String,
    pub(crate) headers: HttpHeaders,
    pub(crate) body: HttpBody,
}

impl HttpResponse {
    pub fn create_from<T: Read + 'static>(input: T) -> Result<HttpResponse, Error> {
        let mut bufread = BufReader::new(input);
        let mut line = String::new();
        let _read = bufread.read_line(&mut line)?;
        let mut status = line.split(" ");
        let version = status.next().unwrap_or_default().to_string();
        let code = status.next().unwrap_or_default().to_string();
        let status = status.collect::<Vec<&str>>().join(" ");

        let HeaderResponse {
            headers,
            mut remaining,
        } = HttpHeaders::create_from(bufread)?;

        let content_length = headers
            .get_header("Content-Length")
            .maybe_map(|v| u64::from_str(v).ok())
            .unwrap_or(0);
        let body = if content_length == 0 {
            HttpBody::Empty
        } else if content_length < 1_000_000 {
            let mut buf: Vec<u8> = Vec::with_capacity(1_000_000);
            remaining.read_to_end(&mut buf)?;
            HttpBody::Bytes(buf)
        } else {
            HttpBody::Read(Box::new(remaining))
        };

        Ok(HttpResponse {
            version,
            code,
            status,
            headers,
            body,
        })
    }
    pub fn body_read(self) -> Box<dyn Read> {
        match self.body {
            HttpBody::Read(r) => r,
            HttpBody::Empty => Box::<ReadEmpty>::default(),
            HttpBody::String(s) => Box::new(ReadAny::from(s)),
            HttpBody::Bytes(b) => Box::new(ReadAny::from(b)),
        }
    }
}

impl Debug for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut val: Vec<u8> = Vec::with_capacity(4096);
        let _ = write!(val, "{} {} {}\r\n", self.version, self.code, self.status);
        let _ = self.headers.write_to(&mut val);
        match &self.body {
            HttpBody::String(s) => {
                let _ = write!(val, "{s}");
            }
            HttpBody::Bytes(b) => val.extend_from_slice(b),
            _ => {}
        }
        write!(f, "{}", String::from_utf8_lossy(&val))?;
        Ok(())
    }
}
