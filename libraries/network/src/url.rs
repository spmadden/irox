// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum UrlError {
    MissingScheme,
    MissingAuthorityDelimiter,
    PortNotU16(String),
    MissingAuthorityHost,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Scheme {
    Http,
    Https,
    Ws,
    Wss,
    Other(String),
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct URL {
    pub scheme: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub query_parts: BTreeMap<String, String>,
    pub fragment: Option<String>,
}

#[derive(Default)]
pub struct Authority {
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: String,
    pub port: Option<u16>,
}

impl Authority {
    fn try_from(value: &str) -> Result<(Authority, &str), UrlError> {
        if !value.starts_with("//") {
            return Err(UrlError::MissingAuthorityDelimiter);
        };
        let (_, value) = value.split_at(2);
        let split = value.find(['/', '?', '#']).unwrap_or(value.len());
        let (authority, remaining) = value.split_at(split);
        let mut out = Authority::default();
        let hostport = if let Some(at_idx) = authority.find('@') {
            let (userinfo, hostport) = authority.split_at(at_idx);
            let (_, hostport) = hostport.split_at(1);
            if let Some(col_idx) = userinfo.find(':') {
                let (user, pass) = userinfo.split_at(col_idx);
                let (_, pass) = pass.split_at(1);
                out.username = Some(user.to_string());
                out.password = Some(pass.to_string());
            } else {
                out.username = Some(userinfo.to_string());
            }
            hostport
        } else {
            authority
        };
        if let Some(col_idx) = hostport.find(':') {
            let (host, port) = hostport.split_at(col_idx);
            out.host = host.to_string();
            let (_, port) = port.split_at(1);
            let Ok(port) = u16::from_str(port) else {
                return Err(UrlError::PortNotU16(port.to_string()));
            };
            out.port = Some(port);
        } else {
            out.host = hostport.to_string();
        }
        if out.host.is_empty() {
            return Err(UrlError::MissingAuthorityHost);
        }
        Ok((out, remaining))
    }
}

impl FromStr for URL {
    type Err = UrlError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some(scheme_idx) = value.find(':') else {
            return Err(UrlError::MissingScheme);
        };
        let mut out = URL::default();
        let (scheme, rest) = value.split_at(scheme_idx);
        out.scheme = scheme.to_string();
        match scheme {
            "http" | "ws" => out.port = Some(80),
            "https" | "wss" => out.port = Some(443),
            _ => {}
        };
        let (_, rest) = rest.split_at(1);
        if !rest.starts_with("//") {
            return Err(UrlError::MissingAuthorityHost);
        }
        let (authority, rest) = Authority::try_from(rest)?;
        out.username = authority.username;
        out.password = authority.password;
        out.host = authority.host;
        out.port = authority.port;
        if let Some(next_idx) = rest.find(['?', '#']) {
            let (path, rest) = rest.split_at(next_idx);
            out.path = Some(path.to_string());

            let query = if let Some(next_idx) = rest.find('#') {
                let (query, frag) = rest.split_at(next_idx);
                let (_, frag) = frag.split_at(1);
                out.fragment = Some(frag.to_string());
                query
            } else {
                rest
            };
            if !query.is_empty() {
                let (_, query) = query.split_at(1);
                for item in query.split('&') {
                    let (key, val) = item.split_at(item.find('=').unwrap_or(item.len()));
                    out.query_parts.insert(key.to_string(), val.to_string());
                }
            }
        } else if !rest.is_empty() {
            out.path = Some(rest.to_string());
        };

        if let Some(pth) = &out.path {
            if pth.is_empty() {
                out.path = Some("/".to_string());
            }
        } else if out.path.is_none() {
            out.path = Some("/".to_string());
        }

        Ok(out)
    }
}

impl TryFrom<String> for URL {
    type Error = UrlError;

    fn try_from(value: String) -> Result<URL, Self::Error> {
        FromStr::from_str(&value)
    }
}

impl URL {
    pub fn scheme(&self) -> &str {
        &self.scheme
    }
    pub fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }
    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn port(&self) -> Option<u16> {
        self.port
    }
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }
    pub fn query_parts(&self) -> &BTreeMap<String, String> {
        &self.query_parts
    }
    pub fn fragment(&self) -> Option<&String> {
        self.fragment.as_ref()
    }

    pub fn get_path_query_fragment(&self) -> String {
        let mut out = String::new();
        if let Some(path) = self.path() {
            if !path.starts_with('/') {
                out.push('/');
            }
            out.push_str(path);
        } else {
            out.push('/');
        }
        let query = self.query_parts();
        if !query.is_empty() {
            out.push('?');
            out.push_str(
                &query
                    .iter()
                    .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
                    .collect::<Vec<String>>()
                    .join("&"),
            );
        }
        if let Some(fragment) = self.fragment() {
            out.push('#');
            out.push_str(&url_encode(fragment));
        }
        out
    }
}

#[derive(Clone)]
pub struct URLBuilder {
    url: URL,
}
impl URLBuilder {
    pub fn new(scheme: &str, host: &str) -> URLBuilder {
        URLBuilder {
            url: URL {
                scheme: scheme.to_string(),
                username: None,
                password: None,
                host: host.to_string(),
                port: None,
                path: None,
                query_parts: Default::default(),
                fragment: None,
            },
        }
    }

    pub fn with_username(&mut self, username: &str) -> &mut Self {
        self.url.username = Some(username.to_string());
        self
    }
    pub fn with_password(&mut self, password: &str) -> &mut Self {
        self.url.password = Some(password.to_string());
        self
    }
    pub fn with_port(&mut self, port: u16) -> &mut Self {
        self.url.port = Some(port);
        self
    }
    pub fn with_path(&mut self, path: &str) -> &mut Self {
        self.url.path = Some(path.to_string());
        self
    }
    pub fn add_query(&mut self, key: &str, val: &str) -> &mut Self {
        self.url
            .query_parts
            .insert(key.to_string(), val.to_string());
        self
    }
    pub fn with_fragment(&mut self, frag: &str) -> &mut Self {
        self.url.fragment = Some(frag.to_string());
        self
    }
    pub fn build(self) -> URL {
        self.url
    }
}

macro_rules! url {
    ($scheme:literal, $host:literal) => {{
        crate::url::URLBuilder::new($scheme, $host).build()
    }};
    ($scheme:literal, $host:literal, $path:literal) => {{
        let mut tmp = crate::url::URLBuilder::new($scheme, $host);
        tmp.with_path($path);
        tmp.build()
    }};
    ($scheme:literal, $host:literal, $path:literal, $frag:literal) => {{
        let mut tmp = crate::url::URLBuilder::new($scheme, $host);
        tmp.with_path($path);
        tmp.with_fragment($frag);
        tmp.build()
    }};
    ($scheme:literal, $host:literal, $path:literal,{$($qk:literal,$qv:literal)+}) => {{
        let mut tmp = crate::url::URLBuilder::new($scheme, $host);
        tmp.with_path($path);
        $(
            tmp.add_query($qk,$qv);
        )+
        tmp.build()
    }};
    ($scheme:literal, $host:literal, $path:literal, $frag:literal, {$($qk:literal,$qv:literal)+}) => {{
        let mut tmp = crate::url::URLBuilder::new($scheme, $host);
        tmp.with_path($path);
        tmp.with_fragment($frag);
        $(
            tmp.add_query($qk,$qv);
        )+
        tmp.build()
    }};
}

pub fn url_encode<T: AsRef<str>>(input: T) -> String {
    let input = input.as_ref();
    let mut out = String::with_capacity(input.len());

    for ch in input.chars() {
        let add = match ch {
            ' ' => "%20",
            '!' => "%21",
            '\"' => "%22",
            '#' => "%23",
            '$' => "%24",
            '%' => "%25",
            '&' => "%26",
            '\'' => "%27",
            '(' => "%28",
            ')' => "%29",
            '*' => "%2A",
            '+' => "%2B",
            ',' => "%2C",
            '/' => "%2F",
            ':' => "%3A",
            ';' => "%3B",
            '=' => "%3D",
            '?' => "%3F",
            '@' => "%40",
            '[' => "%5B",
            ']' => "%5D",
            v => {
                out.push(v);
                ""
            }
        };
        out.push_str(add);
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::url::{UrlError, URL};
    use std::str::FromStr;

    #[allow(clippy::panic_in_result_fn)]
    #[test]
    pub fn test() -> Result<(), UrlError> {
        let url = "https://user:password@host:80/path?query#seg";
        let url: URL = URL::from_str(url)?;

        assert_eq!("https", url.scheme);
        assert_eq!(Some("user".to_string()), url.username);
        assert_eq!(Some("password".to_string()), url.password);
        assert_eq!("host".to_string(), url.host);
        assert_eq!(Some(80), url.port);
        assert_eq!(Some("/path".to_string()), url.path);
        assert_eq!(Some("seg".to_string()), url.fragment);

        Ok(())
    }

    #[allow(clippy::panic_in_result_fn)]
    #[test]
    pub fn tests() -> Result<(), UrlError> {
        let mut tests: Vec<(URL, &str)> = Vec::new();
        tests.push((url!("http", "a", "/b/c/g"), "http://a/b/c/g"));
        tests.push((url!("http", "a", "/b/c/g/"), "http://a/b/c/g/"));
        tests.push((
            url!("http", "a", "/b/c/g;p", {"y",""}),
            "http://a/b/c/g;p?y",
        ));
        tests.push((url!("http", "a", "/b/c/g", {"y",""}), "http://a/b/c/g?y"));
        tests.push((
            url!("http", "a", "/b/c/d;p","s", {"q",""}),
            "http://a/b/c/d;p?q#s",
        ));
        tests.push((
            url!("http", "a", "/b/c/g", "s/../x"),
            "http://a/b/c/g#s/../x",
        ));

        for (url, chk) in tests {
            let chk: URL = URL::from_str(chk)?;
            assert_eq!(url, chk);
        }

        Ok(())
    }
}
