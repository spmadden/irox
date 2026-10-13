// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use irox_bits::BitsError;
use std::env::VarError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    SubprocessError,
    IOError,
    VarError,
    DBError,
}
#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
    pub kind: ErrorKind,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GISInterfaceError/{:?}: {}", self.kind, self.msg)
    }
}
impl std::error::Error for Error {}
impl Error {
    pub fn err<T>(kind: ErrorKind, msg: impl std::fmt::Display) -> Result<T, Error> {
        Err(Error {
            msg: msg.to_string(),
            kind,
        })
    }
    pub fn new(kind: ErrorKind, msg: impl std::fmt::Display) -> Self {
        Self {
            kind,
            msg: msg.to_string(),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            msg: format!("IOError: {value}"),
            kind: ErrorKind::IOError,
        }
    }
}
impl From<std::env::VarError> for Error {
    fn from(value: VarError) -> Self {
        Self {
            msg: format!("VarError: {value}"),
            kind: ErrorKind::VarError,
        }
    }
}
impl From<BitsError> for Error {
    fn from(value: BitsError) -> Self {
        Self {
            msg: format!("BitsError: {value}"),
            kind: ErrorKind::IOError,
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl From<native_tls::Error> for Error {
    fn from(value: native_tls::Error) -> Self {
        Self {
            msg: format!("tls error: {value}"),
            kind: ErrorKind::DBError,
        }
    }
}
impl From<tokio_postgres::Error> for Error {
    fn from(value: tokio_postgres::Error) -> Self {
        Self {
            msg: format!("DB error: {value:?}"),
            kind: ErrorKind::DBError,
        }
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self {
            msg: format!("Encoding error: {value}"),
            kind: ErrorKind::IOError,
        }
    }
}
