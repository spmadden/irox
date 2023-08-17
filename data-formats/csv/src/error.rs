// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum CSVErrorType {
    IOError,
    MissingHeaderError,
    HeaderDataMismatchError,
    DuplicateKeyInHeaderError,
}

#[derive(Debug, Clone)]
pub struct CSVError {
    error_type: CSVErrorType,
    error: String
}

impl Display for CSVError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}: {}", self.error_type, self.error))
    }
}

impl std::error::Error for CSVError {

}

impl CSVError {
    pub fn new(error_type: CSVErrorType, error: String) -> CSVError {
        CSVError {
            error, error_type
        }
    }
    pub fn err<T>(error_type: CSVErrorType, error: String) -> Result<T, CSVError> {
        Err(Self::new(error_type, error))
    }
}

impl From<std::io::Error> for CSVError {
    fn from(value: std::io::Error) -> Self {
        CSVError::new(CSVErrorType::IOError, format!("{:?}", value))
    }
}

