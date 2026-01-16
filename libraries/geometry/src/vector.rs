// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Point, Point2D};
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use irox_tools::FloatIsh;

pub trait Vector2D<T: FloatIsh>: Default + Copy + Clone + PartialEq + PartialOrd {
    fn vx(&self) -> T;
    fn vy(&self) -> T;
    fn magnitude(&self) -> T;
    #[must_use]
    fn normalize(&self) -> Self;
    fn dot(&self, other: &Self) -> T;
    fn cross(&self, other: &Self) -> T;
    fn angle(&self, other: &Self) -> T;
    #[must_use]
    fn perpendicular(&self) -> Self;
    #[must_use]
    fn rotate(&self, angle: T) -> Self;
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
        todo!()
    }

    fn dot(&self, _other: &Self) -> T {
        todo!()
    }

    fn cross(&self, _other: &Self) -> T {
        todo!()
    }

    fn angle(&self, _other: &Self) -> T {
        todo!()
    }

    fn perpendicular(&self) -> Self {
        todo!()
    }

    fn rotate(&self, _angle: T) -> Self {
        todo!()
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
