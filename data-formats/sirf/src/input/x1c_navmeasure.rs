// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_structs::Struct;
use irox_tools::bits::Bits;
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone, Struct)]
pub struct NavLibMeasurement {
    pub channel: u8,
    pub time_tag: u32,
    pub sv_id: u8,
    pub gps_sw_time: f64,
    pub pseudorange: f64,
    pub carrier_freq: f32,
    pub carrier_phase: f64,
    pub time_in_track: u16,
    pub sync_flags: u8,
    pub cno_1: u8,
    pub cno_2: u8,
    pub cno_3: u8,
    pub cno_4: u8,
    pub cno_5: u8,
    pub cno_6: u8,
    pub cno_7: u8,
    pub cno_8: u8,
    pub cno_9: u8,
    pub cno_10: u8,
    pub delta_range_interval: u16,
    pub mean_delta_range_time: u16,
    pub extrapolation_time: i16,
    pub phase_error_count: u8,
    pub low_power_count: u8,
}

impl Packet for NavLibMeasurement {
    type PacketType = ();

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Struct::as_bytes(self)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct NavLibMeasureBuilder;
pub static BUILDER: NavLibMeasureBuilder = NavLibMeasureBuilder;
impl PacketBuilder<NavLibMeasurement> for NavLibMeasureBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<NavLibMeasurement, Self::Error> {
        NavLibMeasurement::parse_from(input)
    }
}
