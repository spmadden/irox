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
use sqlite::Connection;

pub use sqlite_helpers::*;

pub struct MBTiles {
    name: String,
    format: ImageFormat,

    connection: Connection,
}

impl MBTiles {
    pub fn open(path: &impl AsRef<Path>) -> Result<MBTiles> {
        Self::open_options(path, &OpenOptions::default())
    }
    pub fn open_options(path: &impl AsRef<Path>, options: &OpenOptions) -> Result<MBTiles> {
        let conn = Connection::open(path)?;

        for pragma in &options.pragmas {
            pragma.set(&conn)?
        }

        let mut tables: Vec<String> = Vec::new();
        for row in conn.prepare("select name from sqlite_master;")? {
            let row = row?;
            let name: &str = row.try_read(0)?;
            tables.push(name.to_string());
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
        let mut st = self.connection.prepare(
            "select tile_data from tiles where
        tile_column = :tile_column and tile_row = :tile_row and zoom_level = :zoom_level;",
        )?;
        st.bind((":tile_column", tile_column as i64))?;
        st.bind((":tile_row", tile_row as i64))?;
        st.bind((":zoom_level", zoom_level as i64))?;

        if let Some(row) = st.into_iter().next() {
            let row = row?;
            let res: &[u8] = row.try_read(0)?;
            return Ok(Vec::from(res));
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

        let mut st = self.connection.prepare(
            "insert or replace into 
            tiles (tile_index, tile_row, tile_column, zoom_level, tile_data) 
            values (:index, :tile_row, :tile_column, :zoom_level, :tile_data);",
        )?;
        st.bind((":index", index as i64))?;
        st.bind((":tile_row", tile_row as i64))?;
        st.bind((":tile_column", tile_column as i64))?;
        st.bind((":zoom_level", zoom_level as i64))?;
        st.bind((":tile_data", tile_data.as_ref()))?;

        st.execute()
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

    pub fn foreach_tile<T: FnMut(&Tile)>(&mut self, cb: &mut T) {
        let conn = self.connection();
        let Ok(st) = conn
            .prepare("select tile_row, tile_column, zoom_level, tile_data from tiles;") else {
            eprintln!("Error processing statement");
            return;
        };
        for res in st.into_iter() {
            let Ok(row) = res else {
                eprintln!("Error retrieving tile row.");
                return;
            };

            let Ok(tile_row) : std::result::Result<i64, _> = row.try_read("tile_row") else {
                return;
            };
            let Ok(tile_column): std::result::Result<i64, _> = row.try_read("tile_column") else {
                return;
            };
            let Ok(zoom_level): std::result::Result<i64, _> = row.try_read("zoom_level") else {
                return;
            };
            let Ok(tile_data) = row.try_read("tile_data") else {
                return;
            };

            let tile = Tile {
                tile_row: tile_row as u64,
                tile_column: tile_column as u64,
                zoom_level: zoom_level as u8,
                tile_data,
            };
            cb(&tile);
        }
    }
}

impl Drop for MBTiles {
    fn drop(&mut self) {
        if let Err(e) = self.connection.execute("commit;") {
            eprintln!("Error committing DB: {e}")
        }

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

    let conn = sqlite::Connection::open(path)?;
    for pragma in &options.pragmas {
        pragma.set(&conn)?
    }

    conn.execute(
        "CREATE TABLE metadata (
        name text primary key, 
        value text
    );",
    )?;
    conn.execute("CREATE TABLE tiles (tile_index integer primary key autoincrement, zoom_level integer, tile_column integer, tile_row integer, tile_data blob);")?;

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

pub fn create_mbtiles_db_hashdedup(path: &impl AsRef<Path>) -> Result<MBTiles> {
    todo!()
}

pub fn get_metadata(conn: &Connection, name: &str) -> Result<String> {
    let mut st = conn.prepare("select value from metadata where name = :name;")?;
    st.bind((":name", name))?;

    let Some(result) = st.into_iter().next() else {
        return Error::not_found(format!("Could not find metadata with name {name}").as_str());
    };
    let row = result?;
    let value: &str = row.try_read(0)?;

    Ok(value.to_string())
}

pub fn set_metadata(conn: &Connection, name: &str, value: &impl AsRef<str>) -> Result<()> {
    let mut st = conn.prepare(
        "insert or replace into 
        metadata (name, value) 
        values (:name, :value);",
    )?;
    st.bind((":name", name))?;
    st.bind((":value", value.as_ref()))?;

    st.execute()
}

pub fn get_name(conn: &Connection) -> Result<String> {
    get_metadata(conn, "name")
}

pub fn contains_metadata(conn: &Connection, name: &str) -> Result<bool> {
    let mut st = conn.prepare("select count(*) from metadata where name = :name")?;
    st.bind((":name", name))?;

    if let Some(row) = st.into_iter().next() {
        let row = row?;
        let res: i64 = row.try_read(0)?;
        return Ok(res > 0);
    }
    Ok(false)
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
                Pragma::CacheSizeBytes(DataSizeUnits::Bytes.from(1, DataSizeUnits::GigaBytes)),
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
