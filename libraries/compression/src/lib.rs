// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Compression Algorithms
//!

#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

pub mod deflate;
pub mod lzw;
