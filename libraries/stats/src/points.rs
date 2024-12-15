// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::rects::Rect2D;
use core::cmp::Ordering;
///
/// Geometric Cartesian Coordinate Spaces
use core::ops::Add;
use irox_bits::{FromBEBytes, ToBEBytes};

/// Geometric point in Cartesian 2D Space.  X and Y values.
pub trait Point2D {
    type Output: Copy + Add;
    fn get_x(&self) -> Self::Output;
    fn get_y(&self) -> Self::Output;

    #[must_use]
    fn new(x: Self::Output, y: Self::Output) -> Self;

    #[must_use]
    fn new_f64(x: f64, y: f64) -> Double2D {
        Double2D { x, y }
    }
    #[must_use]
    fn new_f32(x: f32, y: f32) -> Float2D {
        Float2D { x, y }
    }
}

pub trait EncodablePoint2D<const N: usize>: Point2D {
    fn encode(&self) -> [u8; N];
    fn decode(value: [u8; N]) -> Self;
}

/// Geometric point in Cartesian 2D Space.  X and Y values. Backed by two [`f64`]'s
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Double2D {
    pub x: f64,
    pub y: f64,
}
impl Double2D {
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Double2D { x, y }
    }
    #[must_use]
    pub fn new_f64(x: f64, y: f64) -> Self {
        Double2D { x, y }
    }
    #[must_use]
    pub fn get_x(&self) -> f64 {
        self.x
    }
    #[must_use]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    #[must_use]
    pub fn translate(&self, vec: &Vec2D) -> Double2D {
        Self {
            x: self.x + vec.x,
            y: self.y + vec.y,
        }
    }

    pub fn translate_mut(&mut self, vec: &Vec2D) {
        self.x += vec.x;
        self.y += vec.y;
    }
}
impl From<[f64; 2]> for Double2D {
    fn from(val: [f64; 2]) -> Double2D {
        let [x, y] = val;
        Double2D { x, y }
    }
}
impl Eq for Double2D {}
impl PartialOrd for Double2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Double2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = self.x.total_cmp(&other.x);
        if x != Ordering::Equal {
            return x;
        }
        self.y.total_cmp(&other.y)
    }
}

impl Point2D for Double2D {
    type Output = f64;

    fn get_x(&self) -> Self::Output {
        self.x
    }

    fn get_y(&self) -> Self::Output {
        self.y
    }

    fn new(x: Self::Output, y: Self::Output) -> Self {
        Double2D { x, y }
    }
}
impl EncodablePoint2D<16> for Double2D {
    fn encode(&self) -> [u8; 16] {
        [self.x, self.y].to_be_bytes()
    }

    fn decode(val: [u8; 16]) -> Self {
        <[f64; 2]>::from_be_bytes(val).into()
    }
}

/// Geometric point in Cartesian 2D Space.  X and Y values. Backed by two [`f32`]'s
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Float2D {
    pub x: f32,
    pub y: f32,
}
impl From<[f32; 2]> for Float2D {
    fn from(val: [f32; 2]) -> Float2D {
        let [x, y] = val;
        Float2D { x, y }
    }
}
impl Point2D for Float2D {
    type Output = f32;

    fn get_x(&self) -> Self::Output {
        self.x
    }

    fn get_y(&self) -> Self::Output {
        self.y
    }

    fn new(x: Self::Output, y: Self::Output) -> Self {
        Float2D { x, y }
    }
}
impl EncodablePoint2D<8> for Float2D {
    fn encode(&self) -> [u8; 8] {
        [self.x, self.y].to_be_bytes()
    }

    fn decode(val: [u8; 8]) -> Self {
        <[f32; 2]>::from_be_bytes(val).into()
    }
}

/// Geometric point in Cartesian 2D Space.  X and Y values. Backed by two [`u64`]'s
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Quad2D {
    pub x: u64,
    pub y: u64,
}
impl From<[u64; 2]> for Quad2D {
    fn from(val: [u64; 2]) -> Quad2D {
        let [x, y] = val;
        Quad2D { x, y }
    }
}
impl Point2D for Quad2D {
    type Output = u64;

    fn get_x(&self) -> Self::Output {
        self.x
    }

    fn get_y(&self) -> Self::Output {
        self.y
    }

    fn new(x: Self::Output, y: Self::Output) -> Self {
        Quad2D { x, y }
    }
}
impl EncodablePoint2D<16> for Quad2D {
    fn encode(&self) -> [u8; 16] {
        [self.x, self.y].to_be_bytes()
    }

    fn decode(val: [u8; 16]) -> Self {
        <[u64; 2]>::from_be_bytes(val).into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}
impl Default for Vec2D {
    fn default() -> Self {
        Vec2D { x: 1.0, y: 1.0 }
    }
}
impl Eq for Vec2D {}
impl PartialOrd for Vec2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Vec2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = self.x.total_cmp(&other.x);
        if x != Ordering::Equal {
            return x;
        }
        self.y.total_cmp(&other.y)
    }
}
impl Vec2D {
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Vec2D { x, y }
    }
    #[must_use]
    pub fn get_x(&self) -> f64 {
        self.x
    }
    #[must_use]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    #[must_use]
    pub fn with_origin(&self, origin: Double2D) -> Rect2D {
        Rect2D {
            origin,
            size: *self,
        }
    }
}
#[cfg(feature = "emath")]
impl From<emath::Vec2> for Vec2D {
    fn from(pos: emath::Vec2) -> Self {
        Self {
            x: pos.x as f64,
            y: pos.y as f64,
        }
    }
}
#[cfg(feature = "emath")]
impl From<emath::Pos2> for Double2D {
    fn from(pos: emath::Pos2) -> Self {
        Self {
            x: pos.x as f64,
            y: pos.y as f64,
        }
    }
}
#[cfg(feature = "emath")]
impl From<Double2D> for emath::Pos2 {
    fn from(value: Double2D) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}
