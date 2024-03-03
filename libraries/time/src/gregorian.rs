// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Contains [`Date`] and associated elements to represent a Proleptic Gregorian Date.
//!

use core::fmt::{Display, Formatter};
use core::ops::{Add, AddAssign, Sub};

use irox_enums::{EnumIterItem, EnumName, EnumTryFromStr};
use irox_units::bounds::{GreaterThanEqualToValueError, LessThanValue, Range};
use irox_units::units::duration::{Duration, DurationUnit};

use crate::epoch::{UnixTimestamp, UNIX_EPOCH};
use crate::format::iso8601::ExtendedDateFormat;
use crate::format::{Format, FormatError, FormatParser};
use crate::julian::{JulianDate, JulianDayNumber, PrimeDate, JULIAN_EPOCH};
use crate::SECONDS_IN_DAY;

extern crate alloc;
pub use alloc::string::String;

/// Days per 4 Year Window
///
/// Each window has 1 extra leap day in it
pub const DAYS_PER_4YEAR: u32 = 365 * 4 + 1;

/// Days per 100 Year Window
///
/// There are 25x 4Y Windows in a 100Y Window, except the 100th isn't a Leap.
pub const DAYS_PER_100YEAR: u32 = DAYS_PER_4YEAR * 25 - 1;

///
/// Days per 400 Year Window
///
/// There are 4x 100Y Windows in a 400Y Window, with an extra leap day
pub const DAYS_PER_400YEAR: u32 = DAYS_PER_100YEAR * 4 + 1;

///
/// Gregorian Month enumeration
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, EnumName, EnumIterItem, EnumTryFromStr,
)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Display for Month {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.name()))
    }
}

impl Month {
    ///
    /// Returns the total number of days in the month for the indicated gregorian
    /// year.
    #[allow(clippy::match_same_arms)]
    pub const fn days_in_month(&self, year: i32) -> u8 {
        match self {
            Month::January => 31,
            Month::February => {
                if is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    ///
    /// Returns the start day of the year for this month for the indicated year, zero-based 01-JAN is 00.
    #[must_use]
    pub const fn start_day_of_year(&self, year: i32) -> u16 {
        if is_leap_year(year) {
            match self {
                Month::January => 0,
                Month::February => 31,
                Month::March => 60,
                Month::April => 91,
                Month::May => 121,
                Month::June => 152,
                Month::July => 182,
                Month::August => 213,
                Month::September => 244,
                Month::October => 274,
                Month::November => 305,
                Month::December => 335,
            }
        } else {
            match self {
                Month::January => 0,
                Month::February => 31,
                Month::March => 59,
                Month::April => 90,
                Month::May => 120,
                Month::June => 151,
                Month::July => 181,
                Month::August => 212,
                Month::September => 243,
                Month::October => 273,
                Month::November => 304,
                Month::December => 334,
            }
        }
    }

    ///
    /// Returns the end day of the year for this month for the indicated year, zero-based, JAN-31 is 30.
    #[must_use]
    pub const fn end_day_of_year(&self, year: i32) -> u16 {
        if is_leap_year(year) {
            match self {
                Month::January => 30,
                Month::February => 59,
                Month::March => 90,
                Month::April => 120,
                Month::May => 151,
                Month::June => 181,
                Month::July => 212,
                Month::August => 243,
                Month::September => 273,
                Month::October => 304,
                Month::November => 334,
                Month::December => 365,
            }
        } else {
            match self {
                Month::January => 30,
                Month::February => 58,
                Month::March => 89,
                Month::April => 119,
                Month::May => 150,
                Month::June => 180,
                Month::July => 211,
                Month::August => 242,
                Month::September => 272,
                Month::October => 303,
                Month::November => 333,
                Month::December => 364,
            }
        }
    }

    ///
    /// Returns a range verifier to check if the indicate day number is valid for
    /// the particular month and year
    #[must_use]
    pub const fn valid_day_number(&self, year: i32) -> LessThanValue<u8> {
        let upper_lim = self.days_in_month(year);
        LessThanValue::new(upper_lim + 1)
    }

    ///
    /// Returns true if the indicate day of the year is within this month
    #[must_use]
    pub const fn day_is_within_month(&self, year: i32, day_of_year: u16) -> bool {
        day_of_year >= self.start_day_of_year(year) && day_of_year <= self.end_day_of_year(year)
    }

    ///
    /// Returns true if the indicated date is within this month
    #[must_use]
    pub const fn date_is_within_month(&self, date: &Date) -> bool {
        self.day_is_within_month(date.year, date.day_of_year)
    }
}

impl TryFrom<u8> for Month {
    type Error = GreaterThanEqualToValueError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let range = LessThanValue::new(13);
        Ok(match value {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            e => return Self::Error::err(e, range),
        })
    }
}

///
/// Gregorian Date - a specific date on a calendar.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Date {
    ///
    /// Year is the Proleptic Gregorian Year
    pub(crate) year: i32,

    ///
    /// Day of Year is the day index into the specified year, range [0, 366)
    pub(crate) day_of_year: u16,
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", ExtendedDateFormat.format(self))
    }
}

impl Date {
    ///
    /// Constructs a new date given the provided gregorian year and day of year.
    /// Returns `Err(GreaterThanEqualToValueError)` if `day_of_year` is outside the valid range.
    pub fn new(year: i32, day_of_year: u16) -> Result<Date, GreaterThanEqualToValueError<u16>> {
        let valid_num_days = if is_leap_year(year) { 366 } else { 365 };

        LessThanValue::new(valid_num_days).check_value_is_valid(&day_of_year)?;

        Ok(Date { year, day_of_year })
    }

    ///
    /// Constructs a new date given the provided values.  If month or day is out
    /// of range, will return `Err(OutsideRangeError)`.
    ///
    /// Note: The 'day' parameter here is in the range of `1..=31` as would be shown on a calendar
    pub fn try_from_values(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date, GreaterThanEqualToValueError<u8>> {
        let month: Month = month.try_into()?;
        Self::try_from(year, month, day)
    }

    ///
    /// Constructs a new date given the provided values, if day is out of range,
    /// will return `Err(OutsideRangeError)`
    ///
    /// Note: The 'day' parameter here is in the range of `1..=31` as would be shown on a calendar
    pub fn try_from(
        year: i32,
        month: Month,
        day: u8,
    ) -> Result<Date, GreaterThanEqualToValueError<u8>> {
        month.valid_day_number(year).check_value_is_valid(&day)?;
        let day_of_year = (month.start_day_of_year(year) + day as u16) - 1;
        Ok(Date { year, day_of_year })
    }

    ///
    /// Returns the gregorian year this date represents
    #[must_use]
    pub fn year(&self) -> i32 {
        self.year
    }

    ///
    /// Returns the day offset into the year.  January 1 is '0', January 31 is '30',
    /// February 1 is '31', and so on.
    #[must_use]
    pub fn day_of_year_offset(&self) -> u16 {
        self.day_of_year
    }

    ///
    /// Returns the calendar day of the year.  January 1 is '1', January 31 is '31',
    /// February 1 is '32', and so on.
    #[must_use]
    pub fn day_of_year(&self) -> u16 {
        self.day_of_year + 1
    }

    ///
    /// Returns the [`Month`] this date is contained within
    #[must_use]
    pub fn month_of_year(&self) -> Month {
        for month in Month::iter_items() {
            if month.date_is_within_month(self) {
                return month;
            }
        }

        // TODO: Proper normalization here.
        Month::January
    }

    ///
    /// Returns the day index into the current month in the range [0,31)
    #[must_use]
    pub fn day_of_month(&self) -> u8 {
        (self.day_of_year - self.month_of_year().start_day_of_year(self.year)) as u8
    }

    /// Adds the specified number of days to this date.
    #[must_use]
    pub fn add_days(&self, days: u32) -> Date {
        let mut days_remaining = days;
        let mut years = self.year;
        let mut days = self.day_of_year as u32;

        loop {
            let days_in_year = days_in_year(years) as u32;
            if days + days_remaining >= days_in_year {
                years += 1;
                days_remaining -= days_in_year - days;
                days = 0;
                continue;
            }
            days += days_remaining;
            break;
        }
        Date {
            year: years,
            day_of_year: days as u16,
        }
    }

    /// Subtracts the given number of days from this date
    #[must_use]
    pub fn sub_days(&self, days: u16) -> Date {
        let mut days_remaining = days;
        let mut years = self.year;
        let mut days = self.day_of_year;

        loop {
            if days_remaining > days {
                years -= 1;
                let days_in_year = days_in_year(years);
                days_remaining -= days;
                days += days_in_year;
                continue;
            }
            days -= days_remaining;
            break;
        }
        Date {
            year: years,
            day_of_year: days,
        }
    }

    /// Adds the specified number of years to this date.
    #[must_use]
    pub fn add_years(&self, years: u16) -> Date {
        Date {
            year: self.year + years as i32,
            day_of_year: self.day_of_year,
        }
    }

    /// Subtracts the specified number of years from this date.
    #[must_use]
    pub fn sub_years(&self, years: u16) -> Date {
        Date {
            year: self.year - years as i32,
            day_of_year: self.day_of_year,
        }
    }

    ///
    /// Returns the [`UnixTimestamp`] of this Date
    #[must_use]
    pub fn as_unix_timestamp(&self) -> UnixTimestamp {
        self.into()
    }

    ///
    /// Returns the [`JulianDate`] of this date
    #[must_use]
    pub fn as_julian_day(&self) -> JulianDate {
        self.into()
    }

    ///
    /// Formats this date using the specified formatter
    #[must_use]
    pub fn format<F: Format<Self>>(&self, format: &F) -> String {
        format.format(self)
    }

    ///
    /// Attempts to parse a date from the string using the specified formatter
    pub fn parse_from<F: FormatParser<Self>>(
        format: &F,
        string: &str,
    ) -> Result<Self, FormatError> {
        format.try_from(string)
    }

    ///
    /// Returns the day-of-the-week name of this date, using ISO8601 convention that the week starts on Monday.
    pub fn day_of_week(&self) -> DayOfWeek {
        let prime: PrimeDate = self.as_julian_day().into();
        let dow = prime.get_day_number() as i32 % 7;
        match dow {
            1 => DayOfWeek::Tuesday,
            2 => DayOfWeek::Wednesday,
            3 => DayOfWeek::Thursday,
            4 => DayOfWeek::Friday,
            5 => DayOfWeek::Saturday,
            6 => DayOfWeek::Sunday,
            _ => DayOfWeek::Monday,
        }
    }

    ///
    /// Returns a pair (year number, week of year)
    pub fn week_number(&self) -> (i32, u8) {
        let jan01 = Date {
            year: self.year,
            day_of_year: 0,
        };
        let dow = self.day_of_week() as i32;
        let wkno = (9 + self.day_of_year as i32 - dow) / 7;

        if wkno == 0 {
            let dow = jan01.day_of_week() as i32;
            if (0..=3).contains(&dow) {
                return (self.year, 1);
            }
            let year = self.year - 1;
            if dow == 4 {
                // friday, always W53 of prev year
                return (year, 53);
            }
            if dow == 5 {
                // saturday, W53 in leaps, W52 otherwise.
                let prev_yr_starts_on = Date {
                    year,
                    day_of_year: 0,
                }
                .day_of_week();
                if is_leap_year(year) && prev_yr_starts_on == DayOfWeek::Thursday {
                    return (year, 53);
                }
            }
            // sunday, always W52
            return (year, 52);
        }
        if wkno == 53 {
            // only actually 53 if is a long year
            if is_long_year(self.year) {
                return (self.year, 53);
            }
            // otherwise is Year 01 of the following year.
            return (self.year + 1, 1);
        }

        (self.year, wkno as u8)
    }
}

///
/// Day of the week enumeration, following ISO8601 convention of "Monday is the start of the week"
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, EnumName, EnumIterItem, EnumTryFromStr,
)]
pub enum DayOfWeek {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

impl TryFrom<u8> for DayOfWeek {
    type Error = GreaterThanEqualToValueError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => DayOfWeek::Monday,
            1 => DayOfWeek::Tuesday,
            2 => DayOfWeek::Wednesday,
            3 => DayOfWeek::Thursday,
            4 => DayOfWeek::Friday,
            5 => DayOfWeek::Saturday,
            6 => DayOfWeek::Sunday,
            e => return GreaterThanEqualToValueError::err(e, LessThanValue::new(8)),
        })
    }
}

///
/// Returns true if the indicated year is a ISO8601 "Long Year" with 53 Weeks in it.
pub fn is_long_year(year: i32) -> bool {
    let start_day = Date {
        year,
        day_of_year: 0,
    }
    .day_of_week();
    let is_leap = is_leap_year(year);
    if start_day == DayOfWeek::Thursday {
        return true;
    }
    if start_day == DayOfWeek::Wednesday && is_leap {
        return true;
    }
    false
}

///
/// Checks if a gregorian year is considered a "leap year"
///
/// Every year that is exactly divisible by four is a leap year, except for
/// years that are exactly divisible by 100, but these centurial years are leap
/// years if they are exactly divisible by 400. For example, the years 1700,
/// 1800, and 1900 are not leap years, but the years 1600 and 2000 are.
///
/// `365 + 1/4 - 1/100 + 1/400 = 365 + 97/400 = 365.2425`
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/Leap_year#Gregorian_calendar)
pub const fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    year % 4 == 0
}

///
/// Returns the total number of days in the indicated calendar year
pub const fn days_in_year(year: i32) -> u16 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

///
/// Returns the total number of seconds in the indicated calendar year
pub const fn seconds_in_year(year: i32) -> u32 {
    days_in_year(year) as u32 * SECONDS_IN_DAY
}

impl From<&Date> for UnixTimestamp {
    fn from(value: &Date) -> Self {
        let years_duration = value.year - UNIX_EPOCH.0.year;
        if years_duration < 0 {
            return UnixTimestamp::default();
        }
        let mut secs_duration: u64 = value.day_of_year as u64 * SECONDS_IN_DAY as u64;
        for year in UNIX_EPOCH.0.year..value.year {
            secs_duration += seconds_in_year(year) as u64;
        }

        UnixTimestamp::from_seconds_f64(secs_duration as f64)
    }
}

/// 01-MAR-2000, a mod400 year after the leap day.
const LEAPOCH: UnixTimestamp = UnixTimestamp::from_seconds(951868800);

impl From<&UnixTimestamp> for Date {
    #[allow(clippy::integer_division)]
    fn from(value: &UnixTimestamp) -> Self {
        // Algorithm impl based on libmusl __secs_to_tm.c
        let sec_in_day = SECONDS_IN_DAY as i64;
        let leapoch = LEAPOCH.get_offset().as_seconds() as i64;
        let offset = value.get_offset().as_seconds() as i64;

        // clever impl - the leapoch is a nice round 400 cycle leap year
        // so we compute the negative offset (for dates before the leapoch)
        // and the positive offset (for dates after the leapoch).  This ensures
        // that the "day calculation" is always 0 aligned to a round leap cycle.
        // Therefore, there's no weird offsets and calculations and even division.
        let seconds = offset - leapoch;
        let mut days = seconds / sec_in_day;
        if seconds % sec_in_day < 0 {
            days -= 1;
        }

        // compute the number of 400 leap year (qc) cycles
        let mut qc_cycles = days / DAYS_PER_400YEAR as i64;
        let mut rem_days = days % DAYS_PER_400YEAR as i64;
        if rem_days < 0 {
            rem_days += DAYS_PER_400YEAR as i64;
            qc_cycles -= 1;
        }

        // compute the remaining number of 100 non-leap year (c) cycles
        let mut c_cycles = rem_days / DAYS_PER_100YEAR as i64;
        if c_cycles == 4 {
            c_cycles -= 1;
        }
        rem_days -= c_cycles * DAYS_PER_100YEAR as i64;

        // compute the remaining number 4 leap year (q) cycles
        let mut q_cycles = rem_days / DAYS_PER_4YEAR as i64;
        if q_cycles == 25 {
            q_cycles -= 1;
        }
        rem_days -= q_cycles * DAYS_PER_4YEAR as i64;

        // and lastly, the number of non-leap years
        let mut rem_years = rem_days / 365;
        if rem_years == 4 {
            rem_years -= 1;
        }
        rem_days -= rem_years * 365;

        let mut year = rem_years + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles + 2000;
        let mut yday = rem_days + 31 + 28; // because the LEAPOCH is 1-MAR
        if is_leap_year(year as i32) {
            if yday + 1 > 365 {
                year += 1;
                yday -= 366;
            }
            yday += 1;
        } else if yday > 364 {
            year += 1;
            yday -= 365;
        }

        Date {
            year: year as i32,
            day_of_year: yday as u16,
        }
    }
}

const JULIAN_DAY_1_JAN_YR0: f64 = 1721059.5;

impl From<&Date> for JulianDate {
    #[allow(clippy::integer_division)]
    fn from(value: &Date) -> Self {
        let mut years = value.year - 1;
        let qc_years = years / 400;
        years -= qc_years * 400;
        let c_years = years / 100;
        years -= c_years * 100;
        let q_years = years / 4 + 1;
        let leap_days = qc_years * 97 + c_years * 24 + q_years;

        let duration_days = value.year * 365 + leap_days + value.day_of_year as i32;

        let duration_days = duration_days as f64 + JULIAN_DAY_1_JAN_YR0;
        JulianDayNumber::new(JULIAN_EPOCH, duration_days)
    }
}

impl From<Date> for JulianDate {
    fn from(value: Date) -> Self {
        From::<&Date>::from(&value)
    }
}

impl From<&JulianDate> for Date {
    fn from(value: &JulianDate) -> Self {
        value.get_epoch().0 + Duration::new(value.get_day_number(), DurationUnit::Day)
    }
}

impl From<JulianDate> for Date {
    fn from(value: JulianDate) -> Self {
        From::<&JulianDate>::from(&value)
    }
}

impl Sub<&Date> for Date {
    type Output = Duration;

    fn sub(self, rhs: &Date) -> Self::Output {
        if self == *rhs {
            return Duration::new_seconds(0.0);
        }
        let duration = self.as_julian_day().get_day_number() - rhs.as_julian_day().get_day_number();
        Duration::new(duration, DurationUnit::Day)
    }
}

impl Sub<Date> for Date {
    type Output = Duration;

    fn sub(self, rhs: Date) -> Self::Output {
        if self == rhs {
            return Duration::new_seconds(0.0);
        }
        let duration = self.as_julian_day().get_day_number() - rhs.as_julian_day().get_day_number();
        Duration::new(duration, DurationUnit::Day)
    }
}

impl Add<&mut Duration> for Date {
    type Output = Date;

    fn add(self, rhs: &mut Duration) -> Self::Output {
        let days = rhs.as_days();
        self.add_days(days as u32)
    }
}

impl Add<&Duration> for Date {
    type Output = Date;

    fn add(self, rhs: &Duration) -> Self::Output {
        let days = rhs.as_days();
        self.add_days(days as u32)
    }
}

impl Add<Duration> for Date {
    type Output = Date;

    fn add(self, rhs: Duration) -> Self::Output {
        let days = rhs.as_days();
        self.add_days(days as u32)
    }
}

impl AddAssign<Duration> for Date {
    fn add_assign(&mut self, rhs: Duration) {
        let date = *self + rhs;
        self.year = date.year;
        self.day_of_year = date.day_of_year;
    }
}
impl AddAssign<&Duration> for Date {
    fn add_assign(&mut self, rhs: &Duration) {
        let date = *self + *rhs;
        self.year = date.year;
        self.day_of_year = date.day_of_year;
    }
}

#[cfg(test)]
mod tests {
    use irox_enums::EnumIterItem;
    use irox_units::bounds::GreaterThanEqualToValueError;

    use crate::epoch::{UnixTimestamp, GPS_EPOCH, PRIME_EPOCH, UNIX_EPOCH};
    use crate::gregorian::{is_leap_year, Date, Month};

    #[test]
    pub fn leap_year_test() {
        assert!(is_leap_year(1996));
        assert!(!is_leap_year(1997));
        assert!(!is_leap_year(1998));
        assert!(!is_leap_year(1999));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(2001));
        assert!(!is_leap_year(2002));
        assert!(!is_leap_year(2003));
        assert!(is_leap_year(2004));
        assert!(is_leap_year(2008));
        assert!(is_leap_year(2012));
        assert!(is_leap_year(1600));
        assert!(!is_leap_year(1700));
        assert!(!is_leap_year(1800));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));
    }

    #[test]
    pub fn test_timestamp_to_date() -> Result<(), GreaterThanEqualToValueError<u16>> {
        assert_eq!(
            UnixTimestamp::from_seconds(1697299822).as_date(),
            Date::new(2023, 286)?
        );

        assert_eq!(
            UnixTimestamp::from_seconds(0).as_date(),
            Date::new(1970, 0)?
        );
        assert_eq!(
            UnixTimestamp::from_seconds(915148800).as_date(),
            Date::new(1999, 0)?
        );
        assert_eq!(
            UnixTimestamp::from_seconds(1095379200).as_date(),
            Date::try_from(2004, Month::September, 17).unwrap()
        );
        Ok(())
    }

    #[test]
    pub fn test_date_subtract() {
        assert_eq!(
            3657.0,
            (GPS_EPOCH.get_gregorian_date() - UNIX_EPOCH.get_gregorian_date()).value()
        );
        assert_eq!(
            0.0,
            (GPS_EPOCH.get_gregorian_date() - GPS_EPOCH.get_gregorian_date()).value()
        );
        assert_eq!(
            -3657.0,
            (UNIX_EPOCH.get_gregorian_date() - GPS_EPOCH.get_gregorian_date()).value()
        );
        assert_eq!(
            25567.0,
            (UNIX_EPOCH.get_gregorian_date() - PRIME_EPOCH.get_gregorian_date()).value()
        );
        assert_eq!(
            -25567.0,
            (PRIME_EPOCH.get_gregorian_date() - UNIX_EPOCH.get_gregorian_date()).value()
        );
    }

    #[test]
    pub fn test_date_add() {
        assert_eq!(
            GPS_EPOCH.get_gregorian_date(),
            UNIX_EPOCH.get_gregorian_date().add_days(3657)
        );
    }

    #[test]
    #[ignore]
    pub fn test_print_year() -> Result<(), GreaterThanEqualToValueError<u8>> {
        let year = 2019;
        for month in Month::iter_items() {
            for day in 1..=month.days_in_month(year) {
                let date = Date::try_from(year, month, day)?;
                println!(
                    "{month} {day} {year}-{} {}",
                    date.day_of_year,
                    date.as_julian_day().get_day_number()
                );
            }
        }

        Ok(())
    }

    #[test]
    #[ignore]
    pub fn test_print_leap_days() {
        let mut year = 0;
        let mut leaps = 0;
        loop {
            println!("{year} : {leaps}");
            if year > 2100 {
                break;
            }
            if is_leap_year(year) {
                leaps += 1;
            }
            let mut sum_leaps = 0;
            for y in 0..year {
                if is_leap_year(y) {
                    sum_leaps += 1;
                }
            }
            println!("sum: {year} {sum_leaps}");
            year += 4;
        }
    }

    #[test]
    pub fn test_julian_day() -> Result<(), GreaterThanEqualToValueError<u8>> {
        assert_eq!(
            Date::try_from_values(1970, 1, 1)?
                .as_julian_day()
                .get_day_number(),
            2440587.5
        );
        assert_eq!(
            Date::try_from_values(1980, 1, 6)?
                .as_julian_day()
                .get_day_number(),
            2444244.5
        );
        assert_eq!(
            Date::try_from_values(1858, 11, 17)?
                .as_julian_day()
                .get_day_number(),
            2400000.5
        );
        assert_eq!(
            Date::try_from_values(1950, 1, 1)?
                .as_julian_day()
                .get_day_number(),
            2433282.5
        );
        assert_eq!(
            Date::try_from_values(2000, 3, 1)?
                .as_julian_day()
                .get_day_number(),
            2451604.5
        );
        assert_eq!(
            Date::try_from_values(2020, 1, 1)?
                .as_julian_day()
                .get_day_number(),
            2458849.5
        );
        assert_eq!(
            Date::try_from_values(2020, 10, 1)?
                .as_julian_day()
                .get_day_number(),
            2459123.5
        );

        assert_eq!(
            Date::try_from_values(2021, 12, 31)?
                .as_julian_day()
                .get_day_number(),
            2459579.5
        );
        assert_eq!(
            Date::try_from_values(2022, 1, 1)?
                .as_julian_day()
                .get_day_number(),
            2459580.5
        );
        assert_eq!(
            Date::try_from_values(2022, 10, 1)?
                .as_julian_day()
                .get_day_number(),
            2459853.5
        );
        assert_eq!(
            Date::try_from_values(2023, 10, 18)?
                .as_julian_day()
                .get_day_number(),
            2460235.5
        );

        Ok(())
    }

    #[test]
    pub fn test_day_of_year() -> Result<(), GreaterThanEqualToValueError<u8>> {
        let date = Date {
            year: 2021,
            day_of_year: 90,
        };
        assert_eq!("2021-04-01", date.to_string());
        let date = Date {
            year: 2021,
            day_of_year: 91,
        };
        assert_eq!("2021-04-02", date.to_string());
        Ok(())
    }
}
