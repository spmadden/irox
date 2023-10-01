// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

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
    str,
    blob,
    null,
}

///
/// A shuttle struct to pass around a primitive type and an associated value of the same type
///
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, EnumName)]
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
    str(String),
    blob(Box<[u8]>),
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
            PrimitiveValue::str(_) => Primitives::str,
            PrimitiveValue::blob(_) => Primitives::blob,
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
            PrimitiveValue::str(v) => v.to_string(),
            PrimitiveValue::blob(v) => String::from_utf8_lossy(v).to_string(),
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

#[derive(Debug, Copy, Clone, EnumName)]
pub enum PrimitiveType {
    Primitive(Primitives),
    Array(Primitives, usize),
}

#[cfg(feature = "syn")]
mod syn;
