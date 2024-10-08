// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

pub mod f32;
pub mod f64;
pub mod u16;
pub mod u32;
pub mod u64;
pub mod u8;

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
