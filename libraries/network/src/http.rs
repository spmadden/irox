// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use serde::{Deserialize, Serialize};

///
/// Basic enumerated type to pick the HTTP protocol & port
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum HttpProtocol {
    HTTP,

    #[default]
    HTTPS,
}

impl HttpProtocol {
    pub const fn port(&self) -> u16 {
        match self {
            HttpProtocol::HTTP => 80,
            HttpProtocol::HTTPS => 443,
        }
    }
}
