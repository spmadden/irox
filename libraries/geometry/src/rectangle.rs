// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::geometry::{Centroid, Geometry};
use crate::{Point, Vector};
use irox_tools::FloatIsh;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Rectangle<T: FloatIsh> {
    pub min: Point<T>,
    pub size: Vector<T>,
}

impl<T: FloatIsh> Centroid<T> for Rectangle<T> {
    fn centroid(&self) -> Point<T> {
        self.min + self.size * T::from_f64(0.5)
    }
}

impl<T: FloatIsh> Geometry<T> for Rectangle<T> {
    fn contains(&self, _point: &Point<T>) -> bool {
        todo!()
    }

    fn distance_to(&self, _point: &Point<T>) -> T {
        todo!()
    }

    fn intersects(&self, _point: &Point<T>) -> bool {
        todo!()
    }

    fn bounding_rectangle(&self) -> Rectangle<T> {
        *self
    }
}
