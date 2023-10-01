// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use miniz_oxide::inflate::DecompressError;

use irox_carto::error::ConvertError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorType {
    SQLError,
    IOError,
    DecodingError,
    XMLError,
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
    pub fn new(error_type: ErrorType, msg: String) -> Error {
        Error { error_type, msg }
    }

    pub fn xml_error<T>(msg: &'static str) -> Result<T, Error> {
        Err(Error::new(ErrorType::XMLError, msg.to_string()))
    }

    pub fn decoding_err<T>(msg: &'static str) -> Result<T, Error> {
        Err(Error::new(ErrorType::DecodingError, msg.to_string()))
    }

    pub fn decoding_str<T>(msg: String) -> Result<T, Error> {
        Err(Error::new(ErrorType::DecodingError, msg))
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        let str = value.to_string();
        Error {
            error_type: ErrorType::SQLError,
            msg: str,
        }
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

impl From<DecompressError> for Error {
    fn from(value: DecompressError) -> Self {
        Error {
            error_type: ErrorType::DecodingError,
            msg: value.to_string(),
        }
    }
}

impl From<xml::reader::Error> for Error {
    fn from(value: xml::reader::Error) -> Self {
        Error {
            error_type: ErrorType::XMLError,
            msg: value.to_string(),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error {
            error_type: ErrorType::DecodingError,
            msg: value.to_string(),
        }
    }
}

impl From<ConvertError> for Error {
    fn from(value: ConvertError) -> Self {
        Error {
            error_type: ErrorType::DecodingError,
            msg: value.to_string(),
        }
    }
}
