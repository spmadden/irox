// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! 2D and 3D Geometry Primitives
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod line;
mod point;
mod polygon;

pub use line::*;
pub use point::*;
pub use polygon::*;
