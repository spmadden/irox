use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use log::{debug, error, trace};
use rusqlite::types::ValueRef;
use rusqlite::{Connection, OpenFlags};

use irox_types::PrimitiveValue;

use crate::error::Error;
use crate::tracks::{Track, TrackData};

pub mod error;
pub mod schema;
pub mod tracks;

pub type Entry = BTreeMap<String, PrimitiveValue>;
pub type Entries = Vec<Entry>;

trait Accesses {
    fn get_str(&self, key: &'static str) -> Option<String>;
    fn get_bool(&self, key: &'static str) -> Option<bool>;
    fn get_duration(&self, key: &'static str) -> Option<Duration>;
    fn get_i64(&self, key: &'static str) -> Option<i64>;
    fn get_blob(&self, key: &'static str) -> Option<Box<[u8]>>;
}
impl Accesses for Entry {
    fn get_str(&self, key: &'static str) -> Option<String> {
        self.get(key).map(ToString::to_string)
    }

    fn get_bool(&self, key: &'static str) -> Option<bool> {
        self.get(key).map(|v| match v {
            PrimitiveValue::bool(v) => *v,
            PrimitiveValue::i64(i) => *i == 1,
            _ => false,
        })
    }

    fn get_duration(&self, key: &'static str) -> Option<Duration> {
        self.get_i64(key).map(|v| Duration::from_secs(v as u64))
    }

    fn get_i64(&self, key: &'static str) -> Option<i64> {
        self.get(key).map(|v| match v {
            PrimitiveValue::i64(i) => *i,
            _ => 0,
        })
    }

    fn get_blob(&self, key: &'static str) -> Option<Box<[u8]>> {
        self.get(key).map(|v| match v {
            PrimitiveValue::blob(b) => b.clone(),
            _ => Box::new([0; 0]),
        })
    }
}

pub struct SDFConnection {
    pub(crate) conn: Connection,
}

impl SDFConnection {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SDFConnection, Error> {
        let mut flags = OpenFlags::empty();
        flags.set(OpenFlags::SQLITE_OPEN_NO_MUTEX, true);
        flags.set(OpenFlags::SQLITE_OPEN_READ_ONLY, true);
        flags.set(OpenFlags::SQLITE_OPEN_URI, true);
        let conn = Connection::open_with_flags(path, flags)?;
        Ok(SDFConnection { conn })
    }

    pub(crate) fn run_sql(&self, sql: &'static str) -> Result<Entries, Error> {
        trace!("Running sql: {sql}");
        let mut out: Vec<BTreeMap<String, PrimitiveValue>> = Vec::new();

        let mut stmt = self.conn.prepare(sql)?;
        let col_cnt = stmt.column_count();
        let mut names: Vec<String> = Vec::new();
        for idx in 0..col_cnt {
            names.push(String::from(stmt.column_name(idx)?));
        }
        debug!("Returned column names: {names:?}");
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            let mut map = BTreeMap::new();
            for idx in 0..col_cnt {
                let Some(col) = names.get(idx) else {
                    error!("Invalid idx: {idx}");
                    continue;
                };
                let val = row.get_ref(idx)?;
                let val: PrimitiveValue = match val {
                    ValueRef::Null => {
                        debug!("Row returned null for {idx}/{col}, skipping output");
                        continue;
                    }
                    ValueRef::Integer(i) => PrimitiveValue::i64(i),
                    ValueRef::Real(r) => PrimitiveValue::f64(r),
                    ValueRef::Text(t) => {
                        PrimitiveValue::str(String::from_utf8_lossy(t).to_string())
                    }
                    ValueRef::Blob(b) => PrimitiveValue::blob(Box::from(b)),
                };
                debug!("Row returned {val:?} for {idx}/{col}");
                map.insert(col.clone(), val);
            }
            out.push(map);
        }

        Ok(out)
    }

    pub fn get_global_props(&self) -> Result<Entries, Error> {
        self.run_sql("SELECT * FROM GlobalProp")
    }

    pub fn get_track_info(&self) -> Result<Vec<TrackData>, Error> {
        Ok(self
            .run_sql("SELECT * FROM Tracks")?
            .iter()
            .enumerate()
            .map(|(idx, f)| TrackData::new(f, idx as i64 + 1))
            .collect())
    }

    pub fn get_tracks(&self) -> Result<Vec<Track>, Error> {
        Ok(self
            .run_sql("SELECT * FROM Tracks")?
            .iter()
            .enumerate()
            .map(|(idx, e)| TrackData::new(e, idx as i64 + 1))
            .map_while(|e| Track::new(self, e).ok())
            .collect())
    }
}
