// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone)]
pub struct NavLibMeasurement {
    channel: u8,
    time_tag: u32,
    sv_id: u8,
    gps_sw_time: f64,
    pseudorange: f64,
    carrier_freq: f32,
    carrier_phase: f64,
    time_in_track: u16,
    sync_flags: u8,
    cno_1: u8,
    cno_2: u8,
    cno_3: u8,
    cno_4: u8,
    cno_5: u8,
    cno_6: u8,
    cno_7: u8,
    cno_8: u8,
    cno_9: u8,
    cno_10: u8,
    delta_range_interval: u16,
    mean_delta_range_time: u16,
    extrapolation_time: i16,
    phase_error_count: u8,
    low_power_count: u8,
}

impl Packet for NavLibMeasurement {
    type PacketType = ();
    type Error = ();

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        todo!()
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
        Ok(NavLibMeasurement {
            channel: input.read_u8()?,
            time_tag: input.read_be_u32()?,
            sv_id: input.read_u8()?,
            gps_sw_time: input.read_f64()?,
            pseudorange: input.read_f64()?,
            carrier_freq: input.read_f32()?,
            carrier_phase: input.read_f64()?,
            time_in_track: input.read_be_u16()?,
            sync_flags: input.read_u8()?,
            cno_1: input.read_u8()?,
            cno_2: input.read_u8()?,
            cno_3: input.read_u8()?,
            cno_4: input.read_u8()?,
            cno_5: input.read_u8()?,
            cno_6: input.read_u8()?,
            cno_7: input.read_u8()?,
            cno_8: input.read_u8()?,
            cno_9: input.read_u8()?,
            cno_10: input.read_u8()?,
            delta_range_interval: input.read_be_u16()?,
            mean_delta_range_time: input.read_be_u16()?,
            extrapolation_time: input.read_be_i16()?,
            phase_error_count: input.read_u8()?,
            low_power_count: input.read_u8()?,
        })
    }
}
