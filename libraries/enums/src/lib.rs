// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! This crate contains derivable traits for enum types to make them a
//! bit easier to use - more like Java enums
//!

#![no_std]

pub use irox_enums_derive::*;
extern crate alloc;

pub type IntoIter<T> = alloc::vec::IntoIter<T>;

///
/// This trait allows an enum to return it's own 'name' - its identifier.
///
pub trait EnumName {
    fn name(&self) -> &'static str;
}

///
/// This trait allows an enum to return an iterable vector of it's elements.
/// This trait is ONLY derivable on traits who's elements have NO fields
///
pub trait EnumIterItem {
    type Item;
    fn iter_items() -> IntoIter<Self::Item>;
}
