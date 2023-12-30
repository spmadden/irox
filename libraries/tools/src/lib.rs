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
#[cfg(feature = "std")]
pub mod base64;
pub mod bits;
#[cfg(feature = "std")]
pub mod codec;
pub mod f64;
#[macro_use]
pub mod fmt;
pub mod f32;
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
pub mod u16;
pub mod u64;
pub mod u8;
pub mod uuid;
pub mod vec;
