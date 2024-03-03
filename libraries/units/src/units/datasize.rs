// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Contains [`DataSize`] and [`DataSizeUnits`] - Physical Quantities of computer binary storage
//!

use crate::units::Unit;
use core::fmt::{Display, Formatter};

use super::FromUnits;

///
/// Physical unit of computer storage, a Byte is eight Bits.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::integer_division)]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum DataSizeUnits {
    /// Eight Bits
    #[default]
    Bytes,
    /// 1000 Bytes
    Kilobytes,
    /// 1000 Kilobytes
    Megabytes,
    /// 1000 Megabytes
    Gigabytes,
    /// 1000 Gigabytes
    Terabytes,
    /// 1000 Terabytes
    Petabytes,
}

macro_rules! from_units_datasize {
    ($type:ident) => {
        #[allow(clippy::integer_division)]
        impl crate::units::FromUnits<$type> for DataSizeUnits {
            fn from(&self, value: $type, units: Self) -> $type {
                match self {
                    DataSizeUnits::Bytes => match units {
                        DataSizeUnits::Bytes => value,
                        DataSizeUnits::Kilobytes => value * KB_TO_BYTES as $type,
                        DataSizeUnits::Megabytes => value * MB_TO_BYTES as $type,
                        DataSizeUnits::Gigabytes => value * GB_TO_BYTES as $type,
                        DataSizeUnits::Terabytes => value * TB_TO_BYTES as $type,
                        DataSizeUnits::Petabytes => value * PB_TO_BYTES as $type,
                    },
                    DataSizeUnits::Kilobytes => match units {
                        DataSizeUnits::Bytes => value / KB_TO_BYTES as $type,
                        DataSizeUnits::Kilobytes => value,
                        DataSizeUnits::Megabytes => value * KB_TO_BYTES as $type,
                        DataSizeUnits::Gigabytes => value * MB_TO_BYTES as $type,
                        DataSizeUnits::Terabytes => value * GB_TO_BYTES as $type,
                        DataSizeUnits::Petabytes => value * TB_TO_BYTES as $type,
                    },
                    DataSizeUnits::Megabytes => match units {
                        DataSizeUnits::Bytes => value / MB_TO_BYTES as $type,
                        DataSizeUnits::Kilobytes => value / KB_TO_BYTES as $type,
                        DataSizeUnits::Megabytes => value,
                        DataSizeUnits::Gigabytes => value * KB_TO_BYTES as $type,
                        DataSizeUnits::Terabytes => value * MB_TO_BYTES as $type,
                        DataSizeUnits::Petabytes => value * GB_TO_BYTES as $type,
                    },
                    DataSizeUnits::Gigabytes => match units {
                        DataSizeUnits::Bytes => value / GB_TO_BYTES as $type,
                        DataSizeUnits::Kilobytes => value / MB_TO_BYTES as $type,
                        DataSizeUnits::Megabytes => value / KB_TO_BYTES as $type,
                        DataSizeUnits::Gigabytes => value,
                        DataSizeUnits::Terabytes => value * KB_TO_BYTES as $type,
                        DataSizeUnits::Petabytes => value * MB_TO_BYTES as $type,
                    },
                    DataSizeUnits::Terabytes => match units {
                        DataSizeUnits::Bytes => value / TB_TO_BYTES as $type,
                        DataSizeUnits::Kilobytes => value / GB_TO_BYTES as $type,
                        DataSizeUnits::Megabytes => value / MB_TO_BYTES as $type,
                        DataSizeUnits::Gigabytes => value / KB_TO_BYTES as $type,
                        DataSizeUnits::Terabytes => value,
                        DataSizeUnits::Petabytes => value * KB_TO_BYTES as $type,
                    },
                    DataSizeUnits::Petabytes => match units {
                        DataSizeUnits::Bytes => value / PB_TO_BYTES as $type,
                        DataSizeUnits::Kilobytes => value / TB_TO_BYTES as $type,
                        DataSizeUnits::Megabytes => value / GB_TO_BYTES as $type,
                        DataSizeUnits::Gigabytes => value / MB_TO_BYTES as $type,
                        DataSizeUnits::Terabytes => value / KB_TO_BYTES as $type,
                        DataSizeUnits::Petabytes => value,
                    },
                }
            }
        }
    };
}

from_units_datasize!(f32);
from_units_datasize!(f64);
from_units_datasize!(i64);
from_units_datasize!(u64);
from_units_datasize!(usize);

basic_unit!(DataSize, DataSizeUnits, Bytes);

impl Unit<DataSizeUnits> for DataSize {
    fn as_unit(&self, units: DataSizeUnits) -> Self
    where
        Self: Sized,
    {
        let value: f64 = units.from(self.value, self.units);
        DataSize { value, units }
    }
}

impl DataSize {
    /// Creates a new DataSize quanitty
    #[must_use]
    pub fn new_bytes(&self, value: u64) -> DataSize {
        Self::new(value as f64, DataSizeUnits::Bytes)
    }

    #[must_use]
    pub fn as_bytes(&self) -> u64 {
        match self.units {
            DataSizeUnits::Bytes => self.value as u64,
            DataSizeUnits::Kilobytes => (self.value * KB_TO_BYTES as f64) as u64,
            DataSizeUnits::Megabytes => (self.value * MB_TO_BYTES as f64) as u64,
            DataSizeUnits::Gigabytes => (self.value * GB_TO_BYTES as f64) as u64,
            DataSizeUnits::Terabytes => (self.value * TB_TO_BYTES as f64) as u64,
            DataSizeUnits::Petabytes => (self.value * PB_TO_BYTES as f64) as u64,
        }
    }
}

impl Display for DataSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let bytes = self.as_bytes();
        let frac = f.precision().unwrap_or(3);
        if bytes < KB_TO_BYTES {
            return write!(f, "{bytes} bytes");
        } else if bytes < MB_TO_BYTES {
            let val = bytes as f64 / KB_TO_BYTES as f64;
            return write!(f, "{val:.frac$} KB");
        } else if bytes < GB_TO_BYTES {
            let val = bytes as f64 / MB_TO_BYTES as f64;
            return write!(f, "{val:.frac$} MB");
        } else if bytes < TB_TO_BYTES {
            let val = bytes as f64 / GB_TO_BYTES as f64;
            return write!(f, "{val:.frac$} GB");
        } else if bytes < PB_TO_BYTES {
            let val = bytes as f64 / TB_TO_BYTES as f64;
            return write!(f, "{val:.frac$} TB");
        }
        let val = bytes as f64 / PB_TO_BYTES as f64;
        write!(f, "{val:.frac$} PB")
    }
}

/// Kilobyte to Byte factor
pub const KB_TO_BYTES: u64 = 1000;
/// Megabyte to Byte factor
pub const MB_TO_BYTES: u64 = KB_TO_BYTES * 1000;
/// Gigabyte to Byte factor
pub const GB_TO_BYTES: u64 = MB_TO_BYTES * 1000;
/// Terabyte to Byte factor
pub const TB_TO_BYTES: u64 = GB_TO_BYTES * 1000;
/// Petabyte to Byte factor
pub const PB_TO_BYTES: u64 = TB_TO_BYTES * 1000;
