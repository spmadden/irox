// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use alloc::sync::Arc;
use irox_bits::{Bits, BitsError, Error, MutBits, ReadFromBEBits, WriteToBEBits};
use irox_tools::buf::Buffer;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ProtoMessage {
    pub fields: Vec<ProtoField>,
}
impl ProtoMessage {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_encoded(&mut self) -> Result<Box<[u8]>, BitsError> {
        let mut buf = Vec::<u8>::new();
        WriteToBEBits::write_be_to(&self, &mut buf)?;
        Ok(buf.into_boxed_slice())
    }
    pub fn required_length(&mut self) -> usize {
        self.fields
            .iter_mut()
            .map(ProtoField::required_length)
            .sum()
    }
}
impl ReadFromBEBits for ProtoMessage {
    fn read_from_be_bits<T: Bits>(_inp: &mut T) -> Result<Self, Error> {
        todo!()
    }
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ProtoFieldDescriptor {
    pub tag: u64,
    pub enctype: ProtoFieldType,
    pub name: String,
}
impl ProtoFieldDescriptor {
    pub fn encode_tag(&self) -> Box<[u8]> {
        self.enctype.encode_with_tag(self.tag)
    }
    pub fn required_length(&mut self) -> usize {
        self.encode_tag().len()
    }
    pub fn with_data(&self, data: ProtoFieldData) -> ProtoField {
        ProtoField {
            descriptor: Arc::new(self.clone()),
            data,
        }
    }
    pub fn shared_data(self: &Arc<Self>, data: ProtoFieldData) -> ProtoField {
        ProtoField {
            descriptor: self.clone(),
            data,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ProtoField {
    pub descriptor: Arc<ProtoFieldDescriptor>,
    pub data: ProtoFieldData,
}
impl ProtoField {
    pub fn get_encoded(&mut self) -> Result<Box<[u8]>, BitsError> {
        let mut buf = Vec::<u8>::new();
        WriteToBEBits::write_be_to(&self, &mut buf)?;
        Ok(buf.into_boxed_slice())
    }
    pub fn encode_tag(&self) -> Box<[u8]> {
        self.descriptor.encode_tag()
    }
    pub fn required_length(&mut self) -> usize {
        self.data.required_length() + self.encode_tag().len()
    }
}
impl ReadFromBEBits for ProtoField {
    fn read_from_be_bits<T: Bits>(_inp: &mut T) -> Result<Self, Error> {
        todo!()
    }
}
macro_rules! writebefield {
    ($($elem:ty)*) => {
        impl WriteToBEBits for $($elem)* {
            fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
                let mut wrote = 0;
                let tag = self.encode_tag();
                wrote += tag.write_be_to(bits)?;
                wrote += WriteToBEBits::write_be_to(&self.data, bits)?;
                Ok(wrote)
            }
        }
    };
}
writebefield!(ProtoField);
writebefield!(&ProtoField);
writebefield!(&mut ProtoField);

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ProtoFieldType {
    Varint = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 5,
}
impl ProtoFieldType {
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(ProtoFieldType::Varint),
            1 => Some(ProtoFieldType::Fixed64),
            2 => Some(ProtoFieldType::LengthDelimited),
            5 => Some(ProtoFieldType::Fixed32),
            _ => None,
        }
    }
    pub fn to_id(&self) -> u8 {
        match self {
            ProtoFieldType::Varint => 0,
            ProtoFieldType::Fixed64 => 1,
            ProtoFieldType::LengthDelimited => 2,
            ProtoFieldType::Fixed32 => 5,
        }
    }
    pub fn encode_with_tag(&self, tag: u64) -> Box<[u8]> {
        let tag = (tag << 3) | self.to_id() as u64;
        let tag = irox_tools::codec::encode_u128bits(tag as u128);
        tag.into_boxed_slice()
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ProtoFieldData {
    VarintDecoded(u128),
    VarintEncoded(Box<[u8]>),
    Fixed64(u64),
    LengthDelimited(Box<[u8]>),
    Fixed32(u32),
    RawFields(ProtoMessage),
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
            ProtoFieldData::RawFields(msg) => msg.required_length(),
        }
    }
    pub fn get_id(&self) -> u8 {
        match self {
            ProtoFieldData::VarintDecoded(_) | ProtoFieldData::VarintEncoded(_) => 0,
            ProtoFieldData::Fixed64(_) => 1,
            ProtoFieldData::LengthDelimited(_) | ProtoFieldData::RawFields(_) => 2,
            ProtoFieldData::Fixed32(_) => 5,
        }
    }
    pub fn get_type(&self) -> ProtoFieldType {
        match self {
            ProtoFieldData::VarintDecoded(_) | ProtoFieldData::VarintEncoded(_) => {
                ProtoFieldType::Varint
            }
            ProtoFieldData::Fixed64(_) => ProtoFieldType::Fixed64,
            ProtoFieldData::LengthDelimited(_) | ProtoFieldData::RawFields(_) => {
                ProtoFieldType::LengthDelimited
            }
            ProtoFieldData::Fixed32(_) => ProtoFieldType::Fixed32,
        }
    }
}
impl WriteToBEBits for ProtoFieldData {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        let mut wrote = 0;
        match self {
            ProtoFieldData::VarintDecoded(vd) => {
                let enc = irox_tools::codec::encode_u128bits(*vd);
                wrote += enc.write_to(bits)?;
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
            ProtoFieldData::RawFields(fields) => {
                fields.write_be_to(bits)?;
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
pub trait ToProtoFieldData {
    fn to_proto_field_data(&self) -> ProtoFieldData;
    fn to_proto_field(&self, name: &str, tag: u64) -> ProtoField {
        let data = self.to_proto_field_data();
        let enctype = data.get_type();
        ProtoField {
            descriptor: Arc::new(ProtoFieldDescriptor {
                tag,
                enctype,
                name: name.to_string(),
            }),
            data,
        }
    }
}
macro_rules! impltoprotofield {
    ($elem:ty) => {
        impl ToProtoFieldData for $elem {
            fn to_proto_field_data(&self) -> ProtoFieldData {
                ProtoFieldData::VarintDecoded(*self as u128)
            }
        }
    };
}
impltoprotofield!(u8);
impltoprotofield!(u16);
impltoprotofield!(u32);
impltoprotofield!(u64);
impltoprotofield!(u128);
impltoprotofield!(usize);
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
    use crate::{ProtoFieldDescriptor, ProtoFieldType};
    use irox_bits::{BitsError, WriteToBEBits};
    use irox_tools::assert_eq_hex_slice;
    use std::sync::Arc;

    #[test]
    pub fn msg1() -> Result<(), BitsError> {
        let msg1 = ProtoMessage {
            fields: vec![ProtoField {
                descriptor: Arc::new(ProtoFieldDescriptor {
                    tag: 1,
                    enctype: ProtoFieldType::Varint,
                    name: "a".to_string(),
                }),
                data: ProtoFieldData::VarintDecoded(150u128),
            }],
        };
        let mut buf = Vec::new();
        msg1.write_be_to(&mut buf)?;
        assert_eq_hex_slice!(&buf, &[0x08, 0x96, 0x01]);
        Ok(())
    }
}
