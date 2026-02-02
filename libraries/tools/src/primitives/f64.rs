// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! A collection of utilities for the f64 built-in
//!

use crate::{FloatIsh, FromF64, One, PrimitiveMath, ToF64, ToSigned, WrappingSub, Zero};

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

pub trait FloatExt: PrimitiveMath {
    type Type;
    type Size;
    fn trunc(self) -> Self::Type;
    fn fract(self) -> Self::Type;
    fn abs(self) -> Self::Type;
    fn round(self) -> Self::Type;
    fn floor(self) -> Self::Type;
    fn ceil(self) -> Self::Type;
    fn signum(self) -> Self::Type;
    fn clamp(self, min: Self, max: Self) -> Self::Type;

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
    fn tan(self) -> Self::Type;
    fn atan(self) -> Self::Type;
    fn atan2(self, o: Self) -> Self::Type;
}
#[allow(unused)]
fn cordic_k(n: usize) -> f64 {
    let mut k = 1.0;
    let mut i = n;
    let mut ik = 1.0;
    while i > 0 {
        k *= 1. / f64::sqrt(1. + ik);
        ik /= 4.0;
        i -= 1;
    }
    k
}
const CORDIC_K_64: f64 = 0.6072529350088814;
const CORDIC_ITERS: usize = 64;
const THETA_TABLE: [f64; CORDIC_ITERS] = [
    core::f64::consts::FRAC_PI_4,
    0.4636476090008061,
    0.24497866312686414,
    0.12435499454676144,
    0.06241880999595735,
    0.031239833430268277,
    0.015623728620476831,
    0.007812341060101111,
    0.0039062301319669718,
    0.0019531225164788188,
    0.0009765621895593195,
    0.0004882812111948983,
    0.00024414062014936177,
    0.00012207031189367021,
    0.00006103515617420877,
    0.000030517578115526096,
    0.000015258789061315762,
    0.00000762939453110197,
    0.000003814697265606496,
    0.000001907348632810187,
    0.0000009536743164059608,
    0.00000047683715820308884,
    0.00000023841857910155797,
    0.00000011920928955078068,
    0.00000005960464477539055,
    0.000000029802322387695303,
    0.000000014901161193847655,
    0.000000007450580596923828,
    0.000000003725290298461914,
    0.000000001862645149230957,
    0.0000000009313225746154785,
    0.0000000004656612873077393,
    0.00000000023283064365386963,
    0.00000000011641532182693481,
    0.00000000005820766091346741,
    0.000000000029103830456733704,
    0.000000000014551915228366852,
    0.000000000007275957614183426,
    0.000000000003637978807091713,
    0.0000000000018189894035458565,
    0.0000000000009094947017729282,
    0.0000000000004547473508864641,
    0.00000000000022737367544323206,
    0.00000000000011368683772161603,
    0.00000000000005684341886080802,
    0.00000000000002842170943040401,
    0.000000000000014210854715202004,
    0.000000000000007105427357601002,
    0.000000000000003552713678800501,
    0.0000000000000017763568394002505,
    0.0000000000000008881784197001252,
    0.0000000000000004440892098500626,
    0.0000000000000002220446049250313,
    0.00000000000000011102230246251565,
    0.00000000000000005551115123125783,
    0.000000000000000027755575615628914,
    0.000000000000000013877787807814457,
    0.000000000000000006938893903907228,
    0.000000000000000003469446951953614,
    0.000000000000000001734723475976807,
    0.0000000000000000008673617379884035,
    0.0000000000000000004336808689942018,
    0.0000000000000000002168404344971009,
    0.00000000000000000010842021724855044,
];
/// https://en.wikipedia.org/wiki/CORDIC
pub fn cordic<T: PrimitiveMath + One + Zero + FromF64 + PartialOrd + Copy>(alpha: T) -> (T, T) {
    let k_n = T::from_f64(CORDIC_K_64);
    let mut theta = T::ZERO;
    let mut x = T::ONE;
    let mut y = T::ZERO;
    let mut p2i = T::ONE;
    let mut idx = 0;
    while idx < CORDIC_ITERS {
        #[allow(clippy::indexing_slicing)]
        let atan = T::from_f64(THETA_TABLE[idx]);
        let xt = x;
        if theta < alpha {
            theta += atan;
            x -= y * p2i;
            y += xt * p2i;
        } else {
            theta -= atan;
            x += y * p2i;
            y -= xt * p2i;
        }
        p2i /= T::from_f64(2.);
        idx += 1;
    }
    (x * k_n, y * k_n)
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

    fn clamp(self, min: Self, max: Self) -> Self::Type {
        if self < min {
            return min;
        } else if self > max {
            return max;
        }
        self
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
        cordic(self).0
    }
    fn cos(self) -> Self::Type {
        cordic(self).1
    }

    fn tan(self) -> Self::Type {
        self.sin() / self.cos()
    }

    fn atan(self) -> Self::Type {
        todo!()
    }

    fn atan2(self, _o: Self) -> Self::Type {
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

    fn clamp(self, min: Self, max: Self) -> Self::Type {
        f64::clamp(self, min, max)
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

    fn tan(self) -> Self::Type {
        f64::tan(self)
    }

    fn atan(self) -> Self::Type {
        f64::atan(self)
    }

    fn atan2(self, o: Self) -> Self::Type {
        f64::atan2(self, o)
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
    use crate::f64::{cordic, cordic_k};
    use std::f64::consts::PI;

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

    #[test]
    #[ignore]
    pub fn theta_table() {
        println!("{}\n", cordic_k(80));
        for i in 0..80 {
            println!("{},", 1f64.atan2(2f64.powi(i)))
        }
    }
    #[test]
    pub fn test_cordic() {
        for x in -90..90 {
            let x = x as f64;
            let angr = x * (PI / 180.);
            let (cosx, sinx) = cordic(angr);
            let dcos = cosx - angr.cos();
            let dsin = sinx - angr.sin();
            let dcosulps = dcos / f64::EPSILON;
            let dsinulps = dsin / f64::EPSILON;
            println!("{x:0.05} {sinx:0.16} {dsin:0.16} {dsinulps:0.1} {cosx:0.16} {dcos:0.16} {dcosulps:0.1}");
            assert!(dcosulps.abs() < 5.0, "{x}");
            assert!(dsinulps.abs() < 5.0, "{x}");
        }
    }
}
