// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use time::macros::date;
use time::{Date, Duration, Month};

use irox_carto::coordinate::{EllipticalCoordinateBuilder, Latitude, Longitude};
use irox_carto::geo::standards::wgs84::WGS84_SHAPE;
use irox_carto::geo::EllipticalShape;
use irox_carto::gps::GPSFixType;
use irox_sirf::input::x29_geonavdata::{GeodeticNavigationData, PositionFixType};
use irox_sirf::packet::PacketType;
use irox_tools::options::MaybeFrom;
use irox_units::units::angle::Angle;

use crate::output::{Frame, FramePayload, TPV};

impl MaybeFrom<PacketType> for Frame {
    fn maybe_from(value: PacketType) -> Option<Self> {
        match value {
            PacketType::GeodeticNavigationData(gnd) => Some(Frame {
                device: None,
                payload: FramePayload::TPV(Box::new(gnd.into())),
                raw: None,
            }),
            PacketType::Unknown(_, _) => None,
            _ => None,
        }
    }
}

impl From<GeodeticNavigationData> for TPV {
    fn from(value: GeodeticNavigationData) -> Self {
        let lat = Latitude(Angle::new_degrees(value.latitude as f64 / 1e7));
        let lon = Longitude(Angle::new_degrees(value.longitude as f64 / 1e7));

        let datum = match value.map_datum {
            21 => WGS84_SHAPE,
            e => EllipticalShape::EpsgDatum(e as u32),
        };
        let mut coordbuilder = EllipticalCoordinateBuilder::new();
        coordbuilder.with_latitude(lat);
        coordbuilder.with_longitude(lon);
        coordbuilder.with_reference_frame(datum);

        let month = Month::try_from(value.utc_month).unwrap_or(Month::January);
        let date = Date::from_calendar_date(value.utc_year as i32, month, value.utc_day);
        if let Ok(date) = date {
            let utcdate = date - date!(1970 - 01 - 01);
            let secs = Duration::seconds_f64(
                value.utc_hour as f64 * 3600.
                    + value.utc_minute as f64 * 60.
                    + value.utc_second as f64 / 1000.,
            );
            let utcdate = utcdate + secs;
            coordbuilder.with_timestamp(core::time::Duration::from_secs_f64(
                utcdate.as_seconds_f64(),
            ));
        }
        let coordinate = coordbuilder.build().ok();

        let mode = match value.fix_type() {
            PositionFixType::NoFix => GPSFixType::NoFix,
            PositionFixType::Solution4MoreSV => GPSFixType::TwoDim,
            _ => GPSFixType::Unknown,
        };

        TPV {
            mode,
            status: None,
            climb: None,
            depth: None,
            dgps_age: None,
            dgps_sta: None,
            epc: None,
            epd: None,
            eph: None,
            eps: None,
            ept: None,
            epx: None,
            epy: None,
            epv: None,
            geoid_sep: None,
            coordinate,
            leapseconds: None,
            track: None,
            magvar: None,
            speed: None,
            ecef: None,
            ecefp_acc: None,
            ecefvx: None,
            ecefvy: None,
            ecefvz: None,
            ecefv_acc: None,
            rel_d: None,
            rel_e: None,
            rel_n: None,
            vel_d: None,
            vel_e: None,
            vel_n: None,
            wanglem: None,
            wangler: None,
            wanglet: None,
            wspeedr: None,
            wspeedt: None,
            wtemp: None,
        }
    }
}
