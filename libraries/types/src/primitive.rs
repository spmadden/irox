// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use irox_enums::{EnumIterItem, EnumName, EnumTryFromStr};
use std::char::ParseCharError;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::{ParseFloatError, ParseIntError};
use std::str::{FromStr, ParseBoolError};

///
/// A set of possible primitives
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumName, EnumIterItem, EnumTryFromStr)]
#[non_exhaustive]
#[repr(u8)]
pub enum Primitives {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    f32,
    u64,
    i64,
    f64,
    u128,
    i128,

    bool,
    char,

    null,
}

impl TryFrom<u8> for Primitives {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for v in Primitives::iter_items() {
            if value == v as u8 {
                return Ok(v);
            }
        }
        Err(())
    }
}

pub enum PrimitiveParseError {
    IntError(ParseIntError),
    FloatError(ParseFloatError),
    BoolError(ParseBoolError),
    CharError(ParseCharError),
    StringWasNotNullError,
}

impl From<ParseIntError> for PrimitiveParseError {
    fn from(value: ParseIntError) -> Self {
        PrimitiveParseError::IntError(value)
    }
}

impl From<ParseFloatError> for PrimitiveParseError {
    fn from(value: ParseFloatError) -> Self {
        PrimitiveParseError::FloatError(value)
    }
}

impl From<ParseBoolError> for PrimitiveParseError {
    fn from(value: ParseBoolError) -> Self {
        PrimitiveParseError::BoolError(value)
    }
}

impl From<ParseCharError> for PrimitiveParseError {
    fn from(value: ParseCharError) -> Self {
        PrimitiveParseError::CharError(value)
    }
}

impl Primitives {
    /// Returns the number of bytes required to encode/decode this value.
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn bytes_length(&self) -> usize {
        match self {
            Primitives::u8 => 1,
            Primitives::i8 => 1,
            Primitives::u16 => 2,
            Primitives::i16 => 2,
            Primitives::u32 => 4,
            Primitives::i32 => 4,
            Primitives::f32 => 4,
            Primitives::u64 => 8,
            Primitives::i64 => 8,
            Primitives::f64 => 8,
            Primitives::u128 => 16,
            Primitives::i128 => 16,
            Primitives::bool => 1,
            Primitives::char => 4,
            Primitives::null => 0,
        }
    }

    pub fn try_value_from_str(&self, val: &str) -> Result<PrimitiveValue, PrimitiveParseError> {
        match self {
            Primitives::u8 => Ok(PrimitiveValue::u8(u8::from_str(val)?)),
            Primitives::i8 => Ok(PrimitiveValue::i8(i8::from_str(val)?)),
            Primitives::u16 => Ok(PrimitiveValue::u16(u16::from_str(val)?)),
            Primitives::i16 => Ok(PrimitiveValue::i16(i16::from_str(val)?)),
            Primitives::u32 => Ok(PrimitiveValue::u32(u32::from_str(val)?)),
            Primitives::i32 => Ok(PrimitiveValue::i32(i32::from_str(val)?)),
            Primitives::f32 => Ok(PrimitiveValue::f32(f32::from_str(val)?)),
            Primitives::u64 => Ok(PrimitiveValue::u64(u64::from_str(val)?)),
            Primitives::i64 => Ok(PrimitiveValue::i64(i64::from_str(val)?)),
            Primitives::f64 => Ok(PrimitiveValue::f64(f64::from_str(val)?)),
            Primitives::u128 => Ok(PrimitiveValue::u128(u128::from_str(val)?)),
            Primitives::i128 => Ok(PrimitiveValue::i128(i128::from_str(val)?)),
            Primitives::bool => Ok(PrimitiveValue::bool(bool::from_str(val)?)),
            Primitives::char => Ok(PrimitiveValue::char(char::from_str(val)?)),
            Primitives::null => {
                if "null" == val.to_ascii_lowercase() {
                    Ok(PrimitiveValue::null)
                } else {
                    Err(PrimitiveParseError::StringWasNotNullError)
                }
            }
        }
    }

    #[cfg(feature = "bits")]
    pub fn read_be_from<T: irox_bits::Bits>(
        &self,
        src: &mut T,
    ) -> Result<PrimitiveValue, irox_bits::BitsError> {
        match self {
            Primitives::u8 => Ok(PrimitiveValue::u8(src.read_u8()?)),
            Primitives::i8 => Ok(PrimitiveValue::i8(src.read_i8()?)),
            Primitives::u16 => Ok(PrimitiveValue::u16(src.read_be_u16()?)),
            Primitives::i16 => Ok(PrimitiveValue::i16(src.read_be_i16()?)),
            Primitives::u32 => Ok(PrimitiveValue::u32(src.read_be_u32()?)),
            Primitives::i32 => Ok(PrimitiveValue::i32(src.read_be_i32()?)),
            Primitives::f32 => Ok(PrimitiveValue::f32(src.read_be_f32()?)),
            Primitives::u64 => Ok(PrimitiveValue::u64(src.read_be_u64()?)),
            Primitives::i64 => Ok(PrimitiveValue::i64(src.read_be_i64()?)),
            Primitives::f64 => Ok(PrimitiveValue::f64(src.read_be_f64()?)),
            Primitives::u128 => Ok(PrimitiveValue::u128(src.read_be_u128()?)),
            Primitives::i128 => Ok(PrimitiveValue::i128(src.read_be_i128()?)),
            Primitives::bool => Ok(PrimitiveValue::bool(src.read_bool()?)),
            Primitives::char => Ok(PrimitiveValue::char(src.read_be_utf8_char()?)),
            Primitives::null => Ok(PrimitiveValue::null),
        }
    }
}

///
/// A shuttle struct to pass around a primitive type and an associated value of the same type
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, EnumName)]
#[non_exhaustive]
pub enum PrimitiveValue {
    u8(u8),
    i8(i8),
    u16(u16),
    i16(i16),
    u32(u32),
    i32(i32),
    f32(f32),
    u64(u64),
    i64(i64),
    f64(f64),
    u128(u128),
    i128(i128),
    bool(bool),
    char(char),

    null,
}
impl Hash for PrimitiveValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.primitive().hash(state);
        match self {
            PrimitiveValue::u8(v) => v.hash(state),
            PrimitiveValue::i8(v) => v.hash(state),
            PrimitiveValue::u16(v) => v.hash(state),
            PrimitiveValue::i16(v) => v.hash(state),
            PrimitiveValue::u32(v) => v.hash(state),
            PrimitiveValue::i32(v) => v.hash(state),
            PrimitiveValue::f32(v) => v.to_bits().hash(state),
            PrimitiveValue::u64(v) => v.hash(state),
            PrimitiveValue::i64(v) => v.hash(state),
            PrimitiveValue::f64(v) => v.to_bits().hash(state),
            PrimitiveValue::u128(v) => v.hash(state),
            PrimitiveValue::i128(v) => v.hash(state),
            PrimitiveValue::bool(v) => v.hash(state),
            PrimitiveValue::char(v) => v.hash(state),
            PrimitiveValue::null => {}
        }
    }
}

impl PrimitiveValue {
    /// Returns the type of this primitive
    #[must_use]
    pub const fn primitive(&self) -> Primitives {
        match self {
            PrimitiveValue::u8(_) => Primitives::u8,
            PrimitiveValue::i8(_) => Primitives::i8,
            PrimitiveValue::u16(_) => Primitives::u16,
            PrimitiveValue::i16(_) => Primitives::i16,
            PrimitiveValue::u32(_) => Primitives::u32,
            PrimitiveValue::i32(_) => Primitives::i32,
            PrimitiveValue::f32(_) => Primitives::f32,
            PrimitiveValue::u64(_) => Primitives::u64,
            PrimitiveValue::i64(_) => Primitives::i64,
            PrimitiveValue::f64(_) => Primitives::f64,
            PrimitiveValue::u128(_) => Primitives::u128,
            PrimitiveValue::i128(_) => Primitives::i128,
            PrimitiveValue::bool(_) => Primitives::bool,
            PrimitiveValue::char(_) => Primitives::char,
            PrimitiveValue::null => Primitives::null,
        }
    }

    pub fn as_be_bytes(&self) -> Box<[u8]> {
        match self {
            PrimitiveValue::u8(v) => Box::new([*v]),
            PrimitiveValue::i8(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::u16(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::i16(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::u32(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::i32(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::f32(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::u64(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::i64(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::f64(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::u128(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::i128(v) => Box::new(v.to_be_bytes()),
            PrimitiveValue::bool(v) => {
                if *v {
                    Box::new([1])
                } else {
                    Box::new([0])
                }
            }
            PrimitiveValue::char(v) => {
                let mut buf = [0u8; 4];
                v.encode_utf8(&mut buf);
                Box::new(buf)
            }
            PrimitiveValue::null => Box::new([]),
        }
    }

    #[cfg(feature = "bits")]
    pub fn write_be_to<T: irox_bits::MutBits>(
        &self,
        out: &mut T,
    ) -> Result<(), irox_bits::BitsError> {
        match self {
            PrimitiveValue::u8(v) => out.write_u8(*v),
            PrimitiveValue::i8(v) => out.write_i8(*v),
            PrimitiveValue::u16(v) => out.write_be_u16(*v),
            PrimitiveValue::i16(v) => out.write_be_i16(*v),
            PrimitiveValue::u32(v) => out.write_be_u32(*v),
            PrimitiveValue::i32(v) => out.write_be_i32(*v),
            PrimitiveValue::f32(v) => out.write_be_f32(*v),
            PrimitiveValue::u64(v) => out.write_be_u64(*v),
            PrimitiveValue::i64(v) => out.write_be_i64(*v),
            PrimitiveValue::f64(v) => out.write_be_f64(*v),
            PrimitiveValue::u128(v) => out.write_be_u128(*v),
            PrimitiveValue::i128(v) => out.write_be_i128(*v),
            PrimitiveValue::bool(v) => out.write_bool(*v),
            PrimitiveValue::char(v) => {
                out.write_be_utf8_char(*v)?;
                Ok(())
            }
            PrimitiveValue::null => Ok(()),
        }
    }
}
impl Display for PrimitiveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveValue::u8(v) => write!(f, "{v}"),
            PrimitiveValue::i8(v) => write!(f, "{v}"),
            PrimitiveValue::u16(v) => write!(f, "{v}"),
            PrimitiveValue::i16(v) => write!(f, "{v}"),
            PrimitiveValue::u32(v) => write!(f, "{v}"),
            PrimitiveValue::i32(v) => write!(f, "{v}"),
            PrimitiveValue::f32(v) => write!(f, "{v}"),
            PrimitiveValue::u64(v) => write!(f, "{v}"),
            PrimitiveValue::i64(v) => write!(f, "{v}"),
            PrimitiveValue::f64(v) => write!(f, "{v}"),
            PrimitiveValue::u128(v) => write!(f, "{v}"),
            PrimitiveValue::i128(v) => write!(f, "{v}"),
            PrimitiveValue::bool(v) => write!(f, "{v}"),
            PrimitiveValue::char(v) => write!(f, "{v}"),

            PrimitiveValue::null => write!(f, "null"),
        }
    }
}
macro_rules! impl_into_primivevalue {
    ($ty:ty, $pv:tt) => {
        impl From<$ty> for PrimitiveValue {
            fn from(value: $ty) -> Self {
                PrimitiveValue::$pv(value)
            }
        }
    };
}
impl_into_primivevalue!(u8, u8);
impl_into_primivevalue!(i8, i8);
impl_into_primivevalue!(u16, u16);
impl_into_primivevalue!(i16, i16);
impl_into_primivevalue!(u32, u32);
impl_into_primivevalue!(i32, i32);
impl_into_primivevalue!(f32, f32);
impl_into_primivevalue!(u64, u64);
impl_into_primivevalue!(i64, i64);
impl_into_primivevalue!(f64, f64);
impl_into_primivevalue!(u128, u128);
impl_into_primivevalue!(i128, i128);
impl_into_primivevalue!(bool, bool);
impl_into_primivevalue!(char, char);
impl From<()> for PrimitiveValue {
    fn from((): ()) -> Self {
        PrimitiveValue::null
    }
}

///
/// A struct to "Name" a primitive - like a Field with an associated type
///
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NamedPrimitive {
    name: String,
    primitive: Primitives,
}

impl NamedPrimitive {
    #[must_use]
    pub fn new(name: String, primitive: Primitives) -> NamedPrimitive {
        NamedPrimitive { name, primitive }
    }

    /// Returns the name of the field
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the type of the field
    #[must_use]
    pub fn primitive(&self) -> Primitives {
        self.primitive
    }
}

///
/// A struct to "Name" a primitive with an associated value, like a Field with a value
///
#[derive(Debug, Clone, PartialEq)]
pub struct NamedPrimitiveValue {
    pub(crate) name: String,
    pub(crate) value: PrimitiveValue,
}

impl NamedPrimitiveValue {
    #[must_use]
    pub fn new(name: String, value: PrimitiveValue) -> NamedPrimitiveValue {
        NamedPrimitiveValue { name, value }
    }

    /// Returns the name of this field
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the stored value of the field
    #[must_use]
    pub fn value(&self) -> &PrimitiveValue {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveField {
    pub name: String,
    pub inner: PrimitiveFieldInner,
}
impl PrimitiveField {
    pub fn new_unset(name: &str, primitive: Primitives) -> Self {
        Self {
            name: name.to_string(),
            inner: PrimitiveFieldInner::Unset(primitive),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveFieldInner {
    Unset(Primitives),
    Set(PrimitiveValue),
}
impl PrimitiveFieldInner {
    pub fn primitive(&self) -> Primitives {
        match self {
            PrimitiveFieldInner::Unset(v) => *v,
            PrimitiveFieldInner::Set(v) => v.primitive(),
        }
    }
    pub fn value(&self) -> Option<&PrimitiveValue> {
        if let PrimitiveFieldInner::Set(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
