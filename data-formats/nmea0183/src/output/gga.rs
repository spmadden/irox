// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};
use std::time::Duration;

use time::Time;

use irox_carto::altitude::{Altitude, AltitudeReferenceFrame};
use irox_carto::coordinate::{Latitude, Longitude};
use irox_enums::EnumName;
use irox_tools::bits::Bits;
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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GGA {
    timestamp: Option<time::Time>,
    latitude: Option<Latitude>,
    longitude: Option<Longitude>,
    quality: Option<GPSQualityIndicator>,
    num_sats: Option<u8>,
    hdop: Option<Length>,
    ant_alt: Option<Altitude>,
    geoid_sep: Option<Length>,
    dgps_age: Option<Duration>,
    stn_id: Option<u16>,
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
                let (hh, mm, ss, milli) = timestamp.as_hms_milli();
                let ss = ss as f64 + (milli as f64) / 1000.;
                format!("{hh:02}{mm:02}{ss:02.03}")
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
            format!("{lat_deg:02}{lat_min:02.04},{ns}")
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
            format!("{lon_deg:02}{lon_min:02.04},{ew}")
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
            .map(|hdop| format!("{:0.1}", hdop.as_meters().value()))
            .unwrap_or_default();
        let msl_alt = self
            .ant_alt
            .maybe_map(|a| {
                if a.reference_frame() != AltitudeReferenceFrame::Geoid {
                    return None;
                };
                Some(format!("{:01.1},M", a.value().as_meters().value()))
            })
            .unwrap_or(String::from(","));
        let geoid_sep = self
            .geoid_sep
            .map(|f| format!("{:01.1},M", f.as_meters().value()))
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

    let sint = ss as u8;
    let millis = ((ss - sint as f64) * 1000.) as u16;

    Time::from_hms_milli(hh, mm, sint, millis).ok()
}

pub struct GGABuilder;
impl PacketBuilder<GGA> for GGABuilder {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<GGA, Self::Error> {
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

#[cfg(test)]
mod test {
    #[test]
    pub fn test() {}
}
