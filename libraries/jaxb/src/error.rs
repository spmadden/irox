// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ErrorType {
    XMLError,
    IOError,
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
    pub fn new(error_type: ErrorType, error: String) -> Error {
        Error { error_type, error }
    }

    pub fn io_err<T>(msg: &'static str) -> Result<T, Error> {
        Err(Error::new(ErrorType::IOError, msg.to_string()))
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::new(ErrorType::IOError, value.to_string())
    }
}
