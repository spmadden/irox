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
#![cfg_attr(not(feature = "std"), no_std)]

mod aead;
mod aes;
mod chacha20;
pub mod ed25519;
mod pbkdf2;
mod poly1305;
mod salsa;
mod scrypt;
pub mod x25519;

pub use aead::*;
pub use aes::*;
pub use chacha20::*;
pub use irox_tools::hash::sha2;
pub use pbkdf2::*;
pub use poly1305::*;
pub use salsa::*;

use irox_tools::cfg_feature_std;
cfg_feature_std! {
    mod crng;
    pub use crng::*;
}
