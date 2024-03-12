// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ErrorType {
    IOError(std::io::Error)
}

#[derive(Debug)]
pub struct Error {
    error_type: ErrorType,
    message: String
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TarError({:?}): {}", self.error_type, self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn io_error<T>(err: std::io::Error, msg: &'static str) -> Result<T, Error> {
        Err(Error {
            error_type: ErrorType::IOError(err),
            message: msg.to_string()
        })
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            error_type: ErrorType::IOError(value),
            message: "IOError".to_string()
        }
    }
}
