// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Number Traits

/// A number, signed or unsigned, floating point or integer, any size.
pub trait Number<'a, T: Sized + 'a>:
    core::marker::Sized
    + core::marker::Copy
    + core::marker::Send
    + core::clone::Clone
    + core::default::Default
    + core::fmt::Display
    + core::fmt::Debug
    + core::fmt::LowerExp
    + core::fmt::UpperExp
    + core::ops::Add<Output = T>
    + core::ops::Add<&'a T, Output = T>
    + core::ops::Sub<Output = T>
    + core::ops::Sub<&'a T, Output = T>
    + core::ops::Mul<Output = T>
    + core::ops::Mul<&'a T, Output = T>
    + core::ops::Div<Output = T>
    + core::ops::Div<&'a T, Output = T>
    + core::ops::Rem<Output = T>
    + core::str::FromStr
    + core::cmp::PartialEq
    + core::cmp::PartialOrd
{
}
/// A mutable number
pub trait NumberMut<'a, T: Sized + 'a>:
    Number<'a, T>
    + core::ops::AddAssign
    + core::ops::SubAssign
    + core::ops::MulAssign
    + core::ops::DivAssign
    + core::ops::RemAssign
{
}

/// A number that can have bitwise-operations performed on it (IE, not floating point)
pub trait NumberBits<'a, T: Sized + 'a>:
    Number<'a, T>
    + core::fmt::Binary
    + core::fmt::LowerHex
    + core::fmt::UpperHex
    + core::fmt::Octal
    + core::ops::BitAnd<Output = T>
    + core::ops::BitOr<Output = T>
    + core::ops::BitXor<Output = T>
    + core::ops::Not<Output = T>
    + core::ops::Shl<Output = T>
    + core::ops::Shr<Output = T>
    + core::hash::Hash
{
}
// A mutable bitwise-number (not floating point)
pub trait NumberBitsMut<'a, T: Sized + 'a>:
    NumberMut<'a, T>
    + NumberBits<'a, T>
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
pub trait NumberExact<'a, T: Sized + 'a>:
    Number<'a, T> + core::cmp::Eq + core::cmp::Ord + core::hash::Hash
{
}
/// A floating point number
pub trait NumberFloating<'a, T: Sized + 'a>:
    Number<'a, T> + core::fmt::LowerExp + core::fmt::UpperExp
{
}

/// A signed number
pub trait NumberSigned<'a, T: Sized + 'a>: Number<'a, T> + core::ops::Neg<Output = T> {}
/// Any unsigned integer
pub trait AnyUnsignedInteger<'a, T: Sized + 'a>:
    Number<'a, T> + NumberMut<'a, T> + NumberBits<'a, T> + NumberBitsMut<'a, T> + NumberExact<'a, T>
{
}
pub trait AnySignedInteger<'a, T: Sized + 'a>:
    AnyUnsignedInteger<'a, T> + NumberSigned<'a, T>
{
}
pub trait AnyFloat<'a, T: Sized + 'a>:
    Number<'a, T> + NumberMut<'a, T> + NumberFloating<'a, T> + NumberSigned<'a, T>
{
}
macro_rules! impl_exact {
    ($($typ:ty) *) => {
        $(
            impl<'a> Number<'a, $typ> for $typ {}
            impl<'a> NumberMut<'a, $typ> for $typ {}
            impl<'a> NumberBits<'a, $typ> for $typ {}
            impl<'a> NumberBitsMut<'a, $typ> for $typ {}
            impl<'a> NumberExact<'a, $typ> for $typ {}
            impl<'a> AnyUnsignedInteger<'a, $typ> for $typ {}
        )*
    };
}
macro_rules! impl_float {
    ($($typ:ty) *) => {
        $(
            impl<'a> Number<'a, $typ> for $typ {}
            impl<'a> NumberMut<'a, $typ> for $typ {}
            impl<'a> NumberSigned<'a, $typ> for $typ {}
            impl<'a> NumberFloating<'a, $typ> for $typ {}
            impl<'a> AnyFloat<'a, $typ> for $typ {}
        )*
    };
}
macro_rules! impl_signed {
    ($($typ:ty) *) => {
        $(
            impl<'a> NumberSigned<'a, $typ> for $typ {}
            impl<'a> AnySignedInteger<'a, $typ> for $typ {}
        )*
    };
}

impl_float!(f64 f32);
impl_exact!(u128 u64 u32 u16 u8 usize isize i8 i16 i32 i64 i128);
impl_signed!(i8 i16 i32 i64 i128 isize);
