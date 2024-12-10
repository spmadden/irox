// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

///
/// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at the boundary of the type.
pub trait WrappingAdd {
    #[must_use]
    /// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at the boundary of the type.
    fn wrapping_add(&self, rhs: Self) -> Self;
}

///
/// Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around at the boundary of the type.
pub trait WrappingSub {
    #[must_use]
    /// Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around at the boundary of the type.
    fn wrapping_sub(&self, rhs: Self) -> Self;
}

///
/// Wrapping (modular) multiplication. Computes `self * rhs`, wrapping around at the boundary of the type.
pub trait WrappingMul {
    #[must_use]
    /// Wrapping (modular) multiplication. Computes `self * rhs`, wrapping around at the boundary of the type.
    fn wrapping_mul(&self, rhs: Self) -> Self;
}
