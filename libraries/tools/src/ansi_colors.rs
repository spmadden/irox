// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! List of some basic ANSI Console colors
//!

use crate::cfg_feature_std;

macro_rules! csi {
    () => {
        "\u{1B}["
    };
}
macro_rules! dcs {
    () => {
        "\u{1B}P"
    };
}
macro_rules! st {
    () => {
        "\u{1B}\\"
    };
}
pub const DCS: &str = dcs!();
pub const CSI: &str = csi!();
pub const ST: &str = st!();
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
/// Creates a ANSI Terminal Foreground Color Code using the specified RGB values
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
/// Creates a ANSI Terminal Background Color Code using the specified RGB values
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

def_item!(GET_TEXTAREA_SIZE_PIXELS, concat!(csi!(), "14t"));
def_item!(GET_TEXTAREA_SIZE_CHARS, concat!(csi!(), "18t"));
def_item!(GET_TERMINFO, concat!(dcs!(), "+q"));

cfg_feature_std! {
    pub fn get_textarea_size_pixels() -> Result<alloc::vec::Vec<u8>, std::io::Error> {
        use std::io::{BufRead, Write};
        let mut stdout = std::io::stdout().lock();
        let mut stdin = std::io::stdin().lock();
        stdout.write_all(GET_TEXTAREA_SIZE_CHARS.as_bytes())?;
        let mut out = alloc::vec::Vec::<u8>::new();
        stdin.read_until(b't', &mut out)?;
        Ok(out)
    }
    pub fn get_termcap(s: &str) -> Result<alloc::vec::Vec<u8>, std::io::Error> {
        use std::io::{Write};
        let mut stdout = std::io::stdout().lock();
        let _stdin = std::io::stdin().lock();
        stdout.write_all(GET_TERMINFO.as_bytes())?;
        stdout.write_all(s.as_bytes())?;
        stdout.write_all(ST.as_bytes())?;
        let _out = alloc::vec::Vec::<u8>::new();
        // stdin.read(&mut out)?;
        Ok(_out)
    }
}
