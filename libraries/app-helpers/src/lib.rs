// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Application Helpers Toolkit, by IROX
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]

mod args;
mod os;

pub use args::*;
pub use os::*;
