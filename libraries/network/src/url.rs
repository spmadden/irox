// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Scheme {
    Http,
    Https,
    Ws,
    Wss,
    Other(String)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct URL {
    pub(crate) scheme: String,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) host: String,
    pub(crate) port: Option<u16>,
    pub(crate) path: Option<String>,
    pub(crate) query: Vec<String>,
    pub(crate) fragment: Option<String>,
}

