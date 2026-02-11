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

impl<T: FloatIsh> Rectangle<T> {
    ///
    /// Returns a vector value representing the normalized vector to the provided point.  A returned
    /// vector of [0,0] is the minimum value of the rectangle, [1,1] is the maximum value of the
    /// rectangle.
    pub fn normalized_point(&self, pnt: &Point<T>) -> Vector<T> {
        (*pnt - self.min) / self.size
    }
}

impl<T: FloatIsh> Centroid<T> for Rectangle<T> {
    fn centroid(&self) -> Point<T> {
        self.min + self.size * T::from_f64(0.5)
    }
}

impl<T: FloatIsh> Geometry<T> for Rectangle<T> {
    fn contains(&self, point: &Point<T>) -> bool {
        let nrm = self.normalized_point(point);
        nrm >= Vector::<T>::ZERO && nrm <= Vector::<T>::ONE
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
