// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::gsa::GNSSSystemID;
use crate::{maybe_latitude, maybe_longitude, maybe_timestamp, Error, MessageType};
use core::str::FromStr;
use irox_bits::Bits;
use irox_carto::altitude::{Altitude, AltitudeReferenceFrame};
use irox_carto::coordinate::{Latitude, Longitude};
use irox_carto::gps::DilutionOfPrecision;
use irox_time::Time;
use irox_tools::options::{MaybeFrom, MaybeInto};
use irox_tools::packetio::{Packet, PacketBuilder};
use irox_units::units::duration::Duration;
use irox_units::units::length::Length;
use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum NavigationalStatus {
    Safe,
    Caution,
    Unsafe,
    NotValidForNavigation,

    #[default]
    Unknown,
}

impl MaybeFrom<Option<&str>> for NavigationalStatus {
    fn maybe_from(value: Option<&str>) -> Option<Self> {
        if let Some(value) = value {
            if let Some(value) = value.chars().next() {
                return Some(match value {
                    'S' => NavigationalStatus::Safe,
                    'C' => NavigationalStatus::Caution,
                    'U' => NavigationalStatus::Unsafe,
                    'V' => NavigationalStatus::NotValidForNavigation,
                    _ => NavigationalStatus::Unknown,
                });
            }
        }
        None
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ModeIndicator {
    Autonomous,
    Differential,
    Estimated,
    RTKFloat,
    ManualInput,
    NoValidFix,
    Precise,
    RTKInteger,
    Simulator,

    #[default]
    UnsetUnknown,
}
impl From<Option<char>> for ModeIndicator {
    fn from(value: Option<char>) -> Self {
        if let Some(value) = value {
            return match value {
                'A' => ModeIndicator::Autonomous,
                'D' => ModeIndicator::Differential,
                'E' => ModeIndicator::Estimated,
                'F' => ModeIndicator::RTKFloat,
                'M' => ModeIndicator::ManualInput,
                'N' => ModeIndicator::NoValidFix,
                'P' => ModeIndicator::Precise,
                'R' => ModeIndicator::RTKInteger,
                'S' => ModeIndicator::Simulator,
                _ => ModeIndicator::UnsetUnknown,
            };
        }
        Default::default()
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct SystemMode {
    gps: ModeIndicator,
    glonass: ModeIndicator,
    galileo: ModeIndicator,
    beidu: ModeIndicator,
    qzss: ModeIndicator,
}
impl Display for SystemMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GPS:{:?} ", self.gps)?;
        write!(f, "GLN:{:?} ", self.glonass)?;
        write!(f, "GAL:{:?} ", self.galileo)?;
        write!(f, "BEI:{:?} ", self.beidu)?;
        write!(f, "QZS:{:?} ", self.qzss)?;
        Ok(())
    }
}
impl From<Option<&str>> for SystemMode {
    fn from(value: Option<&str>) -> Self {
        if let Some(value) = value {
            let mut chars = value.chars();
            return SystemMode {
                gps: chars.next().into(),
                glonass: chars.next().into(),
                galileo: chars.next().into(),
                beidu: chars.next().into(),
                qzss: chars.next().into(),
            };
        }
        SystemMode::default()
    }
}

///
/// GNS - Fix Data
///
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GNS {
    system_id: GNSSSystemID,
    timestamp: Option<Time>,
    latitude: Option<Latitude>,
    longitude: Option<Longitude>,
    mode_indicator: SystemMode,
    total_satellites_in_use: Option<u8>,
    horizontal_dop: Option<DilutionOfPrecision>,
    antenna_altitude: Option<Altitude>,
    geoid_separation: Option<Length>,
    age_of_differential_data: Option<Duration>,
    dgps_station_id: Option<String>,
    nav_status: Option<NavigationalStatus>,
}
impl Display for GNS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "System [{}]", self.system_id)?;
        if let Some(ts) = &self.timestamp {
            write!(f, " Time[{ts}]")?;
        }
        if let Some(latitude) = &self.latitude {
            write!(f, " {latitude}")?;
        }
        if let Some(longitude) = &self.longitude {
            write!(f, " {longitude}")?;
        }
        write!(f, " {}", self.mode_indicator)?;
        if let Some(ts) = &self.total_satellites_in_use {
            write!(f, " Sats[{ts}]")?;
        }
        if let Some(dop) = &self.horizontal_dop {
            write!(f, " DOP[{dop}m]")?;
        }
        if let Some(ant) = &self.antenna_altitude {
            write!(f, " AntAlt[{ant}]")?;
        }
        if let Some(geoid) = &self.geoid_separation {
            write!(f, " GeoidSep[{geoid}]")?;
        }
        if let Some(dgps) = &self.age_of_differential_data {
            write!(f, " DGPSAge[{dgps}]")?;
        }
        if let Some(dgps) = &self.dgps_station_id {
            write!(f, " DGPSStn[{dgps}]")?;
        }
        if let Some(nav) = &self.nav_status {
            write!(f, " NavStatus[{nav:?}]")?;
        }

        Ok(())
    }
}

pub struct GNSBuilder;
impl PacketBuilder<GNS> for GNSBuilder {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<GNS, Self::Error> {
        let buf = input.read_all_str_lossy()?;

        let mut split = buf.split(',');
        let system_id = split
            .next()
            .map(GNSSSystemID::from_sender)
            .unwrap_or_default();
        let timestamp = maybe_timestamp(split.next());
        let latitude = maybe_latitude(split.next(), split.next());
        let longitude = maybe_longitude(split.next(), split.next());
        let mode_indicator = split.next().into();
        let total_satellites_in_use = split.next().and_then(|v| v.parse::<u8>().ok());
        let horizontal_dop = split
            .next()
            .and_then(|v| f64::from_str(v).ok())
            .map(DilutionOfPrecision);
        let antenna_altitude = split
            .next()
            .and_then(|v| f64::from_str(v).ok())
            .map(|v| Altitude::new(Length::new_meters(v), AltitudeReferenceFrame::Geoid));
        let geoid_separation = split
            .next()
            .and_then(|v| f64::from_str(v).ok())
            .map(Length::new_meters);
        let age_of_differential_data = split
            .next()
            .and_then(|v| f64::from_str(v).ok())
            .map(Duration::from_seconds_f64);
        let dgps_station_id = split.next().map(ToString::to_string);
        let nav_status = split.next().maybe_into();

        Ok(GNS {
            system_id,
            timestamp,
            latitude,
            longitude,
            mode_indicator,
            total_satellites_in_use,
            horizontal_dop,
            antenna_altitude,
            geoid_separation,
            age_of_differential_data,
            dgps_station_id,
            nav_status,
        })
    }
}

impl Packet for GNS {
    type PacketType = MessageType;

    fn get_bytes(&self) -> Result<Vec<u8>, irox_bits::Error> {
        todo!()
    }

    fn get_type(&self) -> Self::PacketType {
        MessageType::GNS
    }
}
