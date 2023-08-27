// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_structs::Struct;
use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone, Struct)]
pub struct NavLibSVState {
    pub sv_id: u8,
    pub gps_time: f64,
    pub ecef_pos_x: f64,
    pub ecef_pos_y: f64,
    pub ecef_pos_z: f64,
    pub ecef_vel_x: f64,
    pub ecef_vel_y: f64,
    pub ecef_vel_z: f64,
    pub clock_bias: f64,
    pub clock_drift: f32,
    pub ephemeris_flag: u8,
    pub reserved_1: u32,
    pub reserved_2: u32,
    pub ionospheric_delay: f32,
}

impl Packet for NavLibSVState {
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

pub struct NavLibSVStateBuilder;
pub static BUILDER: NavLibSVStateBuilder = NavLibSVStateBuilder;
impl PacketBuilder<NavLibSVState> for NavLibSVStateBuilder {
    type Error = crate::error::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<NavLibSVState, Self::Error> {
        Ok(NavLibSVState::parse_from(input)?)
    }
}
