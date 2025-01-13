// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Error Types

use std::error::Error;
use std::fmt::{Display, Formatter};

use irox_enums::EnumName;

#[derive(Debug, Clone, EnumName)]
pub enum ConvertError {
    MissingValue(String),
    MissingProjection(String),
    MismatchedReferenceFrame(String),
}

impl ConvertError {
    pub fn error(&self) -> &str {
        match self {
            ConvertError::MissingValue(e) => e,
            ConvertError::MissingProjection(a) => a,
            ConvertError::MismatchedReferenceFrame(r) => r,
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

#[derive(Debug, Clone)]
pub enum ParseError {
    Error,
}
