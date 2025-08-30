// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{Bits, Error};
use irox_structs::Struct;
use irox_tools::packetio::{Packet, PacketBuilder};

use crate::packet::PacketType;

#[derive(Default, Debug, Clone, PartialEq, Struct)]
pub struct ClockStatus {
    pub extended_gps_week: u16,

    /// gps time of week, seconds
    pub gps_tow: f64,

    /// sats
    pub svs: u8,

    /// clock drive in hz
    pub clock_drift: u32,

    /// clock bias in ns
    pub clock_bias: u32,

    /// est gps time in ms
    pub est_gps_time: u32,
}

impl Packet for ClockStatus {
    type PacketType = PacketType;

    fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        Struct::as_bytes(self)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct ClockStatusBuilder;
pub static BUILDER: ClockStatusBuilder = ClockStatusBuilder;
impl PacketBuilder<ClockStatus> for ClockStatusBuilder {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<ClockStatus, Self::Error> {
        ClockStatus::parse_from(input)
    }
}
