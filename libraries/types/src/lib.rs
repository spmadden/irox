// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! This module contains a rudimentary reflection/type system
//!

use irox_enums_derive::{EnumIterItem, EnumName, EnumTryFromStr};

#[cfg(feature = "syn")]
pub use crate::syn::*;

///
/// A set of possible primitives
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumName, EnumIterItem, EnumTryFromStr)]
#[non_exhaustive]
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
}

impl ToString for PrimitiveValue {
    fn to_string(&self) -> String {
        match self {
            PrimitiveValue::u8(v) => v.to_string(),
            PrimitiveValue::i8(v) => v.to_string(),
            PrimitiveValue::u16(v) => v.to_string(),
            PrimitiveValue::i16(v) => v.to_string(),
            PrimitiveValue::u32(v) => v.to_string(),
            PrimitiveValue::i32(v) => v.to_string(),
            PrimitiveValue::f32(v) => v.to_string(),
            PrimitiveValue::u64(v) => v.to_string(),
            PrimitiveValue::i64(v) => v.to_string(),
            PrimitiveValue::f64(v) => v.to_string(),
            PrimitiveValue::u128(v) => v.to_string(),
            PrimitiveValue::i128(v) => v.to_string(),
            PrimitiveValue::bool(v) => v.to_string(),
            PrimitiveValue::char(v) => v.to_string(),

            PrimitiveValue::null => "null".to_string(),
        }
    }
}

///
/// A struct to "Name" a primitive - like a Field with an associated type
///
#[derive(Debug, Clone, Eq, PartialEq)]
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
    name: String,
    value: PrimitiveValue,
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

///
/// An enumeration to describe the type of a pseudo-primitve
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumName)]
pub enum PrimitiveType {
    Primitive(Primitives),
    Array(Primitives, usize),
    DynamicallySized(VariableType),
}

impl PrimitiveType {
    /// Returns the number of bytes required to encode/decode this value.  If dynamic, returns None.
    #[must_use]
    pub fn bytes_length(&self) -> Option<usize> {
        match self {
            PrimitiveType::Primitive(p) => Some(p.bytes_length()),
            PrimitiveType::Array(p, l) => Some(p.bytes_length() * *l),
            PrimitiveType::DynamicallySized(_) => None,
        }
    }
}

impl From<Primitives> for PrimitiveType {
    fn from(value: Primitives) -> Self {
        PrimitiveType::Primitive(value)
    }
}

impl From<VariableType> for PrimitiveType {
    fn from(value: VariableType) -> Self {
        PrimitiveType::DynamicallySized(value)
    }
}

///
/// An enumeration to describe a variable-length type
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumName, EnumIterItem, EnumTryFromStr)]
#[non_exhaustive]
pub enum VariableType {
    str,
    u8_blob,
    u16_blob,
    u32_blob,
    u64_blob,
}

///
/// An enumeration to store the value of a dynamic/variable sized element
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, EnumName)]
#[non_exhaustive]
pub enum DynamicallySizedValue {
    /// A string
    str(String),
    /// A blob of u8 of max length 255 (u8)
    u8_blob(Vec<u8>),
    /// A blob of u8 of max length 65535 (u16)
    u16_blob(Vec<u8>),
    /// A blob of u8 of max length [`u32::MAX`] (u32)
    u32_blob(Vec<u8>),
    /// A blob of u8 of max length [`u64::MAX`] (u64)
    u64_blob(Vec<u8>),
}

impl DynamicallySizedValue {
    /// Returns the type of this primitive
    #[must_use]
    pub const fn primitive(&self) -> VariableType {
        match self {
            DynamicallySizedValue::str(_) => VariableType::str,
            DynamicallySizedValue::u8_blob(_) => VariableType::u8_blob,
            DynamicallySizedValue::u16_blob(_) => VariableType::u16_blob,
            DynamicallySizedValue::u32_blob(_) => VariableType::u32_blob,
            DynamicallySizedValue::u64_blob(_) => VariableType::u64_blob,
        }
    }
}

impl ToString for DynamicallySizedValue {
    fn to_string(&self) -> String {
        match self {
            DynamicallySizedValue::str(v) => v.to_string(),
            DynamicallySizedValue::u8_blob(v)
            | DynamicallySizedValue::u16_blob(v)
            | DynamicallySizedValue::u32_blob(v)
            | DynamicallySizedValue::u64_blob(v) => String::from_utf8_lossy(v).to_string(),
        }
    }
}

impl From<DynamicallySizedValue> for VariableValue {
    fn from(value: DynamicallySizedValue) -> Self {
        VariableValue::DynamicallySized(value)
    }
}

impl From<PrimitiveValue> for VariableValue {
    fn from(value: PrimitiveValue) -> Self {
        VariableValue::Primitive(value)
    }
}
///
/// A value type that can be either statically sized (primitive) or variably sized (dynamic)
#[derive(Debug, Clone, PartialEq, EnumName)]
pub enum VariableValue {
    Primitive(PrimitiveValue),
    DynamicallySized(DynamicallySizedValue),
}

impl ToString for VariableValue {
    fn to_string(&self) -> String {
        match self {
            VariableValue::Primitive(p) => p.to_string(),
            VariableValue::DynamicallySized(d) => d.to_string(),
        }
    }
}

/// An element that has both a Name and a Type
#[derive(Debug, Clone, PartialEq)]
pub struct NamedVariable {
    name: String,
    ty: PrimitiveType,
}

impl NamedVariable {
    #[must_use]
    pub fn new(name: String, ty: PrimitiveType) -> Self {
        Self { name, ty }
    }

    /// Returns the name of the field
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the type of the field
    #[must_use]
    pub fn variable_type(&self) -> PrimitiveType {
        self.ty
    }
}

/// An element that has both a Name and a Type
#[derive(Debug, Clone, PartialEq)]
pub struct NamedVariableValue {
    name: String,
    value: VariableValue,
}
impl NamedVariableValue {
    #[must_use]
    pub fn new(name: String, value: VariableValue) -> NamedVariableValue {
        NamedVariableValue { name, value }
    }

    /// Returns the name of this field
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the stored value of the field
    #[must_use]
    pub fn value(&self) -> &VariableValue {
        &self.value
    }
}
impl From<NamedPrimitiveValue> for NamedVariableValue {
    fn from(value: NamedPrimitiveValue) -> Self {
        NamedVariableValue {
            name: value.name,
            value: VariableValue::Primitive(value.value),
        }
    }
}

#[cfg(feature = "syn")]
mod syn;
