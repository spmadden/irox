// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use crate::address::AddressError;
use irox_tools::{impl_err_fn, impl_error, impl_from_error};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
    IOError,
    AddressError,
    MissingPortError,
}

impl_error!(Error, ErrorType);
impl_from_error!(Error, std::io::Error, ErrorType::IOError);
impl_from_error!(Error, AddressError, ErrorType::AddressError);

impl_err_fn!(
    Error,
    ErrorType::MissingPortError,
    missing_port,
    missing_port_err
);
