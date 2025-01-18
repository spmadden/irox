// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Error Types

extern crate alloc;
use alloc::string::String;
use core::error::Error;
use core::fmt::{Display, Formatter};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
