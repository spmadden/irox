// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};
use std::io::Read;
use std::time::Duration;

use irox_carto::altitude::{Altitude, AltitudeReferenceFrame};
use irox_carto::coordinate::{Latitude, Longitude};
use irox_enums::EnumName;
use irox_time::Time;
use irox_tools::format::DecimalFormatF64;
use irox_tools::options::{MaybeInto, MaybeMap};
use irox_tools::packetio::{Packet, PacketBuilder};
use irox_units::units::length::Length;

use crate::{
    calculate_checksum, maybe_altitude, maybe_latitude, maybe_length, maybe_longitude, Error,
    MessageType,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumName)]
pub enum GPSQualityIndicator {
    NotAvailable,
    GPSFix,
    DGPSFix,
    PPSFix,
    RTK,
    FloatRTK,
    EstimatedDR,
    Manual,
    Simulation,
    OtherUnknown(u8),
}

impl GPSQualityIndicator {
    pub const fn value(&self) -> u8 {
        match self {
            GPSQualityIndicator::NotAvailable => 0,
            GPSQualityIndicator::GPSFix => 1,
            GPSQualityIndicator::DGPSFix => 2,
            GPSQualityIndicator::PPSFix => 3,
            GPSQualityIndicator::RTK => 4,
            GPSQualityIndicator::FloatRTK => 5,
            GPSQualityIndicator::EstimatedDR => 6,
            GPSQualityIndicator::Manual => 7,
            GPSQualityIndicator::Simulation => 8,
            GPSQualityIndicator::OtherUnknown(u) => *u,
        }
    }
}

impl From<u8> for GPSQualityIndicator {
    fn from(qual: u8) -> Self {
        match qual {
            0 => GPSQualityIndicator::NotAvailable,
            1 => GPSQualityIndicator::GPSFix,
            2 => GPSQualityIndicator::DGPSFix,
            3 => GPSQualityIndicator::PPSFix,
            4 => GPSQualityIndicator::RTK,
            5 => GPSQualityIndicator::FloatRTK,
            6 => GPSQualityIndicator::EstimatedDR,
            7 => GPSQualityIndicator::Manual,
            8 => GPSQualityIndicator::Simulation,
            u => GPSQualityIndicator::OtherUnknown(u),
        }
    }
}

///
/// GGA - Global Positioning System Fix Data
//
// This is one of the sentences commonly emitted by GPS units.
//
// Time, Position and fix related data for a GPS receiver.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GGA {
    ///
    /// `hhmmss.ss`
    ///
    /// UTC of this position report, hh is hours, mm is minutes, ss.ss is seconds.
    timestamp: Option<Time>,

    ///
    /// `ddmm.mm`
    ///
    /// Latitude, dd is degrees, mm.mm is minutes
    latitude: Option<Latitude>,

    ///
    /// `ddmm.mm`
    ///
    /// Longitude, dd is degrees, mm.mm is minutes
    longitude: Option<Longitude>,

    /// GPS Quality Indicator - enum values
    quality: Option<GPSQualityIndicator>,

    /// Number of satellites in use, 00 - 12
    num_sats: Option<u8>,

    /// Horizontal Dilution of precision (meters)
    hdop: Option<Length>,

    /// Antenna Altitude above/below mean-sea-level (geoid) (in meters)
    ant_alt: Option<Altitude>,

    /// Geoidal separation, the difference between the WGS-84 earth
    /// ellipsoid and mean-sea-level (geoid), "-" means mean-sea-level
    /// below ellipsoid
    geoid_sep: Option<Length>,

    /// Age of differential GPS data, time in seconds since last SC104
    /// type 1 or 9 update, null field when DGPS is not used
    dgps_age: Option<Duration>,

    /// Differential reference station ID, 0000-1023
    stn_id: Option<u16>,
}

impl GGA {
    pub fn timestamp(&self) -> Option<Time> {
        self.timestamp
    }
    pub fn latitude(&self) -> Option<Latitude> {
        self.latitude
    }
    pub fn longitude(&self) -> Option<Longitude> {
        self.longitude
    }
    pub fn quality(&self) -> Option<GPSQualityIndicator> {
        self.quality
    }
    pub fn num_sats(&self) -> Option<u8> {
        self.num_sats
    }
    pub fn hdop(&self) -> Option<Length> {
        self.hdop
    }
    pub fn ant_alt(&self) -> Option<Altitude> {
        self.ant_alt
    }
    pub fn geoid_sep(&self) -> Option<Length> {
        self.geoid_sep
    }
    pub fn dgps_age(&self) -> Option<Duration> {
        self.dgps_age
    }
    pub fn stn_id(&self) -> Option<u16> {
        self.stn_id
    }
}

impl Display for GGA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let mut buf = String::new();
        if let Some(timestamp) = self.timestamp {
            buf.write_fmt(format_args!("Duration: {timestamp:?} "))?;
        }
        if let Some(lat) = self.latitude {
            buf.write_fmt(format_args!("{lat} "))?;
        }
        if let Some(lon) = self.longitude {
            buf.write_fmt(format_args!("{lon} "))?;
        }
        if let Some(quality) = self.quality {
            buf.write_fmt(format_args!("FixType: {quality:?} "))?;
        }
        if let Some(num_sats) = self.num_sats {
            buf.write_fmt(format_args!("NumSats: {num_sats} "))?;
        }
        if let Some(hdop) = self.hdop {
            buf.write_fmt(format_args!("HDOP: {hdop} "))?;
        }
        if let Some(ant_alt) = self.ant_alt {
            buf.write_fmt(format_args!("Ant: {ant_alt} "))?;
        }
        if let Some(geoid_sep) = self.geoid_sep {
            buf.write_fmt(format_args!("Undulation: {geoid_sep} "))?;
        }
        if let Some(dgps_age) = self.dgps_age {
            buf.write_fmt(format_args!("DGPSAge: {dgps_age:?} "))?;
        }
        if let Some(stn) = self.stn_id {
            buf.write_fmt(format_args!("Station: {stn}"))?;
        }

        f.write_str(buf.as_str())
    }
}

impl Packet for GGA {
    type PacketType = MessageType;

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        use std::io::Write;
        let mut buf: Vec<u8> = Vec::new();

        let utctime = self
            .timestamp
            .map(|timestamp| {
                let (hh, mm, ss) = timestamp.as_hms_f64();
                format!("{hh:02}{mm:02}{}", DecimalFormatF64(2, 2, ss))
            })
            .unwrap_or_default();

        let latitude = self.latitude.map_or(String::from(","), |lat| {
            let (mut lat_deg, lat_min) = lat.0.as_deg_min();
            let ns = match lat_deg.is_positive() {
                true => "N",
                false => {
                    lat_deg = lat_deg.abs();
                    "S"
                }
            };
            format!("{lat_deg:02}{},{ns}", DecimalFormatF64(2, 5, lat_min))
        });

        let longitude = self.longitude.map_or(String::from(","), |lon| {
            let (mut lon_deg, lon_min) = lon.0.as_deg_min();
            let ew = match lon_deg.is_positive() {
                true => "E",
                false => {
                    lon_deg = lon_deg.abs();
                    "W"
                }
            };
            format!("{lon_deg:02}{},{ew}", DecimalFormatF64(2, 5, lon_min))
        });
        let fix = self
            .quality
            .map(|fix| format!("{}", fix.value()))
            .unwrap_or_default();
        let sats_used = self
            .num_sats
            .map_or(String::new(), |used| format!("{used}"));
        let hdop = self
            .hdop
            .map(|hdop| format!("{}", DecimalFormatF64(1, 2, hdop.as_meters().value())))
            .unwrap_or_default();
        let msl_alt = self
            .ant_alt
            .maybe_map(|a| {
                if a.reference_frame() != AltitudeReferenceFrame::Geoid {
                    return None;
                };
                Some(format!(
                    "{},M",
                    DecimalFormatF64(2, 2, a.value().as_meters().value())
                ))
            })
            .unwrap_or(String::from(","));
        let geoid_sep = self
            .geoid_sep
            .map(|f| format!("{},M", DecimalFormatF64(1, 2, f.as_meters().value())))
            .unwrap_or(String::from(","));
        let dgps_age = self
            .dgps_age
            .map(|f| format!("{}", f.as_secs()))
            .unwrap_or_default();
        let ref_id = self.stn_id.map(|f| format!("{f}")).unwrap_or_default();
        buf.write_fmt(format_args!(
            "$GPGGA,{utctime},{latitude},{longitude},{fix},{sats_used},{hdop},{msl_alt},{geoid_sep},{dgps_age},{ref_id}*"
        ))?;

        let csh = calculate_checksum(&buf);
        buf.write_fmt(format_args!("{csh:02X}\r\n"))?;

        Ok(buf)
    }

    fn get_type(&self) -> Self::PacketType {
        MessageType::GGA
    }
}

fn maybe_timestamp(val: Option<&str>) -> Option<Time> {
    let Some(time) = val else {
        return None;
    };

    let hh = time.get(0..2)?.parse::<u8>().ok()?;
    let mm = time.get(2..4)?.parse::<u8>().ok()?;
    let ss = time.get(4..)?.parse::<f64>().ok()?;

    Time::from_hms_f64(hh, mm, ss).ok()
}

#[derive(Default)]
pub struct GGABuilder {
    gga: GGA,
}

impl PacketBuilder<GGA> for GGABuilder {
    type Error = Error;

    fn build_from<T: Read>(&self, input: &mut T) -> Result<GGA, Self::Error> {
        let mut ent = String::new();
        let _read = input.read_to_string(&mut ent)?;
        let mut split = ent.split(',');

        let _key = split.next();
        let time = split.next();
        let lat = split.next();
        let ns = split.next();
        let lon = split.next();
        let ew = split.next();
        let qual = split.next();
        let num_sat = split.next();
        let hdops = split.next();
        let ant = split.next();
        let ant_unit = split.next();
        let sep = split.next();
        let sep_unit = split.next();
        let _dgps_age = split.next();
        let _refid = split.next();
        let _csum = split.next();

        let timestamp = maybe_timestamp(time);
        let latitude = maybe_latitude(lat, ns);
        let longitude = maybe_longitude(lon, ew);
        let quality = MaybeInto::<u8>::maybe_into(qual).map(Into::into);
        let num_sats = num_sat.maybe_into();
        let hdop = hdops.maybe_into().map(Length::new_meters);
        let ant_alt = maybe_altitude(ant, ant_unit, AltitudeReferenceFrame::Geoid);
        let geoid_sep = maybe_length(sep, sep_unit);
        Ok(GGA {
            timestamp,
            latitude,
            longitude,
            quality,
            num_sats,
            hdop,
            ant_alt,
            geoid_sep,
            ..Default::default()
        })
    }
}

impl GGABuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> GGA {
        self.gga
    }

    pub fn set_timestamp(&mut self, time: Time) {
        self.gga.timestamp = Some(time);
    }

    #[must_use]
    pub fn with_timestamp(mut self, time: Time) -> Self {
        self.gga.timestamp = Some(time);
        self
    }

    pub fn set_latitude(&mut self, lat: Latitude) {
        self.gga.latitude = Some(lat);
    }

    #[must_use]
    pub fn with_latitude(mut self, lat: Latitude) -> Self {
        self.gga.latitude = Some(lat);
        self
    }

    pub fn set_longitude(&mut self, lon: Longitude) {
        self.gga.longitude = Some(lon);
    }

    #[must_use]
    pub fn with_longitude(mut self, lon: Longitude) -> Self {
        self.gga.longitude = Some(lon);
        self
    }

    pub fn set_quality(&mut self, quality: GPSQualityIndicator) {
        self.gga.quality = Some(quality);
    }

    #[must_use]
    pub fn with_quality(mut self, quality: GPSQualityIndicator) -> Self {
        self.gga.quality = Some(quality);
        self
    }

    pub fn set_num_sats(&mut self, num_sats: u8) {
        self.gga.num_sats = Some(num_sats)
    }

    #[must_use]
    pub fn with_num_sats(mut self, num_sats: u8) -> Self {
        self.gga.num_sats = Some(num_sats);
        self
    }

    pub fn set_hdop(&mut self, hdop: Length) {
        self.gga.hdop = Some(hdop);
    }

    #[must_use]
    pub fn with_hdop(mut self, hdop: Length) -> Self {
        self.gga.hdop = Some(hdop);
        self
    }

    pub fn set_ant_alt(&mut self, alt: Altitude) {
        self.gga.ant_alt = Some(alt);
    }

    #[must_use]
    pub fn with_ant_alt(mut self, alt: Altitude) -> Self {
        self.gga.ant_alt = Some(alt);
        self
    }

    pub fn set_geoid_sep(&mut self, sep: Length) {
        self.gga.geoid_sep = Some(sep);
    }

    #[must_use]
    pub fn with_geoid_sep(mut self, sep: Length) -> Self {
        self.gga.geoid_sep = Some(sep);
        self
    }

    pub fn set_dgps_age(&mut self, age: Duration) {
        self.gga.dgps_age = Some(age);
    }

    #[must_use]
    pub fn with_dgps_age(mut self, age: Duration) -> Self {
        self.gga.dgps_age = Some(age);
        self
    }

    pub fn set_stn_id(&mut self, stn_id: u16) {
        self.gga.stn_id = Some(stn_id);
    }

    #[must_use]
    pub fn with_stn_id(mut self, stn_id: u16) -> Self {
        self.gga.stn_id = Some(stn_id);
        self
    }
}
