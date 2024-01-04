// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
pub mod ansi_colors;
pub mod arrays;
#[macro_use]
pub mod assert;
pub mod base64;
pub mod bits;
pub mod codec;
#[macro_use]
pub mod fmt;
#[cfg(feature = "std")]
pub mod hex;
pub mod identifier;
pub mod iterators;
pub mod murmur3;
pub mod options;
#[cfg(feature = "std")]
pub mod packetio;
pub mod random;
#[cfg(feature = "std")]
pub mod read;
#[cfg(feature = "std")]
pub mod scanner;
#[cfg(feature = "std")]
pub mod sync;
pub mod uuid;
pub mod vec;

pub use primitives::*;
mod primitives;
