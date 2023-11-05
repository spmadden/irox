// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! List of some basic ANSI Console colors
//!

macro_rules! csi {
    () => {
        "\u{1B}["
    };
}

pub const DCS: &str = "\u{1B}P";
pub const CSI: &str = csi!();
pub const ST: &str = "\u{1B}\\";
pub const OSC: &str = "\u{1B}]";

macro_rules! sgr {
    ($($lit:expr)*) => {
        concat!(csi!(), $($lit,)* "m")
    };
}

macro_rules! def_item {
    ($name:ident,$value:expr) => {
        pub const $name: &str = $value;
    };
}

def_item!(FORMAT_RESET, sgr!(0));
def_item!(FORMAT_BOLD, sgr!(1));

def_item!(FORMAT_COLOR_FG_BLACK, sgr!(30));
def_item!(FORMAT_COLOR_FG_RED, sgr!(31));
def_item!(FORMAT_COLOR_FG_GREEN, sgr!(32));
def_item!(FORMAT_COLOR_FG_YELLOW, sgr!(33));
def_item!(FORMAT_COLOR_FG_BLUE, sgr!(34));
def_item!(FORMAT_COLOR_FG_MAGENTA, sgr!(35));
def_item!(FORMAT_COLOR_FG_CYAN, sgr!(36));
def_item!(FORMAT_COLOR_FG_WHITE, sgr!(37));
def_item!(FORMAT_COLOR_FG_DEFAULT, sgr!(39));

///
/// ```
/// use irox_tools::format_fg_color;
/// pub const VARBL : &str = format_fg_color!(0x10,0x10,0x10);
///
/// assert_eq!("\u{1B}[38;2;16;16;16m", VARBL);
/// ```
#[macro_export]
macro_rules! format_fg_color {
    ($red:literal,$green:literal,$blue:literal) => {
        concat!("\u{1B}[38;2;", $red, ";", $green, ";", $blue, "m")
    };
}

def_item!(FORMAT_COLOR_BG_BLACK, sgr!(40));
def_item!(FORMAT_COLOR_BG_RED, sgr!(41));
def_item!(FORMAT_COLOR_BG_GREEN, sgr!(42));
def_item!(FORMAT_COLOR_BG_YELLOW, sgr!(43));
def_item!(FORMAT_COLOR_BG_BLUE, sgr!(44));
def_item!(FORMAT_COLOR_BG_MAGENTA, sgr!(45));
def_item!(FORMAT_COLOR_BG_CYAN, sgr!(46));
def_item!(FORMAT_COLOR_BG_WHITE, sgr!(47));
def_item!(FORMAT_COLOR_BG_DEFAULT, sgr!(49));

///
/// ```
/// use irox_tools::format_bg_color;
/// pub const VARBL : &str = format_bg_color!(0x10,0x10,0x10);
///
/// assert_eq!("\u{1B}[48;2;16;16;16m", VARBL);
/// ```
#[macro_export]
macro_rules! format_bg_color {
    ($red:literal,$green:literal,$blue:literal) => {
        concat!("\u{1B}[48;2;", $red, ";", $green, ";", $blue, "m")
    };
}
