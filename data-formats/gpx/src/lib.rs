// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

use std::fmt::{Display, Formatter};
use std::time::Duration;

use irox_carto::altitude::Altitude;
use irox_carto::coordinate::{EllipticalCoordinate, Latitude, Longitude};
use irox_carto::gps::DilutionOfPrecision;
use irox_units::units::angle::Angle;
pub use writer::*;

pub mod error;
mod reader;
mod writer;

pub const NAMESPACE: &str = "http://www.topografix.com/GPX/1/1";

///
/// Main top-level file element
///
/// GPX documents contain a metadata header, followed by waypoints, routes, and
/// tracks.  You can add your own elements to the extensions section of the GPX
/// document.
pub struct GPX {
    /// Metadata about the file.
    pub metadata: Option<Metadata>,

    /// A list of waypoints.
    pub wpt: Vec<Waypoint>,

    /// A list of routes.
    pub rte: Vec<Route>,

    /// A list of tracks.
    pub trk: Vec<Track>,

    /// You can add extend GPX by adding your own elements from another schema
    /// here.
    pub extensions: Option<Extensions>,

    /// You must include the version number in your GPX document.
    pub version: String,

    /// You must include the name or URL of the software that created your GPX
    /// document.  This allows others to inform the creator of a GPX instance
    /// document that fails to validate.
    pub creator: String,
}
impl Default for GPX {
    fn default() -> Self {
        GPX::new()
    }
}
impl GPX {
    pub fn new() -> GPX {
        GPX {
            metadata: None,
            wpt: vec![],
            rte: vec![],
            trk: vec![],
            extensions: None,
            version: "1.1".to_string(),
            creator: "irox-gpx https://github.com/spmadden/irox".to_string(),
        }
    }
}

///
/// Information about the GPX file, author, and copyright restrictions goes in
/// the metadata section.  Providing rich, meaningful information about your
/// GPX files allows others to search for and use your GPS data.
pub struct Metadata {
    /// The name of the GPX file.
    pub name: Option<String>,

    /// A description of the contents of the GPX file.
    pub desc: Option<String>,

    /// The person or organization who created the GPX file.
    pub author: Option<Person>,

    /// Copyright and license information governing use of the file.
    pub copyright: Option<Copyright>,

    /// URLs associated with the location described in the file.
    pub link: Vec<Link>,

    /// The creation date of the file.
    pub time: Option<Duration>,

    /// Keywords associated with the file.  Search engines or databases can use
    /// this information to classify the data.
    pub keywords: Vec<String>,

    /// Minimum and maximum coordinates which describe the extent of the
    /// coordinates in the file.
    pub bounds: Option<Bounds>,

    /// You can add extend GPX by adding your own elements from another schema
    /// here.
    pub extensions: Option<Extensions>,
}

///
/// wpt represents a waypoint, point of interest, or named feature on a map.
pub struct Waypoint {
    /// Elevation (in meters) of the point.
    pub ele: Option<f64>,

    /// Creation/modification timestamp for element. Date and time in are in
    /// Univeral Coordinated Time (UTC), not local time! Conforms to ISO 8601
    /// specification for date/time representation. Fractional seconds are
    /// allowed for millisecond timing in tracklogs.
    pub time: Option<Duration>,

    /// Magnetic variation (in degrees) at the point
    pub magvar: Option<Angle>,

    /// Height (in meters) of geoid (mean sea level) above WGS84 earth
    /// ellipsoid.  As defined in NMEA GGA message.
    pub geoidheight: Option<Altitude>,

    /// The GPS name of the waypoint. This field will be transferred to and
    /// from the GPS. GPX does not place restrictions on the length of this
    /// field or the characters contained in it. It is up to the receiving
    /// application to validate the field before sending it to the GPS.
    pub name: Option<String>,

    /// GPS waypoint comment. Sent to GPS as comment.
    pub cmt: Option<String>,

    /// A text description of the element. Holds additional information about
    /// the element intended for the user, not the GPS.
    pub desc: Option<String>,

    /// Source of data. Included to give user some idea of reliability and
    /// accuracy of data.  "Garmin eTrex", "USGS quad Boston North", e.g.
    pub src: Option<String>,

    /// Link to additional information about the waypoint.
    pub link: Vec<Link>,

    /// Text of GPS symbol name. For interchange with other programs, use the
    /// exact spelling of the symbol as displayed on the GPS.  If the GPS
    /// abbreviates words, spell them out.
    pub sym: Option<String>,

    /// Type (classification) of the waypoint.
    pub wpt_type: Option<String>,

    /// Type of GPX fix.
    pub fix: Option<Fix>,

    /// Number of satellites used to calculate the GPX fix.
    pub sat: Option<u32>,

    /// Horizontal dilution of precision.
    pub hdop: Option<DilutionOfPrecision>,

    /// Vertical dilution of precision.
    pub vdop: Option<DilutionOfPrecision>,

    /// Position dilution of precision.
    pub pdop: Option<DilutionOfPrecision>,

    /// Number of seconds since last DGPS update.
    pub ageofdgpsdata: Option<Duration>,

    /// ID of DGPS station used in differential correction.
    pub dgpsid: Option<DGPSStationType>,

    /// You can add extend GPX by adding your own elements from another schema
    /// here.
    pub extensions: Option<Extensions>,

    /// The latitude of the point.  This is always in decimal degrees, and
    /// always in WGS84 datum.
    pub lat: Latitude,

    /// The longitude of the point.  This is always in decimal degrees, and
    /// always in WGS84 datum.
    pub lon: Longitude,
}
impl Waypoint {
    pub fn new(lat: Latitude, lon: Longitude) -> Waypoint {
        Waypoint {
            ele: None,
            time: None,
            magvar: None,
            geoidheight: None,
            name: None,
            cmt: None,
            desc: None,
            src: None,
            link: vec![],
            sym: None,
            wpt_type: None,
            fix: None,
            sat: None,
            hdop: None,
            vdop: None,
            pdop: None,
            ageofdgpsdata: None,
            dgpsid: None,
            extensions: None,
            lat,
            lon,
        }
    }
}
impl From<EllipticalCoordinate> for Waypoint {
    fn from(value: EllipticalCoordinate) -> Self {
        let mut wpt = Waypoint::new(*value.get_latitude(), *value.get_longitude());
        wpt.time = *value.get_timestamp();
        wpt.ele = value.get_altitude().map(|v| v.value().as_meters().value());

        wpt
    }
}

///
/// rte represents route - an ordered list of waypoints representing a series of
/// turn points leading to a destination.
pub struct Route {
    /// GPS name of route.
    pub name: Option<String>,

    /// GPS comment for route.
    pub cmt: Option<String>,

    /// Text description of route for user.  Not sent to GPS.
    pub desc: Option<String>,

    /// Source of data. Included to give user some idea of reliability and
    /// accuracy of data.
    pub src: Option<String>,

    /// Links to external information about the route.
    pub link: Vec<Link>,

    /// GPS route number.
    pub number: Option<u32>,

    /// Type (classification) of route.
    pub rte_type: Option<String>,

    /// You can add extend GPX by adding your own elements from another schema
    /// here.
    pub extensions: Option<Extensions>,

    /// A list of route points
    pub waypoints: Vec<Waypoint>,
}

#[derive(Default)]
pub struct Track {
    /// GPS name of track.
    pub name: Option<String>,

    /// GPS comment for track.
    pub cmt: Option<String>,

    /// Text description of track for user.  Not sent to GPS.
    pub desc: Option<String>,

    /// Source of data. Included to give user some idea of reliability and
    /// accuracy of data.
    pub src: Option<String>,

    /// Links to external information about the track.
    pub link: Vec<Link>,

    /// GPS track number.
    pub number: Option<u32>,

    /// Type (classification) of track.
    pub trk_type: Option<String>,

    /// You can add extend GPX by adding your own elements from another schema
    /// here.
    pub extensions: Option<Extensions>,

    /// A Track Segment holds a list of Track Points which are logically
    /// connected in order. To represent a single GPS track where GPS reception
    /// was lost, or the GPS receiver was turned off, start a new Track Segment
    /// for each continuous span of track data.
    pub trkseg: Vec<TrackSegment>,
}

impl Track {
    pub fn new() -> Track {
        Track::default()
    }
}

pub struct Extensions {}

/// A person or organization.
pub struct Person {
    /// Name of person or organization.
    pub name: Option<String>,

    /// Email address.
    pub email: Option<Email>,

    /// Link to Web site or other external information about person.
    pub link: Option<Link>,
}

/// An email address.  Broken into two parts (id and domain) to help prevent
/// email harvesting.
pub struct Email {
    /// id half of email address (billgates2004)
    pub id: String,

    /// domain half of email address (hotmail.com)
    pub domain: String,
}

/// Information about the copyright holder and any license governing use of
/// this file.  By linking to an appropriate license, you may place your data
/// into the public domain or grant additional usage rights.
pub struct Copyright {
    /// Year of copyright.
    pub year: Option<u16>,

    /// Link to external file containing license text.
    pub license: Option<String>,

    /// Copyright holder (TopoSoft, Inc.)
    pub author: String,
}

/// A link to an external resource (Web page, digital photo, video clip, etc)
/// with additional information.
pub struct Link {
    /// Text of hyperlink.
    pub text: Option<String>,

    /// Mime type of content (image/jpeg)
    pub link_type: Option<String>,

    /// URL of hyperlink.
    pub href: String,
}

/// Two lat/lon pairs defining the extent of an element.
pub struct Bounds {
    /// The minimum latitude.
    pub min_lat: Latitude,
    /// The minimum longitude.
    pub min_lon: Longitude,
    /// The maximum latitude.
    pub max_lat: Latitude,
    /// The maximum longitude.
    pub max_lon: Longitude,
}

/// Type of GPS fix.  none means GPS had no fix.  To signify "the fix info is
/// unknown, leave out fixType entirely. pps = military signal used
pub enum Fix {
    None,
    TwoD,
    ThreeD,
    DGPS,
    PPS,
}
impl Fix {
    fn name(&self) -> &'static str {
        match self {
            Fix::None => "none",
            Fix::TwoD => "2d",
            Fix::ThreeD => "3d",
            Fix::DGPS => "dgps",
            Fix::PPS => "pps",
        }
    }
}
impl Display for Fix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.name()))
    }
}

/// Represents a differential GPS station.  Valid range `[0,1023]`
pub struct DGPSStationType(u16);

/// A Track Segment holds a list of Track Points which are logically connected
/// in order. To represent a single GPS track where GPS reception was lost, or
/// the GPS receiver was turned off, start a new Track Segment for each
/// continuous span of track data.
#[derive(Default)]
pub struct TrackSegment {
    /// A Track Point holds the coordinates, elevation, timestamp, and metadata
    /// for a single point in a track.
    pub track_point: Vec<Waypoint>,

    /// You can add extend GPX by adding your own elements from another schema
    /// here.
    pub extensions: Option<Extensions>,
}
impl TrackSegment {
    pub fn new() -> TrackSegment {
        TrackSegment::default()
    }
}
