// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Date Formatters
//!
//! | Letter | Component    | Description                                         | Repeated Ex.   | Example        |
//! |--------|--------------|-----------------------------------------------------|----------------|----------------|
//! |   Y    | Year         | Proleptic Gregorian Calendar Year                   | `YY` or `YYYY` | `05` or `2005` |
//! |   M    | Month        | Month of the year as decimal 01-12                  | `M` or `MM`    | `01`           |
//! |   D    | Day of Year  | Day of the year as decimal 01-366                   | `D` or `DDD`   | `075`          |
//! |   d    | Day of Month | Day of the month as decimal 01-31                   | `d` or `dd`    | `09`           |
//! |   H    | Hour         | Hour of day as decimal 00-24                        | `H` or `HH`    | `08`           |
//! |   m    | Minute       | Minute of hour as decimal 00-59                     | `m` or `mm`    | `07`           |
//! |   s    | Second       | Second of minute as decimal 00-59                   | `s` or `ss`    | `06`           |
//! |   S    | Millis       | Millisecond of Second as decimal 000-999            | `S` or `SSS`   | `051`          |
//! |   U    | Micros       | Microsecond of Second as decimal 000000-999999      | `U`            | `051020`       |
//! |   N    | Nanos        | Nanosecond of Second as decimal 000000000-999999999 | `N`            | `051020946`    |
//!
//!

use core::fmt::{Display, Formatter};
use core::num::{ParseFloatError, ParseIntError};

pub use alloc::string::{String, ToString};
use irox_units::bounds::GreaterThanEqualToValueError;

pub mod iso8601;
pub mod rfc3339;

///
/// Provides a mechanism to translate a date or a time to a [`String`]
pub trait Format<T> {
    ///
    /// Implementation-specific format of a date or time
    fn format(&self, date: &T) -> alloc::string::String;
}

///
/// Provides a mechanism to parse a date or time from a string.
///
pub trait FormatParser<T> {
    ///
    /// Tries to parse the specified string into the resultant item.
    fn try_from(&self, data: &str) -> Result<T, FormatError>;
}

///
/// Different format error conditions
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FormatErrorType {
    IOError,
    NumberFormatError,
    OutOfRangeError,
    Other,
}

///
/// Error type returned by the [`FormatParser`]s and [`Format`]ters
#[derive(Debug)]
pub struct FormatError {
    error_type: FormatErrorType,
    msg: alloc::string::String,
}

impl Display for FormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:?}{}", self.error_type, self.msg))
    }
}

impl core::error::Error for FormatError {}

impl FormatError {
    /// Creates a new format error
    #[must_use]
    pub fn new(error_type: FormatErrorType, msg: alloc::string::String) -> FormatError {
        FormatError { error_type, msg }
    }

    /// Helper for returning errors
    pub fn err<T>(msg: alloc::string::String) -> Result<T, Self> {
        Err(Self::new(FormatErrorType::Other, msg))
    }

    /// Helper for returning errors
    pub fn err_str<T>(msg: &'static str) -> Result<T, Self> {
        Err(Self::new(FormatErrorType::Other, msg.to_string()))
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for FormatError {
    fn from(value: std::io::Error) -> Self {
        FormatError {
            error_type: FormatErrorType::IOError,
            msg: value.to_string(),
        }
    }
}
impl From<ParseIntError> for FormatError {
    fn from(value: ParseIntError) -> Self {
        FormatError {
            error_type: FormatErrorType::NumberFormatError,
            msg: value.to_string(),
        }
    }
}
impl From<ParseFloatError> for FormatError {
    fn from(value: ParseFloatError) -> Self {
        FormatError {
            error_type: FormatErrorType::NumberFormatError,
            msg: value.to_string(),
        }
    }
}
impl From<GreaterThanEqualToValueError<u8>> for FormatError {
    fn from(value: GreaterThanEqualToValueError<u8>) -> Self {
        FormatError {
            error_type: FormatErrorType::OutOfRangeError,
            msg: value.to_string(),
        }
    }
}

impl From<GreaterThanEqualToValueError<u16>> for FormatError {
    fn from(value: GreaterThanEqualToValueError<u16>) -> Self {
        FormatError {
            error_type: FormatErrorType::OutOfRangeError,
            msg: value.to_string(),
        }
    }
}

impl From<GreaterThanEqualToValueError<f64>> for FormatError {
    fn from(value: GreaterThanEqualToValueError<f64>) -> Self {
        FormatError {
            error_type: FormatErrorType::OutOfRangeError,
            msg: value.to_string(),
        }
    }
}

impl From<GreaterThanEqualToValueError<u32>> for FormatError {
    fn from(value: GreaterThanEqualToValueError<u32>) -> Self {
        FormatError {
            error_type: FormatErrorType::OutOfRangeError,
            msg: value.to_string(),
        }
    }
}
