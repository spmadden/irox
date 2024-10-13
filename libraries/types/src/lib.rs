// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! This module contains a rudimentary reflection/type system
//!

mod num;
mod primitive;
mod schema;
#[cfg(feature = "syn")]
mod syn;

use irox_enums_derive::{EnumIterItem, EnumName, EnumTryFromStr};
use std::fmt::{Display, Formatter};

pub use crate::num::*;
pub use crate::primitive::*;
pub use crate::schema::*;
#[cfg(feature = "syn")]
pub use crate::syn::*;

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

impl Display for DynamicallySizedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicallySizedValue::str(v) => write!(f, "{v}"),
            DynamicallySizedValue::u8_blob(v)
            | DynamicallySizedValue::u16_blob(v)
            | DynamicallySizedValue::u32_blob(v)
            | DynamicallySizedValue::u64_blob(v) => write!(f, "{}", String::from_utf8_lossy(v)),
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

impl Display for VariableValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableValue::Primitive(v) => write!(f, "{v}"),
            VariableValue::DynamicallySized(v) => write!(f, "{v}"),
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
