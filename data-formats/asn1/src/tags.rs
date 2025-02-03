// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{BitsError, MutBits};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum TagType {
    PrimitiveTag = 0,
    ConstructedTag = 1,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum TagClass {
    Universal = 0,
    Application = 1,
    ContextSpecific = 2,
    Private = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Tag {
    tag_class: TagClass,
    tag_type: TagType,
    value: u32,
}

impl Tag {
    pub fn encode_to<T: MutBits>(&self, out: &mut T) -> Result<usize, BitsError> {
        let numbits = 32 - self.0.leading_zeros();

        todo!()
    }
}
