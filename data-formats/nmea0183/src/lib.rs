#![forbid(unsafe_code)]

use std::fmt::{Display, Formatter};

use log::trace;

pub use error::*;
use irox_carto::altitude::{Altitude, AltitudeReferenceFrame};
use irox_carto::coordinate::{Latitude, Longitude};
use irox_tools::bits::Bits;
use irox_tools::options::MaybeInto;
use irox_tools::packetio::{Packet, PacketBuilder, PacketData, Packetization};
use irox_tools::read::read_until;
use irox_units::units::angle::Angle;
use irox_units::units::length::{Length, LengthUnits};
pub use output::*;

use crate::gga::GGABuilder;
use crate::gsa::GSABuilder;
use crate::gsv::GSVBuilder;

mod error;
pub mod input;
mod output;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MessageType {
    GGA,
    GLL,
    GSA,
    GSV,
    RMC,
    VTG,
    MSS,
    ZDA,
    SRF103,
    SRF125,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub payload: FramePayload,
    pub raw: Option<String>,
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("NMEA Frame: {}", self.payload))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FramePayload {
    GGA(gga::GGA),
    GSA(gsa::GSA),
    GSV(gsv::GSV),
    Unknown(String),
}

impl Display for FramePayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FramePayload::GGA(gga) => f.write_fmt(format_args!("GGA: {gga}")),
            FramePayload::GSA(gsa) => f.write_fmt(format_args!("GSA: {gsa}")),
            FramePayload::GSV(gsv) => f.write_fmt(format_args!("GSV: {gsv}")),
            FramePayload::Unknown(unk) => f.write_fmt(format_args!("UNK: {unk}")),
        }
    }
}

impl Packet for Frame {
    type PacketType = ();

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        if let Some(raw) = &self.raw {
            return Ok(Vec::from(raw.as_bytes()));
        }
        match &self.payload {
            FramePayload::GGA(gga) => gga.get_bytes(),
            FramePayload::GSA(gsa) => gsa.get_bytes(),
            FramePayload::GSV(gsv) => gsv.get_bytes(),
            FramePayload::Unknown(_) => Err(std::io::ErrorKind::Unsupported.into()),
        }
    }

    fn get_type(&self) -> Self::PacketType {}
}

pub struct NMEAParser;

impl PacketBuilder<Frame> for NMEAParser {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<Frame, Self::Error> {
        let packet = NMEAPacketizer::new().read_next_packet(input)?;
        let raw = String::from_utf8_lossy(&packet).to_string();
        trace!("PKT: {}", raw);

        let key = read_until(&mut packet.as_slice(), &[b','])?;
        let mut pkt = packet.as_slice();
        let payload = if key.ends_with("GGA".as_bytes()) {
            FramePayload::GGA(GGABuilder.build_from(&mut pkt)?)
        } else if key.ends_with("GSA".as_bytes()) {
            FramePayload::GSA(GSABuilder.build_from(&mut pkt)?)
        } else if key.ends_with("GSV".as_bytes()) {
            FramePayload::GSV(GSVBuilder.build_from(&mut pkt)?)
        } else {
            let key = String::from_utf8_lossy(key.as_slice()).to_string();
            FramePayload::Unknown(key)
        };
        Ok(Frame {
            raw: Some(raw),
            payload,
        })
    }
}

#[derive(Default, Copy, Clone)]
pub struct NMEAPacketizer;
impl NMEAPacketizer {
    pub fn new() -> NMEAPacketizer {
        NMEAPacketizer {}
    }
}
impl<T: Bits> Packetization<T> for NMEAPacketizer {
    fn read_next_packet(&mut self, source: &mut T) -> Result<PacketData, std::io::Error> {
        loop {
            let val = source.read_u8()?;
            // search for SOF
            if val == b'$' {
                break;
            }
        }

        let mut packet: Vec<u8> = vec![b'$'];
        packet.append(&mut read_until(source, &[b'\r', b'\n'])?);
        Ok(packet)
    }
}

pub fn calculate_checksum<T: AsRef<[u8]>>(data: &T) -> u8 {
    let mut sl = data.as_ref();
    if sl.starts_with(&[b'$']) {
        (_, sl) = sl.split_at(1);
    }

    let mut out: u8 = 0;
    for v in sl {
        if *v == b'*' {
            break;
        }
        out ^= v;
    }
    out
}

#[allow(clippy::match_same_arms)]
pub(crate) fn maybe_latitude(val: Option<&str>, ns: Option<&str>) -> Option<Latitude> {
    let val = val?;
    if val.len() < 3 {
        return None;
    }
    let deg = &val.get(0..2)?.parse::<f64>().ok()?;
    let min = &val.get(2..)?.parse::<f64>().ok()?;

    let sig = match ns.maybe_into()? {
        'N' | 'n' => 1.0,
        'S' | 's' => -1.0,
        _ => 1.0,
    };

    Some(Latitude(Angle::new_degrees(sig * (deg + min / 60.))))
}
#[allow(clippy::match_same_arms)]
pub(crate) fn maybe_longitude(val: Option<&str>, ew: Option<&str>) -> Option<Longitude> {
    let val = val?;
    if val.len() < 4 {
        return None;
    }
    let deg = &val.get(0..3)?.parse::<f64>().ok()?;
    let min = &val.get(3..)?.parse::<f64>().ok()?;

    let sig = match ew.maybe_into()? {
        'E' | 'e' => 1.0,
        'W' | 'w' => -1.0,
        _ => 1.0,
    };

    Some(Longitude(Angle::new_degrees(sig * (deg + min / 60.))))
}

#[allow(clippy::match_same_arms)]
pub(crate) fn maybe_length(val: Option<&str>, unit: Option<&str>) -> Option<Length> {
    let val = val.maybe_into()?;
    let unit = unit.maybe_into().unwrap_or('M');

    let unit = match unit {
        'M' | 'm' => LengthUnits::Meters,
        'F' | 'f' => LengthUnits::Feet,
        _ => LengthUnits::Meters,
    };
    Some(Length::new(val, unit))
}

pub(crate) fn maybe_altitude(
    val: Option<&str>,
    unit: Option<&str>,
    frame: AltitudeReferenceFrame,
) -> Option<Altitude> {
    Some(Altitude::new(maybe_length(val, unit)?, frame))
}
