// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

pub use error::StructError;
pub use irox_structs_derive::*;
use irox_tools::bits::{Bits, MutBits};

pub mod error;

pub trait Struct {
    type ImplType;

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), StructError>;

    fn as_bytes(&self) -> Result<Vec<u8>, StructError> {
        let mut buf: Vec<u8> = Vec::new();
        self.write_to(&mut buf)?;
        Ok(buf)
    }

    fn parse_from<T: Bits>(input: &mut T) -> Result<Self::ImplType, StructError>;
}
