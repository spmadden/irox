// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Copy, Clone, Debug, Default)]
pub struct FiftyBPSData {
    channel: u8,
    sv_id: u8,
    word: [u32; 10],
}

impl Packet for FiftyBPSData {
    type PacketType = ();
    type Error = ();

    fn write_to<T: MutBits>(&self, _out: &mut T) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct FiftyBPSBuilder;
pub static BUILDER: FiftyBPSBuilder = FiftyBPSBuilder;
impl PacketBuilder<FiftyBPSData> for FiftyBPSBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<FiftyBPSData, Self::Error> {
        let channel = input.read_u8()?;
        let sv_id = input.read_u8()?;

        let word: [u32; 10] = [
            input.read_be_u32()?,
            input.read_be_u32()?,
            input.read_be_u32()?,
            input.read_be_u32()?,
            input.read_be_u32()?,
            input.read_be_u32()?,
            input.read_be_u32()?,
            input.read_be_u32()?,
            input.read_be_u32()?,
            input.read_be_u32()?,
        ];
        Ok(FiftyBPSData {
            channel,
            sv_id,
            word,
        })
    }
}
