// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_bits::BitsError;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
    IOError,
    InvalidData,
    StructError,
    UnimplementedMessage,
}
#[derive(Debug, Clone)]
pub struct Error {
    error_type: ErrorType,
    error: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Error({:?}): {}", self.error_type, self.error))
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn kind(&self) -> ErrorType {
        self.error_type
    }
    pub(crate) fn new(error_type: ErrorType, error: &'static str) -> Error {
        Error::new_str(error_type, String::from(error))
    }
    pub(crate) fn new_str(error_type: ErrorType, error: String) -> Error {
        Error { error, error_type }
    }
    pub(crate) fn unsupported<T>(msg: &'static str) -> Result<T, Error> {
        Err(Error::new(ErrorType::UnimplementedMessage, msg))
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::new_str(ErrorType::IOError, format!("{value:?}"))
    }
}

impl From<BitsError> for Error {
    fn from(value: BitsError) -> Self {
        Error::new_str(ErrorType::IOError, format!("{value:?}"))
    }
}

impl From<std::io::ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Error::new_str(ErrorType::IOError, format!("{value:?}"))
    }
}
impl From<ErrorType> for Error {
    fn from(value: ErrorType) -> Self {
        Error::new_str(value, format!("{value:?}"))
    }
}
