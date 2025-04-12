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
// #![cfg_attr(not(feature = "_std"), no_std)]

mod aes;
mod chacha20;
pub mod ed25519;
mod pbkdf2;
mod poly1305;
pub mod x25519;

pub use aes::*;
pub use chacha20::*;
pub use irox_tools::hash::sha2;
pub use pbkdf2::*;
pub use poly1305::*;
