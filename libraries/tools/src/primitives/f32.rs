// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! A collection of utilities for the f32 built-in
//!

#[cfg(not(feature = "std"))]
impl crate::f64::FloatExt for f32 {
    type Type = f32;

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
