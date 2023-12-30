// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::net::TcpStream;
use std::str::FromStr;

use crate::address::IPv4Address;
use crate::http::{HttpRequest, HttpResponse};

pub const DEFAULT_USER_AGENT: &str = "irox-networking/0.1.0";

pub struct Client;

impl Client {
    pub fn request(&mut self, req: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        let url = &req.url;
        let ip = IPv4Address::from_str(url.host()).unwrap();
        let port = url.port().unwrap();
        let mut stream = TcpStream::connect(ip.sockaddr(port))?;
        req.write_to(&mut stream)?;

        let result = HttpResponse::create_from(stream)?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use log::Level;

    use crate::http::{Client, HttpRequest};
    use crate::url::{UrlError, URL};

    #[test]
    pub fn test() -> Result<(), UrlError> {
        irox_log::init_console_level(Level::Debug);
        let req = HttpRequest::new(URL::from_str("http://10.169.0.27:8080/get")?);
        let resp = Client.request(req).expect("No errors!");
        println!("{resp:?}");
        Ok(())
    }
}
