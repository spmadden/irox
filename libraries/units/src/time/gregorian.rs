// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_enums::{EnumIterItem, EnumName, EnumTryFromStr};

use crate::bounds::{OutsideRangeError, Range, WithinRange};

///
/// Gregorian Month enumeration
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, EnumName, EnumIterItem, EnumTryFromStr,
)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
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
    /// Returns the start day of the year for this month for the indicated year.
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
    /// Returns the end day of the year for this month for the indicated year.
    #[must_use]
    pub const fn end_day_of_year(&self, year: i32) -> u16 {
        if is_leap_year(year) {
            match self {
                Month::January => 31,
                Month::February => 60,
                Month::March => 91,
                Month::April => 121,
                Month::May => 152,
                Month::June => 182,
                Month::July => 213,
                Month::August => 244,
                Month::September => 274,
                Month::October => 305,
                Month::November => 335,
                Month::December => 366,
            }
        } else {
            match self {
                Month::January => 31,
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
        }
    }

    ///
    /// Returns a range verifier to check if the indicate day number is valid for
    /// the particular month and year
    pub const fn valid_day_number(&self, year: i32) -> WithinRange<u8> {
        let upper_lim = self.days_in_month(year);
        WithinRange::new(0, upper_lim + 1)
    }
}

impl TryFrom<u8> for Month {
    type Error = OutsideRangeError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let range = WithinRange::new(0, 13);
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Date {
    pub(crate) year: i32,
    pub(crate) day_of_year: u16,
}

impl Date {
    ///
    /// Constructs a new date given the provided gregorian year and day of year.
    /// Returns `Err(OutsideRangeErr)` if `day_of_year` is outside the valid range.
    pub fn new(year: i32, day_of_year: u16) -> Result<Date, OutsideRangeError<u16>> {
        let valid_num_days = if is_leap_year(year) { 366 } else { 365 };

        WithinRange::new(0, valid_num_days + 1).check_value_is_valid(&day_of_year)?;

        Ok(Date { year, day_of_year })
    }

    ///
    /// Constructs a new date given the provided values.  If month or day is out
    /// of range, will return `Err(OutsideRangeError)`
    pub fn try_from_values(year: i32, month: u8, day: u8) -> Result<Date, OutsideRangeError<u8>> {
        let month: Month = month.try_into()?;
        month.valid_day_number(year).check_value_is_valid(&day)?;
        let day_of_year = month.start_day_of_year(year) + day as u16;
        Ok(Date { year, day_of_year })
    }

    ///
    /// Returns the gregorian year this date represents
    #[must_use]
    pub fn year(&self) -> i32 {
        self.year
    }

    ///
    /// Returns the day of the year.  January 1 is '1', January 31 is '31',
    /// February 1 is '32', and so on.
    #[must_use]
    pub fn day_of_year(&self) -> u16 {
        self.day_of_year
    }

    /// Adds the specified number of days to this date.
    #[must_use]
    pub fn add_days(&self, days: u16) -> Date {
        let mut days_remaining = days;
        let mut years = self.year;
        let mut days = self.day_of_year - 1;

        loop {
            let days_in_year = days_in_year(years);
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
            day_of_year: days + 1,
        }
    }

    /// Subtracts the given number of days from this date
    #[must_use]
    pub fn sub_days(&self, days: u16) -> Date {
        let mut days_remaining = days;
        let mut years = self.year;
        let mut days = self.day_of_year - 1;

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
            day_of_year: days + 1,
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

#[cfg(test)]
mod tests {
    use crate::time::gregorian::is_leap_year;

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
}
