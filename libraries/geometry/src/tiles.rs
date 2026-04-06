// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::transform::LinearTransform;
use crate::{Rectangle, Vector};
use irox_tools::{FloatIsh, ToSigned};

pub const DEFAULT_VIEWPORT_WIDTH: f64 = 1920.0;
pub const DEFAULT_VIEWPORT_HEIGHT: f64 = 1080.0;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tile {
    pub tile_zoom: u32,
    pub tile_x_idx: u32,
    pub tile_y_idx: u32,
}

#[derive(Debug, Clone)]
pub struct Tileizer<T: FloatIsh> {
    pub tile_size: T,
    pub viewport: Rectangle<T>,
    pub zoom_offset: T,
    pub transform: LinearTransform<T>,
}
impl<T: FloatIsh> Default for Tileizer<T> {
    fn default() -> Self {
        Self {
            tile_size: T::from_f64(256.0),
            viewport: Rectangle {
                min: Default::default(),
                size: Vector::new(
                    T::from_f64(DEFAULT_VIEWPORT_WIDTH),
                    T::from_f64(DEFAULT_VIEWPORT_HEIGHT),
                ),
            },
            zoom_offset: T::from_f64(0.0),
            transform: LinearTransform::default(),
        }
    }
}

impl<T: FloatIsh> Tileizer<T>
where
    T: ToSigned<Output = T>,
{
    pub fn get_tiles<F: FnMut(&Tile)>(&self, mut cb: F) {
        let z = (self.transform.scale / self.tile_size).log2();
        let z0 = z + self.zoom_offset;
        let k = ((z - z0).powi(2) * self.tile_size).min(T::ONE);
        let ctr = self.transform.translate - self.transform.scale / T::from_f64(2.);
        let scaled = (self.viewport - ctr) / k;
        let my = (1u32 << z0.to_f64() as u32).saturating_sub(1);
        let startx = scaled.min.x.floor().max(T::ZERO).to_f64() as u32;
        let starty = scaled.min.y.floor().max(T::ZERO).to_f64() as u32;
        let startx = startx.min(my);
        let starty = starty.min(my);
        let end = scaled.far_point();
        let endx = (end.x.ceil().to_f64() as u32).min(my);
        let endy = (end.y.ceil().to_f64() as u32).min(my);
        let z = z.ceil().to_f64() as u32;
        for x in startx..=endx {
            for y in starty..=endy {
                let tile = Tile {
                    tile_zoom: z,
                    tile_x_idx: x,
                    tile_y_idx: y,
                };
                cb(&tile);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tiles::Tileizer;

    #[test]
    #[cfg(feature = "std")]
    pub fn test() {
        let tm = Tileizer::<f64>::default();
        tm.get_tiles(|tile| {
            println!("Tile: {:?}", tile);
        });
    }

    #[test]
    #[cfg(feature = "std")]
    pub fn test2() {
        let mut tm = Tileizer::<f64>::default();

        tm.get_tiles(|tile| {
            println!("Tile: {:?}", tile);
        });
    }
}
