// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! A collection of utilities for the f64 built-in
//!

use crate::{FloatIsh, FromF64, PrimitiveMath, ToF64, ToSigned, WrappingSub};

///
/// Finds the minimum and maximum value in the provided iterator.
/// Example:
/// ```
/// let values : Vec<f64> = vec![0.0, 5.0, 30.0, 20.0, 2.0];
/// let (min, max) = irox_tools::f64::min_max(&values);
///
/// assert_eq!(min, 0.0);
/// assert_eq!(max, 30.0);
/// ```
#[must_use]
pub fn min_max(iter: &[f64]) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    for val in iter {
        min = min.min(*val);
        max = max.max(*val);
    }

    (min, max)
}

pub trait FloatExt {
    type Type;
    type Size;
    fn trunc(self) -> Self::Type;
    fn fract(self) -> Self::Type;
    fn abs(self) -> Self::Type;
    fn round(self) -> Self::Type;
    fn floor(self) -> Self::Type;
    fn ceil(self) -> Self::Type;
    fn signum(self) -> Self::Type;

    fn exp(self) -> Self::Type;
    fn ln(self) -> Self::Type;
    fn log10(self) -> Self::Type;

    fn powi(self, val: i32) -> Self::Type;
    fn powf(self, val: Self::Type) -> Self::Type;

    fn sqrt(self) -> Self::Type;
    fn to_bits(self) -> Self::Size;
    fn exponent(self) -> u16;
    fn significand(self) -> Self::Size;
    fn sin(self) -> Self::Type;
    fn cos(self) -> Self::Type;
}
#[cfg(not(feature = "std"))]
impl FloatExt for f64 {
    type Type = f64;
    type Size = u64;

    ///
    /// Truncate the value
    /// Just casts to u64 then back to f64.
    fn trunc(self) -> f64 {
        (self as u64) as f64
    }

    fn fract(self) -> f64 {
        self - self.trunc()
    }

    ///
    /// Force the value to be positive by zeroing out the highest sign bit.
    fn abs(self) -> f64 {
        f64::from_bits(self.to_bits() & 0x7FFF_FFFF_FFFF_FFFFu64)
    }

    fn round(self) -> f64 {
        (self + 0.5 * self.signum()).trunc()
    }

    fn floor(self) -> f64 {
        if self.is_sign_negative() {
            return (self - 1.0).trunc();
        }
        self.trunc()
    }

    fn ceil(self) -> f64 {
        if self.is_sign_positive() {
            return (self + 1.0).trunc();
        }
        self.trunc()
    }

    fn signum(self) -> f64 {
        if self.is_nan() {
            return f64::NAN;
        }
        if self.is_sign_negative() {
            return -1.0;
        }
        1.0
    }

    ///
    /// Implementation of Exponential Function from NIST DTMF eq 4.2.19: `<https://dlmf.nist.gov/4.2.E19>`
    fn exp(self) -> Self::Type {
        if self.is_nan() || self.is_infinite() {
            return self;
        }
        let mut out = 1.0;
        let i = self;
        let mut idx = 1;
        let mut next = self;

        while next.abs() != 0.0 {
            out += next;
            idx += 1;
            next *= i / idx as Self::Type;
        }

        out
    }

    ///
    /// Implementation of Natural Logarithm using NIST DLMF eq 4.6.4: `<https://dlmf.nist.gov/4.6.E4>`
    fn ln(self) -> Self::Type {
        if !self.is_normal() {
            return self;
        }
        let z = self;
        if z == 0. {
            return 1.;
        } else if z < 0. {
            return f64::NAN;
        }
        let iter = (z - 1.) / (z + 1.);
        let mut out = 0.0;
        let mut next = 2.0 * iter;
        let mut idx = 1.0;
        let mut base = iter;
        while next != 0.0 {
            out += next;
            idx += 2.0;
            base *= iter * iter;
            next = 2.0 * base / idx;
        }
        out
    }

    fn log10(self) -> Self::Type {
        self.ln() / core::f64::consts::LN_10
    }

    ///
    /// Implementation of general power function using NIST DLMF eq 4.2.26: `<https://dlmf.nist.gov/4.2.E26>`
    fn powf(self, a: Self::Type) -> Self::Type {
        if !self.is_normal() {
            return self;
        }
        let z = self;

        (a * z.ln()).exp()
    }

    /// Naive implementation of integer power fn.  Will do something smarter later.
    fn powi(self, val: i32) -> Self::Type {
        if !self.is_normal() {
            return self;
        }
        let mut out = self;
        let i = self;
        for _ in 0..val.abs() {
            out *= i;
        }
        out
    }

    fn sqrt(self) -> Self::Type {
        self.powf(0.5)
    }

    fn to_bits(self) -> Self::Size {
        f64::to_bits(self)
    }

    fn exponent(self) -> u16 {
        ((self.to_bits() >> 52) & 0x7FF) as u16
    }

    fn significand(self) -> Self::Size {
        self.to_bits() & 0xF_FFFF_FFFF_FFFF
    }

    fn sin(self) -> Self::Type {
        todo!()
    }
    fn cos(self) -> Self::Type {
        todo!()
    }
}

#[cfg(feature = "std")]
impl FloatExt for f64 {
    type Type = f64;
    type Size = u64;

    fn trunc(self) -> Self::Type {
        f64::trunc(self)
    }

    fn fract(self) -> Self::Type {
        f64::fract(self)
    }

    fn abs(self) -> Self::Type {
        f64::abs(self)
    }

    fn round(self) -> Self::Type {
        f64::round(self)
    }

    fn floor(self) -> Self::Type {
        f64::floor(self)
    }

    fn ceil(self) -> Self::Type {
        f64::ceil(self)
    }

    fn signum(self) -> Self::Type {
        f64::signum(self)
    }

    fn exp(self) -> Self::Type {
        f64::exp(self)
    }

    fn ln(self) -> Self::Type {
        f64::ln(self)
    }

    fn log10(self) -> Self::Type {
        f64::log10(self)
    }

    fn powi(self, val: i32) -> Self::Type {
        f64::powi(self, val)
    }

    fn powf(self, val: Self::Type) -> Self::Type {
        f64::powf(self, val)
    }

    fn sqrt(self) -> Self::Type {
        f64::sqrt(self)
    }

    fn to_bits(self) -> Self::Size {
        f64::to_bits(self)
    }

    fn exponent(self) -> u16 {
        ((self.to_bits() >> 52) & 0x7FF) as u16
    }

    fn significand(self) -> Self::Size {
        self.to_bits() & 0xF_FFFF_FFFF_FFFF
    }

    fn sin(self) -> Self::Type {
        f64::sin(self)
    }

    fn cos(self) -> Self::Type {
        f64::cos(self)
    }
}

impl WrappingSub for f64 {
    fn wrapping_sub(&self, rhs: Self) -> Self {
        self - rhs
    }
}
impl ToF64 for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}
impl FromF64 for f64 {
    fn from_f64(value: f64) -> Self {
        value
    }
}
impl ToSigned for f64 {
    type Output = f64;

    fn to_signed(self) -> Self::Output {
        self
    }
    fn negative_one() -> Self::Output {
        -1.
    }
}

impl PrimitiveMath for f64 {}
impl FloatIsh for f64 {}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_ln() {
        assert_eq_eps!(0.0, crate::f64::FloatExt::ln(1.0f64), 1e-16);
        assert_eq_eps!(1.0, crate::f64::FloatExt::ln(core::f64::consts::E), 1e-15);
        assert_eq_eps!(4.605170185988092, crate::f64::FloatExt::ln(100f64), 1e-13);
        assert_eq_eps!(
            11.090339630053647,
            crate::f64::FloatExt::ln(u16::MAX as f64),
            1e-11
        );
    }

    #[test]
    pub fn test_exp() {
        assert_eq_eps!(1.0, crate::f64::FloatExt::exp(0.0f64), 1e-16);
        assert_eq_eps!(
            core::f64::consts::E,
            crate::f64::FloatExt::exp(1.0f64),
            1e-15
        );
        assert_eq_eps!(7.38905609893065, crate::f64::FloatExt::exp(2.0f64), 1e-14);
        assert_eq_eps!(
            15.154262241479262,
            crate::f64::FloatExt::exp(core::f64::consts::E),
            1e-15
        );
    }

    #[test]
    pub fn test_sqrt() {
        assert_eq!(2., crate::f64::FloatExt::sqrt(4.0f64));
    }
}
