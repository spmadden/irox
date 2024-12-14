// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

mod classic;
mod tableau;
mod turbo;

use crate::Color;
pub use classic::*;
use irox_tools::ToF64;
pub use tableau::*;
pub use turbo::*;

#[derive(Clone, Copy)]
pub struct Colormap {
    data: &'static [Color],
}
impl Colormap {
    pub const fn new(data: &'static [Color]) -> Colormap {
        Colormap { data }
    }
    pub fn interp_percent(&self, percent: f64) -> Color {
        let percent = percent.clamp(0f64, 1f64);
        let idx = (percent * self.data.len() as f64) as usize;
        let colors = self.data.windows(2).nth(idx).unwrap_or_default();
        let first = colors.first().copied().unwrap_or_default();
        let second = colors.get(1).copied().unwrap_or_default();
        let offset = percent - (idx as f64);

        let [r1, g1, b1] = first.rgb_values();
        let [r2, g2, b2] = second.rgb_values();
        let r = interp_part(r1, r2, offset);
        let g = interp_part(g1, g2, offset);
        let b = interp_part(b1, b2, offset);
        Color::rgb_parts(r, g, b)
    }
}
pub struct ValueMap<T> {
    colors: Colormap,
    min_value: T,
    max_value: T,
}
impl<T: core::cmp::Ord + Copy + core::ops::Sub<Output = F>, F: ToF64> ValueMap<T> {
    pub fn new(colors: Colormap, min_value: T, max_value: T) -> Self {
        Self {
            colors,
            min_value,
            max_value,
        }
    }
    pub fn color_for_value(&self, value: T) -> Color {
        let value = value.clamp(self.min_value, self.max_value);
        let range = self.max_value - self.min_value;
        let offset = value - self.min_value;
        let pct = offset.to_f64() / range.to_f64();
        self.colors.interp_percent(pct)
    }
}

const fn const_clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
const fn interp_part(first: u8, second: u8, offset: f64) -> u8 {
    let offset = const_clamp(offset, 0f64, 1f64);
    let first = first as f64;
    let second = second as f64;
    let scaled = (second - first) * offset;
    (first + scaled) as u8
}
