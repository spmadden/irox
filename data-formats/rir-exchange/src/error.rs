// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_time::format::FormatError;
use std::fmt::{Display, Formatter};
use std::net::AddrParseError;
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
    MissingElement,
    IOError,
    Invalid,
    DatabaseError,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    error_type: ErrorType,
    msg: String,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}: {}", self.error_type, self.msg))
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn missing<T>(msg: &'static str) -> Result<T, Error> {
        Err(Error {
            error_type: ErrorType::MissingElement,
            msg: msg.to_string(),
        })
    }
    pub fn invalid<T>(msg: String) -> Result<T, Error> {
        Err(Error {
            error_type: ErrorType::Invalid,
            msg,
        })
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            error_type: ErrorType::IOError,
            msg: value.to_string(),
        }
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error {
            error_type: ErrorType::Invalid,
            msg: value.to_string(),
        }
    }
}
impl From<FormatError> for Error {
    fn from(value: FormatError) -> Self {
        Error {
            error_type: ErrorType::Invalid,
            msg: value.to_string(),
        }
    }
}
impl From<AddrParseError> for Error {
    fn from(value: AddrParseError) -> Self {
        Error {
            error_type: ErrorType::Invalid,
            msg: value.to_string(),
        }
    }
}
impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Error {
            error_type: ErrorType::DatabaseError,
            msg: value.to_string(),
        }
    }
}
