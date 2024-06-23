// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Various utilities and helpers

pub mod base64;
pub mod identifier;
crate::cfg_feature_std! {
    pub mod scanner;
}
pub mod uuid;
