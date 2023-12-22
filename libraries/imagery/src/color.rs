// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//


#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ARGBColor {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct HSVColor {
    pub hue: u32,
    pub saturation: u32,
    pub value: u32,
}

pub enum Color<'a> {

    RGB(RGBColor),

    ARGB(ARGBColor),

    HSV(HSVColor),

    Raw(&'a[u8])
}