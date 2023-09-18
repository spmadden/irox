// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter, Write};
use std::time::Duration;

use time::ext::NumericalDuration;
use time::macros::datetime;
use time::OffsetDateTime;
use windows::Devices::Geolocation::Geocoordinate;

use irox_carto::coordinate::EllipticalCoordinate;
use irox_carto::gps::DOPs;
use irox_units::units::angle::Angle;
use irox_units::units::compass::{CompassReference, RotationDirection, Track};
use irox_units::units::speed::Speed;

pub const WIN_EPOCH: OffsetDateTime = datetime!(1601-01-01 00:00:00 +0);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PositionSource {
    Cellular,
    Satellite,
    WiFi,
    IPAddress,
    Unknown,
    Default,
    Obfuscated,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PositionStatus {
    Ready,
    Initializing,
    NoData,
    Disabled,
    NotInitialized,
    NotAvailable,
    OtherUnknown(i32),
}

impl From<i32> for PositionStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => PositionStatus::Ready,
            1 => PositionStatus::Initializing,
            2 => PositionStatus::NoData,
            3 => PositionStatus::Disabled,
            4 => PositionStatus::NotInitialized,
            5 => PositionStatus::NotAvailable,
            u => PositionStatus::OtherUnknown(u),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct WindowsCoordinate {
    coordinate: Option<EllipticalCoordinate>,
    heading: Option<Track>,
    speed: Option<Speed>,
    timestamp: Option<OffsetDateTime>,
    dops: Option<DOPs>,
    source: Option<PositionSource>,
}

impl WindowsCoordinate {
    #[must_use]
    pub fn coordinate(&self) -> Option<EllipticalCoordinate> {
        self.coordinate
    }

    #[must_use]
    pub fn heading(&self) -> Option<Track> {
        self.heading
    }

    #[must_use]
    pub fn speed(&self) -> Option<Speed> {
        self.speed
    }

    #[must_use]
    pub fn timestamp(&self) -> Option<OffsetDateTime> {
        self.timestamp
    }

    #[must_use]
    pub fn dilution_of_precision(&self) -> Option<DOPs> {
        self.dops
    }

    #[must_use]
    pub fn source(&self) -> Option<PositionSource> {
        self.source
    }
}

#[allow(clippy::match_same_arms)]
impl From<&Geocoordinate> for WindowsCoordinate {
    fn from(value: &Geocoordinate) -> Self {
        let coord = TryFrom::<&Geocoordinate>::try_from(value);
        let mut coordinate: Option<EllipticalCoordinate> = coord.ok();

        let mut heading = None;
        if let Ok(hdg) = value.Heading() {
            if let Ok(hdg) = hdg.GetDouble() {
                if !hdg.is_infinite() && !hdg.is_nan() {
                    let ang = Angle::new_degrees(hdg);
                    let hdg = Track::new_track(
                        ang,
                        RotationDirection::PositiveClockwise,
                        CompassReference::TrueNorth,
                    );
                    heading = Some(hdg)
                }
            }
        }
        let mut speed = None;
        if let Ok(spd) = value.Speed() {
            if let Ok(spd) = spd.GetDouble() {
                if spd.is_finite() && !spd.is_nan() {
                    speed = Some(Speed::new_meters_per_second(spd));
                }
            }
        }

        let mut timestamp = None;
        if let Ok(ts) = value.Timestamp() {
            // jfc.  UniversalTime is the # of 100ns intervals since 01-JAN-1601 00:00:00
            let micros_since_win_epoch = ts.UniversalTime / 10;
            let unix_epoch =
                micros_since_win_epoch.microseconds() - (OffsetDateTime::UNIX_EPOCH - WIN_EPOCH);
            let nanos = unix_epoch.whole_nanoseconds();
            if nanos >= 0 {
                timestamp = OffsetDateTime::UNIX_EPOCH.checked_add(unix_epoch);
            }
        }
        if let Some(coord) = coordinate {
            if coord.get_timestamp().is_none() {
                if let Some(ts) = timestamp {
                    let dur = Duration::from_nanos(ts.unix_timestamp_nanos() as u64);
                    coordinate = Some(coord.with_timestamp(dur));
                }
            }
        }

        let dops: Option<DOPs> = DOPs::maybe_from(value);

        let mut source = None;
        if let Ok(src) = value.PositionSource() {
            source = Some(match src.0 {
                0 => PositionSource::Cellular,
                1 => PositionSource::Satellite,
                2 => PositionSource::WiFi,
                3 => PositionSource::IPAddress,
                4 => PositionSource::Unknown,
                5 => PositionSource::Default,
                6 => PositionSource::Obfuscated,
                _ => PositionSource::Unknown,
            })
        }

        WindowsCoordinate {
            coordinate,
            heading,
            speed,
            timestamp,
            dops,
            source,
        }
    }
}

impl Display for WindowsCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        write!(out, "WindowsCoordinate {{")?;
        if let Some(coord) = self.coordinate {
            writeln!(out, "\tpos: {coord}")?;
        }
        if let Some(hdg) = self.heading {
            writeln!(out, "\thdg: {hdg}")?;
        }
        if let Some(speed) = self.speed {
            writeln!(out, "\tspd: {speed}")?;
        }
        if let Some(dops) = self.dops {
            writeln!(out, "\tdop: {dops}")?;
        }
        if let Some(at) = self.timestamp {
            writeln!(out, "\tat: {at}")?;
        }
        if let Some(source) = self.source {
            writeln!(out, "\tfrom: {source:?}")?;
        }
        write!(out, "}}")?;
        f.write_str(out.as_str())
    }
}
