// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Low-level zip file I/O
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod scanner;
pub mod types;
