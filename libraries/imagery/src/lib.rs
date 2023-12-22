// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]

pub use pixel::*;
pub use color::*;
pub use error::*;

mod pixel;
mod color;
mod error;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ImageSpace {
    #[default]
    PIXEL,

    WORLD,

    OTHER(&'static str)
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ImageDimensions<T> {
    pub width: T,
    pub height: T,
}

pub trait Image {
    type DimType; 

    fn get_dimensions(&self, space: ImageSpace) -> Self::DimType;

    fn get_width_pixels(&self) -> u32;

    fn get_height_pixels(&self) -> u32;

    fn get_pixel_value(&self, x: u32, y: u32) -> Option<Color>;
}

pub trait ImageMut {
    fn set_pixel_value(&self, x: u32, y:u32, color: Color) -> Result<(), ImageError>;
}
