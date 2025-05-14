// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use irox_tools::{impl_error, impl_from_error};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
    Crypto,
    ParseError,
    BitsError,
}

impl_error!(Error, ErrorType);

// impl_from_error!(Error, ed25519_dalek::SignatureError, ErrorType::Crypto);
impl_from_error!(
    Error,
    irox_cryptids::ed25519::Ed25519Error,
    ErrorType::Crypto
);
impl_from_error!(Error, core::num::ParseIntError, ErrorType::ParseError);
impl_from_error!(Error, irox_bits::BitsError, ErrorType::BitsError);
