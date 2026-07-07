// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::geometry::{Centroid, Geometry};
use crate::{Point, Vector, Vector2D};
use core::cmp::Ordering;
use core::ops::{Div, Sub};
use irox_tools::{FloatIsh, MaxValue, Zero};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Rectangle<T: FloatIsh> {
    pub min: Point<T>,
    pub size: Vector<T>,
}

impl<T: FloatIsh> Rectangle<T> {
    pub const EMPTY: Rectangle<T> = Rectangle {
        min: Point::MAX_VALUE,
        size: Vector::splat(T::MIN_VALUE),
    };
}
impl<T: FloatIsh> Zero for Rectangle<T> {
    const ZERO: Self = Rectangle {
        min: Point::ZERO,
        size: Vector::ZERO,
    };
}

impl<T: FloatIsh> Rectangle<T> {
    ///
    /// Returns a vector value representing the normalized vector to the provided point.  A returned
    /// vector of [0,0] is the minimum value of the rectangle, [1,1] is the maximum value of the
    /// rectangle.
    #[must_use]
    pub fn normalized_point(&self, pnt: &Point<T>) -> Vector<T> {
        (*pnt - self.min) / self.size
    }

    #[must_use]
    pub fn far_point(&self) -> Point<T> {
        self.min + self.size
    }

    #[must_use]
    pub fn far_x(&self) -> Point<T> {
        self.min + Vector::new(self.size.vx, T::ZERO)
    }

    #[must_use]
    pub fn far_y(&self) -> Point<T> {
        self.min + Vector::new(T::ZERO, self.size.vy)
    }

    #[must_use]
    pub fn max_x(&self) -> T {
        self.far_point().x
    }

    #[must_use]
    pub fn max_y(&self) -> T {
        self.far_point().y
    }
    pub fn add_point(&mut self, point: Point<T>) {
        let xc = self.min.x.partial_cmp(&point.x).unwrap_or(Ordering::Equal);
        let yc = self.min.y.partial_cmp(&point.y).unwrap_or(Ordering::Equal);
        if matches!(xc, Ordering::Greater) || matches!(yc, Ordering::Greater) {
            let oldfar = self.far_point();
            self.min = point;
            self.size = oldfar - self.min;
        }
        let farpoint = self.far_point();
        let xc = farpoint.x.partial_cmp(&point.x).unwrap_or(Ordering::Equal);
        if matches!(xc, Ordering::Less) {
            self.size.vx += point.x - farpoint.x;
        }
        let yc = farpoint.y.partial_cmp(&point.y).unwrap_or(Ordering::Equal);
        if matches!(yc, Ordering::Less) {
            self.size.vy += point.y - farpoint.y;
        }
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn to_polygon(&self) -> crate::Polygon<T> {
        let mut out = crate::Polygon::empty();
        out.add_point(self.min);
        out.add_point(self.far_x());
        out.add_point(self.far_point());
        out.add_point(self.far_y());
        out
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

impl<T: FloatIsh> Div<T> for Rectangle<T> {
    type Output = Rectangle<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            min: (self.min.to_vector() / rhs).to_point(),
            size: self.size / rhs,
        }
    }
}
impl<T: FloatIsh> Sub<Vector<T>> for Rectangle<T> {
    type Output = Rectangle<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self {
            min: (self.min - rhs.to_point()).to_point(),
            size: self.size - rhs,
        }
    }
}
