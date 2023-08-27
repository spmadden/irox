// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_structs::Struct;
use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone, Struct)]
pub struct GeodeticNavigationData {
    nav_valid: u16,
    nav_type: u16,
    extended_week_number: u16,
    gps_tow: u32,
    utc_year: u16,
    utc_month: u8,
    utc_day: u8,
    utc_hour: u8,
    utc_minute: u8,
    utc_second: u16,

    satellite_id_list: u32,

    latitude: i32,
    longitude: i32,
    alt_ellipsoid: i32,
    alt_geoid: i32,
    map_datum: u8,
    speed_over_ground: u16,
    course_over_ground: u16,
    mag_var: i16,
    climb_rate: i16,
    heading_rate: i16,

    est_hor_pos_err: u32,
    est_ver_pos_err: u32,
    est_time_err: u32,
    est_hor_vel_err: u16,
    clock_bias: u32,
    clock_bias_err: u32,
    clock_drift: i32,
    clock_drift_err: u32,

    distance: u32,
    distance_err: u16,
    heading_err: u16,

    num_svs: u8,
    hdop: u8,

    mode_info: u8,
}

impl Packet for GeodeticNavigationData {
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

pub struct GeoNavDataBuilder;
pub static BUILDER: GeoNavDataBuilder = GeoNavDataBuilder;
impl PacketBuilder<GeodeticNavigationData> for GeoNavDataBuilder {
    type Error = crate::error::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<GeodeticNavigationData, Self::Error> {
        Ok(GeodeticNavigationData::parse_from(input)?)
    }
}
