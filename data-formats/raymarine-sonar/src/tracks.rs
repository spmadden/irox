use crate::{Accesses, Entry};
use rusqlite::Connection;
use std::time::Duration;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TrackData {
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
}

impl From<&Entry> for TrackData {
    fn from(value: &Entry) -> Self {
        TrackData {
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
            ..TrackData::default()
        }
    }
}

pub struct Track<'a> {
    conn: &'a Connection,
}
