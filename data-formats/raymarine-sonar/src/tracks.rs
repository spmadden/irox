// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::collections::VecDeque;
use std::ops::Deref;
use std::time::Duration;

use log::{debug, error, info, warn};
use rusqlite::named_params;
use rusqlite::types::ValueRef;

use irox_carto::altitude::{Altitude, AltitudeReferenceFrame};
use irox_carto::coordinate::{
    CartesianCoordinateBuilder, CoordinateType, EllipticalCoordinateBuilder, Latitude, Longitude,
};
use irox_carto::geo::standards::StandardShapes;
use irox_tools::bits::Bits;
use irox_units::units::angle::Angle;
use irox_units::units::length::Length;

use crate::error::Error;
use crate::schema::SchemaContext;
use crate::{Accesses, Entry, SDFConnection};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TrackData {
    track_id: i64,
    lib_version: Option<String>,
    cartogr_proj: Option<String>,
    vertical_units: Option<String>,
    z_is_depth: Option<bool>,
    creation_date_utc: Option<String>,
    creation_date_local: Option<String>,
    start_time_utc: Option<Duration>,
    end_time_utc: Option<Duration>,
    duration: Option<Duration>,
    point_count: Option<i64>,
    sonar_offset_meters: Option<f64>,
    sdf_point_schema: Option<String>,
    bounds: Option<Box<[u8]>>,
}

impl TrackData {
    pub fn new(value: &Entry, track_id: i64) -> TrackData {
        TrackData {
            track_id,
            lib_version: value.get_str("libVersion"),
            cartogr_proj: value.get_str("cartogrProj"),
            vertical_units: value.get_str("verticalUnits"),
            z_is_depth: value.get_bool("zIsDepth"),
            creation_date_utc: value.get_str("creationDateUtcStr"),
            creation_date_local: value.get_str("creationDateLocalStr"),
            start_time_utc: value.get_duration("startTimeUtc"),
            end_time_utc: value.get_duration("endTimeUtc"),
            duration: value.get_duration("duration"),
            point_count: value.get_i64("pointCount"),
            sdf_point_schema: value.get_str("blbSchemaXml"),
            bounds: value.get_blob("bounds"),
            ..TrackData::default()
        }
    }
}

pub struct Track<'a> {
    conn: &'a SDFConnection,
    data: TrackData,
    context: SchemaContext,
}

impl<'a> Track<'a> {
    pub fn new(conn: &'a SDFConnection, data: TrackData) -> Result<Self, Error> {
        let Some(schema_str) = &data.sdf_point_schema else {
            return Error::xml_error("Missing sdf point schema");
        };
        let context = SchemaContext::new_from_xml(schema_str)?;
        debug!("created context from xml: {context:?}");
        Ok(Track {
            conn,
            data,
            context,
        })
    }

    pub fn get_bounds(&self) -> Result<(), std::io::Error> {
        let Some(data) = &self.data.bounds else {
            return Ok(());
        };
        let mut data = data.deref();
        let first = data.read_f64()?;
        info!("{first}");
        Ok(())
    }

    pub fn iter(&self) -> Result<Iter, Error> {
        debug!("Iter {}", self.data.track_id);
        let mut stmt = self
            .conn
            .conn
            .prepare("SELECT data from Subtracks where trackId = :track_id")?;
        let mut rows = stmt.query(named_params! {":track_id": self.data.track_id})?;
        let mut subtrack_data = VecDeque::new();
        while let Some(row) = rows.next()? {
            let elem = row.get_ref(0)?;
            let ValueRef::Blob(b) = elem else {
                warn!("Subtrack query didn't return a blob for query");
                continue;
            };
            subtrack_data.push_back(Vec::from(b));
        }
        debug!("Got {} rows of subtrack data", subtrack_data.len());

        Ok(Iter {
            subtrack_data,
            point_cache: VecDeque::new(),
            context: self.context.clone(),
            start_time_utc: self.data.start_time_utc,
        })
    }
}

// #[derive(Default)]
pub struct Iter {
    subtrack_data: VecDeque<Vec<u8>>,
    point_cache: VecDeque<CoordinateType>,
    context: SchemaContext,
    start_time_utc: Option<Duration>,
}

impl Iter {
    fn fill_cache(&mut self) -> Result<(), Error> {
        if !self.point_cache.is_empty() {
            return Ok(());
        }
        let Some(data) = self.subtrack_data.pop_front() else {
            return Ok(());
        };
        let data = miniz_oxide::inflate::decompress_to_vec_zlib(data.as_slice())?;

        let num_points = data.len() / self.context.struct_size;
        debug!("Converting {num_points} points");

        for idx in 0..num_points {
            let mut bldr = CartesianCoordinateBuilder::new();

            for field in &self.context.fields {
                let val = field.decode_field(&data, idx, num_points)?;
                match field.name.as_str() {
                    "x" => {
                        bldr.with_x(Length::new_meters(val));
                    }
                    "y" => {
                        bldr.with_y(Length::new_meters(val));
                    }
                    "z" => {
                        bldr.with_z(Length::new_meters(val));
                    }
                    "t" => {
                        let mut dur = Duration::from_millis(val as u64);
                        if let Some(start) = self.start_time_utc {
                            dur += start;
                        }
                        bldr.with_timestamp(dur);
                    }
                    _ => {}
                }
            }

            if let Ok(coord) = bldr.build() {
                let lon = Longitude(Angle::new_radians(
                    coord.get_x().as_meters().value() / 6378388.0,
                ));
                let y = coord.get_y().as_meters().value() / 6378388.0;
                let lat = y.exp().atan() * 2.0 - std::f64::consts::FRAC_PI_2;
                let lat = (lat.tan() * 1.00676425).atan();
                // let lat = (pnt.get_y().as_meters().value() / 6378388.0).sinh().atan();
                let lat = Latitude(Angle::new_radians(lat));
                let mut bldr = EllipticalCoordinateBuilder::new();
                bldr.with_latitude(lat)
                    .with_longitude(lon)
                    .with_reference_frame(StandardShapes::Hayford_International.into());
                if let Some(ts) = coord.get_timestamp() {
                    bldr.with_timestamp(*ts);
                }
                bldr.with_altitude(Altitude::new(*coord.get_z(), AltitudeReferenceFrame::Geoid));
                let coord = bldr.build()?;
                self.point_cache
                    .push_back(CoordinateType::Elliptical(coord));
            }
        }
        //             "sog_kn" => {
        //                 let val = read_be_u32(&mut point.mem)?.to_le();
        //                 let val = f32::from_bits(val);
        //                 if val == f32::MAX {
        //                     continue;
        //                 }
        //                 // let val = read_f32(&mut point.mem)?;
        //                 let spd = Speed::new(val as f64, SpeedUnits::Knots);
        //                 // println!("sog {spd:?}");
        //             }
        //             "water_speed_kn" => {
        //                 // let val = read_f32(&mut point.mem)?;
        //                 let val = read_be_u32(&mut point.mem)?.to_le();
        //                 let val = f32::from_bits(val);
        //                 if val == f32::MAX {
        //                     continue;
        //                 }
        //                 let spd = Speed::new(val as f64, SpeedUnits::Knots);
        //                 // println!("water {spd:?}");
        //             }
        //             _ => {}
        //         };
        //     }
        // }

        Ok(())
    }
}

impl Iterator for Iter {
    type Item = CoordinateType;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(e) = self.fill_cache() {
            error!("{e}");
            return None;
        };
        self.point_cache.pop_front()
    }
}
