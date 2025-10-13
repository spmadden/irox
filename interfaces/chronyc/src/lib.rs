// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Native interface to chrony
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod msgs;
irox_tools::cfg_unix! {
    mod client;
    pub use client::*;
}
