// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::{ToF64, WrappingAdd, WrappingMul, WrappingSub};

impl WrappingAdd for u128 {
    fn wrapping_add(&self, rhs: Self) -> Self {
        u128::wrapping_add(*self, rhs)
    }
}
impl WrappingSub for u128 {
    fn wrapping_sub(&self, rhs: Self) -> Self {
        u128::wrapping_sub(*self, rhs)
    }
}
impl WrappingMul for u128 {
    fn wrapping_mul(&self, rhs: Self) -> Self {
        u128::wrapping_mul(*self, rhs)
    }
}
impl WrappingAdd for i128 {
    fn wrapping_add(&self, rhs: Self) -> Self {
        i128::wrapping_add(*self, rhs)
    }
}
impl WrappingSub for i128 {
    fn wrapping_sub(&self, rhs: Self) -> Self {
        i128::wrapping_sub(*self, rhs)
    }
}
impl WrappingMul for i128 {
    fn wrapping_mul(&self, rhs: Self) -> Self {
        i128::wrapping_mul(*self, rhs)
    }
}
impl ToF64 for i128 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl ToF64 for u128 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
