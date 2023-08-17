// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ErrorType {
    UrlParseError,
    RequestTransportError,
    RequestErrorCode(u16),
    IOError,
    CSVError, 
    MissingKeyError(String)
}

#[derive(Debug, Clone)]
pub struct Error {
    pub(crate) error_type: ErrorType,
    pub(crate) error: String,
}

impl Error {
    pub fn new(error_type: ErrorType, error: &'static str) -> Error {
        Error {
            error_type,
            error: String::from(error),
        }
    }

    pub fn err<T>(error_type: ErrorType, error: &'static str) -> Result<T, Error> {
        Err(Self::new(error_type, error))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} : {}", self.error_type, self.error))
    }
}

impl std::error::Error for Error {}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Error {
            error_type: ErrorType::UrlParseError,
            error: format!("{:?}", value),
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        let error = format!("{:?}", value);
        match value {
            ureq::Error::Status(code, _resp) => Error {
                error_type: ErrorType::RequestErrorCode(code),
                error,
            },
            ureq::Error::Transport(_resp) => Error {
                error_type: ErrorType::RequestTransportError,
                error,
            },
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            error_type: ErrorType::IOError,
            error: format!("{:?}", value),
        }
    }
}

impl From<irox_csv::error::CSVError> for Error {
    fn from(value: irox_csv::error::CSVError) -> Self {
        Error {
            error_type: ErrorType::CSVError,
            error: format!("{:?}", value)
        }
    }
}
