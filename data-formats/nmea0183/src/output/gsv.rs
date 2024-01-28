// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};
use std::io::Write;

use irox_carto::coordinate::Elevation;
use irox_carto::gps::SatelliteSignal;
use irox_tools::bits::Bits;
use irox_tools::iterators::Itertools;
use irox_tools::options::MaybeInto;
use irox_tools::packetio::{Packet, PacketBuilder};
use irox_tools::vec::PrettyVec;
use irox_units::units::angle::Angle;
use irox_units::units::compass::{Azimuth, CompassReference, RotationDirection};

use crate::{calculate_checksum, Error, MessageType};

#[derive(Debug, Clone, PartialEq)]
pub struct GSV {
    pub sentence_total: u8,
    pub sentence_idx: u8,
    pub sats_in_view: u8,
    pub sat_signals: Vec<SatelliteSignal>,
}

impl Display for GSV {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "SatsInView[{}] Sats: {:#}",
            self.sats_in_view,
            PrettyVec(&self.sat_signals)
        ))
    }
}

impl Packet for GSV {
    type PacketType = MessageType;

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buf: Vec<u8> = Vec::new();

        let num = self.sentence_total;
        let idx = self.sentence_idx;
        let nsats = self.sats_in_view;

        let sats = self
            .sat_signals
            .iter()
            .map(|f| {
                let prn = f.prn;
                let az = f.azimuth.angle().as_degrees().value() as u16;
                let elev = f.elevation.0.as_degrees().value() as u16;
                let snr = f.snr;
                format!("{prn},{elev},{az},{snr}")
            })
            .collect_exact_or(4, String::from(",,,"))
            .join(",");

        buf.write_fmt(format_args!("$GPGSV,{num},{idx},{nsats},{sats}*"))?;
        let cksm = calculate_checksum(&buf);
        buf.write_fmt(format_args!("{cksm:02X}\r\n"))?;

        Ok(buf)
    }

    fn get_type(&self) -> Self::PacketType {
        MessageType::GSV
    }
}

pub struct GSVBuilder;
impl PacketBuilder<GSV> for GSVBuilder {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<GSV, Self::Error> {
        let buf = input.read_all_str_lossy()?;

        let mut split = buf.split(',');
        let _key = split.next();
        let sentence_total = split
            .next()
            .maybe_into()
            .ok_or_else(|| Error::missing("Missing Sentence Total"))?;
        let sentence_idx = split
            .next()
            .maybe_into()
            .ok_or_else(|| Error::missing("Missing Sentence Index"))?;
        let sats_in_view = split
            .next()
            .maybe_into()
            .ok_or_else(|| Error::missing("Missing Sats in View"))?;

        let mut sat_signals = Vec::new();
        loop {
            let prn = split.next();
            let elev = split.next();
            let az = split.next();
            let snr = split.next();
            if snr.is_none() {
                break;
            }
            let Some(prn) = prn.maybe_into() else {
                break;
            };
            let Some(elev) = elev.maybe_into() else {
                break;
            };
            let Some(az) = az.maybe_into() else {
                break;
            };
            let snr = snr.maybe_into().unwrap_or_default();

            let azimuth = Azimuth::new_azimuth(
                Angle::new_degrees(az),
                RotationDirection::PositiveClockwise,
                CompassReference::TrueNorth,
            );
            let elevation = Elevation(Angle::new_degrees(elev));

            sat_signals.push(SatelliteSignal {
                prn,
                snr,
                azimuth,
                elevation,
            })
        }

        Ok(GSV {
            sat_signals,
            sentence_total,
            sentence_idx,
            sats_in_view,
        })
    }
}
