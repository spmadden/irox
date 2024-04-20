// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Associated Error types

use std::fmt::{Display, Formatter};

///
/// Various different error types that can be returned from the APIs.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ErrorType {
    /// An IO Error has occurred
    IOError(std::io::ErrorKind),

    /// An error has occurred when building a concrete type.
    BuilderError(&'static str),

    /// The specified builder is not supported by the codec.
    BuilderNotSupported(&'static str),

    /// During a building operation, a required field was not set.  The field name is provided.
    BuilderMissingRequiredFieldError(&'static str),

    /// During a parsing operation, an error has occurred.
    ParserError(&'static str),

    /// During an encoding operation, an error has occurred.
    EncoderError(&'static str),

    /// Some other, possibly unspecified error has occurred.
    Other(&'static str),

    /// Fallback - a generic 'unknown' error has occurred.
    #[default]
    Unknown,
}

impl ErrorType {
    /// Construct a returnable [`Err(Error)`] for this particular type with a message.
    pub fn error_msg<T>(&self, message: String) -> Result<T, Error> {
        Err(Error {
            error_type: *self,
            message: Some(message),
            source: None,
        })
    }

    /// Construct a returnable [`Err(Error)`] for this particular type with no message.
    pub fn error<T>(&self) -> Result<T, Error> {
        Err(Error {
            error_type: *self,
            message: None,
            source: None,
        })
    }

    /// Construct a returnable [`Err(Error)`] for this particular type with a source error
    pub fn source<T>(&self, source: Box<dyn std::error::Error>) -> Result<T, Error> {
        Err(Error {
            error_type: *self,
            message: None,
            source: Some(source),
        })
    }

    /// Construct a returnable [`Err(Error)`] for this particular type with a source error and a
    /// message
    pub fn source_msg<T>(
        &self,
        message: String,
        source: Box<dyn std::error::Error>,
    ) -> Result<T, Error> {
        Err(Error {
            error_type: *self,
            message: Some(message),
            source: Some(source),
        })
    }
}

///
/// An error type with associated detailed message.
#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: Option<String>,
    pub source: Option<Box<dyn std::error::Error>>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(source) = &self.source {
            if let Some(msg) = &self.message {
                return write!(
                    f,
                    "EIEIO Error: {:?} {msg}, source: {}",
                    self.error_type, source
                );
            }
            return write!(f, "EIEIO Error: {:?}, source: {}", self.error_type, source);
        }
        if let Some(msg) = &self.message {
            write!(f, "EIEIO Error: {:?}: {msg}", self.error_type)
        } else {
            write!(f, "EIEIO Error: {:?}", self.error_type)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Some(e) = &self.source {
            return Some(e.as_ref());
        }
        None
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            error_type: ErrorType::IOError(value.kind()),
            message: None,
            source: Some(Box::new(value)),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Error {
            error_type: ErrorType::ParserError("UTF8 Error"),
            message: None,
            source: Some(Box::new(value)),
        }
    }
}

impl From<irox_bits::Error> for Error {
    fn from(value: irox_bits::Error) -> Self {
        Error {
            error_type: ErrorType::IOError(value.kind().into()),
            message: None,
            source: Some(Box::new(value)),
        }
    }
}
