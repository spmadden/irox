// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone)]
pub struct TrackerLoadStatus {
    load_state: u32,
    reserved_1: u32,
    load_error: u32,
    time_tag: u32,
}

impl Packet for TrackerLoadStatus {
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

pub struct TrackerLoadBuilder;
pub static BUILDER: TrackerLoadBuilder = TrackerLoadBuilder;
impl PacketBuilder<TrackerLoadStatus> for TrackerLoadBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<TrackerLoadStatus, Self::Error> {
        Ok(TrackerLoadStatus {
            load_state: input.read_be_u32()?,
            reserved_1: input.read_be_u32()?,
            load_error: input.read_be_u32()?,
            time_tag: input.read_be_u32()?,
        })
    }
}
