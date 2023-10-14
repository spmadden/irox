// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use crate::time::epoch::UnixTimestamp;
use crate::time::gregorian::Date;
use crate::time::Time;

///
/// Represents a Gregorian Date and Time in UTC
pub struct UTCDateTime {
    date: Date,
    time: Time,
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
