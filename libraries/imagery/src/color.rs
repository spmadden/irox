// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{BitStreamDecoder, Bits, BitsError};
use irox_tools::cfg_feature_egui;

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

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct HSVColor {
    /// Hue range [0,360)
    pub hue: u32,
    /// Saturation range [0,1]
    pub saturation: f64,
    /// Value range [0..1]
    pub value: f64,
}
impl HSVColor {
    #[must_use]
    pub const fn new(hue: u32, saturation: f64, value: f64) -> Self {
        Self {
            hue,
            saturation,
            value,
        }
    }
    #[must_use]
    pub const fn to_rgb(&self) -> RGBColor {
        let HSVColor {
            hue,
            saturation,
            value,
        } = *self;
        let saturation = saturation.clamp(0.0, 1.0);
        let value = value.clamp(0.0, 1.0);
        let c = value * saturation;
        let adj = ((hue as f64 / 60.) % 2.) - 1.;
        let adj = 1. - adj.abs();
        let x = c * adj;
        let m = value - c;
        let [r, g, b] = match hue {
            0..60 => [c, x, 0.0],
            60..120 => [x, c, 0.0],
            120..180 => [0.0, c, x],
            180..240 => [0.0, x, c],
            240..300 => [x, 0.0, c],
            300.. => [c, 0.0, x],
        };
        let red = ((r + m) * 255.) as u8;
        let green = ((g + m) * 255.) as u8;
        let blue = ((b + m) * 255.) as u8;
        RGBColor { red, green, blue }
    }
}
impl Eq for HSVColor {}
impl From<HSVColor> for RGBColor {
    fn from(value: HSVColor) -> Self {
        value.to_rgb()
    }
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Greyscale8Bit {
    pub value: u8,
}
impl From<u8> for Greyscale8Bit {
    fn from(value: u8) -> Self {
        Self { value }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    RGB(RGBColor),

    ARGB(ARGBColor),

    HSV(HSVColor),

    Greyscale(Greyscale8Bit),

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
            Self::HSV(hsv) => Self::RGB(hsv.to_rgb()),
            Self::Greyscale(g) => {
                let v = g.value;
                Self::argb_array(&[0xFF, v, v, v])
            }
        }
    }
    #[must_use]
    pub const fn rgb_values(&self) -> [u8; 3] {
        match self {
            Color::RGB(v) => [v.red, v.green, v.blue],
            Color::ARGB(a) => [a.red, a.green, a.blue],
            Color::HSV(h) => {
                let v = h.to_rgb();
                [v.red, v.green, v.blue]
            }
            Color::Raw(v) => {
                let [_, red, green, blue] = *v;
                [red, green, blue]
            }
            Color::Greyscale(g) => {
                let v = g.value;
                [v, v, v]
            }
        }
    }
    #[must_use]
    pub const fn argb_values(&self) -> [u8; 4] {
        match self {
            Color::RGB(v) => [255, v.red, v.green, v.blue],
            Color::ARGB(a) => [a.alpha, a.red, a.green, a.blue],
            Color::HSV(h) => {
                let v = h.to_rgb();
                [255, v.red, v.green, v.blue]
            }
            Color::Raw(v) => {
                let [alpha, red, green, blue] = *v;
                [alpha, red, green, blue]
            }
            Color::Greyscale(g) => {
                let v = g.value;
                [0xFF, v, v, v]
            }
        }
    }
}
cfg_feature_egui! {
    impl From<Color> for egui::Color32 {
        fn from(value: Color) -> Self {
            let [a, r,g,b] = value.argb_values();
            egui::ecolor::Color32::from_rgba_unmultiplied(r,g,b,a)
        }
    }
    impl From<&Color> for egui::Color32 {
        fn from(value: &Color) -> Self {
            let [a, r,g,b] = value.argb_values();
            egui::ecolor::Color32::from_rgba_unmultiplied(r,g,b,a)
        }
    }
}

pub enum ColorDepth {
    OneBitPerColor,
    TwoBitPerColor,
    ThreeBitPerColor,
    FourBitPerColor,
    OneBytePerColor,
    TwoBytePerColor,
}
impl ColorDepth {
    pub fn bits_per_color(&self) -> u8 {
        match self {
            ColorDepth::OneBitPerColor => 1,
            ColorDepth::TwoBitPerColor => 2,
            ColorDepth::ThreeBitPerColor => 3,
            ColorDepth::FourBitPerColor => 4,
            ColorDepth::OneBytePerColor => 8,
            ColorDepth::TwoBytePerColor => 16,
        }
    }
    pub fn next_raw_color_part<T: Bits>(
        &self,
        inp: &mut BitStreamDecoder<T>,
    ) -> Result<u16, BitsError> {
        Ok(inp.read_u32_bits(self.bits_per_color())? as u16)
    }
    pub fn next_byte_stretched_color<T: Bits>(
        &self,
        inp: &mut BitStreamDecoder<T>,
    ) -> Result<u8, BitsError> {
        let v = self.next_raw_color_part(inp)?;
        let shift = self.bits_per_color() - 1;
        Ok(v as u8 * (0xFFu8 >> shift))
    }
    pub fn next_greyscale_pixel<T: Bits>(
        &self,
        inp: &mut BitStreamDecoder<T>,
    ) -> Result<Greyscale8Bit, BitsError> {
        Ok(Greyscale8Bit {
            value: self.next_byte_stretched_color(inp)?,
        })
    }
    pub fn next_rgb_pixel<T: Bits>(
        &self,
        inp: &mut BitStreamDecoder<T>,
    ) -> Result<RGBColor, BitsError> {
        Ok(RGBColor {
            red: self.next_byte_stretched_color(inp)?,
            green: self.next_byte_stretched_color(inp)?,
            blue: self.next_byte_stretched_color(inp)?,
        })
    }
    pub fn next_argb_pixel<T: Bits>(
        &self,
        inp: &mut BitStreamDecoder<T>,
    ) -> Result<ARGBColor, BitsError> {
        Ok(ARGBColor {
            alpha: self.next_byte_stretched_color(inp)?,
            red: self.next_byte_stretched_color(inp)?,
            green: self.next_byte_stretched_color(inp)?,
            blue: self.next_byte_stretched_color(inp)?,
        })
    }
}
