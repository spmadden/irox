// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::colormaps::Colormap;
use crate::Color;

pub static CLASSIC_10: &[Color; 10] = &[
    Color::rgb_hex(0x1F77B4),
    Color::rgb_hex(0xFF7F0E),
    Color::rgb_hex(0x2CA02C),
    Color::rgb_hex(0xD62728),
    Color::rgb_hex(0x9467BD),
    Color::rgb_hex(0x8C564B),
    Color::rgb_hex(0xE377C2),
    Color::rgb_hex(0x7F7F7F),
    Color::rgb_hex(0xBCBD22),
    Color::rgb_hex(0x17BECF),
];

pub static CLASSIC_10_COLORMAP: Colormap = Colormap::new(CLASSIC_10);

pub static CLASSIC_20: &[Color; 20] = &[
    Color::rgb_hex(0x1F77B4),
    Color::rgb_hex(0xAEC7E8),
    Color::rgb_hex(0xFF7F0E),
    Color::rgb_hex(0xFFBB78),
    Color::rgb_hex(0x2CA02C),
    Color::rgb_hex(0x98DF8A),
    Color::rgb_hex(0xD62728),
    Color::rgb_hex(0xFF9896),
    Color::rgb_hex(0x9467BD),
    Color::rgb_hex(0xC5B0D5),
    Color::rgb_hex(0x8C564B),
    Color::rgb_hex(0xC49C94),
    Color::rgb_hex(0xE377C2),
    Color::rgb_hex(0xF7B6D2),
    Color::rgb_hex(0x7F7F7F),
    Color::rgb_hex(0xC7C7C7),
    Color::rgb_hex(0xBCBD22),
    Color::rgb_hex(0xDBDB8D),
    Color::rgb_hex(0x17BECF),
    Color::rgb_hex(0x9EDAE5),
];

pub static CLASSIC_20_COLORMAP: Colormap = Colormap::new(CLASSIC_20);
