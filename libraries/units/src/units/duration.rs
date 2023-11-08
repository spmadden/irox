// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Contains [`Duration`] and [`DurationUnit`], a Physical Quantity of amount of Time passed.
//!

use std::fmt::{Display, Formatter};

use crate::basic_unit;
use crate::units::{FromUnits, Unit};

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
}

macro_rules! from_units_duration {
    ($type:ident) => {
        impl crate::units::FromUnits<$type> for DurationUnit {
            fn from(&self, value: $type, units: Self) -> $type {
                match self {
                    // target
                    DurationUnit::Nanosecond => match units {
                        // source
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

impl Unit<DurationUnit> for Duration {
    fn as_unit(&self, units: DurationUnit) -> Self {
        Duration {
            value: units.from(self.value, self.units),
            units,
        }
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Duration::new(value.as_secs_f64(), DurationUnit::Second)
    }
}

impl From<Duration> for std::time::Duration {
    fn from(value: Duration) -> Self {
        let secs = value.as_seconds();
        let frac_sec = value.as_seconds_f64() - secs as f64;
        let nanos = DurationUnit::Second.as_nanos(frac_sec) as u32;
        std::time::Duration::new(secs, nanos)
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

    /// Returns the value of this duration as whole microseconds, with any
    /// fractional element truncated off.
    pub fn as_micros(&self) -> u64 {
        self.as_unit(DurationUnit::Microsecond).value() as u64
    }

    /// Returns the value of this duration as whole microseconds, with any
    /// fractional element truncated off.
    pub fn as_nanos(&self) -> u64 {
        self.as_unit(DurationUnit::Nanosecond).value() as u64
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

// Backwards compatibility for [`std::time::Duration`] drop-in creation
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

    pub const fn from_minutes(minutes: u64) -> Duration {
        Duration::new(minutes as f64, DurationUnit::Minute)
    }

    pub const fn from_hours(hours: u64) -> Duration {
        Duration::new(hours as f64, DurationUnit::Hour)
    }

    pub const fn from_days(days: u64) -> Duration {
        Duration::new(days as f64, DurationUnit::Day)
    }

    pub const fn from_years(years: u64) -> Duration {
        Duration::new(years as f64, DurationUnit::Year)
    }

    pub const fn from_seconds(seconds: u64) -> Duration {
        Duration::new(seconds as f64, DurationUnit::Second)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {:?}", self.value, self.units))
    }
}

// going up
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

// going down double jumps
pub const YEAR_TO_HOUR: f64 = YEAR_TO_DAY * DAY_TO_HOUR;
pub const DAY_TO_MIN: f64 = DAY_TO_HOUR * HOUR_TO_MIN;
pub const HOUR_TO_SEC: f64 = HOUR_TO_MIN * MIN_TO_SEC;
pub const MIN_TO_MILLIS: f64 = MIN_TO_SEC * SEC_TO_MILLIS;
pub const SEC_TO_MICROS: f64 = SEC_TO_MILLIS * MILLIS_TO_MICROS;
pub const MILLIS_TO_NANOS: f64 = MILLIS_TO_MICROS * MICROS_TO_NANOS;

// going up double jumps
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

// going up triples
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

// going up quads
pub const NANOS_TO_MIN: f64 = NANOS_TO_SEC * SEC_TO_MIN;
pub const MICROS_TO_HOUR: f64 = MICROS_TO_MIN * MIN_TO_HOUR;
pub const MILLIS_TO_DAY: f64 = MILLIS_TO_HOUR * HOUR_TO_DAY;
pub const SEC_TO_YEAR: f64 = SEC_TO_DAY * DAY_TO_YEAR;

// going down pentas
pub const YEAR_TO_MILLIS: f64 = YEAR_TO_SEC * SEC_TO_MILLIS;
pub const DAY_TO_MICROS: f64 = DAY_TO_MILLIS * MILLIS_TO_MICROS;
pub const HOUR_TO_NANOS: f64 = HOUR_TO_MICROS * MICROS_TO_NANOS;

// going up pentas
pub const NANOS_TO_HOUR: f64 = NANOS_TO_MIN * MIN_TO_HOUR;
pub const MICROS_TO_DAY: f64 = MICROS_TO_HOUR * HOUR_TO_DAY;
pub const MILLIS_TO_YEAR: f64 = MILLIS_TO_DAY * DAY_TO_YEAR;

// going down hexas
pub const YEAR_TO_MICROS: f64 = YEAR_TO_MILLIS * MILLIS_TO_MICROS;
pub const DAY_TO_NANOS: f64 = DAY_TO_MICROS * MICROS_TO_NANOS;

// going up hexas
pub const NANOS_TO_DAY: f64 = NANOS_TO_HOUR * HOUR_TO_DAY;
pub const MICROS_TO_YEAR: f64 = MICROS_TO_DAY * DAY_TO_YEAR;

// going down septs
pub const YEAR_TO_NANOS: f64 = YEAR_TO_MICROS * MICROS_TO_NANOS;

// going up septs
pub const NANOS_TO_YEAR: f64 = NANOS_TO_DAY * DAY_TO_YEAR;
