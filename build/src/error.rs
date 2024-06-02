// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

#[derive(Debug, Clone)]
pub enum ErrorKind {
    SubprocessError,
    IOError,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
    pub kind: ErrorKind
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            msg: format!("IOError: {value}"),
            kind: ErrorKind::IOError
        }
    }
}
