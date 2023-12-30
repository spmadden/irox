// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::net::TcpStream;
use crate::address::IPAddress;
use crate::http::HttpMethod;
use crate::url::URL;

pub struct Client {
    pub(crate) remote: IPAddress,
}

impl Client {
    pub fn request(&mut self, method: HttpMethod, url: URL) {

    }
}