use crate::input::x02_mesnavdata::MeasuredNavigationData;
use crate::input::x04_meastrackdata::MeasuredTrackData;
use crate::input::x07_clockstatus::ClockStatus;
use crate::input::x08_50bpsdata::FiftyBPSData;
use crate::input::x1e_navsvstate::NavLibSVState;
use crate::input::x32_sbasparams::SBASParameters;
use crate::input::{
    x02_mesnavdata, x04_meastrackdata, x07_clockstatus, x08_50bpsdata, x1e_navsvstate,
    x32_sbasparams,
};
use irox_tools::bits::Bits;
use irox_tools::packetio::{Packet, PacketBuilder};
use irox_tools::read::{read_exact, read_exact_vec, read_until};
use std::any::Any;
use std::io::{ErrorKind, Read, Write};

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
#[repr(u16)]
pub enum PacketType {
    // Messages to the SiRF device
    // AdvancedPowerManagement = 0x35,
    // InitializeDataSource = 0x80,

    // Messages from the SiRF device
    ReferenceNavigationData = 0x01,
    MeasuredNavigationData(MeasuredNavigationData) = 0x02,
    TrueTrackerData = 0x03,
    MeasuredTrackingData(MeasuredTrackData) = 0x04,
    RawTrackerDataOut = 0x05,
    SoftwareVersionString = 0x06,
    ClockStatusData(ClockStatus) = 0x07,
    FiftyBPSData(FiftyBPSData) = 0x08,

    NavLibSVState(NavLibSVState) = 0x1E,

    SBASParameters(SBASParameters) = 0x32,

    Unknown(u8) = 0x256,
}

impl Packet for PacketType {
    type PacketType = PacketType;
    type Error = std::io::Error;

    fn write_to<T: Write>(&self, out: &mut T) -> Result<(), Self::Error> {
        out.write_all(self.get_bytes()?.as_slice())
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct PacketParser;

impl PacketBuilder<PacketType> for PacketParser {
    type Error = std::io::Error;

    fn build_from<T: Read>(&self, input: &mut T) -> Result<PacketType, Self::Error> {
        loop {
            if let Err(e) = read_start(input) {
                if e.kind() != ErrorKind::InvalidData {
                    return Err(e);
                }
                read_until(input, &END_SEQ)?;
                continue;
            }
            break;
        }

        let payload_len = input.read_be_u16()?;
        let payload = read_exact_vec(input, payload_len as usize)?;
        let checksum = input.read_be_u16()?;
        let end = input.read_be_u16()?;

        if !check_checksum(payload.as_slice(), checksum) {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Invalid checksum",
            ));
        }
        if end != END_VAL {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Invalid packet, missing end bytes",
            ));
        }

        let msg_type = payload[0];
        let mut payload = &mut &payload[1..];
        Ok(match msg_type {
            0x02 => PacketType::MeasuredNavigationData(
                x02_mesnavdata::BUILDER.build_from(&mut payload)?,
            ),
            0x04 => PacketType::MeasuredTrackingData(
                x04_meastrackdata::BUILDER.build_from(&mut payload)?,
            ),
            0x07 => PacketType::ClockStatusData(x07_clockstatus::BUILDER.build_from(&mut payload)?),
            0x08 => PacketType::FiftyBPSData(x08_50bpsdata::BUILDER.build_from(&mut payload)?),
            0x1E => PacketType::NavLibSVState(x1e_navsvstate::BUILDER.build_from(&mut payload)?),
            0x32 => PacketType::SBASParameters(x32_sbasparams::BUILDER.build_from(&mut payload)?),
            e => PacketType::Unknown(e),
        })
    }
}

fn read_start<T: Read>(input: &mut T) -> Result<(), std::io::Error> {
    let buf: [u8; START_LEN] = read_exact(input)?;

    if buf.eq(&START_SEQ) {
        return Ok(());
    }

    Err(std::io::Error::from(ErrorKind::InvalidData))
}

fn check_checksum(payload: &[u8], checksum: u16) -> bool {
    let mut calc: u16 = 0;
    for val in payload {
        calc = calc.wrapping_add(*val as u16);
    }

    calc == checksum
}
