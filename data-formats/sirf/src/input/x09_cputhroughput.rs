// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_structs::Struct;
use irox_tools::bits::Bits;
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone, Struct)]
pub struct CPUThroughput {
    seg_stat_max: u16,
    seg_stat_lat: u16,
    avg_trk_time: u16,
    last_millisecond: u16,
}

impl Packet for CPUThroughput {
    type PacketType = ();

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Struct::as_bytes(self)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct CPUThroughputBuilder;
pub static BUILDER: CPUThroughputBuilder = CPUThroughputBuilder;
impl PacketBuilder<CPUThroughput> for CPUThroughputBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<CPUThroughput, Self::Error> {
        CPUThroughput::parse_from(input)
    }
}
