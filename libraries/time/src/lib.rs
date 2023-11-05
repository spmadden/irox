// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Module Structure
//! -----------------
//!  * [`crate`] - Contains the base `Time` struct, describing a standard `Hours/minutes/seconds` framework.
//!  * [`datetime`] - Contains `UTCDateTime` structs, describing a `Date` with a `Time`
//!  * [`epoch`] - Contains `Epoch`, `UnixEpoch`, `GPSEpoch`, and others, providing the datum anchor for timestamps
//!     `UnixTimestamp`, `GPSTimestamp`, etc.
//!  * [`gregorian`] - Contains `Date` and `Month`, that describe a gregorian calendar date.
//!  * [`julian`] - Contains `JulianDate` and it's associated epochs.
//!  * [`crate::format`] - Contains `Format` and `FormatParser` to tranlate dates to and from strings.
//!    * [`crate::format::iso8601`] - ISO8601 Implementations of `DateFormat` and `DateFormatParser`
//!
//! The top level module Contains the various representations of [`Time`]
//!
//! A [`Time`] is a specific time offset into a Day.  Intended for use where Hour:Minute:Seconds are
//! needed.
//!
//! The following are variants of [`epoch::Timestamp`], with specific methods and sizes to
//! to represent the Duration against an [`Epoch`].  These follow the same binary format as the NTP
//! Timestamp format, if used with the `NTP Epoch`.
//! * A [`Time32`] is a Q16.16 `Timestamp` where Seconds and Fractional Seconds are `u16`'s
//! * A [`Time64`] is a Q32.32 `Timestamp` where Seconds and Fractional Seconds are `u32`'s
//! * A [`Time128`] is a Q64.64 `Timestamp` where Seconds and Fractional Seconds are `u64`'s
//!
#![forbid(unsafe_code)]

use std::fmt::{Display, Formatter};

pub use irox_units::bounds::{GreaterThanEqualToValueError, LessThanValue, Range};
pub use irox_units::units::duration::{Duration, DurationUnit};
use irox_units::units::duration::{NANOS_TO_SEC, SEC_TO_NANOS};

use crate::epoch::Epoch;
use crate::format::{Format, FormatError, FormatParser};

pub mod datetime;
pub mod epoch;
pub mod format;
pub mod gregorian;
pub mod julian;

///
/// Represents a time of the day, an offset into the day from midnight.
///
/// Corresponds to a `UTC of day` in section 5.3.3 of ISO8601
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time {
    second_of_day: u32,
    nanoseconds: u32,
}

impl Time {
    ///
    /// Creates a Time from the specified seconds and nanoseconds,
    ///
    /// The valid range of 'second_of_day' is `0..86400`,
    /// The valid range of 'nanoseconds' is `0..1_000_000_000`
    pub fn new(
        second_of_day: u32,
        nanoseconds: u32,
    ) -> Result<Time, GreaterThanEqualToValueError<u32>> {
        LessThanValue::new(86400).check_value_is_valid(&second_of_day)?;
        LessThanValue::new(1_000_000_000).check_value_is_valid(&nanoseconds)?;
        Ok(Time {
            second_of_day,
            nanoseconds,
        })
    }

    ///
    /// Creates a Time from the specified fractional seconds, valid range `0..86400`
    pub fn from_seconds_f64(seconds: f64) -> Result<Time, GreaterThanEqualToValueError<f64>> {
        LessThanValue::new(86400_f64).check_value_is_valid(&seconds)?;

        let second_of_day = seconds as u32;
        let frac_nanos = seconds - second_of_day as f64;
        let nanoseconds = (frac_nanos * SEC_TO_NANOS) as u32;
        Ok(Time {
            second_of_day,
            nanoseconds,
        })
    }

    pub fn from_hms(
        hours: u8,
        minutes: u8,
        seconds: u8,
    ) -> Result<Time, GreaterThanEqualToValueError<u8>> {
        LessThanValue::new(24u8).check_value_is_valid(&hours)?;
        LessThanValue::new(60u8).check_value_is_valid(&minutes)?;
        LessThanValue::new(60u8).check_value_is_valid(&seconds)?;

        let second_of_day = hours as u32 * 3600 + minutes as u32 * 60 + seconds as u32;
        Ok(Time {
            second_of_day,
            nanoseconds: 0,
        })
    }

    pub fn from_hms_f64(
        hours: u8,
        minutes: u8,
        seconds: f64,
    ) -> Result<Time, GreaterThanEqualToValueError<f64>> {
        LessThanValue::new(24u8).check_value_is_valid(&hours)?;
        LessThanValue::new(60u8).check_value_is_valid(&minutes)?;
        LessThanValue::new(60f64).check_value_is_valid(&seconds)?;
        let nanoseconds = (seconds.fract() * NANOS_TO_SEC) as u32;
        let second_of_day = hours as u32 * 3600 + minutes as u32 * 60 + seconds as u32;
        Ok(Time {
            second_of_day,
            nanoseconds,
        })
    }

    pub fn from_hms_millis(
        hours: u8,
        minutes: u8,
        seconds: u8,
        millis: u32,
    ) -> Result<Time, GreaterThanEqualToValueError<u32>> {
        LessThanValue::new(1_000_000).check_value_is_valid(&millis)?;
        let time = Self::from_hms(hours, minutes, seconds)?;
        Ok(Time {
            second_of_day: time.second_of_day,
            nanoseconds: millis * 1000,
        })
    }

    ///
    /// Returns the number of seconds into the "current day"
    #[must_use]
    pub fn get_seconds(&self) -> u32 {
        self.second_of_day
    }

    ///
    /// Returns the number of fractional second nanoseconds
    #[must_use]
    pub fn get_nanoseconds(&self) -> u32 {
        self.nanoseconds
    }

    ///
    /// Converts this time into a duration
    #[must_use]
    pub fn as_duration(&self) -> Duration {
        let time = self.second_of_day as f64 + self.nanoseconds as f64 * NANOS_TO_SEC;
        Duration::new(time, DurationUnit::Second)
    }

    ///
    /// Returns the number of hours represented by this time.
    #[must_use]
    pub fn as_hours(&self) -> u32 {
        (self.second_of_day as f64 / SECONDS_IN_HOUR as f64) as u32
    }

    ///
    /// Returns the minute offset into the day represented by this time.
    #[must_use]
    pub fn as_minutes(&self) -> u32 {
        (self.second_of_day as f64 / SECONDS_IN_MINUTE as f64) as u32
    }

    ///
    /// Returns a triplet, (hours, minutes, seconds) representing this time
    #[must_use]
    pub fn as_hms(&self) -> (u32, u32, u32) {
        let hours = self.as_hours();
        let minutes = self.as_minutes() - hours * MINUTES_IN_HOUR;
        let seconds = self.get_seconds() - hours * SECONDS_IN_HOUR - minutes * SECONDS_IN_MINUTE;
        (hours, minutes, seconds)
    }

    ///
    /// Returns ONLY the fractional seconds component of the timestamp
    #[must_use]
    pub fn get_secondsfrac(&self) -> f64 {
        self.nanoseconds as f64 / NANOS_TO_SEC
    }

    ///
    /// Formats this Time using the specified formatter
    #[must_use]
    pub fn format<F: Format<Self>>(&self, format: &F) -> String {
        format.format(self)
    }

    ///
    /// Tries to parse a Time from the string using the specified Formatter
    pub fn parse_from<F: FormatParser<Self>>(
        format: &F,
        string: &str,
    ) -> Result<Self, FormatError> {
        format.try_from(string)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (h, m, s) = self.as_hms();
        f.write_fmt(format_args!("{h:0}:{m:02}:{s:02}"))
    }
}

impl From<Time> for Duration {
    fn from(value: Time) -> Self {
        value.as_duration()
    }
}

/// 24 Hours in a Day
pub const HOURS_IN_DAY: u32 = 24;

/// 60 Minutes in an Hour
pub const MINUTES_IN_HOUR: u32 = 60;

/// 60 Seconds in a Minute
pub const SECONDS_IN_MINUTE: u32 = 60;

/// 1440 Minutes in a Day
pub const MINUTES_IN_DAY: u32 = 1440;

/// 3600 Seconds in an Hour
pub const SECONDS_IN_HOUR: u32 = 3600;

///
/// Generally 86400, but occasionally 86401 for leap seconds.
pub const SECONDS_IN_DAY: u32 = 86400;

///
/// 32 Bit Fixed Precision Time Format, storing 16 bits of Seconds, and 16 bits
/// of Fractional Seconds.  This is the equivalent of Q16.16, and is semantically
/// equivalent to the NTP Short Format if using the [`epoch::NTP_EPOCH`].
///
/// The 16-bit seconds field can resolve a little over 18 hours, and the
/// 16-bit fractional seconds field can resolve a little over 15 microseconds.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Time32 {
    /// The Reference Epoch
    epoch: Epoch,

    /// The number of seconds into the reference epoch
    seconds: u16,

    /// The fractional number of seconds into the current second.  Divide this
    /// number by 2^16 to get the actual fractional component.
    fractional_seconds: u16,
}

impl Time32 {
    #[must_use]
    pub fn new(epoch: Epoch, seconds: u16, fractional_seconds: u16) -> Self {
        Self {
            epoch,
            seconds,
            fractional_seconds,
        }
    }

    ///
    /// Returns the value of this Time32 as a Q16.16
    #[must_use]
    pub fn as_u32(&self) -> u32 {
        ((self.seconds as u32) << 16) | (self.fractional_seconds as u32)
    }
}

///
/// 64 Bit Fixed Precision Time Format, storing 32 bits of Seconds, and 32 bits
/// of Fractional Seconds.  This is the equivalent of Q32.32, and is semantically
/// equivalent to the NTP Timestamp Format if using the [`epoch::NTP_EPOCH`].
///
/// The 32-bit seconds field can resolve 136 years, and the 32-bit fractional field
/// can resolve down to 232 picoseconds.
///
/// The raw value is 64 bits wide, if you take the middle 32
/// bits, this is identical to a [`Time32`] - (lower 16 of `seconds`, upper 16 of
/// `fractional_seconds`).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Time64 {
    /// The Reference Epoch
    epoch: Epoch,

    /// The number of seconds into the current reference epoch
    seconds: u32,

    /// The fractional element into the current second.  Divide this number by
    /// 2^32 to get the actual fractional component.
    fractional_seconds: u32,
}

impl Time64 {
    #[must_use]
    pub fn new(epoch: Epoch, seconds: u32, fractional_seconds: u32) -> Self {
        Self {
            epoch,
            seconds,
            fractional_seconds,
        }
    }

    ///
    /// Returns the value of this Time64 as a Q32.32
    #[must_use]
    pub fn as_u64(&self) -> u64 {
        ((self.seconds as u64) << 32) | (self.fractional_seconds as u64)
    }

    ///
    /// Returns the reference epoch of this Time64
    #[must_use]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }
}

///
/// 128 Bit Fixed Precision Time Format, storing 64 bits of Seconds, and 64 bits
/// of Fractional Seconds.  This is the equivalent of Q64.64, and is semantically
/// equivalent to the NTP Datestamp Format if using the [`epoch::NTP_EPOCH`].
///
/// The 64-bit seconds field can resolve 584 million years, and the 64-bit
/// fractional field can resolve down to 54 zepto-seconds (5.4e-20).
///
/// 580 million years ago, multicellular life started.  580 million years from,
/// now, the average temperature of the Earth will be 25C higher - 40C.
///
/// The raw value is 128 bits wide, if you take the middle 64 bits, this is
/// identical to a [`Time64`] - (lower 32 of `seconds`, upper 32 of
/// `fractional_seconds`).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time128 {
    ///
    /// Reference Epoch Date
    epoch: Epoch,

    /// The number of seconds into the reference epoch
    seconds: u64,

    /// The fractional element into the current second.  Divide this number by
    /// 2^64 to get the actual fractional component.
    fractional_seconds: u64,
}

impl Time128 {
    #[must_use]
    pub fn new(epoch: Epoch, seconds: u64, fractional_seconds: u64) -> Self {
        Self {
            epoch,
            seconds,
            fractional_seconds,
        }
    }

    ///
    /// Returns the value of this Time128 as a Q64.64
    #[must_use]
    pub fn as_u128(&self) -> u128 {
        ((self.seconds as u128) << 64) | (self.fractional_seconds as u128)
    }

    ///
    /// Returns the reference epoch of this Time128
    #[must_use]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }
}
