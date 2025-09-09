// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Implementation of the binary on-wire format of Protobuf
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod message;

use irox_bits::{Bits, BitsError, MutBits};
pub use message::*;
pub extern crate irox_bits;

pub trait ProtobufBinary {
    fn write_to<T: MutBits>(&self, output: &mut T) -> Result<usize, BitsError>;
    fn read_from<T: Bits>(input: &mut T) -> Result<Self, BitsError>
    where
        Self: Sized;
}

pub trait ProtobufText {
    fn write_to<T: MutBits>(output: &mut T) -> Result<(), BitsError>;
    fn read_from<T: Bits>(input: &mut T) -> Result<Self, BitsError>
    where
        Self: Sized;
}
