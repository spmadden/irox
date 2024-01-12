// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_structs::Struct;
use irox_tools::bits::Bits;
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Copy, Clone, Debug, Default, Struct)]
pub struct FiftyBPSData {
    channel: u8,
    sv_id: u8,
    word: [u32; 10],
}

impl Packet for FiftyBPSData {
    type PacketType = ();

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Struct::as_bytes(self)
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
        FiftyBPSData::parse_from(input)
    }
}
