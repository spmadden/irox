// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Number Traits

/// A number, signed or unsigned, floating point or integer, any size.
pub trait Number:
    core::marker::Sized
    + core::marker::Copy
    + core::marker::Send
    + core::clone::Clone
    + core::default::Default
    + core::fmt::Display
    + core::fmt::Debug
    + core::fmt::LowerExp
    + core::fmt::UpperExp
    + core::ops::Add
    + core::ops::Sub
    + core::ops::Mul
    + core::ops::Div
    + core::ops::Rem
    + core::str::FromStr
    + core::cmp::PartialEq
    + core::cmp::PartialOrd
{
}
/// A mutable number
pub trait NumberMut:
    Number
    + core::ops::AddAssign
    + core::ops::SubAssign
    + core::ops::MulAssign
    + core::ops::DivAssign
    + core::ops::RemAssign
{
}

/// A number that can have bitwise-operations performed on it (IE, not floating point)
pub trait NumberBits:
    Number
    + core::fmt::Binary
    + core::fmt::LowerHex
    + core::fmt::UpperHex
    + core::fmt::Octal
    + core::ops::BitAnd
    + core::ops::BitOr
    + core::ops::BitXor
    + core::ops::Not
    + core::ops::Shl
    + core::ops::Shr
    + core::hash::Hash
{
}
// A mutable bitwise-number (not floating point)
pub trait NumberBitsMut:
    NumberMut
    + NumberBits
    + core::fmt::Binary
    + core::fmt::LowerHex
    + core::fmt::UpperHex
    + core::fmt::Octal
    + core::ops::BitAndAssign
    + core::ops::BitOrAssign
    + core::ops::BitXorAssign
    + core::ops::ShlAssign
    + core::ops::ShrAssign
{
}
/// An exact number (not floating point)
pub trait NumberExact: Number + core::cmp::Eq + core::cmp::Ord + core::hash::Hash {}
/// A floating point number
pub trait NumberFloating: Number + core::fmt::LowerExp + core::fmt::UpperExp {}

/// A signed number
pub trait NumberSigned: Number + core::ops::Neg {}
/// Any unsigned integer
pub trait AnyUnsignedInteger:
    Number + NumberMut + NumberBits + NumberBitsMut + NumberExact
{
    fn as_u8(&self) -> u8;
    fn as_u16(&self) -> u16;
    fn as_u32(&self) -> u32;
    fn as_u64(&self) -> u64;
    fn as_u128(&self) -> u128;
}
pub trait AnySignedInteger: AnyUnsignedInteger + NumberSigned {}
pub trait AnyFloat: Number + NumberMut + NumberFloating + NumberSigned {}
macro_rules! impl_exact {
    ($($typ:ty) *) => {
        $(
            impl<'a> Number for $typ {}
            impl<'a> NumberMut for $typ {}
            impl<'a> NumberBits for $typ {}
            impl<'a> NumberBitsMut for $typ {}
            impl<'a> NumberExact for $typ {}
            impl<'a> AnyUnsignedInteger for $typ {
                fn as_u8(&self) -> u8 {
                    *self as u8
                }
                fn as_u16(&self) -> u16 {
                    *self as u16
                }
                fn as_u32(&self) -> u32 {
                    *self as u32
                }
                fn as_u64(&self) -> u64 {
                    *self as u64
                }
                fn as_u128(&self) -> u128 {
                    *self as u128
                }
            }
        )*
    };
}
macro_rules! impl_float {
    ($($typ:ty) *) => {
        $(
            impl<'a> Number for $typ {}
            impl<'a> NumberMut for $typ {}
            impl<'a> NumberSigned for $typ {}
            impl<'a> NumberFloating for $typ {}
            impl<'a> AnyFloat for $typ {}
        )*
    };
}
macro_rules! impl_signed {
    ($($typ:ty) *) => {
        $(
            impl<'a> NumberSigned for $typ {}
            impl<'a> AnySignedInteger for $typ {}
        )*
    };
}

impl_float!(f64 f32);
impl_exact!(u128 u64 u32 u16 u8 usize isize i8 i16 i32 i64 i128);
impl_signed!(i8 i16 i32 i64 i128 isize);
