// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum ErrorType {
    Windows,
}

#[derive(Debug, Clone)]
pub struct Error {
    error_type: ErrorType,
    error: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "WinLocAPI Error({:?}): {}",
            self.error_type, self.error
        ))
    }
}
impl std::error::Error for Error {}

impl From<windows::core::Error> for Error {
    fn from(value: windows::core::Error) -> Self {
        Error {
            error_type: ErrorType::Windows,
            error: format!("{value:?}"),
        }
    }
}
