// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{Bits, Error};
use irox_structs::Struct;
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone, PartialEq, Struct)]
pub struct CPUThroughput {
    seg_stat_max: u16,
    seg_stat_lat: u16,
    avg_trk_time: u16,
    last_millisecond: u16,
}

impl Packet for CPUThroughput {
    type PacketType = ();

    fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        Struct::as_bytes(self)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct CPUThroughputBuilder;
pub static BUILDER: CPUThroughputBuilder = CPUThroughputBuilder;
impl PacketBuilder<CPUThroughput> for CPUThroughputBuilder {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<CPUThroughput, Self::Error> {
        CPUThroughput::parse_from(input)
    }
}
