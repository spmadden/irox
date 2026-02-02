// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Point, Point2D};
use core::ops::DivAssign;
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use irox_tools::math::Matrix;
use irox_tools::FloatIsh;
use irox_units::units::angle::Angle;

pub trait Vector2D<T: FloatIsh>: Default + Copy + Clone + PartialEq + PartialOrd {
    fn vx(&self) -> T;
    fn vy(&self) -> T;
    fn magnitude(&self) -> T;
    #[must_use]
    fn normalize(&self) -> Self;
    fn dot(&self, other: &Self) -> T;

    fn angle(&self, other: &Self) -> Angle;
    #[must_use]
    fn perpendicular(&self) -> Self;
    #[must_use]
    fn rotate(&self, angle: Angle) -> Self;
    #[must_use]
    fn abs(&self) -> Self;
    fn to_point(&self) -> Point<T>;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vector<T: FloatIsh> {
    pub vx: T,
    pub vy: T,
}
impl<T: FloatIsh> Vector<T> {
    pub fn new(vx: T, vy: T) -> Self {
        Self { vx, vy }
    }
    pub fn splat(v: T) -> Self {
        Self::new(v, v)
    }
    pub fn to_matrix(&self) -> Matrix<2, 1, f64> {
        Matrix::new([[self.vy.to_f64()], [self.vy.to_f64()]])
    }
}

impl<T: FloatIsh> Vector2D<T> for Vector<T> {
    fn vx(&self) -> T {
        self.vx
    }

    fn vy(&self) -> T {
        self.vy
    }

    fn magnitude(&self) -> T {
        (self.vx * self.vx + self.vy * self.vy).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            vx: self.vx / mag,
            vy: self.vy / mag,
        }
    }

    fn dot(&self, other: &Self) -> T {
        self.vx * other.vx + self.vy * other.vy
    }

    fn angle(&self, other: &Self) -> Angle {
        let rad = self.vy.atan2(self.vx) - other.vy.atan2(other.vx);
        Angle::new_radians(rad.to_f64())
    }

    fn perpendicular(&self) -> Self {
        self.rotate(Angle::new_degrees(90.))
    }

    fn rotate(&self, angle: Angle) -> Self {
        let Matrix {
            values: [[vx], [vy]],
        } = Matrix::<2, 2, _>::rotation_counterclockwise(angle.as_radians().value())
            .mul(self.to_matrix());
        Self {
            vx: T::from_f64(vx),
            vy: T::from_f64(vy),
        }
    }

    fn abs(&self) -> Self {
        Self {
            vx: self.vx.abs(),
            vy: self.vy.abs(),
        }
    }
    fn to_point(&self) -> Point<T> {
        Point::new_point(self.vx, self.vy)
    }
}

impl<T: FloatIsh> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            vx: self.vx + rhs.vx,
            vy: self.vy + rhs.vy,
        }
    }
}
impl<T: FloatIsh> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.vx += rhs.vx;
        self.vy += rhs.vy;
    }
}
impl<T: FloatIsh> Mul<T> for Vector<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            vx: self.vx * rhs,
            vy: self.vy * rhs,
        }
    }
}
impl<T: FloatIsh> MulAssign<T> for Vector<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.vx *= rhs;
        self.vy *= rhs;
    }
}
impl<T: FloatIsh> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            vx: self.vx - rhs.vx,
            vy: self.vy - rhs.vy,
        }
    }
}
impl<T: FloatIsh> SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.vx -= rhs.vx;
        self.vy -= rhs.vy;
    }
}
impl<T: FloatIsh> DivAssign<T> for Vector<T> {
    fn div_assign(&mut self, rhs: T) {
        self.vx /= rhs;
        self.vy /= rhs;
    }
}
