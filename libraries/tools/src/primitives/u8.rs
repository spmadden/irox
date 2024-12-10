// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! A collection of utilities for the u8 built-in
//!

use crate::{WrappingAdd, WrappingMul, WrappingSub};

///
/// Finds the minimum and maximum value in the provided iterator.
/// Example:
/// ```
/// let values : Vec<u8> = vec![0, 5, 30, 20, 2];
/// let (min, max) = irox_tools::u8::min_max(&values);
///
/// assert_eq!(min, 0);
/// assert_eq!(max, 30);
/// ```
#[must_use]
pub fn min_max(iter: &[u8]) -> (u8, u8) {
    let mut min = u8::MAX;
    let mut max = u8::MIN;

    for val in iter {
        min = min.min(*val);
        max = max.max(*val);
    }

    (min, max)
}

impl WrappingAdd for u8 {
    fn wrapping_add(&self, rhs: Self) -> Self {
        u8::wrapping_add(*self, rhs)
    }
}
impl WrappingAdd for i8 {
    fn wrapping_add(&self, rhs: Self) -> Self {
        i8::wrapping_add(*self, rhs)
    }
}
impl WrappingSub for u8 {
    fn wrapping_sub(&self, rhs: Self) -> Self {
        u8::wrapping_sub(*self, rhs)
    }
}
impl WrappingSub for i8 {
    fn wrapping_sub(&self, rhs: Self) -> Self {
        i8::wrapping_sub(*self, rhs)
    }
}
impl WrappingMul for u8 {
    fn wrapping_mul(&self, rhs: Self) -> Self {
        u8::wrapping_mul(*self, rhs)
    }
}
impl WrappingMul for i8 {
    fn wrapping_mul(&self, rhs: Self) -> Self {
        i8::wrapping_mul(*self, rhs)
    }
}
