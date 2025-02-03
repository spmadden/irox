// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{Bits, BitsError, MutBits};

pub const TAG_BOOLEAN: &[u8] = &[0x01];
pub const TAG_INTEGER: &[u8] = &[0x02];
pub const TAG_BITSTRING: &[u8] = &[0x03];
pub const TAG_OCTETSTRING: &[u8] = &[0x04];
pub const TAG_NULL: &[u8] = &[0x05];
pub const TAG_OID: &[u8] = &[0x06];
pub const TAG_OBJDESC: &[u8] = &[0x07];
pub const TAG_EXTERNAL: &[u8] = &[0x08];
pub const TAG_REAL: &[u8] = &[0x09];
pub const TAG_ENUM: &[u8] = &[0x0A];
pub const TAG_EMB_PDV: &[u8] = &[0x0B];
pub const TAG_UTF8STR: &[u8] = &[0x0C];
pub const TAG_RELOID: &[u8] = &[0x0D];
pub const TAG_TIME: &[u8] = &[0x0E];
pub const TAG_RESVD1: &[u8] = &[0x0F];
pub const TAG_SEQUENCE: &[u8] = &[0x10];
pub const TAG_SET: &[u8] = &[0x11];
pub const TAG_UTCTIME: &[u8] = &[0x17];
pub const TAG_GENTIME: &[u8] = &[0x18];
pub const TAG_DATE: &[u8] = &[0x1F];
pub const TAG_TIMEOFDAY: &[u8] = &[0x20];
pub const TAG_DATETIME: &[u8] = &[0x21];
pub const TAG_DURATION: &[u8] = &[0x22];

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
    pub tag_class: TagClass,
    pub tag_type: TagType,
    pub value: u32,
}

impl Tag {
    pub fn encode_to<T: MutBits>(&self, out: &mut T) -> Result<usize, BitsError> {
        let numbits = 32 - self.value;

        todo!()
    }

    pub fn read_from<T: Bits>(inp: &mut T) -> Result<Tag, BitsError> {
        let a = inp.read_u8()?;
        let ty = a & 0x20;
        let tag_type = if ty == 0 {
            TagType::PrimitiveTag
        } else {
            TagType::ConstructedTag
        };
        let cl = (a & 0xC0) >> 6;
        let tag_class = if cl == 0 {
            TagClass::Universal
        } else if cl == 1 {
            TagClass::Application
        } else if cl == 2 {
            TagClass::ContextSpecific
        } else {
            TagClass::Private
        };
        let tag = a & 0x1F;
        let value = if tag == 0x1F {
            todo!("read length")
        } else {
            tag as u32
        };
        Ok(Tag {
            tag_class,
            tag_type,
            value,
        })
    }
}

pub fn read_length<T: Bits>(inp: &mut T) -> Result<u64, BitsError> {
    let a = inp.read_u8()?;
    if a > 0x80 {
        let num_next = a & 0x7F;
        let mut buf = inp.read_exact_vec(num_next as usize)?;
        let mut out = 0u64;
        let mut shift = 0;
        while let Some(b) = buf.pop() {
            out |= (b as u64) << shift;
            shift += 8;
        }
        Ok(out)
    } else {
        Ok(a as u64)
    }
}
