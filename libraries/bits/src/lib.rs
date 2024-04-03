// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]

pub use bits::*;
pub use error::*;
pub use mutbits::*;
#[cfg(feature = "std")]
pub use stdwrappers::*;

#[cfg(feature = "alloc")]
pub mod allocimpls;
mod bits;
mod error;
mod mutbits;
#[cfg(feature = "std")]
mod stdimpls;
#[cfg(feature = "std")]
mod stdwrappers;
