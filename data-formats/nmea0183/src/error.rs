// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::num::{ParseFloatError, ParseIntError};

use time::error::ComponentRange;

#[derive(Debug, Copy, Clone)]
pub enum ErrorType {
    IOError,
    ParseInt,
    MissingValue,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub error_type: ErrorType,
    pub error: String,
}

impl Error {
    pub fn new(error_type: ErrorType, error: String) -> Error {
        Error { error_type, error }
    }
    pub fn new_str(error_type: ErrorType, error: &'static str) -> Error {
        Error {
            error_type,
            error: String::from(error),
        }
    }
    pub fn missing(error: &'static str) -> Error {
        Error::new_str(ErrorType::MissingValue, error)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::new(ErrorType::IOError, format!("{value:?}"))
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::new(ErrorType::ParseInt, format!("{value:?}"))
    }
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Error::new(ErrorType::ParseInt, format!("{value:?}"))
    }
}

impl From<ComponentRange> for Error {
    fn from(value: ComponentRange) -> Self {
        Error::new(ErrorType::ParseInt, format!("{value:?}"))
    }
}
