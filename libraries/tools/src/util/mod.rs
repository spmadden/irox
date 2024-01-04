// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Various utilities and helpers

pub mod base64;
pub mod bits;
pub mod identifier;
#[cfg(feature = "std")]
pub mod scanner;
pub mod uuid;