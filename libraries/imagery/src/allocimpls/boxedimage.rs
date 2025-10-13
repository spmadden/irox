// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Color, Image, ImageError, ImageMut, ImageSpace};
use alloc::boxed::Box;
use alloc::vec::Vec;
pub struct BoxedImage {
    data: Box<[Box<[Color]>]>,
    width: usize,
    height: usize,
    world_dimensions: Option<(f64, f64)>,
}
impl BoxedImage {
    pub fn new(width: usize, height: usize, initial_color: Color) -> Self {
        let mut cols = Vec::new();
        cols.resize(width, initial_color);
        let mut rows = Vec::new();
        rows.resize(height, cols.into_boxed_slice());
        Self {
            data: rows.into_boxed_slice(),
            width,
            height,
            world_dimensions: None,
        }
    }
}
impl Image for BoxedImage {
    type DimType = Option<(f64, f64)>;

    fn get_dimensions(&self, space: ImageSpace) -> Self::DimType {
        match space {
            ImageSpace::PIXEL => Some((self.width as f64, self.height as f64)),
            ImageSpace::WORLD => self.world_dimensions,
            ImageSpace::OTHER(_) => None,
        }
    }

    fn get_width_pixels(&self) -> usize {
        self.width
    }

    fn get_height_pixels(&self) -> usize {
        self.height
    }

    fn get_pixel_value(&self, x: usize, y: usize) -> Option<Color> {
        let col = self.data.get(y)?;
        col.get(x).copied()
    }
}
impl ImageMut for BoxedImage {
    fn set_pixel_value(&mut self, x: usize, y: usize, color: Color) -> Result<(), ImageError> {
        let Some(col) = self.data.get_mut(y) else {
            return Err(ImageError::bad_height(y));
        };
        let Some(row) = col.get_mut(x) else {
            return Err(ImageError::bad_width(x));
        };
        *row = color;
        Ok(())
    }
}
