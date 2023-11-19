// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_units::bounds::GreaterThanEqualToValueError;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, Copy, Clone)]
pub enum ErrorType {
    IOError,
    ParseInt,
    MissingValue,
    BadValue,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub error_type: ErrorType,
    pub error: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NMEA0183 Error: {:?}: {}", self.error_type, self.error)
    }
}

impl std::error::Error for Error {}

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

    pub fn missing_err<T>(error: &'static str) -> Result<T, Error> {
        Err(Self::missing(error))
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

impl From<GreaterThanEqualToValueError<u8>> for Error {
    fn from(value: GreaterThanEqualToValueError<u8>) -> Self {
        Error::new(ErrorType::BadValue, format!("{value}"))
    }
}
