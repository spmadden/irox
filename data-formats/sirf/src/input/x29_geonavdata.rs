// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone)]
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

pub struct GeoNavDataBuilder;
pub static BUILDER: GeoNavDataBuilder = GeoNavDataBuilder;
impl PacketBuilder<GeodeticNavigationData> for GeoNavDataBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<GeodeticNavigationData, Self::Error> {
        Ok(GeodeticNavigationData {
            nav_valid: input.read_be_u16()?,
            nav_type: input.read_be_u16()?,
            extended_week_number: input.read_be_u16()?,
            gps_tow: input.read_be_u32()?,
            utc_year: input.read_be_u16()?,
            utc_month: input.read_u8()?,
            utc_day: input.read_u8()?,
            utc_hour: input.read_u8()?,
            utc_minute: input.read_u8()?,
            utc_second: input.read_be_u16()?,
            satellite_id_list: input.read_be_u32()?,
            latitude: input.read_be_i32()?,
            longitude: input.read_be_i32()?,
            alt_ellipsoid: input.read_be_i32()?,
            alt_geoid: input.read_be_i32()?,
            map_datum: input.read_u8()?,
            speed_over_ground: input.read_be_u16()?,
            course_over_ground: input.read_be_u16()?,
            mag_var: input.read_be_i16()?,
            climb_rate: input.read_be_i16()?,
            heading_rate: input.read_be_i16()?,
            est_hor_pos_err: input.read_be_u32()?,
            est_ver_pos_err: input.read_be_u32()?,
            est_time_err: input.read_be_u32()?,
            est_hor_vel_err: input.read_be_u16()?,
            clock_bias: input.read_be_u32()?,
            clock_bias_err: input.read_be_u32()?,
            clock_drift: input.read_be_i32()?,
            clock_drift_err: input.read_be_u32()?,
            distance: input.read_be_u32()?,
            distance_err: input.read_be_u16()?,
            heading_err: input.read_be_u16()?,
            num_svs: input.read_u8()?,
            hdop: input.read_u8()?,
            mode_info: input.read_u8()?,
        })
    }
}
