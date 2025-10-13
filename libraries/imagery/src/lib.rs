// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Image processing, manipulation, formats
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub use color::*;
pub use error::*;
use irox_tools::{cfg_feature_alloc, cfg_feature_std};
pub use pixel::*;

cfg_feature_std! {
pub use tiff::*;
mod tiff;
}

pub mod bitpacked;
mod color;
pub mod colormaps;
mod error;
mod pixel;
mod stacked;

cfg_feature_alloc! {
    mod allocimpls;
    pub use allocimpls::*;
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ImageSpace {
    #[default]
    PIXEL,

    WORLD,

    OTHER(&'static str),
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ImageDimensions<T> {
    pub width: T,
    pub height: T,
}

pub trait Image {
    type DimType;

    fn get_dimensions(&self, space: ImageSpace) -> Self::DimType;

    fn get_width_pixels(&self) -> usize;

    fn get_height_pixels(&self) -> usize;

    fn get_pixel_value(&self, x: usize, y: usize) -> Option<Color>;
}

pub trait ImageMut {
    fn set_pixel_value(&mut self, x: usize, y: usize, color: Color) -> Result<(), ImageError>;
}
