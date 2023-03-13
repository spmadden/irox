use bytes::Bytes;
use irox_mbtiles::Tile;

#[derive(Debug, Clone, Copy)]
pub struct TileAddress {
    pub x_index: u64,
    pub y_index: u64,
    pub zoom_level: u8,
}

impl<'a> TileAddress {
    pub fn as_tile(&'a self, data: &'a Bytes) -> Tile<'a> {
        Tile {
            tile_column: self.x_index,
            tile_row: self.y_index,
            zoom_level: self.zoom_level,
            tile_data: data,
        }
    }

    pub fn as_tile_data(&'a self, data: &'a Bytes) -> Tile<'a> {
        Tile {
            tile_column: self.x_index,
            tile_row: self.y_index,
            zoom_level: self.zoom_level,
            tile_data: data,
        }
    }
}

impl<'a> From<&Tile<'a>> for TileAddress {
    fn from(value: &Tile<'a>) -> Self {
        TileAddress {
            x_index: value.tile_column,
            y_index: value.tile_row,
            zoom_level: value.zoom_level,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TileAddressURL {
    pub address: TileAddress,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct TileData {
    pub address: TileAddressURL,
    pub data: Bytes,
}

impl<'a> TileData {
    pub fn as_tile_data(&'a self) -> Tile<'a> {
        Tile {
            tile_row: self.address.address.y_index,
            tile_column: self.address.address.x_index,
            zoom_level: self.address.address.zoom_level,
            tile_data: &self.data,
        }
    }
}
