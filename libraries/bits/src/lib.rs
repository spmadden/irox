// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]

pub use bits::*;
pub use error::*;
pub use mutbits::*;
pub use seek::*;
#[cfg(feature = "std")]
pub use stdwrappers::*;

#[cfg(feature = "alloc")]
pub mod allocimpls;
mod bits;
mod error;
mod mutbits;
mod seek;
#[cfg(feature = "std")]
mod stdimpls;
#[cfg(feature = "std")]
mod stdwrappers;
pub mod utf;
