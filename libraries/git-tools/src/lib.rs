// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Wrappers and scripts around the `git2` crate
//!

#![forbid(unsafe_code)]

irox_tools::cfg_not_wasm! {
    mod describe;
    mod error;
    mod base;
    pub use base::*;
}
