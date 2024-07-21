// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

pub mod error;
pub mod format;

pub mod merger;
pub mod sqlite_helpers;

use std::{fmt::Debug, path::Path};

pub use error::*;
pub use format::*;
use irox_units::units::{datasize::DataSizeUnits, FromUnits};
use rusqlite::{Connection, named_params, params};

pub use sqlite_helpers::*;

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

        let mut conn = Connection::open(path)?;

        for pragma in &options.pragmas {
            match pragma {
                Pragma::PageSizeBytes(_) => {
                    pragma.set(&conn)?;
                    conn.execute("VACUUM;", params![])?;
                    conn = Connection::open(path)?;
                },
                _ => {}
            }
        }

        for pragma in &options.pragmas {
            match pragma {
                Pragma::PageSizeBytes(_) => {},
                p => p.set(&conn)?
            }
        }

        let mut tables: Vec<String> = Vec::new();
        {
            let mut stmt = conn.prepare("select name from sqlite_master;")?;
            let mut rows = stmt.raw_query();
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

    pub fn open_or_create(path: &impl AsRef<Path>) -> Result<MBTiles> {
        Self::open_or_create_options(path, &CreateOptions::default())
    }

    pub fn open_or_create_options(
        path: &impl AsRef<Path>,
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
            "tile_column": tile_column,
            "tile_row": tile_row,
            "zoom_level": zoom_level
        ])?;

        if let Some(row) = rows.next()? {
            let res: Vec<u8> = row.get(0)?;
            return Ok(res);
        }
        Error::tile_not_found(tile_column, tile_row, zoom_level)
    }

    pub fn set_tile(
        &mut self,
        index: u64,
        tile_column: u64,
        tile_row: u64,
        zoom_level: u8,
        tile_data: &impl AsRef<[u8]>,
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

    pub fn insert_tile(&mut self, tile: &Tile) -> Result<()> {
        self.set_tile(
            tile.index(),
            tile.tile_column,
            tile.tile_row,
            tile.zoom_level,
            &tile.tile_data,
        )
    }

    pub fn foreach_tile<T: FnMut(&Tile)>(&mut self, cb: &mut T) -> std::result::Result<(), Error>{
        let conn = self.connection();
        let mut st = conn
            .prepare_cached("select tile_row, tile_column, zoom_level, tile_data from tiles;")?;
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

            let tile = Tile {
                tile_row: tile_row as u64,
                tile_column: tile_column as u64,
                zoom_level: zoom_level as u8,
                tile_data,
            };
            cb(&tile);
        }
        Ok(())
    }
}

impl Drop for MBTiles {
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

pub fn create_mbtiles_db(path: &impl AsRef<Path>, options: &CreateOptions) -> Result<MBTiles> {
    let path = path.as_ref();
    if path.exists() {
        return Error::io_exists(format!("DB already exists: {path:?}").as_str());
    }

    let conn = rusqlite::Connection::open(path)?;

    for pragma in &options.pragmas {
        match pragma {
            Pragma::PageSizeBytes(_) => {
                pragma.set(&conn)?;
                conn.execute("VACUUM;", params![])?;
                // conn = Connection::open(path)?;
            },
            _ => {}
        }
    }

    for pragma in &options.pragmas {
        match pragma {
            Pragma::PageSizeBytes(_) => {},
            _ => pragma.set(&conn)?
        }
    }

    conn.execute(
        "CREATE TABLE metadata (
        name text primary key, 
        value text
    );", params![]
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

pub fn create_mbtiles_db_hashdedup(_path: &impl AsRef<Path>) -> Result<MBTiles> {
    todo!()
}

pub fn get_metadata(conn: &Connection, name: &str) -> Result<String> {
    let mut st = conn.prepare_cached("select value from metadata where name = :name;")?;
    let mut rows = st.query(named_params! {
        ":name": name,
    })?;

    let Ok(Some(result)) = rows.next() else {
        return Error::not_found(format!("Could not find metadata with name {name}").as_str());
    };
    let value: String = result.get(0)?;

    Ok(value.to_string())
}

pub fn set_metadata(conn: &Connection, name: &str, value: &impl AsRef<str>) -> Result<()> {
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
    let rows = st.query_row(named_params! {
        ":name": name
    }, |r| {
        r.get::<_, i64>(0)
    })?;
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

pub struct Tile<'a> {
    pub tile_row: u64,
    pub tile_column: u64,
    pub zoom_level: u8,
    pub tile_data: &'a [u8],
}

impl<'a> Tile<'a> {
    pub fn index(&self) -> u64 {
        let mut val = ((self.zoom_level & ZOOM_MASK) as u64) << ZOOM_SHIFT;
        val |= (self.tile_row & ROWCOL_MASK) << Y_SHIFT;
        val |= (self.tile_column & ROWCOL_MASK) << X_SHIFT;
        val
    }
}

pub const ZOOM_MASK: u8 = 0x0;
pub const ZOOM_SHIFT: u8 = 58;
pub const ROWCOL_MASK: u64 = 0x1FFF_FFFF;
pub const Y_SHIFT: u8 = 29;
pub const X_SHIFT: u8 = 0;
