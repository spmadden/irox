// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use irox_bits::BitsErrorKind;
use std::env::VarError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    SubprocessError,
    VarError,
    Bits(BitsErrorKind),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub msg: String,
    pub kind: ErrorKind,
}

impl From<irox_bits::Error> for Error {
    fn from(value: irox_bits::Error) -> Self {
        Self {
            msg: value.msg().to_string(),
            kind: ErrorKind::Bits(value.kind()),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        irox_bits::BitsError::from(value).into()
    }
}
impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        Self {
            msg: value.to_string(),
            kind: ErrorKind::VarError,
        }
    }
}
