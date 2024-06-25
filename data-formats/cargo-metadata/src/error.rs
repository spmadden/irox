// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use irox_bits::BitsError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Json(serde_json::Error),
    Bits(irox_bits::BitsError),
    Toml(toml::de::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Json(value)
    }
}
impl From<irox_bits::BitsError> for Error {
    fn from(value: BitsError) -> Self {
        Error::Bits(value)
    }
}
impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::Toml(value)
    }
}
