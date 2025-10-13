// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

pub struct Tile<'a> {
    pub tile_row: u64,
    pub tile_column: u64,
    pub zoom_level: u8,
    pub tile_data: &'a [u8],
}

impl Tile<'_> {
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
