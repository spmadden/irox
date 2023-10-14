// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Implementations of [`DateFormat`] and [`DateFormatParser`] based on the ISO8601 specification
//!

use crate::time::datetime::UTCDateTime;
use crate::time::duration::DurationUnit;
use crate::time::format::{Format, FormatError, FormatParser};
use crate::time::gregorian::Date;
use crate::time::Time;
use crate::units::FromUnits;

///
/// IS0 8601-1:2019 Basic Date Time Format
///
/// Equivalent to `YYYYMMddTHHmmss.SSS`
pub struct BasicDateTime {}
impl Format for BasicDateTime {
    type Item = UTCDateTime;

    fn format(&self, date: &UTCDateTime) -> String {
        format!(
            "{}T{}",
            BasicDate::format(&date.get_date()),
            BasicTime::format(&date.get_time())
        )
    }
}
impl FormatParser for BasicDateTime {
    type Item = UTCDateTime;

    fn try_from(&self, data: &str) -> Result<UTCDateTime, FormatError> {
        // expecting a string of length 8
        // let mut iter = data.chars();
        // let year_str = iter.collect_next_chunk(4);

        todo!()
    }
}

///
/// IS0 8601-1:2019 Basic Date Format
///
/// Equivalent to `YYYYMMdd`
#[derive(Default, Debug, Copy, Clone)]
pub struct BasicDate {}
impl BasicDate {
    pub fn format(date: &Date) -> String {
        BasicDate::default().format(date)
    }
}
impl Format for BasicDate {
    type Item = Date;

    fn format(&self, date: &Self::Item) -> String {
        format!(
            "{:04}{:02}{:02}",
            date.year(),
            date.month_of_year() as u8,
            date.day_of_month(),
        )
    }
}

///
/// IS0 8601-1:2019 Basic Time Format
///
/// Equivalent to `HHmmss.SSS`
#[derive(Default, Debug, Copy, Clone)]
pub struct BasicTime {}

impl Format for BasicTime {
    type Item = Time;

    fn format(&self, date: &Self::Item) -> String {
        let (h, m, s) = date.as_hms();
        format!(
            "{:02}{:02}{:02}.{:03}",
            h,
            m,
            s,
            DurationUnit::Microsecond.from(date.nanoseconds, DurationUnit::Nanosecond)
        )
    }
}
impl BasicTime {
    pub fn format(time: &Time) -> String {
        BasicTime::default().format(time)
    }
}
