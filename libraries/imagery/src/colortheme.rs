// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::Color;
use irox_tools::hex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ColorTheme {
    pub background: Color,
    pub foreground: Color,
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
}

pub static ONE_HALF_LIGHT: ColorTheme = ColorTheme {
    background: Color::rgb_array(&hex!("FAFAFA")),
    foreground: Color::rgb_array(&hex!("383A42")),
    black: Color::rgb_array(&hex!("383A42")),
    red: Color::rgb_array(&hex!("E45649")),
    green: Color::rgb_array(&hex!("50A14F")),
    yellow: Color::rgb_array(&hex!("C18401")),
    blue: Color::rgb_array(&hex!("0184BC")),
    magenta: Color::rgb_array(&hex!("A626A4")),
    cyan: Color::rgb_array(&hex!("0997B3")),
    white: Color::rgb_array(&hex!("FAFAFA")),
};
pub static ONE_HALF_DARK: ColorTheme = ColorTheme {
    background: Color::rgb_array(&hex!("282C34")),
    foreground: Color::rgb_array(&hex!("DCDFE4")),
    black: Color::rgb_array(&hex!("282C34")),
    red: Color::rgb_array(&hex!("E06C75")),
    green: Color::rgb_array(&hex!("98C379")),
    yellow: Color::rgb_array(&hex!("E5C07B")),
    blue: Color::rgb_array(&hex!("61AFEF")),
    magenta: Color::rgb_array(&hex!("C678DD")),
    cyan: Color::rgb_array(&hex!("56B6C2")),
    white: Color::rgb_array(&hex!("DCDFE4")),
};
