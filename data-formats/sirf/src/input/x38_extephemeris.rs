// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_bits::{Bits, Error, ErrorKind};
use irox_structs::Struct;
use irox_tools::packetio::{Packet, PacketBuilder};
use log::warn;

#[derive(Default, Debug, Copy, Clone, Struct)]
pub struct GPSDataEphemerisMask {
    pub gps_time_valid_flag: u8,
    pub gps_week: u16,
    pub gps_tow: u32,
    pub ephemeris_request_mask: u32,
}

#[derive(Default, Debug, Copy, Clone, Struct)]
pub struct ExtendedEphemeris2 {
    pub sat_pos_validity_flag: u32,
    pub sat_clk_validity_flag: u32,
    pub sat_health_flag: u32,
}

#[derive(Debug, Copy, Clone)]
pub enum ExtendedEphemerisData {
    GPSDataEphemerisMask(GPSDataEphemerisMask),
    ExtendedEphemeris2(ExtendedEphemeris2),
    ExtendedEphemeris3(),
    ClockBiasAdjustment(),
    EphemerisExtension(),
    EphemerisAck(),
}

impl Packet for ExtendedEphemerisData {
    type PacketType = ();

    #[allow(clippy::match_same_arms)]
    fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(match self {
            ExtendedEphemerisData::GPSDataEphemerisMask(g) => g.as_bytes()?,
            ExtendedEphemerisData::ExtendedEphemeris2(e) => e.as_bytes()?,
            ExtendedEphemerisData::ExtendedEphemeris3() => {
                return Err(ErrorKind::Unsupported.into())
            }
            ExtendedEphemerisData::ClockBiasAdjustment() => {
                return Err(ErrorKind::Unsupported.into())
            }
            ExtendedEphemerisData::EphemerisExtension() => {
                return Err(ErrorKind::Unsupported.into())
            }
            ExtendedEphemerisData::EphemerisAck() => return Err(ErrorKind::Unsupported.into()),
        })
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct ExtendedEphemerisDataBuilder;
pub static BUILDER: ExtendedEphemerisDataBuilder = ExtendedEphemerisDataBuilder;
impl PacketBuilder<ExtendedEphemerisData> for ExtendedEphemerisDataBuilder {
    type Error = crate::error::Error;

    #[allow(clippy::match_same_arms)]
    fn build_from<T: Bits>(&self, input: &mut T) -> Result<ExtendedEphemerisData, Self::Error> {
        use crate::error::Error as CE;
        let submsg = input.read_u8()?;
        match submsg {
            0x01 => {
                return Ok(ExtendedEphemerisData::GPSDataEphemerisMask(
                    GPSDataEphemerisMask::parse_from(input)?,
                ))
            }
            0x02 => {
                return Ok(ExtendedEphemerisData::ExtendedEphemeris2(
                    ExtendedEphemeris2::parse_from(input)?,
                ))
            }
            0x03 => {
                warn!("Encountered unimplemented case: x38x03")
            }
            0x04 => {
                warn!("Encountered unimplemented case: x38x04")
            }
            0x26 => {
                warn!("Encountered unimplemented case: x38x26")
            }
            0xFF => {
                warn!("Encountered unimplemented case: x38xFF")
            }
            _e => {
                return CE::unsupported("Unsupported operation");
            }
        }
        CE::unsupported("Unsupported operation")
    }
}
