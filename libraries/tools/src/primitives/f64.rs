// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! A collection of utilities for the f64 built-in
//!

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
    fn trunc(self) -> Self::Type;
    fn fract(self) -> Self::Type;
    fn abs(self) -> Self::Type;
    fn round(self) -> Self::Type;
    fn floor(self) -> Self::Type;
    fn ceil(self) -> Self::Type;
    fn signum(self) -> Self::Type;
}

#[cfg(not(feature = "std"))]
impl FloatExt for f64 {
    type Type = f64;

    fn trunc(self) -> f64 {
        (self as u64) as f64
    }

    fn fract(self) -> f64 {
        self - self.trunc()
    }

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
}
