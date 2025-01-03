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

pub trait ToF64 {
    fn to_f64(&self) -> f64;
}
