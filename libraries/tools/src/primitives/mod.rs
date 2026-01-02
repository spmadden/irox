// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

pub mod f32;
pub mod f64;
mod u128;
pub mod u16;
pub mod u32;
pub mod u64;
pub mod u8;
mod wrapping;

use crate::f64::FloatExt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use core::str::FromStr;
use irox_bits::{Error, MutBits, WriteToBEBits};
pub use wrapping::*;

///
/// An integer!
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntegerValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
}

impl IntegerValue {
    pub const fn to_be_u128(&self) -> u128 {
        match self {
            IntegerValue::U8(i) => *i as u128,
            IntegerValue::U16(i) => *i as u128,
            IntegerValue::U32(i) => *i as u128,
            IntegerValue::U64(i) => *i as u128,
            IntegerValue::U128(i) => *i,
            IntegerValue::I8(i) => *i as u128,
            IntegerValue::I16(i) => *i as u128,
            IntegerValue::I32(i) => *i as u128,
            IntegerValue::I64(i) => *i as u128,
            IntegerValue::I128(i) => *i as u128,
        }
    }
    pub const fn to_be_u64(&self) -> u64 {
        match self {
            IntegerValue::U8(i) => *i as u64,
            IntegerValue::U16(i) => *i as u64,
            IntegerValue::U32(i) => *i as u64,
            IntegerValue::U64(i) => *i,
            IntegerValue::U128(i) => *i as u64,
            IntegerValue::I8(i) => *i as u64,
            IntegerValue::I16(i) => *i as u64,
            IntegerValue::I32(i) => *i as u64,
            IntegerValue::I64(i) => *i as u64,
            IntegerValue::I128(i) => *i as u64,
        }
    }
    pub const fn to_be_u32(&self) -> u32 {
        self.to_be_u64() as u32
    }

    #[must_use]
    pub const fn to_le(&self) -> Self {
        match self {
            IntegerValue::U8(_) | IntegerValue::I8(_) => *self,
            IntegerValue::U16(b) => IntegerValue::U16(b.to_le()),
            IntegerValue::U32(b) => IntegerValue::U32(b.to_le()),
            IntegerValue::U64(b) => IntegerValue::U64(b.to_le()),
            IntegerValue::U128(b) => IntegerValue::U128(b.to_le()),
            IntegerValue::I16(b) => IntegerValue::I16(b.to_le()),
            IntegerValue::I32(b) => IntegerValue::I32(b.to_le()),
            IntegerValue::I64(b) => IntegerValue::I64(b.to_le()),
            IntegerValue::I128(b) => IntegerValue::I128(b.to_le()),
        }
    }
}
impl WriteToBEBits for IntegerValue {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        match self {
            IntegerValue::U8(i) => i.write_be_to(bits),
            IntegerValue::U16(i) => i.write_be_to(bits),
            IntegerValue::U32(i) => i.write_be_to(bits),
            IntegerValue::U64(i) => i.write_be_to(bits),
            IntegerValue::U128(i) => i.write_be_to(bits),
            IntegerValue::I8(i) => i.write_be_to(bits),
            IntegerValue::I16(i) => i.write_be_to(bits),
            IntegerValue::I32(i) => i.write_be_to(bits),
            IntegerValue::I64(i) => i.write_be_to(bits),
            IntegerValue::I128(i) => i.write_be_to(bits),
        }
    }
}

macro_rules! impl_from_integer {
    ($typ:ty, $($elem:tt)+) => {
        impl From<$typ> for IntegerValue {
            fn from(value: $typ) -> Self {
                $($elem)+(value)
            }
        }
        impl From<&$typ> for IntegerValue {
            fn from(value: &$typ) -> Self {
                $($elem)+(*value)
            }
        }
        impl From<&mut $typ> for IntegerValue {
            fn from(value: &mut $typ) -> Self {
                $($elem)+(*value)
            }
        }
    };
}
impl_from_integer!(u8, IntegerValue::U8);
impl_from_integer!(i8, IntegerValue::I8);
impl_from_integer!(u16, IntegerValue::U16);
impl_from_integer!(i16, IntegerValue::I16);
impl_from_integer!(u32, IntegerValue::U32);
impl_from_integer!(i32, IntegerValue::I32);
impl_from_integer!(u64, IntegerValue::U64);
impl_from_integer!(i64, IntegerValue::I64);
impl_from_integer!(u128, IntegerValue::U128);
impl_from_integer!(i128, IntegerValue::I128);

///
/// Converts this value into a F64 using the `as` cast operation.
pub trait ToF64 {
    fn to_f64(&self) -> f64;
}

pub trait FromF64 {
    fn from_f64(value: f64) -> Self;
}

///
/// Converts this value to the signed version using the `as` cast operation.
pub trait ToSigned {
    type Output;
    fn to_signed(self) -> Self::Output;
    fn negative_one() -> Self::Output;
}

macro_rules! impl_to_signed {
    ($src:ty, $dst:ty) => {
        impl ToSigned for $src {
            type Output = $dst;
            fn to_signed(self) -> Self::Output {
                self as $dst
            }
            fn negative_one() -> Self::Output {
                -1 as $dst
            }
        }
        impl ToSigned for $dst {
            type Output = $dst;
            fn to_signed(self) -> Self::Output {
                self
            }
            fn negative_one() -> Self::Output {
                -1 as $dst
            }
        }
    };
}
impl_to_signed!(u8, i8);
impl_to_signed!(u16, i16);
impl_to_signed!(u32, i32);
impl_to_signed!(u64, i64);
impl_to_signed!(u128, i128);

///
/// Converts this value to the unsigned version using the `as` cast operation.
pub trait ToUnsigned {
    type Output;
    fn to_unsigned(self) -> Self::Output;
}

macro_rules! impl_to_unsigned {
    ($src:ty, $dst:ty) => {
        impl ToUnsigned for $src {
            type Output = $dst;
            fn to_unsigned(self) -> Self::Output {
                self as $dst
            }
        }
        impl ToUnsigned for $dst {
            type Output = $dst;
            fn to_unsigned(self) -> Self::Output {
                self
            }
        }
    };
}
impl_to_unsigned!(i8, u8);
impl_to_unsigned!(i16, u16);
impl_to_unsigned!(i32, u32);
impl_to_unsigned!(i64, u64);
impl_to_unsigned!(i128, u128);

pub trait ToU64 {
    fn to_u64(&self) -> u64;
}
macro_rules! impl_to_u64 {
    ($ty:ty) => {
        impl ToU64 for $ty {
            fn to_u64(&self) -> u64 {
                *self as u64
            }
        }
    };
}
impl_to_u64!(u8);
impl_to_u64!(i8);
impl_to_u64!(u16);
impl_to_u64!(i16);
impl_to_u64!(u32);
impl_to_u64!(i32);
impl_to_u64!(u64);
impl_to_u64!(i64);
impl_to_u64!(u128);
impl_to_u64!(i128);
impl_to_u64!(f32);
impl_to_u64!(f64);
impl_to_u64!(bool);
impl_to_u64!(char);

pub trait One {
    const ONE: Self;
}
pub trait Zero {
    const ZERO: Self;
}

macro_rules! impl_onezero_f {
    ($typ:ty) => {
        impl One for $typ {
            const ONE: Self = 1.0;
        }
        impl Zero for $typ {
            const ZERO: Self = 0.0;
        }
    };
}
impl_onezero_f!(f32);
impl_onezero_f!(f64);

macro_rules! impl_onezero_i {
    ($typ:ty) => {
        impl One for $typ {
            const ONE: Self = 1;
        }
        impl Zero for $typ {
            const ZERO: Self = 0;
        }
    };
}
impl_onezero_i!(u8);
impl_onezero_i!(i8);
impl_onezero_i!(u16);
impl_onezero_i!(i16);
impl_onezero_i!(u32);
impl_onezero_i!(i32);
impl_onezero_i!(u64);
impl_onezero_i!(i64);
impl_onezero_i!(u128);
impl_onezero_i!(i128);

pub trait PrimitiveMath:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
}

pub trait FloatIsh:
    One
    + Zero
    + ToF64
    + FromF64
    + FromStr
    + ToSigned
    + FloatExt
    + Sized
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Default
    + PrimitiveMath
{
}

pub trait Cast<Output = Self> {
    fn cast(self) -> Output;
}
macro_rules! impl_cast {
    ($src:ty, $($dst:ty)*) => {
        $(
            impl Cast<$dst> for $src {

                fn cast(self) -> $dst {
                    self as $dst
                }
            }
        )*
    };
}
impl_cast!(u8 , u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(i8 , u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(u16, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(i16, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(u32, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(i32, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(u64, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(i64, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(u128, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(i128, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(f32, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_cast!(f64, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
