// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::{fmt::Display, num::ParseIntError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Other(String),
    IO(std::io::Error),
    Sqlite(sqlite::Error),
    NotFound(String),
    NotMBTiles(String),

    TileNotFound(String),
    UnknownFormat(String),
    NumberFormatError(ParseIntError),
}

impl Error {
    pub fn io(kind: std::io::ErrorKind, error: &str) -> Error {
        Error::IO(std::io::Error::new(kind, error))
    }

    pub fn io_exists<T>(error: &str) -> Result<T> {
        Err(Self::io(std::io::ErrorKind::AlreadyExists, error))
    }

    pub fn not_found<T>(error: &str) -> Result<T> {
        Err(Error::NotFound(error.to_string()))
    }

    pub fn not_mbtiles<T>(error: &str) -> Result<T> {
        Err(Error::NotMBTiles(error.to_string()))
    }

    pub fn tile_not_found<T>(tile_column: u64, tile_row: u64, zoom_level: u64) -> Result<T> {
        Err(Error::TileNotFound(format!(
            "Tile x({tile_column}) y({tile_row}) z({zoom_level}) not found"
        )))
    }

    pub fn unknown_format<T>(msg: String) -> Result<T> {
        Err(Error::UnknownFormat(msg))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<sqlite::Error> for Error {
    fn from(value: sqlite::Error) -> Self {
        Error::Sqlite(value)
    }
}

pub fn unwrap<T, E: Into<Error>>(val: std::result::Result<T, E>) -> Result<T> {
    val.map_err(|e| e.into())
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::NumberFormatError(value)
    }
}
