// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! A collection of utilities for the u64 built-in
//!

use crate::{ToF64, WrappingAdd, WrappingMul, WrappingSub};

///
/// Finds the minimum and maximum value in the provided iterator.
/// Example:
/// ```
/// let values : Vec<u64> = vec![0, 5, 30, 20, 2];
/// let (min, max) = irox_tools::u64::min_max(&values);
///
/// assert_eq!(min, 0);
/// assert_eq!(max, 30);
/// ```
#[must_use]
pub fn min_max(iter: &[u64]) -> (u64, u64) {
    let mut min = u64::MAX;
    let mut max = u64::MIN;

    for val in iter {
        min = min.min(*val);
        max = max.max(*val);
    }

    (min, max)
}

impl WrappingAdd for u64 {
    fn wrapping_add(&self, rhs: Self) -> Self {
        u64::wrapping_add(*self, rhs)
    }
}
impl WrappingSub for u64 {
    fn wrapping_sub(&self, rhs: Self) -> Self {
        u64::wrapping_sub(*self, rhs)
    }
}
impl WrappingMul for u64 {
    fn wrapping_mul(&self, rhs: Self) -> Self {
        u64::wrapping_mul(*self, rhs)
    }
}
impl WrappingAdd for i64 {
    fn wrapping_add(&self, rhs: Self) -> Self {
        i64::wrapping_add(*self, rhs)
    }
}
impl WrappingSub for i64 {
    fn wrapping_sub(&self, rhs: Self) -> Self {
        i64::wrapping_sub(*self, rhs)
    }
}
impl WrappingMul for i64 {
    fn wrapping_mul(&self, rhs: Self) -> Self {
        i64::wrapping_mul(*self, rhs)
    }
}

impl ToF64 for u64 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl ToF64 for i64 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
