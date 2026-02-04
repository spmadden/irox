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
#![cfg_attr(not(feature = "std"), no_std)]

use irox_tools::cfg_feature_alloc;

mod geometry;
cfg_feature_alloc! {
    mod kdtree;
    pub use kdtree::*;
    mod polygon;
    pub use polygon::*;
    mod quadtree;
    pub use quadtree::*;
}
mod line;
mod point;
mod rectangle;
mod vector;

pub use geometry::*;
pub use line::*;
pub use point::*;
pub use rectangle::*;
pub use vector::*;
