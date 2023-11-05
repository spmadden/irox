// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Implementations of [`Format`] and [`FormatParser`] based on the ISO8601 specification
//!

use std::str::FromStr;

use irox_tools::iterators::Itertools;
use irox_units::units::duration::Duration;

use crate::datetime::UTCDateTime;
use crate::format::{Format, FormatError, FormatParser};
use crate::gregorian::Date;
use crate::Time;

///
/// IS0 8601-1:2019 Basic Date and Time of Day Format, section 5.4.2
///
/// Equivalent to `YYYYMMddTHHmmssZ`
pub struct BasicDateTimeOfDay;
///
/// IS0 8601-1:2019 Basic Date and Time of Day Format, section 5.4.2
///
/// Equivalent to `YYYYMMddTHHmmssZ`
pub const BASIC_DATE_TIME_OF_DAY: BasicDateTimeOfDay = BasicDateTimeOfDay {};
impl Format<UTCDateTime> for BasicDateTimeOfDay {
    fn format(&self, date: &UTCDateTime) -> String {
        format!(
            "{}{}",
            BasicCalendarDate::format(&date.get_date()),
            BasicTimeOfDay::format(&date.get_time())
        )
    }
}
impl FormatParser<UTCDateTime> for BasicDateTimeOfDay {
    fn try_from(&self, data: &str) -> Result<UTCDateTime, FormatError> {
        let date = Date::parse_from(&BASIC_CALENDAR_DATE, data)?;
        let time = Time::parse_from(&BASIC_TIME_OF_DAY, data.split_at(8).1)?;
        Ok(UTCDateTime { date, time })
    }
}

///
/// IS0 8601-1:2019 Basic Calendar Date Format, of section 5.2.2
///
/// Equivalent to `YYYYMMdd`
#[derive(Default, Debug, Copy, Clone)]
pub struct BasicCalendarDate;
///
/// IS0 8601-1:2019 Basic Calendar Date Format, of section 5.2.2
///
/// Equivalent to `YYYYMMdd`
pub const BASIC_CALENDAR_DATE: BasicCalendarDate = BasicCalendarDate {};
impl BasicCalendarDate {
    pub fn format(date: &Date) -> String {
        BasicCalendarDate.format(date)
    }
}
impl Format<Date> for BasicCalendarDate {
    fn format(&self, date: &Date) -> String {
        format!(
            "{:04}{:02}{:02}",
            date.year(),
            date.month_of_year() as u8,
            date.day_of_month() + 1,
        )
    }
}
impl FormatParser<Date> for BasicCalendarDate {
    fn try_from(&self, data: &str) -> Result<Date, FormatError> {
        // expecting a string of length 8
        let mut iter = data.chars();
        let year_str = iter.collect_next_chunk(4);
        let month_str = iter.collect_next_chunk(2);
        let day_str = iter.collect_next_chunk(2);

        if year_str.len() != 4 {
            return FormatError::err(format!(
                "Expecting year to be length 4, but was {}",
                year_str.len()
            ));
        }
        if month_str.len() != 2 {
            return FormatError::err(format!(
                "Expecting month to be length 2, but was {}",
                month_str.len()
            ));
        }
        if day_str.len() != 2 {
            return FormatError::err(format!(
                "Expecting day to be length 2, but was {}",
                day_str.len()
            ));
        }
        let year_str = String::from_iter(year_str);
        let year = i32::from_str(&year_str)?;
        let month_str = String::from_iter(month_str);
        let month = u8::from_str(&month_str)?;
        let day_str = String::from_iter(day_str);
        let day = u8::from_str(&day_str)?;

        Ok(Date::try_from_values(year, month, day)?)
    }
}

impl Format<UTCDateTime> for BasicCalendarDate {
    fn format(&self, date: &UTCDateTime) -> String {
        BasicCalendarDate.format(&date.date)
    }
}

///
/// IS0 8601-1:2019 Basic Time Of Day Format, of section 5.3.3
///
/// Equivalent to `THHmmssZ`
#[derive(Default, Debug, Copy, Clone)]
pub struct BasicTimeOfDay;
///
/// IS0 8601-1:2019 Basic Time Of Day Format, of section 5.3.3
///
/// Equivalent to `THHmmssZ`
pub const BASIC_TIME_OF_DAY: BasicTimeOfDay = BasicTimeOfDay {};

impl Format<Time> for BasicTimeOfDay {
    fn format(&self, date: &Time) -> String {
        let (h, m, s) = date.as_hms();
        format!("T{h:02}{m:02}{s:02}Z")
    }
}
impl Format<UTCDateTime> for BasicTimeOfDay {
    fn format(&self, date: &UTCDateTime) -> String {
        BasicTimeOfDay::format(&date.time)
    }
}
impl FormatParser<Time> for BasicTimeOfDay {
    fn try_from(&self, data: &str) -> Result<Time, FormatError> {
        let mut iter = data.chars();
        let tee = iter.collect_next_chunk(1);
        let hour_string = iter.collect_next_chunk(2);
        let minute_string = iter.collect_next_chunk(2);
        let second_string = iter.collect_next_chunk(2);
        let zee = iter.collect_next_chunk(1);

        if tee.first() != Some(&'T') {
            return FormatError::err_str("Expecting the string to start with 'T'");
        }
        if zee.first() != Some(&'Z') {
            return FormatError::err_str("Expecting the string to end with 'Z'");
        }

        if hour_string.len() != 2 {
            return FormatError::err(format!(
                "Expecting hours to be length 2, but was {}",
                hour_string.len()
            ));
        }
        if minute_string.len() != 2 {
            return FormatError::err(format!(
                "Expecting minutes to be length 2, but was {}",
                minute_string.len()
            ));
        }
        if second_string.len() != 2 {
            return FormatError::err(format!(
                "Expecting seconds to be length 2, but was {}",
                second_string.len()
            ));
        }

        let hours = u32::from_str(String::from_iter(hour_string).as_str())?;
        let minutes = u32::from_str(String::from_iter(minute_string).as_str())?;
        let seconds = u32::from_str(String::from_iter(second_string).as_str())?;

        let second_of_day = hours * 3600 + minutes * 60 + seconds;

        Ok(Time {
            second_of_day,
            nanoseconds: 0,
        })
    }
}
impl BasicTimeOfDay {
    pub fn format(time: &Time) -> String {
        BasicTimeOfDay.format(time)
    }
}

pub struct ISO8601Duration;
pub const DURATION: ISO8601Duration = ISO8601Duration;

impl Format<Duration> for ISO8601Duration {
    fn format(&self, date: &Duration) -> String {
        let (days, hours, minutes, seconds) = date.as_dhms();
        if days > 0 {
            return format!("P{days}DT{hours:02}H{minutes:02}M{seconds:02}S");
        }
        if hours > 0 {
            return format!("PT{hours}H{minutes:02}M{seconds:02}S");
        }
        if minutes > 0 {
            return format!("PT{minutes}M{seconds:02}S");
        }
        format!("PT{seconds}S")
    }
}

#[cfg(test)]
mod tests {
    use crate::epoch::{
        COMMON_ERA_EPOCH, GPS_EPOCH, GREGORIAN_EPOCH, NTP_EPOCH, PRIME_EPOCH, UNIX_EPOCH,
        WINDOWS_NT_EPOCH,
    };
    use crate::format::iso8601::{BASIC_CALENDAR_DATE, BASIC_TIME_OF_DAY};
    use crate::format::FormatError;
    use crate::gregorian::Date;
    use crate::Time;

    #[test]
    pub fn test_basic_date() -> Result<(), FormatError> {
        let tests = vec![
            ("19700101", UNIX_EPOCH),
            ("19800106", GPS_EPOCH),
            ("19000101", NTP_EPOCH),
            ("19000101", PRIME_EPOCH),
            ("15821015", GREGORIAN_EPOCH),
            ("00010101", COMMON_ERA_EPOCH),
            ("16010101", WINDOWS_NT_EPOCH),
        ];
        for (string, format) in tests {
            assert_eq!(
                string,
                format
                    .get_gregorian_date()
                    .format(&BASIC_CALENDAR_DATE)
                    .as_str()
            );
            assert_eq!(
                format.get_gregorian_date(),
                Date::parse_from(&BASIC_CALENDAR_DATE, string)?
            );
        }
        Ok(())
    }

    #[test]
    pub fn test_basic_time() -> Result<(), FormatError> {
        for hour in 0..24 {
            for minute in 0..60 {
                for second in 0..60 {
                    let time_sec = hour * 3600 + minute * 60 + second;
                    let time = Time::new(time_sec, 0)?;

                    assert_eq!(
                        format!("T{hour:02}{minute:02}{second:02}Z"),
                        time.format(&BASIC_TIME_OF_DAY)
                    );
                }
            }
        }
        Ok(())
    }

    #[test]
    pub fn test_basic_datetime() -> Result<(), FormatError> {
        Ok(())
    }
}
