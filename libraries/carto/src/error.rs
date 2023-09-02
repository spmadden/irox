// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::error::Error;
use std::fmt::{Display, Formatter};

use irox_enums::EnumName;

#[derive(Debug, Clone, EnumName)]
pub enum ConvertError {
    MissingValue(String),
}

impl ConvertError {
    fn error(&self) -> &String {
        match self {
            ConvertError::MissingValue(e) => e,
        }
    }
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ConvertError({}): {}",
            self.name(),
            self.error()
        ))
    }
}

impl Error for ConvertError {}
