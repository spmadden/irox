// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use irox_structs::Struct;

#[derive(Debug, Clone, PartialEq, Struct)]
pub struct IHDR {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

#[derive(Debug, Clone, PartialEq, Struct)]
pub struct PHYS {
    pub pixels_per_unit_x: u32,
    pub pixels_per_unit_y: u32,
    pub unit_specifier: u8,
}

#[derive(Debug, Clone, PartialEq, Struct)]
pub struct GAMA {
    pub gamma: u32,
}

#[derive(Debug, Clone, PartialEq, Struct)]
pub struct SRGB {
    pub rendering_intent: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IDAT {
    pub data: Vec<u8>,
}
