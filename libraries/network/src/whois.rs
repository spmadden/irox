// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use core::str::FromStr;
use irox_bits::{Bits, BitsError};
use std::net::TcpStream;

pub const DEFAULT_SERVER: &str = "whois.iana.org";
pub const DEFAULT_PORT: u16 = 43;

pub struct WhoisOptions {
    pub server: String,
    pub port: u16,
}
impl Default for WhoisOptions {
    fn default() -> Self {
        WhoisOptions {
            server: DEFAULT_SERVER.to_string(),
            port: DEFAULT_PORT,
        }
    }
}
impl WhoisOptions {
    pub fn query<T: AsRef<str>>(self, domain: T) -> Result<Vec<WhoisResult>, BitsError> {
        Whois::query(self.server, self.port, domain)
    }
    pub fn with_server<T: AsRef<str>>(&mut self, server: T) -> &mut Self {
        self.server = server.as_ref().to_string();
        self
    }
    pub fn with_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
}
pub struct Whois;

impl Whois {
    pub fn query<T: AsRef<str>, D: AsRef<str>>(
        server: T,
        port: u16,
        domain: D,
    ) -> Result<Vec<WhoisResult>, BitsError> {
        use std::io::Write;
        let mut tcp = TcpStream::connect(format!("{}:{port}", server.as_ref()))?;
        write!(tcp, "{}\r\n", domain.as_ref())?;
        let mut out = Vec::new();
        let mut res = WhoisResult::default();
        res.try_from(&mut tcp)?;
        // close the connection before trying the next one
        drop(tcp);

        if let Some(refer) = &res.refer {
            let mut opts = WhoisOptions::default();
            if let Some((server, port)) = refer.split_once(':') {
                opts.with_server(server);
                if let Ok(port) = u16::from_str(port) {
                    opts.with_port(port);
                }
            } else {
                opts.with_server(refer);
            }
            out.append(&mut opts.query(domain.as_ref())?);
        }
        out.insert(0, res);
        Ok(out)
    }
}

#[derive(Default, Clone, Debug)]
pub struct WhoisResult {
    pub refer: Option<String>,
    pub domain: Option<String>,
    pub organisation: Option<WhoisContact>,
    pub contact: Vec<WhoisContact>,
    pub name_servers: Vec<WhoisNameServer>,
    pub ds_rdata: Option<String>,
    pub whois: Option<String>,
    pub status: Option<String>,
    pub remarks: Vec<String>,
    pub created: Option<String>,
    pub changed: Option<String>,
    pub source: Option<String>,
    pub other_fields: Vec<(String, String)>,
}
fn parse_line(line: &str) -> Option<(&str, &str)> {
    log::debug!("{line}");
    if line.starts_with('%') {
        // comment line.
        return None;
    }
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    let (key, value) = line.split_once(':')?;
    let key = key.trim();
    let value = value.trim();
    Some((key, value))
}

impl WhoisResult {
    fn update_data<T: Bits>(
        &mut self,
        input: &mut T,
        key: &str,
        value: &str,
    ) -> Result<bool, BitsError> {
        let value = value.to_string();
        if key.starts_with("refer") {
            self.refer = Some(value);
        } else if key.eq("domain") || key.eq("domain name") {
            self.domain = Some(value);
        } else if key.starts_with("status") {
            self.status = Some(value);
        } else if key.starts_with("created") || key.starts_with("creation") {
            self.created = Some(value);
        } else if key.starts_with("remarks") {
            self.remarks.push(value);
        } else if key.starts_with("changed") || key.starts_with("updated") {
            self.changed = Some(value);
        } else if key.starts_with("source") {
            self.source = Some(value);
        } else if key.starts_with("ds-rdata") || key.starts_with("dnssec") {
            self.ds_rdata = Some(value);
        } else if key.starts_with("whois") || key.contains("whois server") {
            self.whois = Some(value);
        } else if key.eq("name server") || key.eq("nserver") {
            self.name_servers.push(value.into());
        } else if key.starts_with("contact") {
            let mut con = WhoisContact {
                contact: Some(value),
                ..Default::default()
            };
            con.try_from(input)?;
            self.contact.push(con);
        } else if key.starts_with("organisation") {
            let mut org = WhoisContact {
                organisation: Some(value),
                ..Default::default()
            };
            org.try_from(input)?;
            self.organisation = Some(org);
        // } else if key.starts_with("registrant ") || key.starts_with("admin ") || key.starts_with("tech ")  {
        //     let mut con = WhoisContact {
        //         contact: Some(value),
        //         ..Default::default()
        //     };
        //     if let Some((key, value)) = con.try_from(input)? {
        //         self.update_data(input, &key, &value)?;
        //     }
        //     self.contact.push(con);
        } else {
            return Ok(false);
        }
        Ok(true)
    }
    pub fn try_from<T: Bits>(&mut self, input: &mut T) -> Result<(), BitsError> {
        while let Some(line) = input.read_line_str_lossy()? {
            let Some((orig_key, value)) = parse_line(&line) else {
                continue;
            };
            let value = value.to_string();
            let key = orig_key.to_lowercase();
            if !self.update_data(input, &key, &value)? {
                self.other_fields.push((orig_key.to_string(), value));
            }
        }
        Ok(())
    }
}

#[derive(Default, Clone, Debug)]
pub struct WhoisContact {
    pub contact: Option<String>,
    pub name: Option<String>,
    pub organisation: Option<String>,
    pub address: Vec<String>,
    pub phone: Option<String>,
    pub phone_ext: Option<String>,
    pub fax: Option<String>,
    pub fax_ext: Option<String>,
    pub email: Option<String>,
}

impl WhoisContact {
    pub fn try_from<T: Bits>(
        &mut self,
        input: &mut T,
    ) -> Result<Option<(String, String)>, BitsError> {
        let mut out = None;
        while let Some(line) = input.read_line_str_lossy()? {
            let Some((key, value)) = parse_line(&line) else {
                break;
            };

            let value = value.to_string();
            if key.eq("contact") {
                self.contact = Some(value);
            } else if key.contains("name") {
                self.name = Some(value);
            } else if key.contains("organisation") || key.contains("organization") {
                self.organisation = Some(value);
            } else if key.eq("address") {
                self.address.push(value);
            } else if key.eq("phone") {
                self.phone = Some(value);
            } else if key.eq("fax-no") {
                self.fax = Some(value);
            } else if key.eq("e-mail") || key.contains("email") {
                self.email = Some(value);
            } else {
                out = Some((key.to_string(), value));
                break;
            }
        }
        Ok(out)
    }
}

#[derive(Default, Clone, Debug)]
pub struct WhoisNameServer {
    pub dns: Option<String>,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}

impl From<String> for WhoisNameServer {
    fn from(value: String) -> Self {
        let mut out = WhoisNameServer::default();
        let mut splits = value.split(' ');
        let Some(dns) = splits.next() else {
            return out;
        };
        out.dns = Some(dns.to_string());
        let Some(ipv4) = splits.next() else {
            return out;
        };
        out.ipv4 = Some(ipv4.to_string());
        let Some(ipv6) = splits.next() else {
            return out;
        };
        out.ipv6 = Some(ipv6.to_string());

        out
    }
}
