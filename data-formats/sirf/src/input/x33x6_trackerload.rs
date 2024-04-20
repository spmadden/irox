// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_bits::{Bits, Error};
use irox_structs::Struct;
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone, Struct)]
pub struct TrackerLoadStatus {
    load_state: u32,
    reserved_1: u32,
    load_error: u32,
    time_tag: u32,
}

impl Packet for TrackerLoadStatus {
    type PacketType = ();

    fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        Struct::as_bytes(self)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct TrackerLoadBuilder;
pub static BUILDER: TrackerLoadBuilder = TrackerLoadBuilder;
impl PacketBuilder<TrackerLoadStatus> for TrackerLoadBuilder {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<TrackerLoadStatus, Self::Error> {
        TrackerLoadStatus::parse_from(input)
    }
}
