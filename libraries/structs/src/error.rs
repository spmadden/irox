// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum StructErrorType {
    IOError,
}

#[derive(Debug, Clone)]
pub struct StructError {
    error_type: StructErrorType,
    error: String,
}
impl Display for StructError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "StructError({:?}): {}",
            self.error_type, self.error
        ))
    }
}
impl std::error::Error for StructError {}

impl StructError {
    pub(crate) fn new_str(error_type: StructErrorType, error: String) -> StructError {
        StructError { error, error_type }
    }
}

impl From<std::io::Error> for StructError {
    fn from(value: std::io::Error) -> Self {
        StructError::new_str(StructErrorType::IOError, format!("{value:?}"))
    }
}
