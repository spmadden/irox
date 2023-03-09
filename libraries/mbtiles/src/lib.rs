pub mod error;
pub mod format;

pub mod merger;
pub mod sqlite_helpers;

use std::{collections::HashMap, fmt::Debug, path::Path};

pub use error::*;
pub use format::*;
use sqlite::Connection;

pub use sqlite_helpers::*;

pub struct MBTiles {
    name: String,
    format: ImageFormat,

    connection: Connection,
}

impl MBTiles {
    pub fn open(path: &impl AsRef<Path>) -> Result<MBTiles> {
        let conn = Connection::open(path)?;

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
            return Self::open(path);
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
        tile_column: i64,
        tile_row: i64,
        zoom_level: i64,
        tile_data: &impl AsRef<[u8]>,
    ) -> Result<()> {
        let mut st = self.connection.prepare(
            "insert into tiles tile_row, tile_column, zoom_level, tile_data 
            values (:tile_row, :tile_column, :zoom_level, :tile_data);",
        )?;
        st.bind((":tile_row", tile_row))?;
        st.bind((":tile_column", tile_column))?;
        st.bind((":zoom_level", zoom_level))?;
        st.bind((":tile_data", tile_data.as_ref()))?;

        st.execute()
    }

    pub fn connection(&mut self) -> &Connection {
        &self.connection
    }

    pub fn insert_tile(&mut self, tile: &Tile) -> Result<()> {
        self.set_tile(
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

            let Ok(tile_row) = row.try_read("tile_row") else {
                return;
            };
            let Ok(tile_column) = row.try_read("tile_column") else {
                return;
            };
            let Ok(zoom_level) = row.try_read("zoom_level") else {
                return;
            };
            let Ok(tile_data) = row.try_read("tile_data") else {
                return;
            };

            let tile = Tile {
                tile_row,
                tile_column,
                zoom_level,
                tile_data,
            };
            cb(&tile);
        }
    }
}

impl Debug for MBTiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut st = f.debug_struct("MBTiles");
        st.field("name", &self.name).field("format", &self.format);

        for pragma in &[Pragma::ApplicationId, Pragma::CacheSize, Pragma::PageSize] {
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
    for (pragma, val) in &options.pragmas {
        pragma.set(&conn, *val)?
    }

    conn.execute("CREATE TABLE metadata (name text, value text);")?;
    conn.execute("CREATE TABLE tiles (zoom_level integer, tile_column integer, tile_row integer, tile_data blob);")?;
    conn.execute("CREATE UNIQUE INDEX tile_index on tiles (zoom_level, tile_column, tile_row);")?;

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
        return Error::not_found("no rows returned");
    };
    let row = result?;
    let value: &str = row.try_read(0)?;

    Ok(value.to_string())
}

pub fn set_metadata(conn: &Connection, name: &str, value: &impl AsRef<str>) -> Result<()> {
    let mut st = conn.prepare("insert into metadata (name, value) values (:name, :value);")?;
    st.bind((":name", name))?;
    st.bind((":value", value.as_ref()))?;

    st.execute()
}

pub fn get_name(conn: &Connection) -> Result<String> {
    get_metadata(conn, "name")
}

pub fn get_format(conn: &Connection) -> Result<ImageFormat> {
    let value = get_metadata(conn, "format")?;
    ImageFormat::try_from(&value)
}

#[derive(Debug, Clone, Default)]
pub struct CreateOptions {
    pub name: String,

    pub format: ImageFormat,

    pub pragmas: HashMap<Pragma, i64>,
}

pub struct Tile<'a> {
    pub tile_row: i64,
    pub tile_column: i64,
    pub zoom_level: i64,
    pub tile_data: &'a [u8],
}
