use serde::ser::SerializeMap;
use serde::Serializer;
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

use irox_carto::altitude::AltitudeReferenceFrame;
use irox_carto::coordinate::{CartesianCoordinate, EllipticalCoordinate, PositionUncertainty};
use irox_units::units::angle::Angle;
use irox_units::units::compass::{CompassReference, Heading, RelativeBearing, Track};
use irox_units::units::length::Length;
use irox_units::units::speed::Speed;

/// NMEA Mode Type
#[derive(Copy, Clone, Debug, Default)]
pub enum NMEAMode {
    #[default]
    Unknown = 0,
    NoFix = 1,
    TwoDim = 2,
    ThreeDim = 3,
}

/// GPS Fix Status
#[derive(Copy, Clone, Debug, Default)]
pub enum FixStatus {
    #[default]
    Unknown = 0,
    Normal = 1,
    DGPS = 2,
    RTKFixed = 3,
    RTKFloating = 4,
    DR = 5,
    GNSSDR = 6,
    TimeSurveyed = 7,
    Simulated = 8,
    PY = 9,
}

/// Time, Position, Velocity
#[derive(Debug, Default, Clone)]
pub struct TPV {
    /// NMEA mode
    pub mode: NMEAMode,

    /// GPS Fix status
    pub status: Option<FixStatus>,

    /// Climb (positive) or sink (negative) rate, meters per second.
    pub climb: Option<Speed>,

    /// Current datum. Hopefully WGS84.
    /// Pulled from the Coordinate
    // datum: Option<String>,

    /// Depth in meters. Probably depth below the keel...
    pub depth: Option<Length>,

    /// Age of DGPS data. In seconds
    pub dgps_age: Option<Duration>,

    /// Station of DGPS data.
    pub dgps_sta: Option<i64>,

    /// Estimated climb error in meters per second. Certainty unknown.
    pub epc: Option<Speed>,

    /// Estimated track (direction) error in degrees. Certainty unknown.
    pub epd: Option<Angle>,

    /// Estimated horizontal Position (2D) Error in meters.
    // Also known as Estimated Position Error (epe). Certainty unknown.
    pub eph: Option<Length>,

    /// Estimated speed error in meters per second. Certainty unknown.
    pub eps: Option<Speed>,

    /// Estimated time stamp error in seconds. Certainty unknown.
    pub ept: Option<Duration>,

    /// Longitude error estimate in meters. Certainty unknown.
    pub epx: Option<Length>,

    /// Latitude error estimate in meters. Certainty unknown.
    pub epy: Option<Length>,

    /// Estimated vertical error in meters. Certainty unknown.
    pub epv: Option<Length>,

    /// Geoid separation is the difference between the WGS84 reference ellipsoid and the geoid
    /// (Mean Sea Level) in meters.
    pub geoid_sep: Option<Length>,

    /// Latitude, Longitude, datum of the position.
    pub coordinate: Option<EllipticalCoordinate>,

    /// Current leap seconds.
    pub leapseconds: Option<Duration>,

    /// Course over ground,
    /// degrees from true north => 'track'
    /// degrees from mag north => 'magtrack', positive is west var, negative is east var.
    pub track: Option<Track>,

    /// Magnetic variation, degrees. Also known as the
    /// magnetic declination (the direction of the horizontal component of the
    /// magnetic field measured clockwise from north) in degrees, Positive is
    /// West variation. Negative is East variation.
    pub magvar: Option<Track>,

    /// Speed over ground, meters per second.
    pub speed: Option<Speed>,

    /// ECEF X, Y, and Z.
    pub ecef: Option<CartesianCoordinate>,

    /// ECEF Position accuracy in meters
    pub ecefp_acc: Option<Length>,

    /// ECEF X velocity in meters/second
    pub ecefvx: Option<Speed>,

    /// ECEF Y velocity in meters/second
    pub ecefvy: Option<Speed>,

    /// ECEF Z velocity in meters/second
    pub ecefvz: Option<Speed>,

    /// ECEF velocity error in meters/second
    pub ecefv_acc: Option<Speed>,

    /// Down component of relative position vector in meters.
    pub rel_d: Option<Length>,

    /// East component of relative position vector in meters.
    pub rel_e: Option<Length>,

    /// North component of relative position vector in meters.
    pub rel_n: Option<Length>,

    /// Down velocity component in meters.
    pub vel_d: Option<Speed>,
    /// East velocity component in meters.
    pub vel_e: Option<Speed>,
    /// North velocity component in meters.
    pub vel_n: Option<Speed>,

    /// Wind angle magnetic in degrees.
    pub wanglem: Option<Heading>,
    /// Wind angle relative in degrees.
    pub wangler: Option<RelativeBearing>,
    /// Wind angle true in degrees.
    pub wanglet: Option<Heading>,

    /// Wind speed relative in meters per second.
    pub wspeedr: Option<Speed>,
    /// Wind speed true in meters per second.
    pub wspeedt: Option<Speed>,

    /// Water temperature in degrees Celsius.
    pub wtemp: Option<f64>,
}

impl TPV {
    pub fn serialize<S>(&self, map: &mut S::SerializeMap) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        map.serialize_entry("mode", &(self.mode as i8))?;

        if let Some(status) = &self.status {
            map.serialize_entry("status", &(*status as i8))?;
        }

        if let Some(climb) = &self.climb {
            map.serialize_entry("climb", &climb.as_meters_per_second().value())?;
        }
        if let Some(depth) = &self.depth {
            map.serialize_entry("depth", &depth.as_meters().value())?;
        }
        if let Some(age) = &self.dgps_age {
            map.serialize_entry("dgpsAge", &age.as_seconds_f32())?;
        }
        if let Some(sta) = &self.dgps_sta {
            map.serialize_entry("dgpsSta", &sta)?;
        }
        if let Some(epc) = &self.epc {
            map.serialize_entry("epc", &epc.as_meters_per_second().value())?;
        }
        if let Some(epd) = &self.epd {
            map.serialize_entry("epd", &epd.as_degrees().value())?;
        }
        if let Some(eph) = &self.eph {
            map.serialize_entry("eph", &eph.as_meters().value())?;
        }
        if let Some(eps) = &self.eps {
            map.serialize_entry("eps", &eps.as_meters_per_second().value())?;
        }
        if let Some(ept) = &self.ept {
            map.serialize_entry("ept", &ept.as_seconds_f32())?;
        }
        if let Some(epx) = &self.epx {
            map.serialize_entry("epx", &epx.as_meters().value())?;
        }
        if let Some(epy) = &self.epy {
            map.serialize_entry("epy", &epy.as_meters().value())?;
        }
        if let Some(epv) = &self.epv {
            map.serialize_entry("epv", &epv.as_meters().value())?;
        }
        if let Some(sep) = &self.geoid_sep {
            map.serialize_entry("geoidSep", &sep.as_meters().value())?;
        }

        if let Some(coord) = &self.coordinate {
            map.serialize_entry("lat", &coord.get_latitude().0.as_degrees().value())?;
            map.serialize_entry("lon", &coord.get_longitude().0.as_degrees().value())?;
            map.serialize_entry("datum", &coord.get_reference_frame().name())?;

            if let Some(alt) = coord.get_altitude() {
                let val = alt.value().as_meters().value();
                let key = match alt.reference_frame() {
                    AltitudeReferenceFrame::Ellipsoid => "altHAE",
                    AltitudeReferenceFrame::Geoid => "altMSL",
                    _ => "alt",
                };
                map.serialize_entry(key, &val)?;
            }
            if let Some(pos_err) = coord.position_uncertainty() {
                let val = match pos_err {
                    PositionUncertainty::CircularUncertainty(c) => {
                        c.as_radius().get_dimension().as_meters().value()
                    }
                    PositionUncertainty::EllipticalUncertainty(e) => e
                        .semi_major_axis()
                        .as_radius()
                        .get_dimension()
                        .as_meters()
                        .value(),
                };
                map.serialize_entry("sep", &val)?;
            }
            if let Some(time) = coord.get_timestamp() {
                if let Some(odt) = OffsetDateTime::UNIX_EPOCH.checked_add(time::Duration::new(
                    time.as_secs() as i64,
                    time.subsec_nanos() as i32,
                )) {
                    if let Ok(fmt) = odt.format(&Rfc3339) {
                        map.serialize_entry("time", &fmt)?;
                    }
                };
            }
        }

        if let Some(sec) = &self.leapseconds {
            map.serialize_entry("leapseconds", &sec.whole_seconds())?;
        }
        if let Some(track) = &self.track {
            let val = track.angle().as_degrees().value();
            match track.reference() {
                CompassReference::TrueNorth => {
                    map.serialize_entry("track", &val)?;
                }
                CompassReference::MagneticNorth => {
                    map.serialize_entry("magtrack", &val)?;
                }
                _ => {}
            }
        }
        if let Some(speed) = &self.speed {
            map.serialize_entry("speed", &speed.as_meters_per_second().value())?;
        }
        if let Some(ecef) = &self.ecef {
            map.serialize_entry("ecefx", &ecef.get_x().as_meters().value())?;
            map.serialize_entry("ecefy", &ecef.get_y().as_meters().value())?;
            map.serialize_entry("ecefz", &ecef.get_z().as_meters().value())?;
        }

        //TODO
        Ok(())
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    use irox_winlocation_api::WindowsCoordinate;

    use crate::output::{NMEAMode, TPV};

    impl From<&WindowsCoordinate> for TPV {
        fn from(value: &WindowsCoordinate) -> Self {
            let mode = match value.coordinate() {
                Some(c) => match c.get_altitude().is_some() {
                    true => NMEAMode::ThreeDim,
                    false => NMEAMode::TwoDim,
                },
                None => NMEAMode::Unknown,
            };

            TPV {
                mode,
                coordinate: value.coordinate(),
                track: value.heading(),
                speed: value.speed(),

                ..Default::default()
            }
        }
    }
}
