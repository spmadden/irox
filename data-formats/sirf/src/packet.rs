// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::io::Write;

use irox_bits::{Bits, BitsError, BitsErrorKind, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder, PacketData, Packetization};

use crate::error::{Error, ErrorType};
use crate::input::x02_mesnavdata::MeasuredNavigationData;
use crate::input::x04_meastrackdata::MeasuredTrackData;
use crate::input::x07_clockstatus::ClockStatus;
use crate::input::x08_50bpsdata::FiftyBPSData;
use crate::input::x09_cputhroughput::CPUThroughput;
use crate::input::x1c_navmeasure::NavLibMeasurement;
use crate::input::x1e_navsvstate::NavLibSVState;
use crate::input::x29_geonavdata::GeodeticNavigationData;
use crate::input::x32_sbasparams::SBASParameters;
use crate::input::x33x6_trackerload::TrackerLoadStatus;
use crate::input::xff_asciidata::AsciiData;
use crate::input::{
    x02_mesnavdata, x04_meastrackdata, x07_clockstatus, x08_50bpsdata, x09_cputhroughput,
    x1c_navmeasure, x1e_navsvstate, x29_geonavdata, x32_sbasparams, x33x6_trackerload,
    xff_asciidata,
};

pub const START_LEN: usize = 2;
pub const START_SEQ: [u8; 2] = [0xA0, 0xA2];

pub const PAYLOAD_LEN_LEN: usize = 2;
pub const MAX_PAYLOAD_LEN: usize = 1023;
pub const CKSUM_LEN: usize = 2;

pub const END_LEN: usize = 2;
pub const END_SEQ: [u8; 2] = [0xB0, 0xB3];
pub const END_VAL: u16 = 0xB0B3;

pub const MAX_PACKET_SIZE: usize =
    START_LEN + PAYLOAD_LEN_LEN + MAX_PAYLOAD_LEN + CKSUM_LEN + END_LEN;

#[derive(Debug, Clone)]
pub enum PacketType {
    // Messages to the SiRF device
    // AdvancedPowerManagement = 0x35,
    // InitializeDataSource = 0x80,

    // Messages from the SiRF device
    ReferenceNavigationData,
    MeasuredNavigationData(MeasuredNavigationData),
    TrueTrackerData,
    MeasuredTrackingData(MeasuredTrackData),
    RawTrackerDataOut,
    SoftwareVersionString,
    ClockStatusData(ClockStatus),
    FiftyBPSData(FiftyBPSData),
    CPUThroughput(CPUThroughput),

    NavLibMeasurement(NavLibMeasurement),
    NavLibSVState(NavLibSVState),

    GeodeticNavigationData(GeodeticNavigationData),
    SBASParameters(SBASParameters),
    TrackerLoadStatus(TrackerLoadStatus),

    ExtendedEphemeris(u8),

    NavLibraryAuxMsg(u8),
    GPIOStateOutput(u8),
    DOPValues,

    HWCtrlOutput(u8),
    CWInterferenceReport(),
    CWMitigationReport(),
    TCXOLearningOutput(u8),

    StatsUnknown(u8),
    StatsTTFF(),
    StatsTTFF2(),
    DataLogCompatRecord(),
    DataLogTerminator(),
    DataLogStatusOutput(),
    DataLogRecordOutput(),

    AsciiString(AsciiData),

    Unknown(u8, u8),
}

impl Packet for PacketType {
    type PacketType = PacketType;

    fn get_bytes(&self) -> Result<Vec<u8>, BitsError> {
        let mut buf: Vec<u8> = Vec::new();
        self.write_to(&mut buf)?;
        Ok(buf)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

#[derive(Default)]
pub struct Packetizer;
impl Packetizer {
    pub fn new() -> Packetizer {
        Packetizer {}
    }
}
impl<T: Bits> Packetization<T> for Packetizer {
    fn read_next_packet(&mut self, input: &mut T) -> Result<PacketData, BitsError> {
        loop {
            if let Err(e) = read_start(input) {
                if e.kind() != BitsErrorKind::InvalidData {
                    return Err(e);
                }
                input.consume_until(&END_SEQ)?;
                continue;
            }
            break;
        }

        let payload_len = input.read_be_u16()?;
        let payload = input.read_exact_vec(payload_len as usize)?;
        let checksum = input.read_be_u16()?;
        let end = input.read_be_u16()?;

        if !check_checksum(payload.as_slice(), checksum) {
            return Err(BitsErrorKind::InvalidData.into());
        }
        if end != END_VAL {
            return Err(BitsErrorKind::InvalidData.into());
        }
        let mut out: Vec<u8> = Vec::new();
        out.write_be_u16(payload_len)?;
        out.write_all(&payload)?;
        out.write_be_u16(checksum)?;
        out.write_be_u16(end)?;
        Ok(out)
    }
}

pub struct PacketParser;

impl PacketBuilder<PacketType> for PacketParser {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<PacketType, Self::Error> {
        let payload = Packetizer::new().read_next_packet(input)?;

        let (_pld_len, payload) = payload.split_at(2);
        let Some((msg_type, mut payload)) = payload.split_first() else {
            return Err(Error::new(
                ErrorType::InvalidData,
                "Payload length is insufficient",
            ));
        };
        Ok(match msg_type {
            0x02 => PacketType::MeasuredNavigationData(
                x02_mesnavdata::BUILDER.build_from(&mut payload)?,
            ),
            0x04 => PacketType::MeasuredTrackingData(
                x04_meastrackdata::BUILDER.build_from(&mut payload)?,
            ),
            0x07 => PacketType::ClockStatusData(x07_clockstatus::BUILDER.build_from(&mut payload)?),
            0x08 => PacketType::FiftyBPSData(x08_50bpsdata::BUILDER.build_from(&mut payload)?),
            0x09 => PacketType::CPUThroughput(x09_cputhroughput::BUILDER.build_from(&mut payload)?),
            0x1C => {
                PacketType::NavLibMeasurement(x1c_navmeasure::BUILDER.build_from(&mut payload)?)
            }
            0x1E => PacketType::NavLibSVState(x1e_navsvstate::BUILDER.build_from(&mut payload)?),
            0x29 => PacketType::GeodeticNavigationData(
                x29_geonavdata::BUILDER.build_from(&mut payload)?,
            ),
            0x32 => PacketType::SBASParameters(x32_sbasparams::BUILDER.build_from(&mut payload)?),
            0x33 => match payload[0] {
                0x06 => PacketType::TrackerLoadStatus(
                    x33x6_trackerload::BUILDER.build_from(&mut &payload[1..])?,
                ),
                e => PacketType::Unknown(0x33, e),
            },
            0x38 => PacketType::ExtendedEphemeris(payload[0]),
            0x40 => PacketType::NavLibraryAuxMsg(payload[0]),
            0x41 => PacketType::GPIOStateOutput(payload[0]),
            0x5B => PacketType::HWCtrlOutput(payload[0]),
            0x5C => match payload[0] {
                0x01 => PacketType::CWInterferenceReport(),
                0x02 => PacketType::CWMitigationReport(),
                _ => PacketType::Unknown(0x5C, payload[0]),
            },
            0x5D => PacketType::TCXOLearningOutput(payload[0]),
            0xE1 => match payload[0] {
                0x6 => PacketType::StatsTTFF(),
                0x7 => PacketType::StatsTTFF2(),
                0x21 => PacketType::DataLogCompatRecord(),
                0x22 => PacketType::DataLogTerminator(),
                0x23 => PacketType::DataLogStatusOutput(),
                0x24 => PacketType::DataLogRecordOutput(),
                _ => PacketType::StatsUnknown(payload[0]),
            },
            0xFF => PacketType::AsciiString(xff_asciidata::BUILDER.build_from(&mut payload)?),
            e => PacketType::Unknown(*e, 0x0),
        })
    }
}

fn read_start<T: Bits>(input: &mut T) -> Result<(), BitsError> {
    let buf = input.read_exact_vec(START_LEN)?;

    if buf.eq(&START_SEQ) {
        return Ok(());
    }

    Err(BitsErrorKind::InvalidData.into())
}

fn check_checksum(payload: &[u8], checksum: u16) -> bool {
    let mut calc: u16 = 0;
    for val in payload {
        calc = calc.wrapping_add(u16::from(*val));
    }

    calc == checksum
}
