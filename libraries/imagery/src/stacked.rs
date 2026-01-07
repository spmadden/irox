// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Color, ColorDepth, Image, ImageError, ImageMut, ImageSpace};
use irox_tools::cfg_feature_egui;

#[derive(Clone, PartialEq)]
pub struct StackedImage<const W: usize, const H: usize> {
    data: [[Color; W]; H],
    world_dimensions: Option<(f64, f64)>,
}
impl<const W: usize, const H: usize> Image for StackedImage<W, H> {
    type DimType = Option<(f64, f64)>;

    fn get_dimensions(&self, space: ImageSpace) -> Self::DimType {
        match space {
            ImageSpace::PIXEL => Some((W as f64, H as f64)),
            ImageSpace::WORLD => self.world_dimensions,
            ImageSpace::OTHER(_) => None,
        }
    }

    fn get_width_pixels(&self) -> usize {
        W
    }

    fn get_height_pixels(&self) -> usize {
        H
    }

    fn get_pixel_value(&self, x: usize, y: usize) -> Option<Color> {
        if x >= W {
            return None;
        }
        if y >= H {
            return None;
        }
        let col = self.data.get(y)?;
        col.get(x).copied()
    }
}
impl<const W: usize, const H: usize> ImageMut for StackedImage<W, H> {
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
#[derive(Debug, Clone, PartialEq)]
pub struct LinearStackedImage<const N: usize> {
    pub(crate) data: [Color; N],
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) world_dimensions: Option<(f64, f64)>,
}
impl<const N: usize> Default for LinearStackedImage<N> {
    fn default() -> Self {
        Self {
            data: [Color::default(); N],
            width: 0,
            height: 0,
            world_dimensions: None,
        }
    }
}
impl<const N: usize> Image for LinearStackedImage<N> {
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
        if x >= self.width {
            return None;
        }
        if y >= self.height {
            return None;
        }
        let idx = y * self.width + x;
        self.data.get(idx).copied()
    }
}
impl<const N: usize> ImageMut for LinearStackedImage<N> {
    fn set_pixel_value(&mut self, x: usize, y: usize, color: Color) -> Result<(), ImageError> {
        if x >= self.width {
            return Err(ImageError::bad_width(x));
        }
        if y >= self.height {
            return Err(ImageError::bad_height(y));
        }
        let idx = y * self.width + x;
        let Some(v) = self.data.get_mut(idx) else {
            return Err(ImageError::not_enough_values());
        };
        *v = color;
        Ok(())
    }
}
cfg_feature_egui! {
    impl<const N: usize> From<LinearStackedImage<N>> for egui::epaint::ColorImage {
        fn from(value: LinearStackedImage<N>) -> Self {
            egui::epaint::ColorImage {
                size: [value.width, value.height],
                pixels: value.data.as_slice().iter().map(Into::into).collect(),
            }
        }

    }
}

pub struct BitPackedImage<const N: usize> {
    data: [Color; N],
    width: usize,
    height: usize,
    color_depth: ColorDepth,
}
impl<const N: usize> Image for BitPackedImage<N> {
    type DimType = Option<(usize, usize)>;

    fn get_dimensions(&self, space: ImageSpace) -> Self::DimType {
        match space {
            ImageSpace::PIXEL => Some((self.width, self.height)),
            ImageSpace::WORLD | ImageSpace::OTHER(_) => None,
        }
    }

    fn get_width_pixels(&self) -> usize {
        self.width
    }

    fn get_height_pixels(&self) -> usize {
        self.height
    }

    fn get_pixel_value(&self, x: usize, y: usize) -> Option<Color> {
        if x >= self.width {
            return None;
        }
        if y >= self.height {
            return None;
        }
        let bitidx = x * self.color_depth.bits_per_color() as usize;
        let idx = bitidx >> 8;
        let _ = self.data.get(idx);
        todo!()
    }
}
