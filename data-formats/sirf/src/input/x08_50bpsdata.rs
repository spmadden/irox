// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_structs::Struct;
use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Copy, Clone, Debug, Default, Struct)]
pub struct FiftyBPSData {
    channel: u8,
    sv_id: u8,
    word: [u32; 10],
}

impl Packet for FiftyBPSData {
    type PacketType = ();
    type Error = crate::error::Error;

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Self::Error> {
        Ok(Struct::write_to(self, out)?)
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        Ok(Struct::as_bytes(self)?)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct FiftyBPSBuilder;
pub static BUILDER: FiftyBPSBuilder = FiftyBPSBuilder;
impl PacketBuilder<FiftyBPSData> for FiftyBPSBuilder {
    type Error = crate::error::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<FiftyBPSData, Self::Error> {
        Ok(FiftyBPSData::parse_from(input)?)
    }
}
