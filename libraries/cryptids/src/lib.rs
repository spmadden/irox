// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! IROX Cryptographic Primitives - probably very hazardous
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]

mod aes;
mod chacha20;

pub use aes::*;
pub use chacha20::*;
