// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use std::fmt::Debug;

use crate::Pragma;
use crate::Result;
use crate::{
    Error, FileExtension, ImageFormat, JournalMode, LockingMode, SynchronousMode, APPLICATION_ID,
};
use irox_carto::coordinate::{Latitude, Longitude};
use irox_units::units::{datasize::DataSizeUnits, FromUnits};
use rusqlite::{named_params, params, Connection};
use std::path::Path;

pub struct MBTiles {
    name: String,
    format: ImageFormat,

    connection: Connection,
}
impl MBTiles {
    pub fn open<T: AsRef<Path>>(path: &T) -> Result<MBTiles> {
        Self::open_options(path, &OpenOptions::default())
    }
    pub fn open_options<T: AsRef<Path>>(path: &T, options: &OpenOptions) -> Result<MBTiles> {
        let conn = {
            let conn = Connection::open(path)?;

            if let Some(psb) = options
                .pragmas
                .iter()
                .find(|v| matches!(v, Pragma::PageSizeBytes(_)))
            {
                psb.set(&conn)?;
                conn.execute("VACUUM;", params![])?;
                drop(conn);
                Connection::open(path)?
            } else {
                conn
            }
        };

        for pragma in &options.pragmas {
            match pragma {
                Pragma::PageSizeBytes(_) => {}
                p => p.set(&conn)?,
            }
        }

        let mut tables: Vec<String> = Vec::new();
        {
            let mut stmt = conn.prepare("select name from sqlite_master;")?;
            let mut rows = stmt.query(params![])?;
            while let Ok(Some(row)) = rows.next() {
                let name: String = row.get(0)?;
                tables.push(name.to_string());
            }
        }

        if !tables.contains(&"tiles".to_string()) && !tables.contains(&"metadata".to_string()) {
            return Error::not_mbtiles("File is not MBTiles");
        }

        let name = get_name(&conn)?;
        let format = get_format(&conn)?;

        Ok(MBTiles {
            connection: conn,
            name,
            format,
        })
    }

    pub fn open_or_create<T: AsRef<Path>>(path: &T) -> Result<MBTiles> {
        Self::open_or_create_options(path, &CreateOptions::default())
    }

    pub fn open_or_create_options<T: AsRef<Path>>(
        path: &T,
        options: &CreateOptions,
    ) -> Result<MBTiles> {
        let path_ref = path.as_ref();
        if path_ref.exists() {
            return Self::open_options(path, &options.into());
        }

        create_mbtiles_db(path, options)
    }

    pub fn get_tile(&self, tile_column: u64, tile_row: u64, zoom_level: u64) -> Result<Vec<u8>> {
        let mut st = self.connection.prepare_cached(
            "select tile_data from tiles where
        tile_column = :tile_column and tile_row = :tile_row and zoom_level = :zoom_level;",
        )?;
        let mut rows = st.query(named_params![
            ":tile_column": tile_column,
            ":tile_row": tile_row,
            ":zoom_level": zoom_level
        ])?;

        if let Some(row) = rows.next()? {
            let res: Vec<u8> = row.get(0)?;
            return Ok(res);
        }
        Error::tile_not_found(tile_column, tile_row, zoom_level)
    }

    pub fn set_tile<T: AsRef<[u8]>>(
        &mut self,
        index: u64,
        tile_column: u64,
        tile_row: u64,
        zoom_level: u8,
        tile_data: &T,
    ) -> Result<()> {
        if tile_data.as_ref().len() == 872 {
            // skip transparent PNGs.
            return Ok(());
        }

        let mut st = self.connection.prepare_cached(
            "insert or replace into
            tiles (tile_index, tile_row, tile_column, zoom_level, tile_data)
            values (:index, :tile_row, :tile_column, :zoom_level, :tile_data);",
        )?;
        st.execute(named_params! {
            ":index": index,
            ":tile_row": tile_row,
            ":tile_column": tile_column,
            ":zoom_level": zoom_level,
            ":tile_data": tile_data.as_ref(),
        })?;

        Ok(())
    }

    pub fn update_min_max_zooms(&mut self, new_zoom: u8) -> Result<()> {
        update_min_max_zooms(&self.connection, new_zoom)
    }

    pub fn connection(&mut self) -> &Connection {
        &self.connection
    }

    pub fn insert_tile(&mut self, tile: &crate::Tile) -> Result<()> {
        self.set_tile(
            tile.index(),
            tile.tile_column,
            tile.tile_row,
            tile.zoom_level,
            &tile.tile_data,
        )
    }

    pub fn foreach_tile<T: FnMut(&crate::Tile) -> Result<()>>(&mut self, cb: &mut T) -> Result<()> {
        let conn = self.connection();
        let mut st =
            conn.prepare_cached("select tile_row, tile_column, zoom_level, tile_data from tiles;")?;
        let tile_row_idx = st.column_index("tile_row")?;
        let tile_col_idx = st.column_index("tile_column")?;
        let zoom_col_idx = st.column_index("zoom_level")?;
        let tile_data_idx = st.column_index("tile_data")?;
        let mut rows = st.query(params![])?;
        while let Ok(Some(res)) = rows.next() {
            let tile_row = res.get::<_, i64>(tile_row_idx)?;
            let tile_column = res.get::<_, i64>(tile_col_idx)?;
            let zoom_level = res.get::<_, i64>(zoom_col_idx)?;
            let tile_data = res.get_ref(tile_data_idx)?.as_bytes()?;

            let tile = crate::Tile {
                tile_row: tile_row as u64,
                tile_column: tile_column as u64,
                zoom_level: zoom_level as u8,
                tile_data,
            };
            cb(&tile)?;
        }
        Ok(())
    }

    pub fn gc(&mut self) -> Result<()> {
        self.connection().execute("VACUUM;", params![])?;
        Ok(())
    }

    pub fn update_bounding_box(
        &mut self,
        lat1: Latitude,
        lat2: Latitude,
        lon1: Longitude,
        lon2: Longitude,
        min_zoom: u8,
        max_zoom: u8,
    ) -> Result<()> {
        let lats = vec![lat1.0.as_degrees().value(), lat2.0.as_degrees().value()];
        let lons = vec![lon1.0.as_degrees().value(), lon2.0.as_degrees().value()];
        let (min_lat_deg, max_lat_deg) = irox_tools::f64::min_max(&lats);
        let (min_lon_deg, max_lon_deg) = irox_tools::f64::min_max(&lons);
        let conn = self.connection();
        let bounds = format!("{min_lon_deg},{min_lat_deg},{max_lon_deg},{max_lat_deg}");
        set_metadata(conn, "bounds", &bounds)?;
        set_metadata(conn, "minzoom", &format!("{min_zoom}"))?;
        set_metadata(conn, "maxzoom", &format!("{max_zoom}"))?;

        let center_lon = (max_lon_deg - min_lon_deg) / 2.0 + min_lon_deg;
        let center_lat = (max_lat_deg - min_lat_deg) / 2.0 + min_lat_deg;
        let center = format!("{center_lon},{center_lat},{min_zoom}");
        set_metadata(conn, "center", &center)?;

        Ok(())
    }
}
impl Drop for MBTiles {
    #[allow(clippy::print_stderr)]
    fn drop(&mut self) {
        if let Err(e) = Pragma::JournalMode(JournalMode::Delete).set(&self.connection) {
            eprintln!("Error clearing journal: {e}");
        }
    }
}
impl Debug for MBTiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut st = f.debug_struct("MBTiles");
        st.field("name", &self.name).field("format", &self.format);

        for pragma in &[
            Pragma::ApplicationId(0),
            Pragma::CacheSizeBytes(0),
            Pragma::PageSizeBytes(0),
        ] {
            st.field(pragma.name(), &pragma.get(&self.connection));
        }

        st.finish()
    }
}
pub fn create_mbtiles_db<T: AsRef<Path>>(path: &T, options: &CreateOptions) -> Result<MBTiles> {
    let path = path.as_ref();
    if path.exists() {
        return Error::io_exists(format!("DB already exists: {}", path.display()).as_str());
    }

    let conn = {
        let conn = Connection::open(path)?;

        if let Some(psb) = options
            .pragmas
            .iter()
            .find(|v| matches!(v, Pragma::PageSizeBytes(_)))
        {
            psb.set(&conn)?;
            conn.execute("VACUUM;", params![])?;
            drop(conn);
            Connection::open(path)?
        } else {
            conn
        }
    };

    for pragma in &options.pragmas {
        match pragma {
            Pragma::PageSizeBytes(_) => {}
            _ => pragma.set(&conn)?,
        }
    }

    conn.execute(
        "CREATE TABLE metadata (
        name text primary key,
        value text
    );",
        params![],
    )?;
    conn.execute("CREATE TABLE tiles (tile_index integer primary key autoincrement, zoom_level integer, tile_column integer, tile_row integer, tile_data blob);", params![])?;

    set_metadata(&conn, "name", &options.name)?;
    set_metadata(&conn, "format", &options.format.extension())?;

    let name = options.name.clone();
    let format = options.format;

    Ok(MBTiles {
        connection: conn,
        name,
        format,
    })
}
pub fn create_mbtiles_db_hashdedup<T: AsRef<Path>>(_path: &T) -> Result<MBTiles> {
    todo!()
}
pub fn get_metadata(conn: &Connection, name: &str) -> Result<String> {
    let mut st = conn.prepare_cached("select value from metadata where name = :name;")?;
    let val = st.query_row(
        named_params! {
            ":name": name,
        },
        |r| r.get::<_, String>(0),
    )?;

    Ok(val)
}
pub fn set_metadata<T: AsRef<str>>(conn: &Connection, name: &str, value: &T) -> Result<()> {
    let mut st = conn.prepare_cached(
        "insert or replace into
        metadata (name, value)
        values (:name, :value);",
    )?;
    st.execute(named_params! {
        ":name": name,
        ":value": value.as_ref()
    })?;
    Ok(())
}
pub fn get_name(conn: &Connection) -> Result<String> {
    get_metadata(conn, "name")
}
pub fn contains_metadata(conn: &Connection, name: &str) -> Result<bool> {
    let mut st = conn.prepare_cached("select count(*) from metadata where name = :name")?;
    let rows = st.query_row(
        named_params! {
            ":name": name
        },
        |r| r.get::<_, i64>(0),
    )?;
    Ok(rows > 0)
}
pub fn get_format(conn: &Connection) -> Result<ImageFormat> {
    let value = get_metadata(conn, "format")?;
    ImageFormat::try_from(&value)
}
pub fn update_min_max_zooms(conn: &Connection, new_zoom: u8) -> Result<()> {
    let zoomstr = format!("{new_zoom}");
    let mut newmin = false;
    let mut newmax = false;
    if !contains_metadata(conn, "minzoom")? {
        set_metadata(conn, "minzoom", &zoomstr)?;
        newmin = true;
    }
    if !contains_metadata(conn, "maxzoom")? {
        set_metadata(conn, "maxzoom", &zoomstr)?;
        newmax = true;
    }

    if newmin && newmax {
        return Ok(());
    }

    let minzoom_str = get_metadata(conn, "minzoom")?;
    let maxzoom_str = get_metadata(conn, "maxzoom")?;

    let minzoom: u8 = minzoom_str.parse()?;
    if new_zoom < minzoom {
        set_metadata(conn, "minzoom", &zoomstr)?;
    }

    let maxzoom: u8 = maxzoom_str.parse()?;
    if new_zoom > maxzoom {
        set_metadata(conn, "maxzoom", &zoomstr)?;
    }

    Ok(())
}

#[derive(Debug, Clone, Default)]
pub struct CreateOptions {
    pub name: String,

    pub format: ImageFormat,

    pub pragmas: Vec<Pragma>,
}

impl From<&CreateOptions> for OpenOptions {
    fn from(value: &CreateOptions) -> Self {
        OpenOptions {
            pragmas: value.pragmas.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OpenOptions {
    pub pragmas: Vec<Pragma>,
}

impl OpenOptions {
    pub fn safe_performance() -> OpenOptions {
        OpenOptions {
            pragmas: vec![
                Pragma::ApplicationId(APPLICATION_ID),
                Pragma::PageSizeBytes(16384),
                Pragma::JournalMode(JournalMode::WAL),
                Pragma::LockingMode(LockingMode::Exclusive),
                Pragma::CacheSizeBytes(DataSizeUnits::Bytes.from(1, DataSizeUnits::Gigabytes)),
                Pragma::SynchronousMode(SynchronousMode::Normal),
            ],
        }
    }
}
