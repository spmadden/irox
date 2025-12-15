// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Fixed-precision mathematics.
//!

#![forbid(unsafe_code)]
#![no_std]

use core::cmp::Ordering;
use core::fmt::Formatter;
use core::str::FromStr;
pub use irox_tools::f64::FloatExt;

macro_rules! consts {
    ($prim:ty, $val:literal, $shift:ident, $valt:ident, $mask:ident) => {
        const $shift: $prim = $val;
        const $valt: $prim = 1 << $shift;
        const $mask: $prim = $valt - 1;
    };
}
consts!(u32, 16, U32_SHIFT, U32_VAL, U32_MASK);
consts!(i32, 16, I32_SHIFT, I32_VAL, I32_MASK);
consts!(u64, 32, U64_SHIFT, U64_VAL, U64_MASK);
consts!(i64, 32, I64_SHIFT, I64_VAL, I64_MASK);
consts!(u128, 64, U128_SHIFT, U128_VAL, U128_MASK);
consts!(i128, 64, I128_SHIFT, I128_VAL, I128_MASK);

macro_rules! impl_partial_cmp_eq {
    ($prim:ty, $($typ:tt)+) => {
        impl core::cmp::PartialEq<$prim> for $($typ)+ {
            fn eq(&self, other: &$prim) -> bool {
                <$prim>::from(*self) == *other
            }
        }
        impl core::cmp::PartialOrd<$prim> for $($typ)+ {
            fn partial_cmp(&self, other: &$prim) -> Option<core::cmp::Ordering> {
                <$prim>::partial_cmp(&self.into(), &other)
            }
        }
    };
}

macro_rules! impl_fromf64 {
    ($shift:ident, $val:ident, $mask:ident, $($typ:tt)+) => {
        impl From<$($typ)+> for f64 {
            fn from(value: $($typ)+) -> Self {
                let val = (value.data >> $shift) as f64;
                val + ((value.data & $mask) as f64 / $val as f64)
            }
        }
    };
}
macro_rules! impl_from {
    ($prim:ty, $shift:expr, $($typ:tt)+) => {
        impl From<$($typ)+> for $prim {
            fn from(value: $($typ)+) -> Self {
                value.data >> $shift
            }
        }
    };
}

macro_rules! impl_ops {
    ($strukt:ty, $prim:ty, $next_prim:ty, $shift:ident, $($typ:tt)+) => {
        impl core::ops::Add<$strukt> for $($typ)+ {
            type Output = $strukt;

            fn add(self, rhs: $strukt) -> Self::Output {
                <$strukt>::from_raw_value(self.data.saturating_add(rhs.data))
            }
        }
        impl core::ops::Sub<$strukt> for $($typ)+ {
            type Output = $strukt;

            fn sub(self, rhs: $strukt) -> Self::Output {
                <$strukt>::from_raw_value(self.data.saturating_sub(rhs.data))
            }
        }
        impl core::ops::AddAssign<$($typ)+> for $strukt {
            fn add_assign(&mut self, rhs: $($typ)+) {
                self.data = self.data.saturating_add(rhs.data)
            }
        }

        impl core::ops::AddAssign<$($typ)+> for &mut $strukt {
            fn add_assign(&mut self, rhs: $($typ)+) {
                self.data = self.data.saturating_add(rhs.data)
            }
        }
        impl core::ops::SubAssign<$($typ)+> for $strukt {
            fn sub_assign(&mut self, rhs: $($typ)+) {
                self.data = self.data.saturating_sub(rhs.data)
            }
        }

        impl core::ops::SubAssign<$($typ)+> for &mut $strukt {
            fn sub_assign(&mut self, rhs: $($typ)+) {
                self.data = self.data.saturating_sub(rhs.data)
            }
        }
        impl core::ops::Mul<$($typ)+> for $strukt {
            type Output = $strukt;

            fn mul(self, rhs: $($typ)+) -> Self::Output {
                let o = (self.data as $next_prim * rhs.data as $next_prim) >> ($shift - 1);
                let add = o & 0x01;
                let o = (o >> 1) + add;
                <$strukt>::from_raw_value(o as $prim)
            }
        }

        impl core::ops::Mul<$($typ)+> for &mut $strukt {
            type Output = $strukt;

            fn mul(self, rhs: $($typ)+) -> Self::Output {
                let o = ((self.data as $next_prim).saturating_mul(rhs.data as $next_prim)) >> ($shift - 1);
                let add = o & 0x01;
                let o = (o >> 1) + add;
                <$strukt>::from_raw_value(o as $prim)
            }
        }
        impl core::ops::Div<$($typ)+> for $strukt {
            type Output = $strukt;

            fn div(self, rhs: $($typ)+) -> Self::Output {
                let a = (self.data as $next_prim) << $shift;
                let b = (rhs.data as $next_prim);
                let o = (a / b) as $prim;
                <$strukt>::from_raw_value(o)
            }
        }
    };
}
macro_rules! impl_mut_ops {
    ($strukt:ty, $prim:ty, $next_prim:ty, $shift:ident) => {
        impl core::ops::MulAssign for $strukt {
            fn mul_assign(&mut self, rhs: $strukt) {
                self.data = core::ops::Mul::mul(*self, rhs).data;
            }
        }
        impl core::ops::MulAssign for &mut $strukt {
            fn mul_assign(&mut self, rhs: &mut $strukt) {
                self.data = core::ops::Mul::mul(**self, rhs).data;
            }
        }
        impl core::ops::DivAssign for $strukt {
            fn div_assign(&mut self, rhs: $strukt) {
                self.data = core::ops::Div::div(*self, rhs).data;
            }
        }
        impl core::ops::DivAssign for &mut $strukt {
            fn div_assign(&mut self, rhs: &mut $strukt) {
                self.data = core::ops::Div::div(**self, rhs).data;
            }
        }
    };
}
macro_rules! impl_fmt_as_f64 {
    ($typ:ty, $f:path) => {
        impl $f for $typ {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <dyn $f>::fmt(&Into::<f64>::into(self), f)
            }
        }
    };
}
macro_rules! impl_fmt_as_inner {
    ($typ:ty, $f:path) => {
        impl $f for $typ {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <dyn $f>::fmt(&self.data, f)
            }
        }
    };
}

macro_rules! impl_unsigned_flops {
    ($typ:ty, $prim:ty, $lower_prim:ty, $shift:ident, $val:ident, $mask:ident) => {
        impl core::ops::Add<f64> for $typ {
            type Output = Self;
            fn add(self, rhs: f64) -> Self::Output {
                let v = <$typ>::from(rhs);
                self + v
            }
        }
        impl core::ops::AddAssign<f64> for $typ {
            fn add_assign(&mut self, rhs: f64) {
                *self = *self + rhs;
            }
        }
        impl core::ops::AddAssign<f64> for &mut $typ {
            fn add_assign(&mut self, rhs: f64) {
                **self = **self + rhs;
            }
        }
        impl core::ops::Sub<f64> for $typ {
            type Output = Self;

            fn sub(self, rhs: f64) -> Self::Output {
                let v = <$typ>::from(rhs);
                self - v
            }
        }
        impl irox_tools::f64::FloatExt for $typ {
            type Type = Self;
            type Size = $prim;

            fn trunc(self) -> Self::Type {
                // just mask out the fractional bits leaving the whole bits.
                Self::from_raw_value(self.data & ($mask << $shift))
            }

            fn fract(self) -> Self::Type {
                // just mask out the whole bits leaving the fractional bits.
                Self::from_raw_value(self.data & $mask)
            }

            fn abs(self) -> Self::Type {
                // no change for unsigned flops.
                self
            }

            fn round(self) -> Self::Type {
                (self + Self::Type::ONE_HALF).trunc()
            }

            fn floor(self) -> Self::Type {
                // same as trunc for unsigned
                self.trunc()
            }

            fn ceil(self) -> Self::Type {
                if Self::Type::fract(&self) == 0 {
                    return self;
                }
                Self::Type::from_parts(self.whole() + 1, 0)
            }

            fn signum(self) -> Self::Type {
                1.into()
            }

            ///
            /// Implementation of Exponential Function from NIST DTMF eq 4.2.19: `<https://dlmf.nist.gov/4.2.E19>`
            fn exp(self) -> Self::Type {
                let mut out = Self::from_parts(1, 0);
                let i = self;
                let mut idx = 1u16;
                let mut next = self;

                while next.abs() != 0.0 {
                    out += next;
                    idx += 1;
                    next *= i / idx;
                }

                out
            }

            ///
            /// Implementation of Natural Logarithm using NIST DLMF eq 4.6.4: `<https://dlmf.nist.gov/4.6.E4>`
            fn ln(self) -> Self::Type {
                let z = self;
                if z == 0. {
                    return Self::Type::from_parts(1, 0);
                }
                let iter = (z - 1u8) / (z + 1u8);
                let mut out = Self::Type::default();
                let mut next = iter * 2u8;
                let mut idx = 1 as $lower_prim;
                let mut base = iter;
                while !next.is_zero() {
                    out += next;
                    idx += 2;
                    base *= iter * iter;
                    next = (base * 2 as $lower_prim) / idx;
                }
                out
            }

            fn log10(self) -> Self::Type {
                self.ln() / Self::LN10
            }

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
                self.powf(0.5.into())
            }
            fn to_bits(self) -> Self::Size {
                self.raw_value()
            }

            fn exponent(self) -> u16 {
                irox_tools::f64::FloatExt::exponent(self.as_f64())
            }

            fn significand(self) -> Self::Size {
                irox_tools::f64::FloatExt::significand(self.as_f64()) as $prim
            }

            fn sin(self) -> Self::Type {
                todo!()
            }

            fn cos(self) -> Self::Type {
                todo!()
            }
        }
    };
}
macro_rules! impl_signed_flops {
    ($typ:ty, $prim:ty, $lower_prim:ty, $shift:ident, $val:ident, $mask:ident) => {
        impl core::ops::Add<f64> for $typ {
            type Output = Self;
            fn add(self, rhs: f64) -> Self::Output {
                let v = <$typ>::from(rhs);
                self + v
            }
        }
        impl core::ops::AddAssign<f64> for $typ {
            fn add_assign(&mut self, rhs: f64) {
                *self = *self + rhs;
            }
        }
        impl core::ops::AddAssign<f64> for &mut $typ {
            fn add_assign(&mut self, rhs: f64) {
                **self = **self + rhs;
            }
        }
        impl core::ops::Sub<f64> for $typ {
            type Output = Self;

            fn sub(self, rhs: f64) -> Self::Output {
                let v = <$typ>::from(rhs);
                self - v
            }
        }
        impl irox_tools::f64::FloatExt for $typ {
            type Type = Self;
            type Size = $prim;

            fn trunc(self) -> Self::Type {
                // just mask out the fractional bits leaving the whole bits.
                Self::from_raw_value(self.data & ($mask << $shift))
            }

            fn fract(self) -> Self::Type {
                // just mask out the whole bits leaving the fractional bits.
                Self::from_raw_value(self.data & $mask)
            }

            fn abs(self) -> Self::Type {
                let bm = $mask | ($mask << $shift);

                Self::from_raw_value(self.data & bm)
            }

            fn round(self) -> Self::Type {
                (self + Self::Type::ONE_HALF).trunc()
            }

            fn floor(self) -> Self::Type {
                todo!()
            }

            fn ceil(self) -> Self::Type {
                if Self::Type::fract(&self) == 0 {
                    return self;
                }
                Self::Type::from_parts(self.whole() + 1, 0)
            }

            fn signum(self) -> Self::Type {
                1.into()
            }

            ///
            /// Implementation of Exponential Function from NIST DTMF eq 4.2.19: `<https://dlmf.nist.gov/4.2.E19>`
            fn exp(self) -> Self::Type {
                let mut out = Self::from_parts(1, 0);
                let i = self;
                let mut idx = 1u16;
                let mut next = self;

                while next.abs() != 0.0 {
                    out += next;
                    idx += 1;
                    next *= i / idx;
                }

                out
            }

            ///
            /// Implementation of Natural Logarithm using NIST DLMF eq 4.6.4: `<https://dlmf.nist.gov/4.6.E4>`
            fn ln(self) -> Self::Type {
                let z = self;
                if z == 0. {
                    return Self::Type::from_parts(1, 0);
                }
                let iter = (z - 1u8) / (z + 1u8);
                let mut out = Self::Type::default();
                let mut next = iter * 2u8;
                let mut idx = 1 as $lower_prim;
                let mut base = iter;
                while !next.is_zero() {
                    out += next;
                    idx += 2;
                    base *= iter * iter;
                    next = (base * 2 as $lower_prim) / idx;
                }
                out
            }
            fn log10(self) -> Self::Type {
                self.ln() / Self::LN10
            }
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
                self.powf(0.5.into())
            }
            fn to_bits(self) -> Self::Size {
                self.raw_value()
            }

            fn exponent(self) -> u16 {
                irox_tools::f64::FloatExt::exponent(self.as_f64())
            }

            fn significand(self) -> Self::Size {
                irox_tools::f64::FloatExt::significand(self.as_f64()) as $prim
            }

            fn sin(self) -> Self::Type {
                todo!()
            }

            fn cos(self) -> Self::Type {
                todo!()
            }
        }
    };
}
macro_rules! impl_prim_ops {
    ($typ:ty, $prim:ty, $rhs:ty) => {
        impl core::ops::Add<$rhs> for $typ {
            type Output = Self;

            fn add(self, rhs: $rhs) -> Self::Output {
                self + Self::from_parts(rhs as $prim, 0)
            }
        }
        impl core::ops::Add<$typ> for $rhs {
            type Output = $typ;

            fn add(self, rhs: $typ) -> Self::Output {
                rhs + self
            }
        }
        impl core::ops::Sub<$rhs> for $typ {
            type Output = Self;

            fn sub(self, rhs: $rhs) -> Self::Output {
                self - Self::from_parts(rhs as $prim, 0)
            }
        }
        impl core::ops::Sub<$typ> for $rhs {
            type Output = $typ;

            fn sub(self, rhs: $typ) -> Self::Output {
                <$typ>::from_parts(self as $prim, 0) - rhs
            }
        }
        impl core::ops::Mul<$rhs> for $typ {
            type Output = Self;

            fn mul(self, rhs: $rhs) -> Self::Output {
                self * Self::from_parts(rhs as $prim, 0)
            }
        }
        impl core::ops::Mul<$typ> for $rhs {
            type Output = $typ;

            fn mul(self, rhs: $typ) -> Self::Output {
                rhs * self
            }
        }
        impl core::ops::Div<$rhs> for $typ {
            type Output = Self;

            fn div(self, rhs: $rhs) -> Self::Output {
                self / Self::from_parts(rhs as $prim, 0)
            }
        }
        impl core::ops::Div<$typ> for $rhs {
            type Output = $typ;

            fn div(self, rhs: $typ) -> Self::Output {
                <$typ>::from_parts(self as $prim, 0) / rhs
            }
        }
        impl core::cmp::PartialEq<$rhs> for $typ {
            fn eq(&self, other: &$rhs) -> bool {
                (*self).eq(&Self::from_parts(*other as $prim, 0))
            }
        }
        impl core::cmp::PartialEq<$typ> for $rhs {
            fn eq(&self, other: &$typ) -> bool {
                <$typ>::from_parts(*self as $prim, 0).eq(other)
            }
        }
        impl core::cmp::PartialOrd<$rhs> for $typ {
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                self.partial_cmp(&Self::from_parts(*other as $prim, 0))
            }
        }
        impl core::cmp::PartialOrd<$typ> for $rhs {
            fn partial_cmp(&self, other: &$typ) -> Option<Ordering> {
                <$typ>::from_parts(*self as $prim, 0).partial_cmp(other)
            }
        }
    };
}
macro_rules! impl_base {
    ($typ:ty, $prim:ty, $lower_prim:ty, $next_prim:ty, $shift:ident, $val:ident, $mask:ident) => {
        impl_fmt_as_f64!($typ, core::fmt::Display);
        impl_fmt_as_f64!($typ, core::fmt::LowerExp);
        impl_fmt_as_f64!($typ, core::fmt::UpperExp);
        impl_fmt_as_inner!($typ, core::fmt::LowerHex);
        impl_fmt_as_inner!($typ, core::fmt::UpperHex);

        impl core::fmt::Debug for $typ {
            fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
                let w = self.whole();
                let p = self.fract();
                f.write_fmt(format_args!("{}: {}/{}", stringify!($typ), w, p))
            }
        }

        impl $typ {
            pub const fn from_parts(whole: $lower_prim, fracs: $lower_prim) -> Self {
                Self {
                    data: (whole as $prim) << $shift | (fracs as $prim),
                }
            }
            pub const fn from_raw_value(data: $prim) -> Self {
                Self { data }
            }
            pub const fn trunc(&self) -> $lower_prim {
                (self.data >> $shift) as $lower_prim
            }
            pub const fn whole(&self) -> $lower_prim {
                self.trunc()
            }
            pub const fn fract(&self) -> $lower_prim {
                (self.data & $mask) as $lower_prim
            }
            pub const fn raw_value(&self) -> $prim {
                self.data
            }
            pub fn as_f64(&self) -> f64 {
                self.into()
            }
            pub const fn is_zero(&self) -> bool {
                self.data == 0 as $prim
            }
        }

        impl From<$prim> for $typ {
            fn from(data: $prim) -> Self {
                Self {
                    data: data << $shift,
                }
            }
        }
        impl From<f64> for $typ {
            fn from(value: f64) -> Self {
                let w = irox_tools::f64::FloatExt::floor(value) as $lower_prim;
                let f = irox_tools::f64::FloatExt::fract(value) * <$lower_prim>::MAX as f64;
                let f = irox_tools::f64::FloatExt::round(f) as $lower_prim;
                Self::from_parts(w, f)
            }
        }
        impl From<f32> for $typ {
            fn from(value: f32) -> Self {
                From::<f64>::from(value as f64)
            }
        }
        impl FromStr for $typ {
            type Err = <f64 as core::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let v = <f64 as core::str::FromStr>::from_str(s)?;
                Ok(v.into())
            }
        }
        impl core::iter::Sum for $typ {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                let mut out = Self::default();
                for v in iter {
                    out += v;
                }
                out
            }
        }
        impl<'a> core::iter::Sum<&'a $typ> for $typ {
            fn sum<I: Iterator<Item = &'a $typ>>(iter: I) -> Self {
                let mut out = Self::default();
                for v in iter {
                    out *= *v;
                }
                out
            }
        }
        impl core::iter::Product for $typ {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                let mut out = Self::default();
                for v in iter {
                    out *= v;
                }
                out
            }
        }
        impl<'a> core::iter::Product<&'a $typ> for $typ {
            fn product<I: Iterator<Item = &'a $typ>>(iter: I) -> Self {
                let mut out = Self::default();
                for v in iter {
                    out *= *v;
                }
                out
            }
        }

        impl_fromf64!($shift, $val, $mask, $typ);
        impl_fromf64!($shift, $val, $mask, &$typ);
        impl_fromf64!($shift, $val, $mask, &mut $typ);
        impl_from!($prim, $shift, $typ);
        impl_from!($prim, $shift, &$typ);
        impl_from!($prim, $shift, &mut $typ);
        impl_ops!($typ, $prim, $next_prim, $shift, $typ);
        impl_ops!($typ, $prim, $next_prim, $shift, &$typ);
        impl_ops!($typ, $prim, $next_prim, $shift, &mut $typ);

        impl_mut_ops!($typ, $prim, $next_prim, $shift);

        impl_partial_cmp_eq!(f64, $typ);
    };
}

///
/// Fixed precision [`u32`] - upper 16 bits is value, lower 16 bits are the fractional portion of the
/// value.  Each fractional portion is 1/[`u16::MAX`] ~= `1.5259e-5` or `0.000_015_259`, or about
/// `15.3 micro`, and can accurately represent SI-prefixes: `milli/1e-3`. The
/// whole portion can represent `0` -> [`u16::MAX`] (`65_535`)
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FixedU32 {
    data: u32,
}
impl FixedU32 {
    pub const E: FixedU32 = FixedU32::from_parts(2, 47_073);
    pub const PI: FixedU32 = FixedU32::from_parts(3, 9_279);
    pub const ONE_HALF: FixedU32 = FixedU32::from_parts(0, 32_768);
    pub const RESOLUTION: FixedU32 = FixedU32::from_parts(0, 1);
    pub const LN10: FixedU32 = FixedU32::from_parts(2, 19_830);
}
impl_base!(FixedU32, u32, u16, u64, U32_SHIFT, U32_VAL, U32_MASK);
impl_prim_ops!(FixedU32, u16, u8);
impl_prim_ops!(FixedU32, u16, u16);
impl_unsigned_flops!(FixedU32, u32, u16, U32_SHIFT, U32_VAL, U32_MASK);

///
/// Fixed precision [`i32`] - upper 16 bits is value, lower 16 bits are the fractional portion of the
/// value.  Each fractional portion is 1/[`u16::MAX`] ~= `1.5259e-5` or `0.000_015_259`, or about
/// `15.3 micro`, and can accurately represent SI-prefixes: `milli/1e-3`. The
/// whole portion can represent [`i16::MIN`] (`-32768`) -> [`i16::MAX`] (`32_727`)
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FixedI32 {
    data: i32,
}
impl_base!(FixedI32, i32, i16, i64, I32_SHIFT, I32_VAL, I32_MASK);
impl_prim_ops!(FixedI32, i16, u8);
impl_prim_ops!(FixedI32, i16, u16);

///
/// Fixed precision [`u64`] - upper 32 bits is value, lower 32 bits are the fractional portion of the
/// value.  Each fractional portion is 1/[`u32::MAX`] ~= `2.328306e-10` or `0.000_000_000_238_306`,
/// or about `238.3 pico`, and can accurately represent SI-prefixes `milli/1e-3`, `micro/1e-6`, and
/// `nano/1e-9`. The whole portion can represent `0` -> [`u32::MAX`] (`4_294_967_295`)
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FixedU64 {
    data: u64,
}
impl FixedU64 {
    pub const E: FixedU64 = FixedU64::from_parts(2, 3_084_996_962);
    pub const PI: FixedU64 = FixedU64::from_parts(3, 608_135_816);
    pub const ONE_HALF: FixedU64 = FixedU64::from_parts(0, 2_147_483_648);
    pub const RESOLUTION: FixedU64 = FixedU64::from_parts(0, 1);
    pub const LN10: FixedU64 = FixedU64::from_parts(2, 1_299_593_075);
}
impl_base!(FixedU64, u64, u32, u128, U64_SHIFT, U64_VAL, U64_MASK);
impl_unsigned_flops!(FixedU64, u64, u32, U64_SHIFT, U64_VAL, U64_MASK);
impl_prim_ops!(FixedU64, u32, u8);
impl_prim_ops!(FixedU64, u32, u16);
impl_prim_ops!(FixedU64, u32, u32);

///
/// Fixed precision [`i64`] - upper 32 bits is value, lower 32 bits are the fractional portion of the
/// value.  Each fractional portion is 1/[`u32::MAX`] ~= `2.328306e-10` or `0.000_000_000_238_306`,
/// or about `238.3 pico`, and can accurately represent SI-prefixes `milli/1e-3`, `micro/1e-6`, and
/// `nano/1e-9`. The whole portion can represent [`i32::MIN`] (`-2_147_483_648`) -> [`i32::MAX`] (`2_147_483_647`)
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FixedI64 {
    data: i64,
}
impl_base!(FixedI64, i64, i32, i128, I64_SHIFT, I64_VAL, I64_MASK);
impl_prim_ops!(FixedI64, i32, u8);
impl_prim_ops!(FixedI64, i32, u16);
impl_prim_ops!(FixedI64, i32, u32);

///
/// Fixed precision [`u128`] - upper 64 bits is value, lower 64 bits are the fractional portion of the
/// value.  Each fractional portion is 1/[`u64::MAX`] ~= `5.4210e-20` or
/// `0.000_000_000_000_000_000_054_210`, or about `54.2 zepto`, and can
/// accurately represent SI-prefixes `milli/1e-3`, `micro/1e-6`, `nano/1e-9`,
/// `pico/1e-12`, `femto/1e-15`, and `atto/1e-18`.  This is probably overkill
/// for what you need.  The whole portion can represent `0` -> [`u64::MAX`] (`1.84467E+19`)
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FixedU128 {
    data: u128,
}
impl FixedU128 {
    pub const E: FixedU128 = FixedU128::from_parts(2, 13_249_961_062_380_153_450);
    pub const PI: FixedU128 = FixedU128::from_parts(3, 2_611_923_443_488_327_891);
    pub const ONE_HALF: FixedU128 = FixedU128::from_parts(0, 9_223_372_036_854_775_808);
    pub const RESOLUTION: FixedU128 = FixedU128::from_parts(0, 1);
    pub const LN10: FixedU128 = FixedU128::from_parts(2, 5_581_709_770_980_770_000);
}
impl_base!(FixedU128, u128, u64, u128, U128_SHIFT, U128_VAL, U128_MASK);
impl_unsigned_flops!(FixedU128, u128, u64, U128_SHIFT, U128_VAL, U128_MASK);
impl_prim_ops!(FixedU128, u64, u8);
impl_prim_ops!(FixedU128, u64, u16);
impl_prim_ops!(FixedU128, u64, u32);
impl_prim_ops!(FixedU128, u64, u64);

///
/// Fixed precision [`u128`] - upper 64 bits is value, lower 64 bits are the fractional portion of the
/// value.  Each fractional portion is 1/[`i64::MAX`] ~= `5.4210e-20` or
/// `0.000_000_000_000_000_000_054_210`, or about `54.2 zepto`, and can
/// accurately represent SI-prefixes `milli/1e-3`, `micro/1e-6`, `nano/1e-9`,
/// `pico/1e-12`, `femto/1e-15`, and `atto/1e-18`.  This is probably overkill
/// for what you need.  The whole portion can represent
/// [`i64::MIN`] (`-9_223_372_036_854_775_808`) -> [`i64::MAX`] (`9_223_372_036_854_775_807`)
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FixedI128 {
    data: i128,
}
impl FixedI128 {
    // pub const E: FixedI128 = FixedI128::from_parts(2, 13_249_961_062_380_153_450);
    pub const PI: FixedI128 = FixedI128::from_parts(3, 2_611_923_443_488_327_891);
    pub const ONE_HALF: FixedI128 = FixedI128::from_parts(0, 9_223_372_036_854_775_807);
    pub const RESOLUTION: FixedI128 = FixedI128::from_parts(0, 1);
    pub const LN10: FixedI128 = FixedI128::from_parts(2, 5_581_709_770_980_770_000);
}
impl_base!(FixedI128, i128, i64, i128, I128_SHIFT, I128_VAL, I128_MASK);
impl_signed_flops!(FixedI128, i128, i64, I128_SHIFT, I128_VAL, I128_MASK);
impl_prim_ops!(FixedI128, i64, u8);
impl_prim_ops!(FixedI128, i64, i8);
impl_prim_ops!(FixedI128, i64, u16);
impl_prim_ops!(FixedI128, i64, i16);
impl_prim_ops!(FixedI128, i64, u32);
impl_prim_ops!(FixedI128, i64, i32);
impl_prim_ops!(FixedI128, i64, u64);
impl_prim_ops!(FixedI128, i64, i64);

pub trait AsFixedPoint {
    fn as_fixed_u32(&self) -> FixedU32;
    fn as_fixed_i32(&self) -> FixedI32;
    fn as_fixed_u64(&self) -> FixedU64;
    fn as_fixed_i64(&self) -> FixedI64;
    fn as_fixed_u128(&self) -> FixedU128;
    fn as_fixed_i128(&self) -> FixedI128;
}
macro_rules! impl_as_fixedpt {
    ($ty:ty) => {
        impl AsFixedPoint for $ty {
            fn as_fixed_u32(&self) -> FixedU32 {
                FixedU32::from(*self)
            }

            fn as_fixed_i32(&self) -> FixedI32 {
                FixedI32::from(*self)
            }

            fn as_fixed_u64(&self) -> FixedU64 {
                FixedU64::from(*self)
            }

            fn as_fixed_i64(&self) -> FixedI64 {
                FixedI64::from(*self)
            }

            fn as_fixed_u128(&self) -> FixedU128 {
                FixedU128::from(*self)
            }

            fn as_fixed_i128(&self) -> FixedI128 {
                FixedI128::from(*self)
            }
        }
    };
}
impl_as_fixedpt!(f32);
impl_as_fixedpt!(f64);
