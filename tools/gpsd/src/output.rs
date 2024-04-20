use log::{debug, warn};
use serde::ser::SerializeMap;
use serde::Serializer;

pub use att::*;
pub use gst::*;
use irox_bits::{Bits, MutBits};
use irox_nmea0183::input::pollswver::PollSWVersion;
use irox_nmea0183::input::ratectrl::RateControlRF103;
use irox_nmea0183::MessageType;
use irox_sirf::error::ErrorType;
use irox_tools::options::MaybeInto;
use irox_tools::packetio::{Packet, PacketBuilder};
pub use sky::*;
pub use tpv::*;

use crate::error::GPSdError;
use crate::transport::serial::EncodingType;

pub mod att;
pub mod device;
pub mod gst;
pub mod poll;
pub mod sky;
pub mod tpv;
pub mod version;
pub mod watch;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    /// Name of originating device
    pub device: Option<String>,
    pub payload: FramePayload,
    pub raw: Option<RawEncoding>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawEncoding {
    NMEA0187(Vec<u8>),
    SIRfBinary(Vec<u8>),
    Unknown(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FramePayload {
    /// A TPV object represents a Time/Position value
    TPV(Box<TPV>),

    /// A SKY object reports a sky view of the GPS satellite positions.
    SKY(Box<SKY>),

    /// A GST object is a pseudorange noise report.
    GST(Box<GST>),

    /// An ATT object is a vehicle-attitude report
    ATT(Box<ATT>),
}

impl FramePayload {
    pub fn name(&self) -> &'static str {
        match self {
            FramePayload::TPV(_) => "TPV",
            FramePayload::SKY(_) => "SKY",
            FramePayload::GST(_) => "GST",
            FramePayload::ATT(_) => "ATT",
        }
    }
}

impl serde::ser::Serialize for Frame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("class", self.payload.name())?;

        if let Some(ref device) = self.device {
            map.serialize_entry("device", device)?;
        }

        match &self.payload {
            FramePayload::TPV(t) => TPV::serialize::<S>(t, &mut map)?,
            FramePayload::SKY(_s) => {}
            FramePayload::GST(_g) => {}
            FramePayload::ATT(_a) => {}
        }

        map.end()
    }
}

impl Frame {
    pub fn to_json(&self) -> Result<String, GPSdError> {
        Ok(serde_json::to_string(self)?)
    }
}

pub struct FrameGenerator<'a, T: Bits + MutBits> {
    encoding: EncodingType,
    source: &'a mut T,
}

impl<'a, T: Bits + MutBits> FrameGenerator<'a, T> {
    pub fn new(encoding: EncodingType, source: &'a mut T) -> Self {
        if let EncodingType::Nmea0183 = encoding {
            let _ = PollSWVersion.write_to(source);
            let _ = RateControlRF103::enable_5hz_nav(false).write_to(source);
            let _ = RateControlRF103::enable_sbas_ranging(true).write_to(source);
            let _ = RateControlRF103::enable_fts(true).write_to(source);
            let _ = RateControlRF103::enable_reverseee(false).write_to(source);
            let _ = RateControlRF103::set_rate(MessageType::GGA, 1).write_to(source);
            let _ = RateControlRF103::set_rate(MessageType::GLL, 1).write_to(source);
            let _ = RateControlRF103::set_rate(MessageType::GSA, 5).write_to(source);
            let _ = RateControlRF103::set_rate(MessageType::GSV, 5).write_to(source);
            let _ = RateControlRF103::set_rate(MessageType::MSS, 1).write_to(source);
            let _ = RateControlRF103::set_rate(MessageType::RMC, 1).write_to(source);
            let _ = RateControlRF103::set_rate(MessageType::VTG, 1).write_to(source);
            let _ = RateControlRF103::set_rate(MessageType::ZDA, 10).write_to(source);
        }
        FrameGenerator { encoding, source }
    }
    fn build_from_nmea(&mut self) -> Result<Frame, GPSdError> {
        loop {
            let frame = irox_nmea0183::NMEAParser.build_from(self.source)?;
            debug!("NMEA: {frame}");
            if let Some(frame) = frame.maybe_into() {
                return Ok(frame);
            }
        }
    }
    fn build_from_sirf(&mut self) -> Result<Frame, GPSdError> {
        loop {
            let frame = match irox_sirf::packet::PacketParser.build_from(self.source) {
                Ok(frame) => frame,
                Err(e) => {
                    if e.kind() == ErrorType::InvalidData {
                        warn!("Received invalid packet: {e}");
                        continue;
                    }
                    return Err(e.into());
                }
            };
            debug!("SIRf: {frame:?}");
            if let Some(frame) = frame.maybe_into() {
                return Ok(frame);
            }
        }
    }
    pub fn build_from(&mut self) -> Result<Frame, GPSdError> {
        match self.encoding {
            EncodingType::Nmea0183 => self.build_from_nmea(),
            EncodingType::SirfBinary => self.build_from_sirf(),
        }
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    use irox_winlocation_api::WindowsCoordinate;

    use crate::output::{Frame, FramePayload, TPV};

    impl From<&WindowsCoordinate> for Frame {
        fn from(value: &WindowsCoordinate) -> Self {
            let tpv: TPV = value.into();
            let device = Some(match value.source() {
                Some(s) => {
                    format!("WindowsAPI({s:?})")
                }
                None => String::from("WindowsAPI"),
            });
            Frame {
                device,
                payload: FramePayload::TPV(Box::new(tpv)),
                raw: None,
            }
        }
    }
}
