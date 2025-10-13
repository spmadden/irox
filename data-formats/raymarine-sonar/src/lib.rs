// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![forbid(unsafe_code)]

use irox_tools::cfg_not_wasm;

cfg_not_wasm! {
    pub mod error;
    pub mod schema;
    pub mod tracks;
    mod base;
    pub use base::*;
}
