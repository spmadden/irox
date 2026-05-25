// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

//!
//! Application Helpers Toolkit, by IROX
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]

mod args;
mod manifest;
mod pgp;
mod updater;

pub mod commit_messages;
mod error;
mod os;

pub use args::*;
pub use error::*;
pub use manifest::*;
pub use os::*;
