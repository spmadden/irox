// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
    IOError,
    XMLError,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    error_type: ErrorType,
    msg: String,
}

impl Error {
    pub fn new(error_type: ErrorType, msg: String) -> Error {
        Error { error_type, msg }
    }
    pub fn ioe<T>(msg: &'static str) -> Result<T, Error> {
        Err(Error::new(ErrorType::IOError, msg.to_string()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "GPXError({:?}): {}",
            self.error_type, self.msg
        ))
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            error_type: ErrorType::IOError,
            msg: value.to_string(),
        }
    }
}

impl From<xml::writer::Error> for Error {
    fn from(value: xml::writer::Error) -> Self {
        Error {
            error_type: ErrorType::XMLError,
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
