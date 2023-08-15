// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#[derive(Debug, Clone)]
pub enum ErrorType {}

#[derive(Debug, Clone)]
pub struct Error {
    pub(crate) error_type: ErrorType,
    pub(crate) error: String,
}
