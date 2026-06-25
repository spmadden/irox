// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Pure Rust implementation of the SQLITE3 file format
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]

pub mod db;
pub mod error;
pub mod header;
pub mod page;
