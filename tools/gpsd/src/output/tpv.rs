use serde::ser::SerializeMap;
use serde::Serializer;
use time::{Duration, OffsetDateTime, UtcOffset};
use time::format_description::well_known::Rfc3339;

use irox_carto::coordinate::EllipticalCoordinate;
use irox_carto::geo::EllipticalShape;
use irox_units::units::angle::Angle;
use irox_units::units::compass::{Compass, Heading, RelativeBearing, Track};
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

    /// Time/date stamp
    pub time: Option<OffsetDateTime>,

    /// Altitude, height above ellipsoid, in meters. Probably WGS84.
    pub alt_hae: Option<Length>,

    /// MSL Altitude in meters. The geoid used is rarely specified and is often inaccurate.
    pub alt_msl: Option<Length>,

    /// Deprecated.  Undefined.  Use altHAE or altMSL
    pub alt: Option<Length>,

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

    /// Course over ground, degrees from true north.
    pub track: Option<Compass<Track>>,

    /// Magnetic variation, degrees. Also known as the
    /// magnetic declination (the direction of the horizontal component of the
    /// magnetic field measured clockwise from north) in degrees, Positive is
    /// West variation. Negative is East variation.
    pub mag_track: Option<Compass<Track>>,

    /// Speed over ground, meters per second.
    pub speed: Option<Speed>,

    /// ECEF X position in meters.
    pub ecefx: Option<Length>,

    /// ECEF y position in meters.
    pub ecefy: Option<Length>,

    /// ECEF z position in meters.
    pub ecefz: Option<Length>,

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

    /// Estimated Spherical (3D) Position Error in meters.
    /// Guessed to be 95% confidence, but many GNSS receivers do not specify,
    /// so certainty unknown.
    pub sep: Option<Length>,

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

        if let Some(time) = &self.time {
            let time = time.to_offset(UtcOffset::UTC);
            if let Ok(fmt) = time.format(&Rfc3339) {
                map.serialize_entry("time", &fmt)?;
            }
        }
        if let Some(alt_hae) = &self.alt_hae {
            map.serialize_entry("altHAE", &alt_hae.as_meters().value())?;
        }
        if let Some(alt_msl) = &self.alt_msl {
            map.serialize_entry("altMSL", &alt_msl.as_meters().value())?;
        }
        if let Some(alt) = &self.alt {
            map.serialize_entry("alt", &alt.as_meters().value())?;
        }
        if let Some(climb) = &self.climb {
            map.serialize_entry("climb", &climb.as_meters_per_second().value())?;
        }
        if let Some(datum) = &self.coordinate {
            let val = match &datum.get_reference_frame() {
                EllipticalShape::EPSG(e) => e,
                EllipticalShape::Ellipse(e) => e.name(),
            };

            map.serialize_entry("datum", &val)?;
        }
        if let Some(depth) = &self.depth {
            map.serialize_entry("depth", &depth.as_meters().value())?;
        }

        todo!();

        Ok(())
    }
}
