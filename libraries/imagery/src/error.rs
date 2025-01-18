// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::{Display, Formatter};
use irox_tools::impl_from_error;

#[derive(Debug, Clone)]
pub enum ImageErrorType {
    BitsError,
    BadMagic,
    BadByteOrder,
    ParseError,
}
impl<T> From<ImageErrorType> for Result<T, ImageError> {
    fn from(ty: ImageErrorType) -> Self {
        Err(ImageError {
            msg: match ty {
                ImageErrorType::BadMagic => "Bad Magic Value".to_string(),
                ImageErrorType::BadByteOrder => "Bad Byte Order Value".to_string(),
                ImageErrorType::BitsError => "Bits Error".to_string(),
                ImageErrorType::ParseError => "Parse Error".to_string(),
            },
            error_type: ty,
        })
    }
}
#[derive(Debug, Clone)]
pub struct ImageError {
    msg: String,
    error_type: ImageErrorType,
}

impl Display for ImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "ImageError({:?}): {}", self.error_type, self.msg)
    }
}

impl core::error::Error for ImageError {}

impl ImageError {
    pub fn bad_magic() -> ImageError {
        ImageError {
            error_type: ImageErrorType::BadMagic,
            msg: "Bad magic number".to_string(),
        }
    }
    pub fn bad_type(ty: u16) -> ImageError {
        ImageError {
            error_type: ImageErrorType::ParseError,
            msg: format!("Bad Type: {ty}"),
        }
    }
    pub fn not_enough_values() -> ImageError {
        ImageError {
            error_type: ImageErrorType::ParseError,
            msg: "Not enough values".to_string(),
        }
    }
}

impl_from_error!(ImageError, irox_bits::BitsError, ImageErrorType::BitsError);
