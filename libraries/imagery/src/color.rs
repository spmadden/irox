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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    RGB(RGBColor),

    ARGB(ARGBColor),

    HSV(HSVColor),

    Raw([u8; 4]),
}
impl Default for Color {
    fn default() -> Self {
        Color::default()
    }
}

impl Color {
    #[must_use]
    pub const fn default() -> Self {
        Self::rgb_hex(0x0)
    }
    #[must_use]
    pub const fn rgb_parts(red: u8, green: u8, blue: u8) -> Self {
        Self::RGB(RGBColor { red, green, blue })
    }
    #[must_use]
    pub const fn argb_parts(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Self::ARGB(ARGBColor {
            alpha,
            red,
            green,
            blue,
        })
    }
    #[must_use]
    pub const fn argb_array(arr: &[u8; 4]) -> Self {
        let [alpha, red, green, blue] = *arr;
        Self::argb_parts(alpha, red, green, blue)
    }
    #[must_use]
    pub const fn rgb_hex(hex: u32) -> Self {
        let [_a, red, green, blue] = hex.to_be_bytes();
        Self::rgb_parts(red, green, blue)
    }
    #[must_use]
    pub const fn argb_hex(hex: u32) -> Self {
        let [alpha, red, green, blue] = hex.to_be_bytes();
        Self::argb_parts(alpha, red, green, blue)
    }

    #[must_use]
    pub const fn to_rgb(&self) -> Self {
        match self {
            Self::RGB(_) | Self::ARGB(_) => *self,
            Self::Raw(val) => Self::argb_array(val),
            Self::HSV(_hsv) => {
                todo!()
            }
        }
    }
    #[must_use]
    pub const fn rgb_values(&self) -> [u8; 3] {
        match self {
            Color::RGB(v) => [v.red, v.green, v.blue],
            Color::ARGB(a) => [a.red, a.green, a.blue],
            Color::HSV(_) => {
                todo!()
            }
            Color::Raw(v) => {
                let [_, red, green, blue] = *v;
                [red, green, blue]
            }
        }
    }
    #[must_use]
    pub const fn argb_values(&self) -> [u8; 4] {
        match self {
            Color::RGB(v) => [255, v.red, v.green, v.blue],
            Color::ARGB(a) => [a.alpha, a.red, a.green, a.blue],
            Color::HSV(_) => {
                todo!()
            }
            Color::Raw(v) => {
                let [alpha, red, green, blue] = *v;
                [alpha, red, green, blue]
            }
        }
    }
}
