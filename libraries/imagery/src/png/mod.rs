// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

pub mod chunks;
pub mod idat;

use crate::png::chunks::{GAMA, IDAT, IHDR, PHYS, SRGB};
use crate::png::idat::IDATStream;
use crate::{BoxedImage, Color, ColorDepth, ImageError, ImageMut};
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};
use irox_bits::{BitStreamDecoder, Bits, BitsError, BitsErrorKind, BitsWrapper};
use irox_compression::deflate::Inflater;
use irox_structs::Struct;
use irox_tools::hash::crc32::CRC32;

#[derive(Debug, Clone, PartialEq)]
pub struct PNGFile {
    chunks: Vec<ChunkType>,
    ihdr: IHDR,
}

impl PNGFile {
    pub fn read_from<T: Bits>(inp: &mut T) -> Result<Self, BitsError> {
        let sig = inp.read_be_u64()?;
        if sig != 0x89504E470D0A1A0A {
            return Err(BitsError::new(
                BitsErrorKind::FormatError,
                "Invalid PNG signature",
            ));
        }

        let mut chunks = Vec::new();
        let mut ihdr = None;
        loop {
            let chunk = ChunkType::read_chunk(inp)?;
            if let ChunkType::IEND() = chunk {
                chunks.push(chunk);
                break;
            }
            if let ChunkType::IHDR(ihdr_chunk) = chunk {
                ihdr = Some(ihdr_chunk);
                continue;
            }
            chunks.push(chunk);
        }
        let Some(ihdr) = ihdr else {
            return Err(BitsError::new(
                BitsErrorKind::FormatError,
                "Missing IHDR chunk",
            ));
        };
        Ok(Self { chunks, ihdr })
    }

    pub fn to_image(mut self) -> Result<BoxedImage, ImageError> {
        let mut img = BoxedImage::new(
            self.ihdr.width as usize,
            self.ihdr.height as usize,
            Color::Raw([0, 0, 0, 0]),
        );
        let chunks = self
            .chunks
            .drain(..)
            .filter_map(|chunk| {
                if let ChunkType::IDAT(idat) = chunk {
                    Some(idat.data)
                } else {
                    None
                }
            })
            .collect::<VecDeque<_>>();
        let iter = IDATStream::new(chunks);
        let inflater = Inflater::new_zlib(BitsWrapper::Owned(iter));
        let mut decoder = BitStreamDecoder::new(BitsWrapper::Owned(inflater.to_bits()));
        let depth = match self.ihdr.bit_depth {
            1 => ColorDepth::OneBitPerColor,
            2 => ColorDepth::TwoBitPerColor,
            4 => ColorDepth::FourBitPerColor,
            8 => ColorDepth::OneBytePerColor,
            16 => ColorDepth::TwoBytePerColor,
            _ => {
                return Err(BitsError::new(BitsErrorKind::FormatError, "Invalid bit depth").into())
            }
        };
        let colortype: PNGColorType = self.ihdr.color_type.try_into()?;
        for y in 0..self.ihdr.height {
            decoder.read_u32_bits(8)?; // skip filter byte
            for x in 0..self.ihdr.width {
                img.set_pixel_value(
                    x as usize,
                    y as usize,
                    colortype.read_from(&depth, &mut decoder)?,
                )?;
            }
        }
        Ok(img)
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PNGColorType {
    Grayscale,
    RGB,
    Indexed,
    GrayscaleAlpha,
    RGBA,
}
impl PNGColorType {
    pub fn read_from<T: Bits>(
        &self,
        depth: &ColorDepth,
        decoder: &mut BitStreamDecoder<T>,
    ) -> Result<Color, BitsError> {
        match self {
            PNGColorType::RGB => Ok(Color::RGB(depth.next_rgb_pixel(decoder)?)),
            PNGColorType::RGBA => Ok(Color::ARGB(depth.next_rgba_pixel(decoder)?)),
            PNGColorType::Indexed => {
                todo!()
            }
            PNGColorType::Grayscale => Ok(Color::Greyscale(depth.next_greyscale_pixel(decoder)?)),
            PNGColorType::GrayscaleAlpha => {
                todo!()
            }
        }
    }
}
impl TryFrom<u8> for PNGColorType {
    type Error = BitsError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PNGColorType::Grayscale),
            2 => Ok(PNGColorType::RGB),
            3 => Ok(PNGColorType::Indexed),
            4 => Ok(PNGColorType::GrayscaleAlpha),
            6 => Ok(PNGColorType::RGBA),
            _ => Err(BitsError::new(
                BitsErrorKind::FormatError,
                "Invalid color type",
            )),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ChunkType {
    IHDR(IHDR),
    PHYS(PHYS),
    IDAT(IDAT),
    GAMA(GAMA),
    SRGB(SRGB),
    IEND(),
    Unknown {
        chunk_type: u32,
        data: Vec<u8>,
        crc: u32,
    },
}
impl Debug for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ChunkType::Unknown {
                chunk_type,
                data,
                crc,
            } => f
                .debug_struct("Unknown")
                .field(
                    "chunk_type",
                    &str::from_utf8(&chunk_type.to_be_bytes()).unwrap_or_default(),
                )
                .field("data length", &data.len())
                .field("crc", crc)
                .finish(),
            ChunkType::IHDR(ihdr) => f.debug_struct("IHDR").field("data", &ihdr).finish(),
            ChunkType::PHYS(phys) => f.debug_struct("PHYS").field("data", &phys).finish(),
            ChunkType::GAMA(gama) => f.debug_struct("GAMA").field("data", &gama).finish(),
            ChunkType::SRGB(srgb) => f.debug_struct("SRGB").field("data", &srgb).finish(),
            ChunkType::IDAT(idat) => f
                .debug_struct("IDAT")
                .field("data len", &idat.data.len())
                .finish(),
            ChunkType::IEND() => f.debug_struct("IEND").finish(),
        }
    }
}

impl ChunkType {
    pub fn read_chunk<T: Bits>(inp: &mut T) -> Result<Self, BitsError> {
        let len = inp.read_be_u32()?;
        let chunk_type = inp.read_be_u32()?;
        let data = inp.read_exact_vec(len as usize)?;
        let crc = inp.read_be_u32()?;

        let mut calc_crc = CRC32::new();
        calc_crc.update(&chunk_type.to_be_bytes());
        calc_crc.update(&data);
        let calc_crc = calc_crc.finalize();

        if crc != calc_crc {
            return Err(BitsError::new(BitsErrorKind::FormatError, "Invalid CRC"));
        }
        Self::try_from(chunk_type, data, crc)
    }
    pub fn try_from(chunk_type: u32, data: Vec<u8>, crc: u32) -> Result<Self, BitsError> {
        match chunk_type {
            0x49484452 => {
                // IHDR
                let ihdr = IHDR::parse_from(&mut data.as_slice())?;
                Ok(Self::IHDR(ihdr))
            }
            0x49444154 => {
                // IDAT
                let idat = IDAT { data };
                Ok(Self::IDAT(idat))
            }
            0x70485973 => {
                // pHYs
                let phys = PHYS::parse_from(&mut data.as_slice())?;
                Ok(Self::PHYS(phys))
            }
            0x67414D41 => {
                // gAMA
                let gama = GAMA::parse_from(&mut data.as_slice())?;
                Ok(Self::GAMA(gama))
            }
            0x73524742 => {
                // sRGB
                let srgb = SRGB::parse_from(&mut data.as_slice())?;
                Ok(Self::SRGB(srgb))
            }
            0x49454E44 => {
                // IEND
                Ok(Self::IEND())
            }
            _ => Ok(Self::Unknown {
                chunk_type,
                data,
                crc,
            }),
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod test {
    use crate::png::PNGFile;
    use crate::{Image, ImageError, ImageSpace};
    use irox_bits::BitsWrapper;

    #[test]
    pub fn test() -> Result<(), ImageError> {
        let data = include_bytes!("../../assets/Ap5j5mb.png");
        let file = PNGFile::read_from(&mut BitsWrapper::Owned(data.as_slice()))?;
        println!("{file:#?}");
        let img = file.to_image()?;
        let dims = img.get_dimensions(ImageSpace::PIXEL);
        println!("{dims:#?}");
        Ok(())
    }
}
