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

    fn exp(self) -> Self::Type;
    fn ln(self) -> Self::Type;

    fn powi(self, val: u32) -> Self::Type;
    fn powf(self, val: Self::Type) -> Self::Type;

    fn sqrt(self) -> Self::Type;
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

    fn exp(self) -> Self::Type {
        let mut out = 1.0;
        let i = self;
        let mut z = self;
        let mut exp = 1.0;
        let mut idx = 1;
        let mut next = self;

        while out + next != out {
            out += next;
            idx += 1;
            z *= i;
            if z.is_infinite() {
                break;
            }
            exp *= idx as Self::Type;
            if exp.is_infinite() {
                break;
            }
            next = z / exp;
            if next.is_infinite() {
                break;
            }
        }

        out
    }

    fn ln(self) -> Self::Type {
        let z = self;
        let iter = (z - 1.) / z;
        let mut out = 0.0;
        let mut next = iter;
        let mut base = iter;
        let mut idx = 1u64;
        while out + next != out {
            out += next;
            idx += 1;
            base *= iter;
            next = base / idx as Self::Type;
        }

        out + next
    }

    fn powf(self, a: Self::Type) -> Self::Type {
        let z = self;

        (a * z.ln()).exp()
    }

    fn powi(self, val: u32) -> Self::Type {
        let mut out = self;
        let i = self;
        for _ in 0..val {
            out *= i;
        }
        out
    }

    fn sqrt(self) -> Self::Type {
        self.powf(0.5)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_ln() {
        assert_eq!(0.0, crate::f64::FloatExt::ln(1.0f64));
        assert_eq_eps!(1.0, crate::f64::FloatExt::ln(core::f64::consts::E), 1e-15);
        assert_eq_eps!(4.605170185988059, crate::f64::FloatExt::ln(100f64), 1e-15);
        assert_eq_eps!(
            22.18070977791825,
            crate::f64::FloatExt::ln(u32::MAX as f64),
            1e-15
        );
    }

    #[test]
    pub fn test_exp() {
        assert_eq_eps!(1.0, crate::f64::FloatExt::exp(0.0f64), 1e-15);
        assert_eq_eps!(
            core::f64::consts::E,
            crate::f64::FloatExt::exp(1.0f64),
            1e-15
        );
        assert_eq_eps!(7.389056098930649, crate::f64::FloatExt::exp(2.0f64), 1e-15);
        assert_eq_eps!(
            15.154262241479262,
            crate::f64::FloatExt::exp(core::f64::consts::E),
            1e-15
        );
    }
}
