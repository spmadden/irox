// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::{Error, Result};

#[derive(Debug, Clone, Copy, Default)]
pub enum RasterFormat {
    JPEG,

    #[default]
    PNG,

    WEBP,

    OtherExtension(&'static str),
}

#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    Vector,
    Raster(RasterFormat),
    OtherIETFMediaType(&'static str),
}

pub trait FileExtension {
    fn extension(&self) -> &'static str;
}

impl FileExtension for RasterFormat {
    fn extension(&self) -> &'static str {
        match self {
            RasterFormat::JPEG => "jpg",
            RasterFormat::PNG => "png",
            RasterFormat::WEBP => "webp",
            RasterFormat::OtherExtension(o) => o,
        }
    }
}

impl FileExtension for ImageFormat {
    fn extension(&self) -> &'static str {
        match self {
            ImageFormat::Vector => "pbf",
            ImageFormat::Raster(r) => r.extension(),
            ImageFormat::OtherIETFMediaType(o) => o,
        }
    }
}

impl TryFrom<&str> for ImageFormat {
    type Error = crate::Error;
    fn try_from(value: &str) -> Result<Self> {
        match value {
            "pbf" => Ok(ImageFormat::Vector),
            "png" => Ok(ImageFormat::Raster(RasterFormat::PNG)),
            "jpg" | "jpeg" => Ok(ImageFormat::Raster(RasterFormat::JPEG)),
            "webp" => Ok(ImageFormat::Raster(RasterFormat::WEBP)),
            _ => Error::unknown_format(format!("Unknown format: {value}")),
        }
    }
}

impl TryFrom<&String> for ImageFormat {
    type Error = crate::Error;

    fn try_from(value: &String) -> std::result::Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(value.as_str())
    }
}

impl Default for ImageFormat {
    fn default() -> Self {
        ImageFormat::Raster(Default::default())
    }
}
