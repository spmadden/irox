// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_tools::options::MaybeMap;
use irox_tools::read::Buffer;
use irox_tools::scanner::{LineEnding, ReadToken, Scanner};
use log::debug;
use std::collections::BTreeMap;
use std::io::{BufReader, Read, Write};

#[derive(Default, Debug, Clone)]
pub struct HttpHeaders {
    pub(crate) headers: BTreeMap<String, Vec<String>>,
}

pub struct HeaderResponse<T> {
    pub headers: HttpHeaders,
    pub remaining: Buffer<BufReader<T>>,
}

impl HttpHeaders {
    pub fn new_empty() -> Self {
        HttpHeaders::default()
    }
    pub fn new_request() -> Self {
        let mut resp = HttpHeaders::new_empty();
        resp.add_header("Accept", "*/*");
        resp.add_header("Connection", "close");
        resp
    }
    pub fn add_header<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        let key = key.as_ref().to_string();
        let val = value.as_ref().to_string();
        self.headers.entry(key).or_default().push(val);
    }

    pub fn get_header<K: AsRef<str>>(&self, key: K) -> Option<&String> {
        let key = key.as_ref().to_string();
        self.headers.get(&key).maybe_map(|v| v.get(0))
    }

    pub fn get_headers<K: AsRef<str>>(&self, key: K) -> Option<&Vec<String>> {
        let key = key.as_ref().to_string();
        self.headers.get(&key)
    }
    pub fn headers(&self) -> &BTreeMap<String, Vec<String>> {
        &self.headers
    }
    pub fn contains_header<K: AsRef<str>>(&self, key: K) -> bool {
        self.headers.contains_key(key.as_ref())
    }
    pub fn maybe_add<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, val: V) {
        let key = key.as_ref();
        if !self.contains_header(key) {
            self.add_header(key, val);
        }
    }

    pub fn write_to<T: Write>(&self, out: &mut T) -> Result<(), std::io::Error> {
        for (name, value) in &self.headers {
            let vals = value.join(" ");
            let hdr = format!("{name}: {vals}");
            debug!("{hdr}");
            write!(out, "{hdr}\r\n")?;
        }
        debug!("");
        write!(out, "\r\n")?;
        Ok(())
    }

    pub fn add_line(&mut self, val: &[u8]) {
        let val = String::from_utf8_lossy(val);
        if let Some(idx) = val.find(':') {
            let (name, val) = val.split_at(idx);
            let val = val.get(2..).unwrap_or_default();
            self.add_header(name, val);
        }
    }

    pub fn create_from<T: Read + Sized>(input: T) -> Result<HeaderResponse<T>, std::io::Error> {
        let mut headers = HttpHeaders::new_empty();
        let mut scanner = Scanner::new_crlf(input);
        loop {
            let next = scanner.read_next()?;
            match next {
                ReadToken::Found { data, token } => {
                    if *token.get_response() != LineEnding::CarriageReturnLineFeed {
                        return Err(std::io::ErrorKind::InvalidData.into());
                    }
                    if data.is_empty() {
                        break;
                    }
                    headers.add_line(&data);
                }
                ReadToken::EndOfData { data } => {
                    headers.add_line(&data);
                }
                ReadToken::NotFound => {}
            }
        }
        Ok(HeaderResponse {
            headers,
            remaining: scanner.take_back(),
        })
    }
}
