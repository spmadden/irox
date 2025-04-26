// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use core::fmt::Debug;
use core::fmt::Formatter;
use irox_bits::{Bits, Error, ErrorKind};
use irox_enums::{EnumIterItem, EnumName};
use irox_time::datetime::UTCDateTime;
use irox_time::epoch::UnixTimestamp;
use irox_tools::arrays::SliceTools;
use irox_tools::hex::to_hex_str_upper;

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIterItem, EnumName)]
pub enum DataFormat {
    Binary,
    Text,
}
impl DataFormat {
    pub fn get_id(self) -> u8 {
        match self {
            DataFormat::Binary => 0x62,
            DataFormat::Text => 0x75,
        }
    }
}
impl TryFrom<u8> for DataFormat {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for f in Self::iter_items() {
            if f.get_id() == value {
                return Ok(f);
            }
        }
        Err(ErrorKind::InvalidInput.into())
    }
}
#[derive(Clone, Eq, PartialEq)]
pub struct LiteralData {
    pub format: DataFormat,
    pub filename: String,
    pub date: UTCDateTime,
    pub data: Vec<u8>,
}
impl Debug for LiteralData {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let data = if self.data.len() > 32 {
            format!(
                "[{}] {}...",
                self.data.len(),
                to_hex_str_upper(self.data.as_slice().limit(32))
            )
        } else {
            format!(
                "[{}] {}",
                self.data.len(),
                to_hex_str_upper(self.data.as_slice())
            )
        };
        f.debug_struct("LiteralData")
            .field("format", &self.format)
            .field("filename", &self.filename)
            .field("date", &self.date.format_iso8601_basic())
            .field("data", &data)
            .finish()
    }
}

impl TryFrom<&[u8]> for LiteralData {
    type Error = Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let format: DataFormat = value.read_u8()?.try_into()?;
        let flen = value.read_u8()?;
        let filename = value.read_str_sized_lossy(flen as usize)?;
        let date_sec = value.read_be_u32()?;
        let date: UTCDateTime = UnixTimestamp::from_seconds(date_sec).into();
        let data = value.read_all_vec()?;
        Ok(LiteralData {
            format,
            filename,
            date,
            data,
        })
    }
}
