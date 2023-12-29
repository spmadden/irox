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
}
