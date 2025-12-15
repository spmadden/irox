// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! A collection of utilities for the f32 built-in
//!

use crate::{ToF64, ToSigned};

impl crate::f64::FloatExt for f32 {
    type Type = f32;
    type Size = u32;

    fn trunc(self) -> f32 {
        (self as u64) as f32
    }

    fn fract(self) -> f32 {
        self - self.trunc()
    }

    fn abs(self) -> f32 {
        f32::from_bits(self.to_bits() & 0x7FFF_FFFF)
    }
    fn round(self) -> f32 {
        (self + 0.5 * self.signum()).trunc()
    }

    fn floor(self) -> f32 {
        if self.is_sign_negative() {
            return (self - 1.0).trunc();
        }
        self.trunc()
    }

    fn ceil(self) -> f32 {
        if self.is_sign_positive() {
            return (self + 1.0).trunc();
        }
        self.trunc()
    }

    fn signum(self) -> f32 {
        if self.is_nan() {
            return f32::NAN;
        }
        if self.is_sign_negative() {
            return -1.0;
        }
        1.0
    }
    ///
    /// Implementation of Exponential Function from NIST DTMF eq 4.2.19: `<https://dlmf.nist.gov/4.2.E19>`
    fn exp(self) -> Self::Type {
        let mut out = 1.0;
        let i = self;
        let mut z = self;
        let mut exp = 1.0;
        let mut idx = 1;
        let mut next = self;

        while next.abs() > f32::EPSILON {
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

    ///
    /// Implementation of Natural Logarithm using NIST DLMF eq 4.6.4: `<https://dlmf.nist.gov/4.6.E4>`
    fn ln(self) -> Self::Type {
        let z = self as f64;
        let iter = (z - 1.) / (z + 1.);
        let mut out = 0.0f64;
        let mut next = iter;
        let mut base = iter;
        let mut idx = 1u64;
        while next.abs() > f64::EPSILON {
            out += next;
            idx += 2;
            base *= iter * iter;
            next = base / idx as f64;
        }
        (out * 2.0) as f32
    }

    fn log10(self) -> Self::Type {
        self.ln() / core::f32::consts::LN_10
    }

    /// Naive implementation of integer power fn.  Will do something smarter later.
    fn powi(self, val: i32) -> Self::Type {
        let mut out = self;
        let i = self;
        for _ in 0..val.abs() {
            out *= i;
        }
        out
    }

    ///
    /// Implementation of general power function using NIST DLMF eq 4.2.26: `<https://dlmf.nist.gov/4.2.E26>`
    fn powf(self, a: Self::Type) -> Self::Type {
        let z = self;

        (a * z.ln()).exp()
    }

    fn sqrt(self) -> Self::Type {
        self.powf(0.5)
    }

    fn to_bits(self) -> Self::Size {
        f32::to_bits(self)
    }

    fn exponent(self) -> u16 {
        ((self.to_bits() >> 23) & 0x0F) as u16
    }

    fn significand(self) -> Self::Size {
        self.to_bits() & 0x7FFFFF
    }

    fn sin(self) -> Self::Type {
        if cfg!(feature = "std") {
            f32::sin(self)
        } else {
            todo!()
        }
    }

    fn cos(self) -> Self::Type {
        if cfg!(feature = "std") {
            f32::cos(self)
        } else {
            todo!()
        }
    }
}

impl ToF64 for f32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToSigned for f32 {
    type Output = f32;

    fn to_signed(self) -> Self::Output {
        self
    }

    fn negative_one() -> Self::Output {
        -1.
    }
}

#[cfg(all(test, not(feature = "std")))]
mod tests {
    #[test]
    pub fn test_ln() {
        assert_eq!(0.0, crate::f64::FloatExt::ln(1.0_f32));
        assert_eq_eps!(1.0, crate::f64::FloatExt::ln(core::f32::consts::E), 1e-6);
        assert_eq_eps!(4.6051702, crate::f64::FloatExt::ln(100f32), 1e-6);
        assert_eq_eps!(
            11.09033963004403,
            crate::f64::FloatExt::ln(u16::MAX as f32),
            1e-6
        );
    }

    #[test]
    pub fn test_exp() {
        assert_eq_eps!(1.0, crate::f64::FloatExt::exp(0.0f32), 1e-6);
        assert_eq_eps!(
            core::f32::consts::E,
            crate::f64::FloatExt::exp(1.0f32),
            1e-6
        );
        assert_eq_eps!(7.389056098930649, crate::f64::FloatExt::exp(2.0f32), 1e-6);
        assert_eq_eps!(
            15.154261,
            crate::f64::FloatExt::exp(core::f32::consts::E),
            1e-6
        );
    }
}
