// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::colormaps::Colormap;
use crate::Color;

pub static TABLEAU_10: &[Color; 10] = &[
    Color::rgb_hex(0x4E79A7),
    Color::rgb_hex(0xF28E2B),
    Color::rgb_hex(0xE15759),
    Color::rgb_hex(0x76B7B2),
    Color::rgb_hex(0x59A14F),
    Color::rgb_hex(0xEDC948),
    Color::rgb_hex(0xB07AA1),
    Color::rgb_hex(0xFF9DA7),
    Color::rgb_hex(0x9C755F),
    Color::rgb_hex(0xBAB0AC),
];

pub static TABLEAU_10_COLORMAP: Colormap = Colormap::new(TABLEAU_10);
