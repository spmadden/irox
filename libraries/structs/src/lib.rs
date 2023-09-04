// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

pub use irox_structs_derive::*;
use irox_tools::bits::{Bits, MutBits};

pub trait Struct {
    type ImplType;

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), std::io::Error>;

    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buf: Vec<u8> = Vec::new();
        self.write_to(&mut buf)?;
        Ok(buf)
    }

    fn parse_from<T: Bits>(input: &mut T) -> Result<Self::ImplType, std::io::Error>;
}
