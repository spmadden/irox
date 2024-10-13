// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

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
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Double2D {
    pub x: f64,
    pub y: f64,
}
impl From<[f64; 2]> for Double2D {
    fn from(val: [f64; 2]) -> Double2D {
        let [x, y] = val;
        Double2D { x, y }
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
