// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Contains [`UTCDateTime`] and associated elements to represent a [`Date`] and [`Time`] in UTC
//!

use crate::epoch::{UnixTimestamp, UNIX_EPOCH};
use crate::format::iso8601::BASIC_DATE_TIME_OF_DAY;
use crate::format::Format;
use crate::gregorian::Date;
use crate::julian::JulianDate;
use crate::Time;
use irox_units::units::duration::Duration;
use std::fmt::{Display, Formatter};
use std::ops::Sub;

///
/// Represents a Gregorian Date and Time in UTC
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UTCDateTime {
    pub(crate) date: Date,
    pub(crate) time: Time,
}

impl Display for UTCDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    pub fn now() -> UTCDateTime {
        UnixTimestamp::now().into()
    }

    #[must_use]
    pub fn format<T: Format<UTCDateTime>>(&self, format: &T) -> String {
        format.format(self)
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
        date_dur += value.time.into();
        Self::from_offset(date_dur)
    }
}
impl From<&UTCDateTime> for UnixTimestamp {
    fn from(value: &UTCDateTime) -> Self {
        let mut date_dur = value.date - UNIX_EPOCH.get_gregorian_date();
        date_dur += value.time.into();
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

        ts2 - ts1
    }
}
impl Sub<&Self> for UTCDateTime {
    type Output = Duration;

    fn sub(self, rhs: &Self) -> Self::Output {
        let ts1: JulianDate = self.into();
        let ts2: JulianDate = rhs.into();

        ts2 - ts1
    }
}
