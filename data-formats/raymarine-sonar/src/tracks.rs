use crate::error::Error;
use crate::schema::{SchemaContext, SchemaWorkingMem};
use crate::{Accesses, Entry, SDFConnection};
use irox_carto::coordinate::{CartesianCoordinate, CoordinateType, EllipticalCoordinate};
use irox_tools::bits::Bits;
use irox_tools::vec::{PrettyVec, PrettyVecDeque};
use log::{debug, error, info, warn};
use rusqlite::types::ValueRef;
use rusqlite::{named_params, params};
use std::collections::VecDeque;
use std::ops::Deref;
use std::time::Duration;

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

        mem.fields.iter_mut().enumerate().for_each(|(field_idx, field)| {
            for _data_idx in 0..field.field.size {
                for point_idx in 0..num_points {
                    let Some(point) = field.points.get_mut(point_idx) else {
                        // can't really happen
                        error!("mem.points mis sized.  expected {num_points} but was {}", field.points.len());
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
        for field_mem in mem.fields {
            let mut x = 0.0_f64;
            let mut y = 0.0_f64;
            let mut z = 0.0_f64;
            let mut t = 0.0_f64;
            let mut sog = 0.0_f64;
            for mut point in field_mem.points {
                let val = match field_mem.field.field.name().as_str() {
                    "x" => with_scale_factor(&mut point.mem, 256_f64)?,
                    "y" => with_scale_factor(&mut point.mem, 256_f64)?,
                    "z" => with_scale_factor(&mut point.mem, 1024_f64)?,
                    "t" => with_scale_factor(&mut point.mem, 1_f64)?,
                    _ => 0.0_f64
                };
                println!(
                    "{}: {val}",
                    field_mem.field.field.name(),
                );
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

fn with_scale_factor<T: Bits>(input: &mut T, scale: f64) -> Result<f64, std::io::Error> {
    let factor = input.read_be_u32()?;
    Ok(factor as f64 / scale)
}