// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! IROX Cryptographic Primitives - probably very hazardous
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]

mod aes;
pub use aes::*;
