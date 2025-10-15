// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Contains [`Duration`] and [`DurationUnit`], a Physical Quantity of amount of Time passed.
//!

use crate::units::{FromUnits, Unit};
use core::fmt::{Display, Formatter, Write};
use core::num::ParseFloatError;
use core::str::Utf8Error;
use irox_tools::buf::{Buffer, FixedU8Buf};
use irox_tools::cfg_feature_serde;
use irox_tools::irox_bits::{BitsError, Error, ErrorKind, FormatBits, MutBits};

///
/// Represents a specific duration unit - SI or otherwise.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum DurationUnit {
    /// SI Base Unit for Duration - Second.
    ///
    /// The second division of the hour by 60.
    #[default]
    Second,

    /// A milli-second, one-thousandth of a second (1e-3)
    Millisecond,

    /// A micro-second, one-millionth of a second (1e-6)
    Microsecond,

    /// A nano-second, one-billionth of a second (1e-9)
    Nanosecond,

    /// a pico-second, one-trillionth of a second (1e-12)
    Picosecond,

    /// A minute, the first division of an hour by 60.
    Minute,

    /// An hour, 24 in a day.
    Hour,

    /// NIST 811 defines a "Day" as 86400 seconds, without the concept of leap seconds.
    Day,

    /// NIST 811 defines a "Year" as 365 days, or 31_536_000 seconds, without the concept of leap
    /// days.
    Year,
}

impl DurationUnit {
    ///
    /// Converts the specified value into Seconds
    pub fn as_seconds(self, value: f64) -> f64 {
        Duration::new(value, self)
            .as_unit(DurationUnit::Second)
            .value
    }
    ///
    /// Converts the specified value into Milliseconds
    pub fn as_millis(self, value: f64) -> f64 {
        Duration::new(value, self)
            .as_unit(DurationUnit::Millisecond)
            .value
    }

    ///
    /// Converts the specified value into Microseconds
    pub fn as_micros(self, value: f64) -> f64 {
        Duration::new(value, self)
            .as_unit(DurationUnit::Microsecond)
            .value
    }

    ///
    /// Converts the specified value into Nanoseconds
    pub fn as_nanos(self, value: f64) -> f64 {
        Duration::new(value, self)
            .as_unit(DurationUnit::Nanosecond)
            .value
    }

    ///
    /// Converts the specified value into Picoseconds
    pub fn as_picos(self, value: f64) -> f64 {
        Duration::new(value, self)
            .as_unit(DurationUnit::Picosecond)
            .value
    }

    ///
    /// Converts the specified value into Minutes
    pub fn as_minutes(self, value: f64) -> f64 {
        Duration::new(value, self)
            .as_unit(DurationUnit::Minute)
            .value
    }

    ///
    /// Converts the specified value into Hours
    pub fn as_hours(self, value: f64) -> f64 {
        Duration::new(value, self).as_unit(DurationUnit::Hour).value
    }

    pub fn abbreviation(&self) -> &'static str {
        match self {
            DurationUnit::Second => "s",
            DurationUnit::Millisecond => "ms",
            DurationUnit::Microsecond => "us",
            DurationUnit::Nanosecond => "ns",
            DurationUnit::Picosecond => "ps",
            DurationUnit::Minute => "min",
            DurationUnit::Hour => "hr",
            DurationUnit::Day => "dy",
            DurationUnit::Year => "yr",
        }
    }
}

macro_rules! from_units_duration {
    ($type:ident) => {
        impl crate::units::FromUnits<$type> for DurationUnit {
            fn from(&self, value: $type, units: Self) -> $type {
                match self {
                    // target
                    DurationUnit::Picosecond => match units {
                        // source
                        DurationUnit::Picosecond => value as $type,
                        DurationUnit::Nanosecond => value * NANOS_TO_PICOS as $type,
                        DurationUnit::Microsecond => value * MICROS_TO_PICOS as $type,
                        DurationUnit::Millisecond => value * MILLIS_TO_PICOS as $type,
                        DurationUnit::Second => value * SEC_TO_PICOS as $type,
                        DurationUnit::Minute => value * MIN_TO_PICOS as $type,
                        DurationUnit::Hour => value * HOUR_TO_PICOS as $type,
                        DurationUnit::Day => value * DAY_TO_PICOS as $type,
                        DurationUnit::Year => value * YEAR_TO_PICOS as $type,
                    },
                    DurationUnit::Nanosecond => match units {
                        // source
                        DurationUnit::Picosecond => value * PICOS_TO_NANOS as $type,
                        DurationUnit::Nanosecond => value as $type,
                        DurationUnit::Microsecond => value * MICROS_TO_NANOS as $type,
                        DurationUnit::Millisecond => value * MILLIS_TO_NANOS as $type,
                        DurationUnit::Second => value * SEC_TO_NANOS as $type,
                        DurationUnit::Minute => value * MIN_TO_NANOS as $type,
                        DurationUnit::Hour => value * HOUR_TO_NANOS as $type,
                        DurationUnit::Day => value * DAY_TO_NANOS as $type,
                        DurationUnit::Year => value * YEAR_TO_NANOS as $type,
                    },
                    DurationUnit::Microsecond => match units {
                        // source
                        DurationUnit::Picosecond => value * PICOS_TO_MICROS as $type,
                        DurationUnit::Nanosecond => value * NANOS_TO_MICROS as $type,
                        DurationUnit::Microsecond => value as $type,
                        DurationUnit::Millisecond => value * MILLIS_TO_MICROS as $type,
                        DurationUnit::Second => value * SEC_TO_MILLIS as $type,
                        DurationUnit::Minute => value * MIN_TO_MICROS as $type,
                        DurationUnit::Hour => value * HOUR_TO_MICROS as $type,
                        DurationUnit::Day => value * DAY_TO_MICROS as $type,
                        DurationUnit::Year => value * YEAR_TO_MICROS as $type,
                    },
                    DurationUnit::Millisecond => match units {
                        // source
                        DurationUnit::Picosecond => value * PICOS_TO_MILLIS as $type,
                        DurationUnit::Nanosecond => value * NANOS_TO_MILLIS as $type,
                        DurationUnit::Microsecond => value * MICROS_TO_MILLIS as $type,
                        DurationUnit::Millisecond => value as $type,
                        DurationUnit::Second => value * SEC_TO_MILLIS as $type,
                        DurationUnit::Minute => value * MIN_TO_MILLIS as $type,
                        DurationUnit::Hour => value * HOUR_TO_MILLIS as $type,
                        DurationUnit::Day => value * DAY_TO_MILLIS as $type,
                        DurationUnit::Year => value * YEAR_TO_MILLIS as $type,
                    },
                    DurationUnit::Second => match units {
                        // source
                        DurationUnit::Picosecond => value * PICOS_TO_SEC as $type,
                        DurationUnit::Nanosecond => value * NANOS_TO_SEC as $type,
                        DurationUnit::Microsecond => value * MICROS_TO_SECS as $type,
                        DurationUnit::Millisecond => value * MILLIS_TO_SEC as $type,
                        DurationUnit::Second => value as $type,
                        DurationUnit::Minute => value * MIN_TO_SEC as $type,
                        DurationUnit::Hour => value * HOUR_TO_SEC as $type,
                        DurationUnit::Day => value * DAY_TO_SEC as $type,
                        DurationUnit::Year => value * YEAR_TO_SEC as $type,
                    },
                    DurationUnit::Minute => match units {
                        // source
                        DurationUnit::Picosecond => value * PICOS_TO_MIN as $type,
                        DurationUnit::Nanosecond => value * NANOS_TO_MIN as $type,
                        DurationUnit::Microsecond => value * MICROS_TO_MIN as $type,
                        DurationUnit::Millisecond => value * MILLIS_TO_MIN as $type,
                        DurationUnit::Second => value * SEC_TO_MIN as $type,
                        DurationUnit::Minute => value as $type,
                        DurationUnit::Hour => value * HOUR_TO_MIN as $type,
                        DurationUnit::Day => value * DAY_TO_MIN as $type,
                        DurationUnit::Year => value * YEAR_TO_MIN as $type,
                    },
                    DurationUnit::Hour => match units {
                        // source
                        DurationUnit::Picosecond => value * PICOS_TO_HOUR as $type,
                        DurationUnit::Nanosecond => value * NANOS_TO_HOUR as $type,
                        DurationUnit::Microsecond => value * MICROS_TO_HOUR as $type,
                        DurationUnit::Millisecond => value * MILLIS_TO_HOUR as $type,
                        DurationUnit::Second => value * SEC_TO_HOUR as $type,
                        DurationUnit::Minute => value * MIN_TO_HOUR as $type,
                        DurationUnit::Hour => value as $type,
                        DurationUnit::Day => value * DAY_TO_HOUR as $type,
                        DurationUnit::Year => value * YEAR_TO_HOUR as $type,
                    },
                    DurationUnit::Day => match units {
                        // source
                        DurationUnit::Picosecond => value * PICOS_TO_DAY as $type,
                        DurationUnit::Nanosecond => value * NANOS_TO_DAY as $type,
                        DurationUnit::Microsecond => value * MICROS_TO_DAY as $type,
                        DurationUnit::Millisecond => value * MILLIS_TO_DAY as $type,
                        DurationUnit::Second => value * SEC_TO_DAY as $type,
                        DurationUnit::Minute => value * MIN_TO_DAY as $type,
                        DurationUnit::Hour => value * HOUR_TO_DAY as $type,
                        DurationUnit::Day => value as $type,
                        DurationUnit::Year => value * YEAR_TO_DAY as $type,
                    },
                    DurationUnit::Year => match units {
                        // source
                        DurationUnit::Picosecond => value * PICOS_TO_YEAR as $type,
                        DurationUnit::Nanosecond => value * NANOS_TO_YEAR as $type,
                        DurationUnit::Microsecond => value * MICROS_TO_YEAR as $type,
                        DurationUnit::Millisecond => value * MILLIS_TO_YEAR as $type,
                        DurationUnit::Second => value * SEC_TO_YEAR as $type,
                        DurationUnit::Minute => value * MIN_TO_YEAR as $type,
                        DurationUnit::Hour => value * HOUR_TO_YEAR as $type,
                        DurationUnit::Day => value * DAY_TO_YEAR as $type,
                        DurationUnit::Year => value as $type,
                    },
                }
            }
        }
    };
}

basic_unit!(Duration, DurationUnit, Second);
from_units_duration!(u32);
from_units_duration!(i32);
from_units_duration!(u64);
from_units_duration!(i64);
from_units_duration!(f32);
from_units_duration!(f64);

impl From<core::time::Duration> for Duration {
    fn from(value: core::time::Duration) -> Self {
        Duration::new(value.as_secs_f64(), DurationUnit::Second)
    }
}

impl From<Duration> for core::time::Duration {
    fn from(value: Duration) -> Self {
        let secs = value.as_seconds();
        let frac_sec = value.as_seconds_f64() - secs as f64;
        let nanos = DurationUnit::Second.as_nanos(frac_sec) as u32;
        core::time::Duration::new(secs, nanos)
    }
}

impl Duration {
    ///
    /// Creates a new duration using the specified number of seconds
    pub const fn new_seconds(value: f64) -> Duration {
        Duration {
            value,
            units: DurationUnit::Second,
        }
    }

    ///
    /// Returns this duration as (Years, Days, Hours, Minutes, Seconds)
    pub fn as_ydhms(&self) -> (u64, u16, u8, u8, u8) {
        let mut rem = *self;
        let years = rem.as_years();
        rem -= Duration::from_years(years);
        let (d, h, m, s) = rem.as_dhms();
        (years, d as u16, h, m, s)
    }

    ///
    /// Returns this duration as (Days, Hours, Minutes, Seconds)
    pub fn as_dhms(&self) -> (u64, u8, u8, u8) {
        let mut rem = *self;
        let days = rem.as_days();
        rem -= Duration::from_days(days);
        let (h, m, s) = rem.as_hms();
        (days, h as u8, m, s)
    }

    ///
    /// Returns this duration as (Hours, Minutes, Seconds)
    pub fn as_hms(&self) -> (u64, u8, u8) {
        let mut rem = *self;
        let hours = rem.as_hours();
        rem -= Duration::from_hours(hours);
        let minutes = rem.as_minutes();
        rem -= Duration::from_minutes(minutes);
        let seconds = rem.as_seconds();
        (hours, minutes as u8, seconds as u8)
    }

    /// Returns the value of this duration as whole seconds, with any fractional
    /// element truncated off.
    pub fn as_seconds(&self) -> u64 {
        self.as_unit(DurationUnit::Second).value() as u64
    }

    /// Returns the value of this duration in fractional seconds
    pub fn as_seconds_f64(&self) -> f64 {
        self.as_unit(DurationUnit::Second).value()
    }

    /// Returns the value of this duration in fractional seconds
    pub fn as_seconds_f32(&self) -> f32 {
        self.as_unit(DurationUnit::Second).value() as f32
    }

    /// Returns the value of this duration as whole milliseconds, with any
    /// fractional element truncated off.
    pub fn as_millis(&self) -> u64 {
        self.as_unit(DurationUnit::Millisecond).value() as u64
    }
    /// Returns the value of this duration in fractional milliseconds
    pub fn as_millis_f64(&self) -> f64 {
        self.as_unit(DurationUnit::Millisecond).value()
    }
    /// Returns the value of this duration in fractional milliseconds
    pub fn as_millis_f32(&self) -> f32 {
        self.as_unit(DurationUnit::Millisecond).value() as f32
    }

    /// Returns the value of this duration as whole microseconds, with any
    /// fractional element truncated off.
    pub fn as_micros(&self) -> u64 {
        self.as_unit(DurationUnit::Microsecond).value() as u64
    }
    /// Returns the value of this duration in fractional microseconds
    pub fn as_micros_f64(&self) -> f64 {
        self.as_unit(DurationUnit::Microsecond).value()
    }
    /// Returns the value of this duration in fractional microseconds
    pub fn as_micros_f32(&self) -> f32 {
        self.as_unit(DurationUnit::Microsecond).value() as f32
    }

    /// Returns the value of this duration as whole microseconds, with any
    /// fractional element truncated off.
    pub fn as_nanos(&self) -> u64 {
        self.as_unit(DurationUnit::Nanosecond).value() as u64
    }
    /// Returns the value of this duration in fractional nanoseconds
    pub fn as_nanos_f64(&self) -> f64 {
        self.as_unit(DurationUnit::Nanosecond).value()
    }
    /// Returns the value of this duration in fractional nanoseconds
    pub fn as_nanos_f32(&self) -> f32 {
        self.as_unit(DurationUnit::Nanosecond).value() as f32
    }

    /// Returns the value of this duration as whole microseconds, with any
    /// fractional element truncated off.
    pub fn as_picos(&self) -> u64 {
        self.as_unit(DurationUnit::Picosecond).value() as u64
    }
    /// Returns the value of this duration in fractional picoseconds
    pub fn as_picos_f64(&self) -> f64 {
        self.as_unit(DurationUnit::Picosecond).value()
    }
    /// Returns the value of this duration in fractional picoseconds
    pub fn as_picos_f32(&self) -> f32 {
        self.as_unit(DurationUnit::Picosecond).value() as f32
    }

    /// Returns the value of this duration as whole minutes, with any fractional
    /// element truncated off
    pub fn as_minutes(&self) -> u64 {
        self.as_unit(DurationUnit::Minute).value() as u64
    }

    /// Returns the value of this duration as whole hours, with any fractional
    /// element truncated off
    pub fn as_hours(&self) -> u64 {
        self.as_unit(DurationUnit::Hour).value() as u64
    }

    /// Returns the value of this duration as whole days, with any fractional
    /// element truncated off
    pub fn as_days(&self) -> u64 {
        self.as_unit(DurationUnit::Day).value() as u64
    }

    /// Returns the value of this duration as whole years, with any fractional
    /// element truncated off
    pub fn as_years(&self) -> u64 {
        self.as_unit(DurationUnit::Year).value() as u64
    }
}

// Backwards compatibility for [`core::time::Duration`] drop-in creation
impl Duration {
    /// Creates a new `Duration` from the specified number of microseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use irox_units::units::duration::Duration;
    ///
    /// let duration = Duration::from_micros(1_000_002);
    ///
    /// assert_eq!(1, duration.as_seconds());
    /// ```
    pub const fn from_micros(micros: u64) -> Duration {
        Duration::new(micros as f64, DurationUnit::Microsecond)
    }

    /// Creates a new `Duration` from the specified number of milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use irox_units::units::duration::Duration;
    ///
    /// let duration = Duration::from_millis(2569);
    ///
    /// assert_eq!(2, duration.as_seconds());
    /// ```
    pub const fn from_millis(millis: u64) -> Duration {
        Duration::new(millis as f64, DurationUnit::Millisecond)
    }

    /// Creates a new `Duration` from the specified number of nanoseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use irox_units::units::duration::Duration;
    ///
    /// let duration = Duration::from_nanos(1_000_000_123);
    ///
    /// assert_eq!(1, duration.as_seconds());
    /// ```
    pub const fn from_nanos(nanos: u64) -> Duration {
        Duration::new(nanos as f64, DurationUnit::Nanosecond)
    }

    /// Creates a new `Duration` from the specified number of picoseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use irox_units::units::duration::Duration;
    ///
    /// let duration = Duration::from_picos(1_000_000_000_123);
    ///
    /// assert_eq!(1, duration.as_seconds());
    /// ```
    pub const fn from_picos(picos: u64) -> Duration {
        Duration::new(picos as f64, DurationUnit::Picosecond)
    }

    /// Creates a new `Duration` from the specified number of minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use irox_units::units::duration::Duration;
    ///
    /// let duration = Duration::from_minutes(1);
    ///
    /// assert_eq!(60, duration.as_seconds());
    /// ```
    pub const fn from_minutes(minutes: u64) -> Duration {
        Duration::new(minutes as f64, DurationUnit::Minute)
    }

    /// Creates a new `Duration` from the specified number of hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use irox_units::units::duration::Duration;
    ///
    /// let duration = Duration::from_hours(1);
    ///
    /// assert_eq!(3600, duration.as_seconds());
    /// ```
    pub const fn from_hours(hours: u64) -> Duration {
        Duration::new(hours as f64, DurationUnit::Hour)
    }

    /// Creates a new `Duration` from the specified number of NIST 811 Days where 1 Day = 86400 Seconds
    ///
    /// # Examples
    ///
    /// ```
    /// use irox_units::units::duration::Duration;
    ///
    /// let duration = Duration::from_days(1);
    ///
    /// assert_eq!(86400, duration.as_seconds());
    /// ```
    pub const fn from_days(days: u64) -> Duration {
        Duration::new(days as f64, DurationUnit::Day)
    }

    /// Creates a new `Duration` from the specified number of NIST 811 Years where 1 Year = 365 Days.
    ///
    /// # Examples
    ///
    /// ```
    /// use irox_units::units::duration::Duration;
    ///
    /// let duration = Duration::from_years(1);
    ///
    /// assert_eq!(31_536_000, duration.as_seconds());
    /// ```
    pub const fn from_years(years: u64) -> Duration {
        Duration::new(years as f64, DurationUnit::Year)
    }

    /// Creates a new `Duration` from the specified number of seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use irox_units::units::duration::Duration;
    ///
    /// let duration = Duration::from_seconds(100);
    ///
    /// assert_eq!(100, duration.as_seconds());
    /// ```
    pub const fn from_seconds(seconds: u64) -> Duration {
        Duration::new(seconds as f64, DurationUnit::Second)
    }

    /// Creates a new `Duration` from the specified number of f64 seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use irox_units::units::duration::Duration;
    ///
    /// let duration = Duration::from_seconds_f64(25.5);
    ///
    /// assert_eq!(25.5, duration.as_seconds_f64());
    /// ```
    pub const fn from_seconds_f64(seconds: f64) -> Duration {
        Duration::new(seconds, DurationUnit::Second)
    }

    pub fn from_hms(hours: u64, minutes: u64, seconds: u64) -> Duration {
        Duration::from_hours(hours)
            + Duration::from_minutes(minutes)
            + Duration::from_seconds(seconds)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.value, self.units.abbreviation()))
    }
}

// going up
pub const PICOS_TO_NANOS: f64 = 1e-3;
pub const NANOS_TO_MICROS: f64 = 1e-3;
pub const MICROS_TO_MILLIS: f64 = 1e-3;
pub const MILLIS_TO_SEC: f64 = 1e-3;
pub const SEC_TO_MIN: f64 = 1. / MIN_TO_SEC;
pub const MIN_TO_HOUR: f64 = 1. / HOUR_TO_MIN;
pub const HOUR_TO_DAY: f64 = 1. / DAY_TO_HOUR;
pub const DAY_TO_YEAR: f64 = 1. / YEAR_TO_DAY;

// going down
pub const YEAR_TO_DAY: f64 = 365_f64;
pub const DAY_TO_HOUR: f64 = 24_f64;
pub const HOUR_TO_MIN: f64 = 60_f64;
pub const MIN_TO_SEC: f64 = 60_f64;
pub const SEC_TO_MILLIS: f64 = 1e3;
pub const MILLIS_TO_MICROS: f64 = 1e3;
pub const MICROS_TO_NANOS: f64 = 1e3;
pub const NANOS_TO_PICOS: f64 = 1e3;

// going down double jumps
pub const YEAR_TO_HOUR: f64 = YEAR_TO_DAY * DAY_TO_HOUR;
pub const DAY_TO_MIN: f64 = DAY_TO_HOUR * HOUR_TO_MIN;
pub const HOUR_TO_SEC: f64 = HOUR_TO_MIN * MIN_TO_SEC;
pub const MIN_TO_MILLIS: f64 = MIN_TO_SEC * SEC_TO_MILLIS;
pub const SEC_TO_MICROS: f64 = SEC_TO_MILLIS * MILLIS_TO_MICROS;
pub const MILLIS_TO_NANOS: f64 = MILLIS_TO_MICROS * MICROS_TO_NANOS;
pub const MICROS_TO_PICOS: f64 = MICROS_TO_NANOS * NANOS_TO_PICOS;

// going up double jumps
pub const PICOS_TO_MICROS: f64 = PICOS_TO_NANOS * NANOS_TO_MICROS;
pub const NANOS_TO_MILLIS: f64 = NANOS_TO_MICROS * MICROS_TO_MILLIS;
pub const MICROS_TO_SECS: f64 = MICROS_TO_MILLIS * MILLIS_TO_SEC;
pub const MILLIS_TO_MIN: f64 = MILLIS_TO_SEC * SEC_TO_MIN;
pub const SEC_TO_HOUR: f64 = SEC_TO_MIN * MIN_TO_HOUR;
pub const MIN_TO_DAY: f64 = MIN_TO_HOUR * HOUR_TO_DAY;
pub const HOUR_TO_YEAR: f64 = HOUR_TO_DAY * DAY_TO_YEAR;

// going down triples
pub const YEAR_TO_MIN: f64 = YEAR_TO_HOUR * HOUR_TO_MIN;
pub const DAY_TO_SEC: f64 = DAY_TO_MIN * MIN_TO_SEC;
pub const HOUR_TO_MILLIS: f64 = HOUR_TO_SEC * SEC_TO_MILLIS;
pub const MIN_TO_MICROS: f64 = MIN_TO_MILLIS * MILLIS_TO_MICROS;
pub const SEC_TO_NANOS: f64 = SEC_TO_MICROS * MICROS_TO_NANOS;
pub const MILLIS_TO_PICOS: f64 = MILLIS_TO_NANOS * NANOS_TO_PICOS;

// going up triples
pub const PICOS_TO_MILLIS: f64 = PICOS_TO_MICROS * MICROS_TO_MILLIS;
pub const NANOS_TO_SEC: f64 = NANOS_TO_MILLIS * MILLIS_TO_SEC;
pub const MICROS_TO_MIN: f64 = MICROS_TO_SECS * SEC_TO_MIN;
pub const MILLIS_TO_HOUR: f64 = MILLIS_TO_MIN * MIN_TO_HOUR;
pub const SEC_TO_DAY: f64 = SEC_TO_HOUR * HOUR_TO_DAY;
pub const MIN_TO_YEAR: f64 = MIN_TO_DAY * DAY_TO_YEAR;

// going down quads
pub const YEAR_TO_SEC: f64 = YEAR_TO_MIN * MIN_TO_SEC;
pub const DAY_TO_MILLIS: f64 = DAY_TO_SEC * SEC_TO_MILLIS;
pub const HOUR_TO_MICROS: f64 = HOUR_TO_MILLIS * MILLIS_TO_MICROS;
pub const MIN_TO_NANOS: f64 = MIN_TO_MICROS * MICROS_TO_NANOS;
pub const SEC_TO_PICOS: f64 = SEC_TO_NANOS * NANOS_TO_PICOS;

// going up quads
pub const PICOS_TO_SEC: f64 = PICOS_TO_MILLIS * MILLIS_TO_SEC;
pub const NANOS_TO_MIN: f64 = NANOS_TO_SEC * SEC_TO_MIN;
pub const MICROS_TO_HOUR: f64 = MICROS_TO_MIN * MIN_TO_HOUR;
pub const MILLIS_TO_DAY: f64 = MILLIS_TO_HOUR * HOUR_TO_DAY;
pub const SEC_TO_YEAR: f64 = SEC_TO_DAY * DAY_TO_YEAR;

// going down pentas
pub const YEAR_TO_MILLIS: f64 = YEAR_TO_SEC * SEC_TO_MILLIS;
pub const DAY_TO_MICROS: f64 = DAY_TO_MILLIS * MILLIS_TO_MICROS;
pub const HOUR_TO_NANOS: f64 = HOUR_TO_MICROS * MICROS_TO_NANOS;
pub const MIN_TO_PICOS: f64 = MIN_TO_NANOS * NANOS_TO_PICOS;

// going up pentas
pub const PICOS_TO_MIN: f64 = PICOS_TO_SEC * SEC_TO_MIN;
pub const NANOS_TO_HOUR: f64 = NANOS_TO_MIN * MIN_TO_HOUR;
pub const MICROS_TO_DAY: f64 = MICROS_TO_HOUR * HOUR_TO_DAY;
pub const MILLIS_TO_YEAR: f64 = MILLIS_TO_DAY * DAY_TO_YEAR;

// going down hexas
pub const YEAR_TO_MICROS: f64 = YEAR_TO_MILLIS * MILLIS_TO_MICROS;
pub const DAY_TO_NANOS: f64 = DAY_TO_MICROS * MICROS_TO_NANOS;
pub const HOUR_TO_PICOS: f64 = HOUR_TO_NANOS * NANOS_TO_PICOS;

// going up hexas
pub const PICOS_TO_HOUR: f64 = PICOS_TO_MIN * MIN_TO_HOUR;
pub const NANOS_TO_DAY: f64 = NANOS_TO_HOUR * HOUR_TO_DAY;
pub const MICROS_TO_YEAR: f64 = MICROS_TO_DAY * DAY_TO_YEAR;

// going down septs
pub const YEAR_TO_NANOS: f64 = YEAR_TO_MICROS * MICROS_TO_NANOS;
pub const DAY_TO_PICOS: f64 = DAY_TO_NANOS * NANOS_TO_PICOS;

// going up septs
pub const PICOS_TO_DAY: f64 = PICOS_TO_HOUR * HOUR_TO_DAY;
pub const NANOS_TO_YEAR: f64 = NANOS_TO_DAY * DAY_TO_YEAR;

// going down octs
pub const YEAR_TO_PICOS: f64 = YEAR_TO_NANOS * NANOS_TO_PICOS;

// going up octs
pub const PICOS_TO_YEAR: f64 = PICOS_TO_DAY * DAY_TO_YEAR;

impl Duration {
    pub fn write_iso8601_exact_to<T: MutBits>(&self, fmt: &mut T) -> Result<(), core::fmt::Error> {
        let mut fmt = FormatBits(fmt);
        fmt.write_str("P")?;
        let y = self.as_years();
        if y > 0 {
            write!(fmt, "{y}Y")?;
        }
        let mut r = self - Duration::from_years(y);
        let (d, h, m, _s) = r.as_dhms();
        if d > 0 {
            write!(fmt, "{d}D")?;
            r -= Duration::from_days(d);
        }
        if r.as_seconds_f64() > 0.0 {
            fmt.write_str("T")?;
        }
        if h > 0 {
            write!(fmt, "{h}H")?;
            r -= Duration::from_hours(h as u64);
        }
        if m > 0 {
            write!(fmt, "{m}M")?;
            r -= Duration::from_minutes(m as u64);
        }
        let s = r.as_seconds_f64();
        if s.abs() > 0.0 {
            write!(fmt, "{s:02.}S")?;
        }

        Ok(())
    }
    pub fn write_iso8601_sec_to<T: MutBits>(&self, fmt: &mut T) -> Result<(), core::fmt::Error> {
        let mut fmt = FormatBits(fmt);
        fmt.write_str("P")?;
        let y = self.as_years();
        if y > 0 {
            write!(fmt, "{y}Y")?;
        }
        let mut r = self - Duration::from_years(y);
        let (d, h, m, s) = r.as_dhms();
        if d > 0 {
            write!(fmt, "{d}D")?;
            r -= Duration::from_days(d);
        }
        if r.as_seconds_f64() > 0.0 {
            fmt.write_str("T")?;
        }
        if h > 0 {
            write!(fmt, "{h}H")?;
            r -= Duration::from_hours(h as u64);
        }
        if m > 0 {
            write!(fmt, "{m}M")?;
            r -= Duration::from_minutes(m as u64);
        }
        if s > 0 {
            write!(fmt, "{s}S")?;
        }

        Ok(())
    }

    ///
    /// Parse a ISO-8601 Duration String like `P1Y2M3DT4H5M2.5S`
    ///
    /// This function assumes 1 month = 30 days.
    pub fn parse_iso8601_from(val: &str) -> Result<Self, ParseError> {
        let b = val.as_bytes();
        let mut seconds = 0.0f64;
        let mut minutes = 0.0f64;
        let mut hours = 0.0f64;
        let mut days = 0.0f64;
        let mut months = 0.0f64;
        let mut years = 0.0f64;

        let mut encountered_t = false;
        let mut buf: FixedU8Buf<25> = Default::default();
        for v in b {
            match v {
                b'S' | b's' => {
                    seconds += buf.as_f64()?;
                    buf.clear();
                }
                b'M' | b'm' => {
                    if encountered_t {
                        minutes += buf.as_f64()?;
                    } else {
                        months += buf.as_f64()?;
                    }
                    buf.clear();
                }
                b'H' | b'h' => {
                    hours += buf.as_f64()?;
                    buf.clear();
                }
                b'D' | b'd' => {
                    days += buf.as_f64()?;
                    buf.clear();
                }
                b'Y' | b'y' => {
                    years += buf.as_f64()?;
                    buf.clear();
                }
                b'0'..=b'9' | b'.' => {
                    if buf.push_back(*v).is_err() {
                        return Err(Error::new(
                            ErrorKind::OutOfMemory,
                            "Buffer overflowed parsing number in Duration::parse_iso8601_from",
                        )
                        .into());
                    }
                }
                b'T' | b't' => {
                    encountered_t = true;
                }
                _ => {}
            }
        }
        Ok(Duration::new(seconds, DurationUnit::Second)
            + Duration::new(minutes, DurationUnit::Minute)
            + Duration::new(hours, DurationUnit::Hour)
            + Duration::new(days, DurationUnit::Day)
            + Duration::new(months * 30., DurationUnit::Day)
            + Duration::new(years, DurationUnit::Year))
    }
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ParseError {
    UTF8(Utf8Error),
    Bits(BitsError),
    Float(ParseFloatError),
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseError::UTF8(u) => {
                write!(f, "UTF8Error: {u}")
            }
            ParseError::Bits(b) => {
                write!(f, "BitsError: {b}")
            }
            ParseError::Float(e) => {
                write!(f, "FloatError: {e}")
            }
        }
    }
}
macro_rules! implerrorfrom {
    ($id:ident, $en:ident) => {
        impl From<$id> for ParseError {
            fn from(value: $id) -> Self {
                Self::$en(value)
            }
        }
    };
}
implerrorfrom!(Utf8Error, UTF8);
implerrorfrom!(BitsError, Bits);
implerrorfrom!(ParseFloatError, Float);

cfg_feature_serde! {
    struct DurationVisitor;
    impl serde::de::Visitor<'_> for DurationVisitor {
        type Value = Duration;

        fn expecting(&self, fmt: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
            write!(fmt, "The visitor expects to receive a string formatted as a ISO 8601 Duration")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
            Duration::parse_iso8601_from(v).map_err(serde::de::Error::custom)
        }
    }
    impl<'de> serde::Deserialize<'de> for Duration {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
            deserializer.deserialize_str(DurationVisitor)
        }
    }
    impl serde::Serialize for Duration {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            let mut buf : FixedU8Buf<128> = Default::default();
            self.write_iso8601_exact_to(&mut buf).map_err(serde::ser::Error::custom)?;
            let s = buf.as_str().map_err(serde::ser::Error::custom)?;
            serializer.serialize_str(s)
        }
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use core::fmt::Error;
    extern crate alloc;
    use crate::units::duration::Duration;

    #[test]
    pub fn test_iso8601() -> Result<(), Error> {
        let mut s = alloc::string::String::new();
        let d = Duration::from_years(1);
        d.write_iso8601_exact_to(&mut s)?;
        assert_eq!("P1Y", s);

        assert_eq!(Ok(d), Duration::parse_iso8601_from(&s));

        let d = Duration::from_hms(1, 2, 3);
        s.clear();
        d.write_iso8601_sec_to(&mut s)?;
        assert_eq!("PT1H2M3S", s);
        assert_eq!(Ok(d), Duration::parse_iso8601_from(&s));

        let d = Duration::from_seconds_f64(123456.789);
        s.clear();
        d.write_iso8601_exact_to(&mut s)?;
        assert_eq!("P1DT10H17M36.78900000000431S", s);
        assert_eq!(Ok(d), Duration::parse_iso8601_from(&s));

        Ok(())
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "std"))]
    pub fn serde_test() -> Result<(), Error> {
        #[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Debug)]
        struct Test {
            a: Duration,
        }
        impl Default for Test {
            fn default() -> Self {
                Self {
                    a: Duration::default(),
                }
            }
        }
        let a = Test {
            a: Duration::from_days(1),
        };
        let s = serde_json::to_string(&a).unwrap_or_default();
        assert_eq!(s, "{\"a\":\"P1D\"}");
        let b: Test = serde_json::from_str(&s).unwrap();
        assert_eq!(a, b);
        Ok(())
    }
}
