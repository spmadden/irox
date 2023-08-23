// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone)]
pub struct CPUThroughput {
    seg_stat_max: u16,
    seg_stat_lat: u16,
    avg_trk_time: u16,
    last_millisecond: u16,
}

impl Packet for CPUThroughput {
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

pub struct CPUThroughputBuilder;
pub static BUILDER: CPUThroughputBuilder = CPUThroughputBuilder;
impl PacketBuilder<CPUThroughput> for CPUThroughputBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<CPUThroughput, Self::Error> {
        Ok(CPUThroughput {
            seg_stat_max: input.read_be_u16()?,
            seg_stat_lat: input.read_be_u16()?,
            avg_trk_time: input.read_be_u16()?,
            last_millisecond: input.read_be_u16()?,
        })
    }
}
