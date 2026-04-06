// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Point, Vector};
use core::ops::Mul;
use irox_tools::{FloatIsh, ToSigned};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LinearTransform<T: FloatIsh> {
    pub scale: T,
    pub translate: Vector<T>,
}
impl<T: FloatIsh> Default for LinearTransform<T> {
    fn default() -> Self {
        Self {
            scale: T::ONE,
            translate: Vector::new(T::ZERO, T::ZERO),
        }
    }
}

impl<T: FloatIsh> LinearTransform<T>
where
    T: ToSigned<Output = T>,
{
    #[must_use]
    pub fn inverse(&self) -> LinearTransform<T> {
        let scale = T::ONE / self.scale;
        let translate = (self.translate * T::negative_one()) * scale;

        Self { scale, translate }
    }
}

impl<T: FloatIsh> Mul<Point<T>> for LinearTransform<T>
where
    Point<T>: Mul<T, Output = Point<T>>,
{
    type Output = Point<T>;

    fn mul(self, rhs: Point<T>) -> Self::Output {
        rhs * self.scale + self.translate
    }
}
