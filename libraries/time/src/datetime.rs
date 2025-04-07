// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Contains [`UTCDateTime`] and associated elements to represent a [`Date`] and [`Time`] in UTC
//!

extern crate alloc;
use crate::epoch::{UnixTimestamp, UNIX_EPOCH};
use crate::format::iso8601::{ISO8601Format, BASIC_DATE_TIME_OF_DAY, ISO8601_DATE_TIME};
use crate::format::{Format, FormatError, FormatParser};
use crate::gregorian::Date;
use crate::julian::JulianDate;
use crate::Time;
pub use alloc::string::String;
use core::fmt::{Display, Formatter};
use core::ops::{Add, AddAssign, Sub};
use irox_units::bounds::GreaterThanEqualToValueError;
use irox_units::units::duration::Duration;

///
/// Represents a Gregorian Date and Time in UTC
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UTCDateTime {
    pub(crate) date: Date,
    pub(crate) time: Time,
}

impl Display for UTCDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.format(&BASIC_DATE_TIME_OF_DAY)))
    }
}

impl UTCDateTime {
    ///
    /// New UTC Date and Time
    #[must_use]
    pub fn new(date: Date, time: Time) -> UTCDateTime {
        UTCDateTime { date, time }
    }

    ///
    /// New UTC Date and Time from the specified values
    pub fn try_from_values(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        seconds: u8,
    ) -> Result<UTCDateTime, GreaterThanEqualToValueError<u8>> {
        let date = Date::try_from_values(year, month, day)?;
        let time = Time::from_hms(hour, minute, seconds)?;
        Ok(UTCDateTime::new(date, time))
    }

    ///
    /// New UTC date and Time from the specified values (fractional seconds)
    pub fn try_from_values_f64(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        seconds: f64,
    ) -> Result<UTCDateTime, GreaterThanEqualToValueError<f64>> {
        let date = Date::try_from_values(year, month, day)?;
        let time = Time::from_hms_f64(hour, minute, seconds)?;
        Ok(UTCDateTime::new(date, time))
    }

    ///
    /// Returns the Gregorian Date portion of this UTCDateTime
    #[must_use]
    pub fn get_date(&self) -> Date {
        self.date
    }

    ///
    /// Returns the Time portion of this UTCDateTime
    #[must_use]
    pub fn get_time(&self) -> Time {
        self.time
    }

    ///
    /// Returns the current instant in time as reported by the local system
    /// clock.
    #[must_use]
    #[cfg(feature = "std")]
    pub fn now() -> UTCDateTime {
        UnixTimestamp::now().into()
    }

    #[must_use]
    pub fn format<T: Format<UTCDateTime>>(&self, format: &T) -> String {
        format.format(self)
    }

    /// Formats this date as a extended ISO8601 Date & Time, `2023-12-31T05:10:25Z`
    #[must_use]
    pub fn format_iso8601_extended(&self) -> String {
        ISO8601_DATE_TIME.format(self)
    }
    /// Formats this date as a basic ISO8601 Date & Time, `20231231T051025Z`, suitable for filenames
    #[must_use]
    pub fn format_iso8601_basic(&self) -> String {
        BASIC_DATE_TIME_OF_DAY.format(self)
    }
    /// Attempts to parse the provided string as either a [`crate::format::iso8601::BasicDateTimeOfDay`] or a [`crate::format::iso8601::ExtendedDateTimeFormat`]
    pub fn try_from_iso8601(val: &str) -> Result<Self, FormatError> {
        ISO8601_DATE_TIME.try_from(val)
    }
}

impl ISO8601Format for UTCDateTime {
    fn format_iso8601_extended(&self) -> String {
        UTCDateTime::format_iso8601_extended(self)
    }

    fn format_iso8601_basic(&self) -> String {
        UTCDateTime::format_iso8601_basic(self)
    }

    fn try_from_iso8601(val: &str) -> Result<Self, FormatError>
    where
        Self: Sized,
    {
        UTCDateTime::try_from_iso8601(val)
    }
}

impl From<&UnixTimestamp> for UTCDateTime {
    fn from(value: &UnixTimestamp) -> Self {
        let date = value.as_date();
        let remaining_seconds = value.get_offset().as_seconds_f64()
            - date.as_unix_timestamp().get_offset().as_seconds_f64();

        let time = Time::from_seconds_f64(remaining_seconds).unwrap_or_default();

        UTCDateTime { date, time }
    }
}
impl From<UnixTimestamp> for UTCDateTime {
    fn from(value: UnixTimestamp) -> Self {
        let date = value.as_date();
        let remaining_seconds = value.get_offset().as_seconds_f64()
            - date.as_unix_timestamp().get_offset().as_seconds_f64();

        let time = Time::from_seconds_f64(remaining_seconds).unwrap_or_default();

        UTCDateTime { date, time }
    }
}

impl From<UTCDateTime> for UnixTimestamp {
    fn from(value: UTCDateTime) -> Self {
        let mut date_dur = value.date - UNIX_EPOCH.get_gregorian_date();
        date_dur += Into::<Duration>::into(value.time);
        Self::from_offset(date_dur)
    }
}
impl From<&UTCDateTime> for UnixTimestamp {
    fn from(value: &UTCDateTime) -> Self {
        let mut date_dur = value.date - UNIX_EPOCH.get_gregorian_date();
        date_dur += Into::<Duration>::into(value.time);
        Self::from_offset(date_dur)
    }
}

impl From<UTCDateTime> for JulianDate {
    fn from(value: UTCDateTime) -> Self {
        let mut date: JulianDate = value.date.into();
        let time: Duration = value.time.into();
        date += time;
        date
    }
}

impl From<&UTCDateTime> for JulianDate {
    fn from(value: &UTCDateTime) -> Self {
        let mut date: JulianDate = value.date.into();
        let time: Duration = value.time.into();
        date += time;
        date
    }
}

impl Sub<Self> for UTCDateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        let ts1: JulianDate = self.into();
        let ts2: JulianDate = rhs.into();

        ts1 - ts2
    }
}
impl Sub<&Self> for UTCDateTime {
    type Output = Duration;

    fn sub(self, rhs: &Self) -> Self::Output {
        let ts1: JulianDate = self.into();
        let ts2: JulianDate = rhs.into();

        ts1 - ts2
    }
}

impl Add<Duration> for UTCDateTime {
    type Output = UTCDateTime;

    fn add(self, rhs: Duration) -> Self::Output {
        let (time, excess) = self.time.wrapping_add(rhs);
        let date = self.date + excess;
        UTCDateTime { date, time }
    }
}
impl Add<&Duration> for UTCDateTime {
    type Output = UTCDateTime;

    fn add(self, rhs: &Duration) -> Self::Output {
        let (time, excess) = self.time.wrapping_add(*rhs);
        let date = self.date + excess;
        UTCDateTime { date, time }
    }
}

impl AddAssign<Duration> for UTCDateTime {
    fn add_assign(&mut self, rhs: Duration) {
        let (time, excess) = self.time.wrapping_add(rhs);
        self.time = time;
        self.date += excess;
    }
}
impl AddAssign<&Duration> for UTCDateTime {
    fn add_assign(&mut self, rhs: &Duration) {
        let (time, excess) = self.time.wrapping_add(*rhs);
        self.time = time;
        self.date += excess;
    }
}
