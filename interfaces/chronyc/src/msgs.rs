// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{Bits, BitsErrorKind, Error, MutBits};
use irox_enums::EnumTryFromRepr;
use irox_structs::Struct;
pub const CURRENT_SUPPORTED_PROTO: u8 = 6;
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumTryFromRepr)]
pub enum ChronycPacketType {
    Request = 1,
    Response = 2,
}
impl Struct for ChronycPacketType {
    type ImplType = Self;

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Error> {
        out.write_u8(*self as u8)
    }

    fn parse_from<T: Bits>(input: &mut T) -> Result<Self::ImplType, Error> {
        Self::try_from(input.read_u8()?).map_err(|()| BitsErrorKind::Unsupported.into())
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChronycRequest {
    pub version: u8,
    pub packet_type: u8,
    pub reserved1: u16,
    pub command: ChronycRequestCommandType,
    pub attempt: u16,
    pub sequence: u32,
    pub pad1: u32,
    pub pad2: u32,

    pub payload: ChronycRequestPayload,
}
impl ChronycRequest {
    pub fn new_num_sources_command() -> Self {
        Self {
            version: CURRENT_SUPPORTED_PROTO,
            packet_type: ChronycPacketType::Request as u8,
            reserved1: 0,
            command: ChronycRequestCommandType::RequestNumSources,
            attempt: 0,
            sequence: 0,
            pad1: 0,
            pad2: 0,
            payload: ChronycRequestPayload::None,
        }
    }
}
impl Struct for ChronycRequest {
    type ImplType = ();

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Error> {
        let mut buf = Vec::<u8>::with_capacity(PAYLOAD_LEN);
        buf.write_u8(self.version)?;
        buf.write_u8(self.packet_type)?;
        buf.write_le_u16(self.reserved1)?;
        buf.write_be_u16(self.command as u16)?;
        buf.write_le_u16(self.attempt)?;
        buf.write_le_u32(self.sequence)?;
        buf.write_le_u32(self.pad1)?;
        buf.write_le_u32(self.pad2)?;

        self.payload.write_to(&mut buf)?;
        buf.resize(PAYLOAD_LEN, 0);
        // buf.resize(32, 0);
        out.write_all_bytes(&buf)?;
        Ok(())
    }

    fn parse_from<T: Bits>(_input: &mut T) -> Result<Self::ImplType, Error> {
        todo!()
    }
}

pub const MAX_DATA_LEN: usize = 396;
pub const PAYLOAD_LEN: usize = 400;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ChronycRequestPayload {
    None,
}
impl ChronycRequestPayload {
    pub const fn get_padding(&self) -> usize {
        match self {
            ChronycRequestPayload::None => MAX_DATA_LEN,
        }
    }
    #[allow(unreachable_patterns)]
    #[allow(clippy::unnecessary_wraps)]
    fn write_to<T: MutBits>(&self, _out: &mut T) -> Result<(), Error> {
        match &self {
            ChronycRequestPayload::None => {
                //noop
            }
            _ => {
                todo!()
            }
        };
        Ok(())
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumTryFromRepr)]
pub enum ChronycRequestCommandType {
    RequestNumSources = 14,
}
impl Struct for ChronycRequestCommandType {
    type ImplType = Self;

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Error> {
        out.write_le_u16(*self as u16)
    }

    fn parse_from<T: Bits>(input: &mut T) -> Result<Self::ImplType, Error> {
        Self::try_from(input.read_le_u16()?).map_err(|()| BitsErrorKind::Unsupported.into())
    }
}
