// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

use log::error;

use irox_units::units::compass::Azimuth;

use crate::coordinate::Elevation;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SatelliteSignal {
    pub prn: u8,
    pub azimuth: Azimuth,
    pub elevation: Elevation,
    pub snr: u8,
}

impl Display for SatelliteSignal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "PRN: {} Az: {} El: {}, SNR: {}",
            self.prn, self.azimuth, self.elevation, self.snr
        ))
    }
}

/// GPS Fix Type
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum GPSFixType {
    #[default]
    Unknown = 0,
    NoFix = 1,
    TwoDim = 2,
    ThreeDim = 3,
}
impl From<i32> for GPSFixType {
    fn from(value: i32) -> Self {
        match value {
            1 => GPSFixType::NoFix,
            2 => GPSFixType::TwoDim,
            3 => GPSFixType::ThreeDim,
            _ => GPSFixType::Unknown,
        }
    }
}
impl From<Option<&str>> for GPSFixType {
    fn from(value: Option<&str>) -> Self {
        if let Some(value) = value {
            if let Ok(value) = value.parse::<i32>() {
                return value.into();
            }
        }
        GPSFixType::Unknown
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct DilutionOfPrecision(pub f64);
impl From<f64> for DilutionOfPrecision {
    fn from(value: f64) -> Self {
        DilutionOfPrecision(value)
    }
}
impl DilutionOfPrecision {
    pub fn maybe_from(val: &str) -> Option<DilutionOfPrecision> {
        if val.len() == 0 {
            return None;
        }
        match val.parse() {
            Ok(v) => Some(DilutionOfPrecision(v)),
            Err(e) => {
                error!("Error converting DOP: {e:?}");
                None
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct DOPs {
    pub geometric: Option<DilutionOfPrecision>,
    pub horizontal: Option<DilutionOfPrecision>,
    pub position: Option<DilutionOfPrecision>,
    pub time: Option<DilutionOfPrecision>,
    pub vertical: Option<DilutionOfPrecision>,
}

impl DOPs {
    #[must_use]
    pub fn new() -> DOPs {
        Default::default()
    }
}

impl Display for DOPs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let print = |x: Option<DilutionOfPrecision>| match x {
            Some(x) => format!("{:0.3}", x.0),
            None => "?".to_string(),
        };
        write!(
            f,
            "hdop: {} vdop: {} pdop: {} gdop: {} tdop: {}",
            print(self.horizontal),
            print(self.vertical),
            print(self.position),
            print(self.geometric),
            print(self.time)
        )
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    use windows::Devices::Geolocation::Geocoordinate;
    use windows::Foundation::IReference;

    use crate::gps::{DOPs, DilutionOfPrecision};

    impl DOPs {
        pub fn maybe_from(coord: &Geocoordinate) -> Option<DOPs> {
            let Ok(sats) = coord.SatelliteData() else {
                return None;
            };

            let get_dop = |v: IReference<f64>| -> Option<DilutionOfPrecision> {
                v.GetDouble().ok().map(DilutionOfPrecision)
            };
            let geometric = sats.GeometricDilutionOfPrecision().ok().and_then(get_dop);
            let horizontal = sats.HorizontalDilutionOfPrecision().ok().and_then(get_dop);
            let position = sats.PositionDilutionOfPrecision().ok().and_then(get_dop);
            let time = sats.TimeDilutionOfPrecision().ok().and_then(get_dop);
            let vertical = sats.VerticalDilutionOfPrecision().ok().and_then(get_dop);

            Some(DOPs {
                geometric,
                horizontal,
                position,
                time,
                vertical,
            })
        }
    }
}
