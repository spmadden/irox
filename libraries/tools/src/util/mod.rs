// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Various utilities and helpers

pub mod base64;
pub mod identifier;
crate::cfg_feature_std! {
    pub mod scanner;
}
pub mod bases;
pub mod uuid;
crate::cfg_feature_alloc! {
    pub mod levenshtein;
}
