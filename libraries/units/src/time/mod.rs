// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

pub use datetime::*;
pub use duration::*;
pub use epoch::*;
pub use gregorian::*;
pub use julian::*;

use crate::bounds::{GreaterThanEqualToValueError, LessThanValue, Range};

mod datetime;
mod duration;
mod epoch;
mod gregorian;
mod julian;

///
/// Represents a time of the day, an offset into the day from midnight.
pub struct Time {
    second_of_day: u32,
}

impl Time {
    pub fn new(second_of_day: u32) -> Result<Time, GreaterThanEqualToValueError<u32>> {
        LessThanValue::new(86401).check_value_is_valid(&second_of_day)?;
        Ok(Time { second_of_day })
    }

    ///
    /// Returns the number of seconds into the "current day"
    #[must_use]
    pub fn as_seconds(&self) -> u32 {
        self.second_of_day
    }

    ///
    /// Converts this time into a duration
    #[must_use]
    pub fn as_duration(&self) -> Duration {
        Duration::new(self.second_of_day as f64, DurationUnit::Second)
    }

    ///
    /// Returns the number of hours represented by this time.
    #[must_use]
    pub fn as_hours(&self) -> u32 {
        (self.second_of_day as f64 / SECONDS_IN_HOUR as f64) as u32
    }

    ///
    /// Returns the number minutes
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
        let seconds = self.as_seconds() - hours * SECONDS_IN_HOUR - minutes * SECONDS_IN_MINUTE;
        (hours, minutes, seconds)
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

pub const HOURS_IN_DAY: u32 = 24;
pub const MINUTES_IN_HOUR: u32 = 60;
pub const SECONDS_IN_MINUTE: u32 = 60;
pub const MINUTES_IN_DAY: u32 = 1440;
pub const SECONDS_IN_HOUR: u32 = 3600;

///
/// Generally 86400, but occasionally 86401 for leap seconds.
pub const SECONDS_IN_DAY: u32 = 86400;
