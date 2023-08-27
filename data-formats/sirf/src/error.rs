// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

use irox_structs::StructError;

#[derive(Debug, Copy, Clone)]
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
    pub(crate) fn new(error_type: ErrorType, error: &'static str) -> Error {
        Error::new_str(error_type, String::from(error))
    }
    pub(crate) fn new_str(error_type: ErrorType, error: String) -> Error {
        Error { error, error_type }
    }
    pub(crate) fn invalid_data<T>(msg: &'static str) -> Result<T, Error> {
        Err(Error::new(ErrorType::InvalidData, msg))
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

impl From<irox_structs::StructError> for Error {
    fn from(value: StructError) -> Self {
        Error::new_str(ErrorType::StructError, format!("{value:?}"))
    }
}
