// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::Vector;
use core::ops::Add;
use irox_tools::FloatIsh;

pub trait Point2D<T: FloatIsh>: Default + Copy + Clone + PartialEq + PartialOrd {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> Option<T>;
    fn m(&self) -> Option<T>;

    fn new_point(x: T, y: T) -> Point<T> {
        Point {
            x,
            y,
            z: None,
            m: None,
        }
    }

    fn as_pointz(&self, or_else_z: T) -> PointZ<T>;
    fn as_pointm(&self, or_else_m: T) -> PointM<T>;
    fn as_pointzm(&self, or_else_z: T, or_else_m: T) -> PointZM<T>;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Point<T: FloatIsh> {
    pub x: T,
    pub y: T,
    pub z: Option<T>,
    pub m: Option<T>,
}
impl<T: FloatIsh> Point<T> {
    pub fn to_vector(&self) -> Vector<T> {
        Vector {
            vx: self.x,
            vy: self.y,
        }
    }
}
impl<T: FloatIsh> Point2D<T> for Point<T> {
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }

    fn z(&self) -> Option<T> {
        self.z
    }

    fn m(&self) -> Option<T> {
        self.m
    }

    fn as_pointz(&self, default_z: T) -> PointZ<T> {
        PointZ {
            x: self.x,
            y: self.y,
            z: self.z.unwrap_or(default_z),
            m: self.m,
        }
    }

    fn as_pointm(&self, or_else_m: T) -> PointM<T> {
        PointM {
            x: self.x,
            y: self.y,
            z: self.z,
            m: self.m.unwrap_or(or_else_m),
        }
    }

    fn as_pointzm(&self, or_else_z: T, or_else_m: T) -> PointZM<T> {
        PointZM {
            x: self.x,
            y: self.y,
            z: self.z.unwrap_or(or_else_z),
            m: self.m.unwrap_or(or_else_m),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct PointZ<T: FloatIsh> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub m: Option<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct PointM<T: FloatIsh> {
    pub x: T,
    pub y: T,
    pub z: Option<T>,
    pub m: T,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct PointZM<T: FloatIsh> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub m: T,
}

impl<T: FloatIsh> Add<Vector<T>> for Point<T> {
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self {
            x: self.x + rhs.vx,
            y: self.y + rhs.vy,
            z: self.z,
            m: self.m,
        }
    }
}
