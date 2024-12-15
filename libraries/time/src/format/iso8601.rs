// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Implementations of [`Format`] and [`FormatParser`] based on the ISO8601 specification
//!

extern crate alloc;
use alloc::string::String;

use core::str::FromStr;

use irox_tools::fmt::DecimalFormatF64;
use irox_tools::format;
use irox_tools::iterators::Itertools;
use irox_units::units::duration::{Duration, SEC_TO_NANOS};

use crate::datetime::UTCDateTime;
use crate::format::{Format, FormatError, FormatParser};
use crate::gregorian::Date;
use crate::Time;

///
/// IS0 8601-1:2019 Basic Date and Time of Day Format, section 5.4.2 Equivalent to `YYYYMMddTHHmmssZ`/`20231231T051025Z`
pub struct BasicDateTimeOfDay;

///
/// IS0 8601-1:2019 Basic Date and Time of Day Format, section 5.4.2 Equivalent to `YYYYMMddTHHmmssZ`/`20231231T051025Z`
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
        let mut iter = data.split(&['T', 't', '_', ' ']);
        let Some(date) = iter.next() else {
            return FormatError::err_str("Expecting date portion");
        };
        let Some(time) = iter.next() else {
            return FormatError::err_str("Expecting time portion");
        };
        let date = Date::parse_from(&BASIC_CALENDAR_DATE, date)?;
        let time = Time::parse_from(&BASIC_TIME_OF_DAY, time)?;
        Ok(UTCDateTime { date, time })
    }
}

///
/// IS0 8601-1:2019 Basic Calendar Date Format, of section 5.2.2. Equivalent to `YYYYMMdd`/`20231231`
#[derive(Default, Debug, Copy, Clone)]
pub struct BasicCalendarDate;

///
/// IS0 8601-1:2019 Basic Calendar Date Format, of section 5.2.2. Equivalent to `YYYYMMdd`/`20231231`
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
        if data.len() < 8 {
            // assume year:dayofyear
            let mut iter = data.chars();
            let year_str = iter.collect_next_chunk(4);
            let day_str = iter.collect_next_chunk(3);
            if year_str.len() != 4 {
                return FormatError::err(format!(
                    "Expecting year to be length 4, but was {}",
                    year_str.len()
                ));
            }
            if day_str.len() != 3 {
                return FormatError::err(format!(
                    "Expecting day to be length 3, but was {}",
                    day_str.len()
                ));
            }
            let year_str = String::from_iter(year_str);
            let year = i32::from_str(&year_str)?;
            let day_str = String::from_iter(day_str);
            let day = u16::from_str(&day_str)?;

            return Ok(Date::new(year, day - 1)?);
        }
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
/// IS0 8601-1:2019 Basic Time Of Day Format, of section 5.3.3. Equivalent to `THHmmssZ`/`T051025Z`
#[derive(Default, Debug, Copy, Clone)]
pub struct BasicTimeOfDay;

///
/// IS0 8601-1:2019 Basic Time Of Day Format, of section 5.3.3. Equivalent to `THHmmssZ`/`T051025Z`
pub const BASIC_TIME_OF_DAY: BasicTimeOfDay = BasicTimeOfDay {};

impl Format<Time> for BasicTimeOfDay {
    fn format(&self, date: &Time) -> String {
        let (h, m, s) = date.as_hms();
        if date.nanoseconds == 0 {
            format!("T{h:02}{m:02}{s:02}Z")
        } else {
            let s = s as f64 + date.get_secondsfrac();
            format!("T{h:02}{m:02}{}", DecimalFormatF64(2, 9, s))
        }
    }
}

impl Format<UTCDateTime> for BasicTimeOfDay {
    fn format(&self, date: &UTCDateTime) -> String {
        BasicTimeOfDay::format(&date.time)
    }
}

impl FormatParser<Time> for BasicTimeOfDay {
    fn try_from(&self, data: &str) -> Result<Time, FormatError> {
        let data = if data.ends_with(['z', 'Z']) {
            data.split_at(data.len() - 1).0
        } else {
            data
        };
        let data = if data.starts_with(['T', 't']) {
            data.split_at(1).1
        } else {
            data
        };
        let mut iter = data.chars();
        let hour_string = iter.collect_next_chunk(2);
        let minute_string = iter.collect_next_chunk(2);
        let second_string = iter.collect_next_chunk(10);

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
        if second_string.len() < 2 {
            return FormatError::err(format!(
                "Expecting seconds to be at least length 2, but was {}",
                second_string.len()
            ));
        }

        let hours = u32::from_str(String::from_iter(hour_string).as_str())?;
        let minutes = u32::from_str(String::from_iter(minute_string).as_str())?;
        let seconds = f64::from_str(String::from_iter(second_string).as_str())?;

        let second_of_day = hours * 3600 + minutes * 60 + seconds as u32;
        let nanoseconds = (irox_tools::f64::FloatExt::fract(seconds) * SEC_TO_NANOS) as u32;

        Ok(Time {
            second_of_day,
            nanoseconds,
        })
    }
}

impl BasicTimeOfDay {
    pub fn format(time: &Time) -> String {
        BasicTimeOfDay.format(time)
    }
}

/// IS0 8601-1:2019 Duration Format, of section 5.5.2. Equivalent to `PddDThhHmmMssS`/`P10DT05H10M25S`
pub struct ISO8601Duration;

/// IS0 8601-1:2019 Duration Format, of section 5.5.2. Equivalent to `PddDThhHmmMssS`/`P10DT05H10M25S`
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

/// ISO 8601-1:2019 Extended Date Format, of section 5.2.2. Equivalent to `YYYY-mm-dd`/`2023-12-31`
pub struct ExtendedDateFormat;

/// ISO 8601-1:2019 Extended Date Format, of section 5.2.2. Equivalent to `YYYY-mm-dd`/`2023-12-31`
pub const EXTENDED_DATE_FORMAT: ExtendedDateFormat = ExtendedDateFormat;

impl Format<Date> for ExtendedDateFormat {
    fn format(&self, date: &Date) -> String {
        format!(
            "{:04}-{:02}-{:02}",
            date.year(),
            date.month_of_year() as u8,
            date.day_of_month() + 1,
        )
    }
}

impl FormatParser<Date> for ExtendedDateFormat {
    fn try_from(&self, data: &str) -> Result<Date, FormatError> {
        let mut splits = data.split('-');
        let Some(year) = splits.next() else {
            return FormatError::err_str("Expecting first part to be year, but didn't exist.");
        };
        let year = i32::from_str(year)?;
        let Some(month) = splits.next() else {
            return FormatError::err_str("Expecting second part, but didn't exist.");
        };
        let Some(day) = splits.next() else {
            // assume that it's year-dayofyear.
            let day = u16::from_str(month)?;
            return Ok(Date::new(year, day - 1)?);
        };
        let month = u8::from_str(month)?;
        let day = u8::from_str(day)?;
        let date = Date::try_from_values(year, month, day)?;
        Ok(date)
    }
}

impl Format<UTCDateTime> for ExtendedDateFormat {
    fn format(&self, date: &UTCDateTime) -> String {
        ExtendedDateFormat.format(&date.date)
    }
}

/// ISO 8601-1:2019 Extended Time Format, of section 5.3.3. Equivalent to `THH:mm:ssZ`/`T05:10:25Z`
pub struct ExtendedTimeFormat;

/// ISO 8601-1:2019 Extended Time Format, of section 5.3.3. Equivalent to `THH:mm:ssZ`/`T05:10:25Z`
pub const EXTENDED_TIME_FORMAT: ExtendedTimeFormat = ExtendedTimeFormat;

impl Format<Time> for ExtendedTimeFormat {
    fn format(&self, date: &Time) -> String {
        let (h, m, s) = date.as_hms();
        if date.nanoseconds == 0 {
            format!("T{h:02}:{m:02}:{s:02}Z")
        } else {
            let s = s as f64 + date.get_secondsfrac();
            format!("T{h:02}:{m:02}:{}Z", DecimalFormatF64(2, 9, s))
        }
    }
}

impl Format<UTCDateTime> for ExtendedTimeFormat {
    fn format(&self, date: &UTCDateTime) -> String {
        ExtendedTimeFormat.format(&date.time)
    }
}

impl FormatParser<Time> for ExtendedTimeFormat {
    fn try_from(&self, data: &str) -> Result<Time, FormatError> {
        let data = if data.starts_with(['T', 't']) {
            data.split_at(1).1
        } else {
            data
        };
        let data = if data.ends_with(['z', 'Z']) {
            data.split_at(data.len() - 1).0
        } else {
            data
        };
        let mut split = data.split(':');
        let Some(hour) = split.next() else {
            return FormatError::err_str("Expecting first part.");
        };
        let Some(minute) = split.next() else {
            return FormatError::err_str("Expecting second part.");
        };
        let Some(second) = split.next() else {
            return FormatError::err_str("Expecting third part.");
        };
        let Some(second) = second.split(['-', '+']).next() else {
            return FormatError::err_str("Expecting to remove TZ info");
        };
        let hours = u8::from_str(hour)?;
        let minutes = u8::from_str(minute)?;
        let seconds = f64::from_str(second)?;
        let time = Time::from_hms_f64(hours, minutes, seconds)?;
        Ok(time)
    }
}

/// ISO 8601-1:2019 Extended Date Time Format, of section 5.4.2. Equivalent to `YYYY-MM-DDTHH:mm:ssZ`/`2023-12-31T05:10:25Z`
pub struct ExtendedDateTimeFormat;
/// ISO 8601-1:2019 Extended Date Time Format, of section 5.4.2. Equivalent to `YYYY-MM-DDTHH:mm:ssZ`/`2023-12-31T05:10:25Z`
pub const EXTENDED_DATE_TIME_FORMAT: ExtendedDateTimeFormat = ExtendedDateTimeFormat;

impl Format<UTCDateTime> for ExtendedDateTimeFormat {
    fn format(&self, date: &UTCDateTime) -> String {
        format!(
            "{}{}",
            ExtendedDateFormat.format(&date.get_date()),
            ExtendedTimeFormat.format(&date.get_time())
        )
    }
}

impl FormatParser<UTCDateTime> for ExtendedDateTimeFormat {
    fn try_from(&self, data: &str) -> Result<UTCDateTime, FormatError> {
        let mut split = data.split(['T', 't', '_', ' ']);
        let Some(date) = split.next() else {
            return FormatError::err_str("Missing date.");
        };
        let Some(time) = split.next() else {
            return FormatError::err_str("Missing time.");
        };
        let date = ExtendedDateFormat.try_from(date)?;
        let time = ExtendedTimeFormat.try_from(time)?;
        Ok(UTCDateTime::new(date, time))
    }
}

/// ISO 8601-1:2019 Date Time Format of section 5.4.2.  Will read either the basic or extended formats, produces the extended format.
pub struct ISO8601DateTime;

/// ISO 8601-1:2019 Date Time Format of section 5.4.2.  Will read either the basic or extended formats, produces the extended format.
pub const ISO8601_DATE_TIME: ISO8601DateTime = ISO8601DateTime;

impl FormatParser<UTCDateTime> for ISO8601DateTime {
    fn try_from(&self, data: &str) -> Result<UTCDateTime, FormatError> {
        if data.contains(':') {
            ExtendedDateTimeFormat.try_from(data)
        } else {
            BasicDateTimeOfDay.try_from(data)
        }
    }
}
impl Format<UTCDateTime> for ISO8601DateTime {
    fn format(&self, date: &UTCDateTime) -> String {
        ExtendedDateTimeFormat.format(date)
    }
}

/// ISO 8601-1:2019 Date Format of section 5.2.2.  Will read either the basic or extended formats, produces the extended format.
pub struct ISO8601Date;

/// ISO 8601-1:2019 Date Format of section 5.2.2.  Will read either the basic or extended formats, produces the extended format.
pub const ISO8601_DATE: ISO8601Date = ISO8601Date;

impl FormatParser<Date> for ISO8601Date {
    fn try_from(&self, data: &str) -> Result<Date, FormatError> {
        if data.contains('-') {
            ExtendedDateFormat.try_from(data)
        } else {
            BasicCalendarDate.try_from(data)
        }
    }
}

impl Format<Date> for ISO8601Date {
    fn format(&self, date: &Date) -> String {
        ExtendedDateFormat.format(date)
    }
}

/// ISO 8601-1:2019 Time Format of section 5.3.3.  Will read either the basic or extended formats, produces the extended format.
pub struct ISO8601Time;
/// ISO 8601-1:2019 Time Format of section 5.3.3.  Will read either the basic or extended formats, produces the extended format.
pub const ISO8601_TIME: ISO8601Time = ISO8601Time;
impl FormatParser<Time> for ISO8601Time {
    fn try_from(&self, data: &str) -> Result<Time, FormatError> {
        if data.contains(':') {
            ExtendedTimeFormat.try_from(data)
        } else {
            BasicTimeOfDay.try_from(data)
        }
    }
}

/// ISO 8601-1:2019 Basic Week Number format of section 5.2.4.2. Equivalent to: `YYYYWww`/`2023W52` (Week 52)
pub struct ISO8601WeekNumber;
/// ISO 8601-1:2019 Basic Week Number format of section 5.2.4.2. Equivalent to: `YYYYWww`/`2023W52` (Week 52)
pub const ISO8601_WEEK_NUMBER: ISO8601WeekNumber = ISO8601WeekNumber;
impl Format<Date> for ISO8601WeekNumber {
    fn format(&self, date: &Date) -> String {
        let (year, wkno) = date.week_number();
        format!("{year}W{wkno:02}")
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use irox_tools::ansi_colors::{FORMAT_COLOR_FG_GREEN, FORMAT_COLOR_FG_RED, FORMAT_RESET};
    use irox_tools::format;
    use irox_units::bounds::GreaterThanEqualToValueError;

    use crate::datetime::UTCDateTime;
    use crate::epoch::{
        COMMON_ERA_EPOCH, GPS_EPOCH, GREGORIAN_EPOCH, NTP_EPOCH, PRIME_EPOCH, UNIX_EPOCH,
        WINDOWS_NT_EPOCH,
    };
    use crate::format::iso8601::{
        ExtendedDateFormat, ExtendedDateTimeFormat, ExtendedTimeFormat, ISO8601Date,
        ISO8601DateTime, ISO8601Time, BASIC_CALENDAR_DATE, BASIC_TIME_OF_DAY,
        EXTENDED_DATE_TIME_FORMAT, ISO8601_DATE_TIME, ISO8601_WEEK_NUMBER,
    };
    use crate::format::{Format, FormatError, FormatParser};
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

    #[test]
    pub fn test_extended_time() -> Result<(), FormatError> {
        let time = Time::from_hms(23, 20, 30)?;

        assert_eq!("T23:20:30Z", ExtendedTimeFormat.format(&time));
        let parsed = ExtendedTimeFormat.try_from("T23:20:30Z")?;
        assert_eq!(time, parsed);
        let parsed = ExtendedTimeFormat.try_from("23:20:30Z")?;
        assert_eq!(time, parsed);

        Ok(())
    }

    #[test]
    pub fn test_extended_date() -> Result<(), FormatError> {
        let date = Date::try_from_values(1985, 04, 12)?;
        assert_eq!("1985-04-12", ExtendedDateFormat.format(&date));

        let parsed = ExtendedDateFormat.try_from("1985-04-12")?;
        assert_eq!(date, parsed);

        Ok(())
    }

    #[test]
    pub fn test_extended_datetime() -> Result<(), FormatError> {
        let date = Date::try_from_values(1985, 04, 12)?;
        let time = Time::from_hms(23, 20, 30)?;
        let dt = UTCDateTime::new(date, time);

        assert_eq!("1985-04-12T23:20:30Z", ExtendedDateTimeFormat.format(&dt));

        let parsed = ExtendedDateTimeFormat.try_from("1985-04-12T23:20:30Z")?;
        assert_eq!(dt, parsed);

        let parsed = ExtendedDateTimeFormat.try_from("1985-04-12T23:20:30")?;
        assert_eq!(dt, parsed);

        Ok(())
    }

    #[test]
    pub fn test_both_formats() -> Result<(), FormatError> {
        let date = Date::try_from_values(1985, 04, 12)?;
        let time = Time::from_hms(23, 20, 30)?;
        let dt = UTCDateTime::new(date, time);

        let parsed = ISO8601DateTime.try_from("1985-04-12T23:20:30Z")?;
        assert_eq!(dt, parsed);
        let parsed = ISO8601DateTime.try_from("19850412T232030Z")?;
        assert_eq!(dt, parsed);
        let parsed = ISO8601DateTime.try_from("19850412T232030Z")?;
        assert_eq!(dt, parsed);
        Ok(())
    }

    macro_rules! run_cases {
        ($cases:ident, $parser:ident) => {
            let mut any_failures = false;
            for case in $cases {
                let res = $parser.try_from(case.0);
                let res = match res {
                    Ok(res) => res,
                    Err(e) => {
                        println!(
                            "{}ERROR PARSING{} : {} // {e:?}",
                            FORMAT_COLOR_FG_RED, FORMAT_RESET, case.0
                        );
                        any_failures = true;
                        continue;
                    }
                };
                if res != case.1 {
                    println!(
                        "{}ERROR EQUALITY{}: {}, {} != {}",
                        FORMAT_COLOR_FG_RED, FORMAT_RESET, case.0, case.1, res
                    );
                    any_failures = true;
                } else {
                    println!(
                        "{}PASSED{}: {}",
                        FORMAT_COLOR_FG_GREEN, FORMAT_RESET, case.0
                    );
                }
            }
            assert_ne!(true, any_failures);
        };
    }

    #[test]
    pub fn compat_report_dates() -> Result<(), FormatError> {
        let test_cases = [
            ("2023-10-13", Date::try_from_values(2023, 10, 13)?),
            ("2023-286", Date::new(2023, 285)?),
            ("20231013", Date::try_from_values(2023, 10, 13)?),
            ("2023286", Date::new(2023, 285)?),
        ];

        run_cases!(test_cases, ISO8601Date);

        Ok(())
    }

    #[test]
    pub fn compat_report_times() -> Result<(), FormatError> {
        let test_cases = [
            ("02:56:16Z", Time::from_hms(02, 56, 16)?),
            ("02:56:16.6Z", Time::from_hms_f64(02, 56, 16.6)?),
            ("02:56:16.61Z", Time::from_hms_f64(02, 56, 16.61)?),
            ("02:56:16.615Z", Time::from_hms_f64(02, 56, 16.615)?),
            ("02:56:16.615283Z", Time::from_hms_f64(02, 56, 16.615283)?),
            ("22:56:16", Time::from_hms(22, 56, 16)?),
            ("22:56:16.6", Time::from_hms_f64(22, 56, 16.6)?),
            ("22:56:16.61", Time::from_hms_f64(22, 56, 16.61)?),
            ("22:56:16.615", Time::from_hms_f64(22, 56, 16.615)?),
            ("22:56:16.615283", Time::from_hms_f64(22, 56, 16.615283)?),
            ("T22:56:16", Time::from_hms(22, 56, 16)?),
            ("T22:56:16.6", Time::from_hms_f64(22, 56, 16.6)?),
            ("T22:56:16.61", Time::from_hms_f64(22, 56, 16.61)?),
            ("T22:56:16.615", Time::from_hms_f64(22, 56, 16.615)?),
            ("T22:56:16.615283", Time::from_hms_f64(22, 56, 16.615283)?),
            ("T02:56:16Z", Time::from_hms(02, 56, 16)?),
            ("T02:56:16.6Z", Time::from_hms_f64(02, 56, 16.6)?),
            ("T02:56:16.61Z", Time::from_hms_f64(02, 56, 16.61)?),
            ("T02:56:16.615Z", Time::from_hms_f64(02, 56, 16.615)?),
            ("T02:56:16.615283Z", Time::from_hms_f64(02, 56, 16.615283)?),
            ("225616", Time::from_hms(22, 56, 16)?),
            ("225616.6", Time::from_hms_f64(22, 56, 16.6)?),
            ("225616.61", Time::from_hms_f64(22, 56, 16.61)?),
            ("225616.615", Time::from_hms_f64(22, 56, 16.615)?),
            ("225616.615283", Time::from_hms_f64(22, 56, 16.615283)?),
            ("025616Z", Time::from_hms(02, 56, 16)?),
            ("025616.6Z", Time::from_hms_f64(02, 56, 16.6)?),
            ("025616.61Z", Time::from_hms_f64(02, 56, 16.61)?),
            ("025616.615Z", Time::from_hms_f64(02, 56, 16.615)?),
            ("025616.615283Z", Time::from_hms_f64(02, 56, 16.615283)?),
            ("T225616", Time::from_hms(22, 56, 16)?),
            ("T225616.6", Time::from_hms_f64(22, 56, 16.6)?),
            ("T225616.61", Time::from_hms_f64(22, 56, 16.61)?),
            ("T225616.615", Time::from_hms_f64(22, 56, 16.615)?),
            ("T225616.615283", Time::from_hms_f64(22, 56, 16.615283)?),
            ("T025616Z", Time::from_hms(02, 56, 16)?),
            ("T025616.6Z", Time::from_hms_f64(02, 56, 16.6)?),
            ("T025616.61Z", Time::from_hms_f64(02, 56, 16.61)?),
            ("T025616.615Z", Time::from_hms_f64(02, 56, 16.615)?),
            ("T025616.615283Z", Time::from_hms_f64(02, 56, 16.615283)?),
        ];

        run_cases!(test_cases, ISO8601Time);
        Ok(())
    }

    #[test]
    pub fn compat_report_datetime() -> Result<(), FormatError> {
        let test_cases = [
            (
                "2023-10-14T02:56:16Z",
                UTCDateTime::try_from_values(2023, 10, 14, 02, 56, 16)?,
            ),
            (
                "2023-10-14T02:56:16.6Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.6)?,
            ),
            (
                "2023-10-14T02:56:16.61Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.61)?,
            ),
            (
                "2023-10-14T02:56:16.615Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615)?,
            ),
            (
                "2023-10-14T02:56:16.615283Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615283)?,
            ),
            (
                "2023-10-14t02:56:16z",
                UTCDateTime::try_from_values(2023, 10, 14, 02, 56, 16)?,
            ),
            (
                "2023-10-14t02:56:16.615z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615)?,
            ),
            (
                "2023-10-14 02:56:16Z",
                UTCDateTime::try_from_values(2023, 10, 14, 02, 56, 16)?,
            ),
            (
                "2023-10-14_02:56:16Z",
                UTCDateTime::try_from_values(2023, 10, 14, 02, 56, 16)?,
            ),
            (
                "2023-10-14 02:56:16z",
                UTCDateTime::try_from_values(2023, 10, 14, 02, 56, 16)?,
            ),
            (
                "2023-10-14_02:56:16z",
                UTCDateTime::try_from_values(2023, 10, 14, 02, 56, 16)?,
            ),
            (
                "2023-10-14 02:56:16.6Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.6)?,
            ),
            (
                "2023-10-14 02:56:16.61Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.61)?,
            ),
            (
                "2023-10-14 02:56:16.615Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615)?,
            ),
            (
                "2023-10-14_02:56:16.615Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615)?,
            ),
            (
                "2023-10-14 02:56:16.615283Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615283)?,
            ),
            (
                "2023-10-14_02:56:16.615283Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615283)?,
            ),
            (
                "2023-10-14 02:56:16.615z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615)?,
            ),
            (
                "2023-10-14_02:56:16.615z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615)?,
            ),
            (
                "2023-10-14 02:56:16.615283z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615283)?,
            ),
            (
                "2023-10-14_02:56:16.615283z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615283)?,
            ),
            (
                "2023-10-13T22:56:16",
                UTCDateTime::try_from_values(2023, 10, 13, 22, 56, 16)?,
            ),
            (
                "2023-10-13T22:56:16.6",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.6)?,
            ),
            (
                "2023-10-13T22:56:16.61",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.61)?,
            ),
            (
                "2023-10-13T22:56:16.615",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.615)?,
            ),
            (
                "2023-10-13T22:56:16.615283",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.615283)?,
            ),
            (
                "2023-286T22:56:16",
                UTCDateTime::try_from_values(2023, 10, 13, 22, 56, 16)?,
            ),
            (
                "2023-286T22:56:16.6",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.6)?,
            ),
            (
                "2023-286T22:56:16.61",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.61)?,
            ),
            (
                "2023-286T22:56:16.615",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.615)?,
            ),
            (
                "2023-286T22:56:16.615283",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.615283)?,
            ),
            (
                "2023-287T02:56:16Z",
                UTCDateTime::try_from_values(2023, 10, 14, 02, 56, 16)?,
            ),
            (
                "2023-287T02:56:16.6Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.6)?,
            ),
            (
                "2023-287T02:56:16.61Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.61)?,
            ),
            (
                "2023-287T02:56:16.615Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615)?,
            ),
            (
                "2023-287T02:56:16.615283Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615283)?,
            ),
            (
                "20231013T225616",
                UTCDateTime::try_from_values(2023, 10, 13, 22, 56, 16)?,
            ),
            (
                "20231013T225616.6",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.6)?,
            ),
            (
                "20231013T225616.61",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.61)?,
            ),
            (
                "20231013T225616.615",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.615)?,
            ),
            (
                "20231013T225616.615283",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.615283)?,
            ),
            (
                "20231014T025616Z",
                UTCDateTime::try_from_values(2023, 10, 14, 02, 56, 16)?,
            ),
            (
                "20231014T025616.6Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.6)?,
            ),
            (
                "20231014T025616.61Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.61)?,
            ),
            (
                "20231014T025616.615Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615)?,
            ),
            (
                "20231014T025616.615283Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615283)?,
            ),
            (
                "2023286T225616",
                UTCDateTime::try_from_values(2023, 10, 13, 22, 56, 16)?,
            ),
            (
                "2023286T225616.6",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.6)?,
            ),
            (
                "2023286T225616.61",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.61)?,
            ),
            (
                "2023286T225616.615",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.615)?,
            ),
            (
                "2023286T225616.615283",
                UTCDateTime::try_from_values_f64(2023, 10, 13, 22, 56, 16.615283)?,
            ),
            (
                "2023287T025616Z",
                UTCDateTime::try_from_values(2023, 10, 14, 02, 56, 16)?,
            ),
            (
                "2023287T025616.6Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.6)?,
            ),
            (
                "2023287T025616.61Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.61)?,
            ),
            (
                "2023287T025616.615Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615)?,
            ),
            (
                "2023287T025616.615283Z",
                UTCDateTime::try_from_values_f64(2023, 10, 14, 02, 56, 16.615283)?,
            ),
        ];

        run_cases!(test_cases, ISO8601DateTime);

        Ok(())
    }

    #[test]
    pub fn test_leading_zeros() -> Result<(), FormatError> {
        let time = UTCDateTime::try_from_values(2023, 01, 04, 01, 01, 01)?;
        assert_eq!(
            "2023-01-04T01:01:01Z",
            format!("{}", time.format(&EXTENDED_DATE_TIME_FORMAT))
        );

        let time = UTCDateTime::try_from_values_f64(2023, 01, 04, 01, 01, 01.01)?;
        assert_eq!(
            "2023-01-04T01:01:01.010000000Z",
            format!("{}", time.format(&EXTENDED_DATE_TIME_FORMAT))
        );

        Ok(())
    }

    #[test]
    pub fn test_april_fools_day() -> Result<(), FormatError> {
        let time = UTCDateTime::try_from_values(2023, 04, 01, 01, 01, 01)?;
        assert_eq!(
            "2023-04-01T01:01:01Z",
            format!("{}", time.format(&EXTENDED_DATE_TIME_FORMAT))
        );
        let time = UTCDateTime::try_from_values(2023, 04, 02, 01, 01, 01)?;
        assert_eq!(
            "2023-04-02T01:01:01Z",
            format!("{}", time.format(&EXTENDED_DATE_TIME_FORMAT))
        );
        let time = ISO8601_DATE_TIME.try_from("2023-04-01T01:01:01Z")?;
        assert_eq!(
            "2023-04-01T01:01:01Z",
            time.format(&EXTENDED_DATE_TIME_FORMAT)
        );

        let time = UTCDateTime::try_from_values_f64(2023, 04, 01, 01, 01, 01.01)?;
        assert_eq!(
            "2023-04-01T01:01:01.010000000Z",
            format!("{}", time.format(&EXTENDED_DATE_TIME_FORMAT))
        );
        Ok(())
    }

    #[test]
    pub fn test_week_numbers() -> Result<(), GreaterThanEqualToValueError<u8>> {
        let test_cases = vec![
            (Date::try_from_values(1977, 01, 01)?, "1976W53"),
            (Date::try_from_values(1977, 01, 02)?, "1976W53"),
            (Date::try_from_values(1977, 01, 03)?, "1977W01"),
            (Date::try_from_values(2000, 01, 02)?, "1999W52"),
            (Date::try_from_values(2000, 01, 03)?, "2000W01"),
            (Date::try_from_values(2000, 03, 05)?, "2000W09"),
            (Date::try_from_values(2000, 03, 06)?, "2000W10"),
            (Date::try_from_values(2000, 10, 29)?, "2000W43"),
            (Date::try_from_values(2000, 10, 30)?, "2000W44"),
            (Date::try_from_values(2019, 12, 29)?, "2019W52"),
            (Date::try_from_values(2019, 12, 30)?, "2020W01"),
            (Date::try_from_values(2019, 12, 31)?, "2020W01"),
            (Date::try_from_values(2020, 01, 01)?, "2020W01"),
            (Date::try_from_values(2020, 01, 06)?, "2020W02"),
            (Date::try_from_values(2021, 03, 31)?, "2021W13"),
            (Date::try_from_values(2021, 04, 01)?, "2021W13"),
            (Date::try_from_values(2021, 04, 04)?, "2021W13"),
            (Date::try_from_values(2021, 04, 05)?, "2021W14"),
            (Date::try_from_values(2023, 04, 28)?, "2023W17"),
            (Date::try_from_values(2023, 10, 31)?, "2023W44"),
        ];
        for (d, e) in test_cases {
            assert_eq!(e, d.format(&ISO8601_WEEK_NUMBER));
        }
        Ok(())
    }
}
