// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_bits::{Bits, Error};
use irox_structs::Struct;
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NavInvalidError {
    SolutionNotOverdetermined,
    InvalidDRSensorData,
    InvalidDRCalibration,
    UnavailableDRGPSCalibration,
    InvalidDRPositionFix,
    InvalidHeading,
    AlmanacBasedPosition,
    PositionDerivedByReverseEE,
    GPSTextMode,
    TrackerLoading,
    NoTrackerData,

    // These come from the NavType field.
    DOPLimitsExceeded,
    InvalidVelocity,
}

pub enum PositionFixType {
    NoFix,
    Solution1SV,
    Solution2SV,
    Solution3SV,
    Solution4MoreSV,
    Solution2DLeastSquares,
    Solution3DLeastSquares,
    SolutionDR,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Struct)]
pub struct GeodeticNavigationData {
    pub nav_valid: u16,
    pub nav_type: u16,
    pub extended_week_number: u16,
    pub gps_tow: u32,
    pub utc_year: u16,
    pub utc_month: u8,
    pub utc_day: u8,
    pub utc_hour: u8,
    pub utc_minute: u8,
    pub utc_millisecond: u16,

    pub satellite_id_list: u32,

    pub latitude: i32,
    pub longitude: i32,
    pub alt_ellipsoid: i32,
    pub alt_geoid: i32,
    pub map_datum: u8,
    pub speed_over_ground: u16,
    pub course_over_ground: u16,
    pub mag_var: i16,
    pub climb_rate: i16,
    pub heading_rate: i16,

    pub est_hor_pos_err: u32,
    pub est_ver_pos_err: u32,
    pub est_time_err: u32,
    pub est_hor_vel_err: u16,
    pub clock_bias: u32,
    pub clock_bias_err: u32,
    pub clock_drift: i32,
    pub clock_drift_err: u32,

    pub distance: u32,
    pub distance_err: u16,
    pub heading_err: u16,

    pub num_svs: u8,
    pub hdop: u8,

    pub mode_info: u8,
}

impl GeodeticNavigationData {
    pub fn errors(&self) -> Vec<NavInvalidError> {
        let mut errors = Vec::new();
        if self.nav_valid & 0x0001 > 0 {
            errors.push(NavInvalidError::SolutionNotOverdetermined);
        }
        if self.nav_valid & 0x0008 > 0 {
            errors.push(NavInvalidError::InvalidDRSensorData);
        }
        if self.nav_valid & 0x0010 > 0 {
            errors.push(NavInvalidError::InvalidDRCalibration);
        }
        if self.nav_valid & 0x0020 > 0 {
            errors.push(NavInvalidError::UnavailableDRGPSCalibration);
        }
        if self.nav_valid & 0x0040 > 0 {
            errors.push(NavInvalidError::InvalidDRPositionFix);
        }
        if self.nav_valid & 0x0080 > 0 {
            errors.push(NavInvalidError::InvalidHeading);
        }
        if self.nav_valid & 0x0100 > 0 {
            errors.push(NavInvalidError::AlmanacBasedPosition);
        }
        if self.nav_valid & 0x0800 > 0 {
            errors.push(NavInvalidError::PositionDerivedByReverseEE);
        }
        if self.nav_valid & 0x2000 > 0 {
            errors.push(NavInvalidError::GPSTextMode);
        }
        if self.nav_valid & 0x4000 > 0 {
            errors.push(NavInvalidError::TrackerLoading);
        }
        if self.nav_valid & 0x8000 > 0 {
            errors.push(NavInvalidError::NoTrackerData);
        }
        if self.nav_type & 0x0040 > 0 {
            errors.push(NavInvalidError::DOPLimitsExceeded);
        }
        if self.nav_type & 0x1000 > 0 {
            errors.push(NavInvalidError::InvalidVelocity);
        }
        errors
    }

    pub fn fix_type(&self) -> PositionFixType {
        match self.nav_type & 0x7 {
            1 => PositionFixType::Solution1SV,
            2 => PositionFixType::Solution2SV,
            3 => PositionFixType::Solution3SV,
            4 => PositionFixType::Solution4MoreSV,
            5 => PositionFixType::Solution2DLeastSquares,
            6 => PositionFixType::Solution3DLeastSquares,
            7 => PositionFixType::SolutionDR,
            _ => PositionFixType::NoFix,
        }
    }
}

impl Packet for GeodeticNavigationData {
    type PacketType = ();

    fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        Struct::as_bytes(self)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct GeoNavDataBuilder;
pub static BUILDER: GeoNavDataBuilder = GeoNavDataBuilder;
impl PacketBuilder<GeodeticNavigationData> for GeoNavDataBuilder {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<GeodeticNavigationData, Self::Error> {
        GeodeticNavigationData::parse_from(input)
    }
}
