// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use irox_bits::{BitsError, Error, MutBits, WriteToBEBits};
use irox_tools::buf::Buffer;

#[derive(Default, Debug, Clone, PartialEq, Hash)]
pub struct ProtoMessage {
    pub fields: Vec<ProtoField>,
}
macro_rules! writebemsg {
    ($($elem:ty)*) => {
        impl WriteToBEBits for $($elem)* {
            fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
                let mut wrote = 0;
                for field in &self.fields {
                    wrote += field.write_be_to(bits)?;
                }
                Ok(wrote)
            }
        }
    };
}
writebemsg!(ProtoMessage);
writebemsg!(&ProtoMessage);
writebemsg!(&mut ProtoMessage);

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ProtoField {
    pub name: String,
    pub tag: u64,
    pub data: ProtoFieldData,
}
impl ProtoField {
    pub fn new(name: String, tag: u64, data: ProtoFieldData) -> Self {
        Self { name, tag, data }
    }
    pub fn get_encoded(&mut self) -> Result<Box<[u8]>, BitsError> {
        let mut buf = Vec::<u8>::new();
        WriteToBEBits::write_be_to(&self, &mut buf)?;
        Ok(buf.into_boxed_slice())
    }
}
macro_rules! writebefield {
    ($($elem:ty)*) => {
        impl WriteToBEBits for $($elem)* {
            fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
                let mut wrote = 0;
                let tag = self.tag << 3 | self.data.get_id() as u64;
                let tag = irox_tools::codec::encode_u128bits(tag as u128);
                wrote += tag.write_to(bits)?;
                wrote += WriteToBEBits::write_be_to(&self.data, bits)?;
                Ok(wrote)
            }
        }
    };
}
writebefield!(ProtoField);
writebefield!(&ProtoField);
writebefield!(&mut ProtoField);

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ProtoFieldData {
    VarintDecoded(u128),
    VarintEncoded(Box<[u8]>),
    Fixed64(u64),
    LengthDelimited(Box<[u8]>),
    Fixed32(u32),
}
impl ProtoFieldData {
    pub fn required_length(&mut self) -> usize {
        match self {
            ProtoFieldData::VarintDecoded(f) => {
                let enc = irox_tools::codec::encode_u128bits(*f);
                let enc = enc.into_boxed_slice();
                let len = enc.len();
                *self = ProtoFieldData::VarintEncoded(enc);
                len
            }
            ProtoFieldData::VarintEncoded(ve) => ve.len(),
            ProtoFieldData::Fixed64(_) => 8,
            ProtoFieldData::LengthDelimited(ld) => ld.len(),
            ProtoFieldData::Fixed32(_) => 4,
        }
    }
    pub fn get_id(&self) -> u8 {
        match self {
            ProtoFieldData::VarintDecoded(_) | ProtoFieldData::VarintEncoded(_) => 0,
            ProtoFieldData::Fixed64(_) => 1,
            ProtoFieldData::LengthDelimited(_) => 2,
            ProtoFieldData::Fixed32(_) => 5,
        }
    }
}
impl WriteToBEBits for ProtoFieldData {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let mut wrote = 0;
        match self {
            ProtoFieldData::VarintDecoded(vd) => {
                wrote += irox_tools::codec::encode_u128bits(*vd).write_to(bits)?;
            }
            ProtoFieldData::VarintEncoded(ve) => {
                wrote += WriteToBEBits::write_be_to(ve, bits)?;
            }
            ProtoFieldData::Fixed64(f6) => {
                wrote += WriteToBEBits::write_be_to(f6, bits)?;
            }
            ProtoFieldData::LengthDelimited(ld) => {
                wrote += irox_tools::codec::encode_u128bits(ld.len() as u128).write_to(bits)?;
                wrote += WriteToBEBits::write_be_to(ld, bits)?;
            }
            ProtoFieldData::Fixed32(f3) => {
                wrote += WriteToBEBits::write_be_to(f3, bits)?;
            }
        }
        Ok(wrote)
    }
}

macro_rules! to_varint {
    ($elem:ty) => {
        impl From<$elem> for ProtoFieldData {
            fn from(elem: $elem) -> Self {
                ProtoFieldData::VarintDecoded(elem as u128)
            }
        }
    };
}

to_varint!(u8);
to_varint!(u16);
to_varint!(u32);
to_varint!(u64);
to_varint!(u128);
// to_varint!(i8);
// to_varint!(i16);
// to_varint!(i32);
// to_varint!(i64);
// to_varint!(i128);

impl From<f64> for ProtoFieldData {
    fn from(elem: f64) -> Self {
        ProtoFieldData::Fixed64(elem as u64)
    }
}
impl From<f32> for ProtoFieldData {
    fn from(elem: f32) -> Self {
        ProtoFieldData::Fixed32(elem as u32)
    }
}

#[cfg(test)]
mod tests {
    use crate::message::{ProtoField, ProtoFieldData, ProtoMessage};
    use irox_bits::{BitsError, WriteToBEBits};
    use irox_tools::assert_eq_hex_slice;

    #[test]
    pub fn msg1() -> Result<(), BitsError> {
        let msg1 = ProtoMessage {
            fields: vec![ProtoField::new(
                "a".to_string(),
                1,
                ProtoFieldData::VarintDecoded(150u128),
            )],
        };
        let mut buf = Vec::new();
        msg1.write_be_to(&mut buf)?;
        assert_eq_hex_slice!(&buf, &[0x08, 0x96, 0x01]);
        Ok(())
    }
}
