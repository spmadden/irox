// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Color, Greyscale8Bit, LinearStackedImage};

pub mod font_8x16;
pub mod font_ter16x32;
pub mod nums;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GlyphInfo<const W: u8, const H: u8, const L: usize> {
    pub glyphmask: [u128; 2],
    pub num_glyphs: usize,
    pub start_glyph: usize,
    pub packed_data: &'static [u8],
}

impl<const W: u8, const H: u8, const L: usize> GlyphInfo<W, H, L> {
    const fn glyphlen(&self) -> usize {
        self.packed_data.len() / self.num_glyphs
    }
    fn subdata(&self, glyph: u8) -> Option<&'static [u8]> {
        let [a, b] = self.glyphmask;
        let mut chk = glyph;
        let mut mask = a;
        if glyph >= 128 {
            chk -= 128;
            mask = b;
        }
        if mask & (1 << chk) == 0 {
            return None;
        }
        let glyphlen = self.glyphlen();
        let start = (glyph as usize - self.start_glyph) * glyphlen;
        let end = start + glyphlen;
        self.packed_data.get(start..end)
    }
    #[allow(clippy::indexing_slicing)]
    const fn expand(&self, img: &[u8]) -> [u8; L] {
        let mut out = [0u8; L];
        let mut idx = 0;
        let mut j = 0;
        while j < self.glyphlen() {
            let px = img[j];
            let mut k = 0;
            while k < 8 {
                let sh = 7 - k;
                let v = (px >> sh) & 0x01;
                out[idx] = v * 0xFF;
                idx += 1;
                k += 1;
            }
            j += 1;
        }
        out
    }

    #[allow(clippy::indexing_slicing)]
    const fn expand_to_img(self, img: &[u8]) -> LinearStackedImage<L> {
        let mut data = [Color::Greyscale(Greyscale8Bit { value: 0 }); L];
        let exp = self.expand(img);
        let mut i = 0;

        while i < L {
            data[i] = Color::Greyscale(Greyscale8Bit { value: exp[i] });
            i += 1;
        }
        LinearStackedImage {
            data,
            width: W as usize,
            height: H as usize,
            world_dimensions: None,
        }
    }
    pub fn get_glyph(&self, glyph: u8) -> Option<LinearStackedImage<L>> {
        Some(self.expand_to_img(self.subdata(glyph)?))
    }
}
