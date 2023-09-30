use std::collections::VecDeque;
use std::ops::Deref;
use std::time::Duration;

use log::{debug, error, info, warn};
use rusqlite::named_params;
use rusqlite::types::ValueRef;

use irox_carto::coordinate::{
    CartesianCoordinateBuilder, CoordinateType, EllipticalCoordinate, Latitude, Longitude,
};
use irox_carto::geo::standards::StandardShapes;
use irox_tools::bits::{read_be_u32, Bits};
use irox_units::units::angle::Angle;
use irox_units::units::length::Length;
use irox_units::units::speed::{Speed, SpeedUnits};

use crate::error::Error;
use crate::schema::{SchemaContext, SchemaWorkingMem};
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
}

impl<'a> Track<'a> {
    pub fn new(conn: &'a SDFConnection, data: TrackData) -> Self {
        Track { conn, data }
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
            ..Iter::default()
        })
    }
}

#[derive(Default)]
pub struct Iter {
    subtrack_data: VecDeque<Vec<u8>>,
    point_cache: VecDeque<CoordinateType>,
    context: SchemaContext,
}

impl Iter {
    fn fill_cache(&mut self) -> Result<(), Error> {
        if !self.point_cache.is_empty() {
            return Ok(());
        }
        let Some(data) = self.subtrack_data.pop_front() else {
            return Ok(());
        };
        // self.subtrack_data.clear();
        let data = miniz_oxide::inflate::decompress_to_vec_zlib(data.as_slice())?;
        let mut data = VecDeque::from(data);

        let num_points = data.len() / &self.context.struct_size;
        info!("Converting {num_points} points");

        let mut mem = SchemaWorkingMem::new(num_points, &self.context);

        mem.fields
            .iter_mut()
            .enumerate()
            .for_each(|(field_idx, field)| {
                for _data_idx in 0..field.field.size {
                    for point_idx in 0..num_points {
                        let Some(point) = field.points.get_mut(point_idx) else {
                            // can't really happen
                            error!(
                                "mem.points mis sized.  expected {num_points} but was {}",
                                field.points.len()
                            );
                            break;
                        };
                        let Some(d) = data.pop_front() else {
                            error!("Data underflow.");
                            break;
                        };
                        point.mem.push_front(d);
                    }
                }
            });

        let mut builders: Vec<CartesianCoordinateBuilder> = (0..num_points)
            .map(|v| CartesianCoordinateBuilder::new())
            .collect();
        for mut field_mem in mem.fields {
            for idx in 0..num_points {
                let Some(point) = field_mem.points.get_mut(idx) else {
                    continue;
                };
                let Some(mut bldr) = builders.get_mut(idx) else {
                    continue;
                };
                match field_mem.field.field.name().as_str() {
                    "x" => {
                        // let val = read_be_u32(&mut point.mem)?;
                        // println!("X: {val:0X}");
                        let val = with_scale_factor(&mut point.mem, 4)?;
                        bldr.with_x(Length::new_meters(val));
                    }
                    "y" => {
                        // let val = read_be_u32(&mut point.mem)?;
                        // println!("Y: {val:0X}");
                        let val = with_scale_factor(&mut point.mem, 4)?;
                        bldr.with_y(Length::new_meters(val));
                    }
                    "z" => {
                        // let val = read_be_u32(&mut point.mem)?;
                        // println!("T: {val:0X}");
                        let val = with_scale_factor(&mut point.mem, 10)?;
                        bldr.with_z(Length::new_meters(val));
                    }
                    "t" => {
                        let val = read_be_u32(&mut point.mem)?;
                        bldr.with_timestamp(Duration::from_millis(val as u64));
                    }
                    "sog_kn" => {
                        let val = read_be_u32(&mut point.mem)?.to_le();
                        let val = f32::from_bits(val);
                        if val == f32::MAX {
                            continue;
                        }
                        // let val = read_f32(&mut point.mem)?;
                        let spd = Speed::new(val as f64, SpeedUnits::Knots);
                        println!("sog {spd:?}");
                    }
                    "water_speed_kn" => {
                        // let val = read_f32(&mut point.mem)?;
                        let val = read_be_u32(&mut point.mem)?.to_le();
                        let val = f32::from_bits(val);
                        if val == f32::MAX {
                            continue;
                        }
                        let spd = Speed::new(val as f64, SpeedUnits::Knots);
                        println!("water {spd:?}");
                    }
                    _ => {}
                };
            }
        }
        for bldr in builders {
            if let Ok(pnt) = bldr.build() {
                println!("{pnt}");
                let lon = Longitude(Angle::new_radians(
                    pnt.get_x().as_meters().value() / 6378388.0,
                ));
                let y = pnt.get_y().as_meters().value() / 6378388.0;
                let lat = y.exp().atan() * 2.0 - std::f64::consts::FRAC_PI_2;
                let lat = (lat.tan() * 1.00676425).atan();
                // let lat = (pnt.get_y().as_meters().value() / 6378388.0).sinh().atan();
                let lat = Latitude(Angle::new_radians(lat));
                let coord = EllipticalCoordinate::new(
                    lat,
                    lon,
                    StandardShapes::Hayford_International.into(),
                );
                println!("{coord}");
            }
        }

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

fn with_scale_factor<T: Bits>(input: &mut T, scale: usize) -> Result<f64, std::io::Error> {
    let mut factor = input.read_be_u32()?;
    let mut mult = 1_f64;
    if factor > 0x7FFF_FFFF {
        factor = !factor;
        mult = -1_f64;
    }
    let whole = (factor >> scale) as f64;
    let div = 1_u32 << scale;
    let mask = div - 1;
    let part = (factor & mask) as f64 / div as f64;

    let val = mult * (whole + part);
    Ok(val)
}
