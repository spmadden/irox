// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::marker::PhantomData;

use crate::time::{Date, Epoch, UnixTimestamp, COMMON_ERA_EPOCH, GREGORIAN_EPOCH, SECONDS_IN_DAY};

//
/// The Julian Epoch, 01-JAN 4713 BC
pub const JULIAN_EPOCH: Epoch = Epoch(Date {
    year: -4713,
    day_of_year: 1,
});

///
/// The Reduced Julian Epoch, 16-NOV-1858
///
/// 2400000 JD after the [`JULIAN_EPOCH`]
pub const REDUCED_JULIAN_EPOCH: Epoch = Epoch(Date {
    year: 1858,
    day_of_year: 320,
});

///
/// The Truncated Julian Epoch, used by NASA, 24-MAY-1968
///
/// 2440000.5 JD after the [`JULIAN_EPOCH`]
pub const TRUNCATED_JULIAN_EPOCH: Epoch = Epoch(Date {
    year: 1968,
    day_of_year: 145,
});

///
/// A Julian Date represents a number of days (86400 seconds) since a particular
/// Epoch.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct JulianDayNumber<T> {
    epoch: Epoch,
    day_number: f64,

    _phantom: PhantomData<T>,
}
impl<T> JulianDayNumber<T> {
    pub fn new(epoch: Epoch, day_number: f64) -> Self {
        JulianDayNumber {
            epoch,
            day_number,
            _phantom: Default::default(),
        }
    }
    pub fn get_day_number(&self) -> f64 {
        self.day_number
    }
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }
}

/// No functionality, used as a static compile-time type check
pub struct JulianEpoch;
/// No functionality, used as a static compile-time type check
pub struct ReducedJulianEpoch;
/// No functionality, used as a static compile-time type check
pub struct ModifiedJulianEpoch;
/// No functionality, used as a static compile-time type check
pub struct TruncatedJulianEpoch;
/// No functionality, used as a static compile-time type check
pub struct LilianEpoch;
/// No functionality, used as a static compile-time type check
pub struct RataDieEpoch;

///
/// The Julian Date is the number of days since the [`JULIAN_EPOCH`]
///
/// Noon 12:00 on 01-JAN, 4713 BC
pub type JulianDate = JulianDayNumber<JulianEpoch>;
pub const JULIAN_JD_OFFSET: f64 = 0.0_f64;

///
/// The Reduced Julian Date is the number of days since the [`REDUCED_JULIAN_EPOCH`]
/// or 2400000 days after the [`JULIAN_EPOCH`]
///
/// Noon 12:00 on 16-NOV-1858
pub type ReducedJulianDate = JulianDayNumber<ReducedJulianEpoch>;
/// The offset from the [`JULIAN_EPOCH`] for the [`ReducedJulianDate`]
pub const REDUCED_JD_OFFSET: f64 = 2400000_f64;

///
/// The Modified Julian Date shifts the Reduced Julian Date by 12 hours forward,
/// or 2400000.5 days after the [`JULIAN_EPOCH`]
///
/// Midnight on 17-NOV-1858
pub type ModifiedJulianDate = JulianDayNumber<ModifiedJulianEpoch>;
/// The offset from the [`JULIAN_EPOCH`] for the [`ModifiedJulianDate`]
pub const MODIFIED_JD_OFFSET: f64 = 2400000.5_f64;

///
/// The Truncated Julian Date uses the [`TRUNCATED_JULIAN_EPOCH`] as a round
/// offset of 2440000.5 after the [`JULIAN_EPOCH`]
///
/// Midnight on 24-MAY-1968
pub type TruncatedJulianDate = JulianDayNumber<TruncatedJulianEpoch>;
/// The offset from the [`JULIAN_EPOCH`] for the [`TruncatedJulianDate`]
pub const TRUNCATED_JD_OFFSET: f64 = 2440000.5_f64;

///
/// The Lilian Date is the day number offset from the [`GREGORIAN_EPOCH`],
/// 2299159.5 JD after the [`JULIAN_EPOCH`]
///
/// Midnight on 15-OCT-1582
pub type LilianDate = JulianDayNumber<LilianEpoch>;
/// The offset from the [`JULIAN_EPOCH`] for the [`LilianDate`]
pub const LILIAN_JD_OFFSET: f64 = 2299159.5_f64;

///
/// The Rata Die (Latin: "Fixed Date") is the fixed number of days in the Common
/// Era, since Midnight 01-01-0001 AD, 1721424.5 after [`JULIAN_EPOCH`]
pub type RataDieDate = JulianDayNumber<RataDieEpoch>;
/// The offset from the [`JULIAN_EPOCH`] for the [`RataDieDate`]
pub const RATA_DIE_JD_OFFSET: f64 = 1721424.5_f64;

/// The offset from the [`JULIAN_EPOCH`] for the [`UnixTimestamp`]
pub const UNIX_TS_JD_OFFSET: f64 = 2240587.5_f64;

macro_rules! impl_julian {
    ($date:ident,$epoch:ident,$offset:ident) => {
        impl From<JulianDate> for $date {
            fn from(value: JulianDate) -> Self {
                $date::new($epoch, value.day_number - $offset)
            }
        }
        impl From<$date> for JulianDate {
            fn from(value: $date) -> Self {
                JulianDate::new(JULIAN_EPOCH, value.day_number + $offset)
            }
        }
        impl From<&JulianDate> for $date {
            fn from(value: &JulianDate) -> Self {
                $date::new($epoch, value.day_number - $offset)
            }
        }
        impl From<&$date> for JulianDate {
            fn from(value: &$date) -> Self {
                JulianDate::new(JULIAN_EPOCH, value.day_number + $offset)
            }
        }
    };
}

impl From<UnixTimestamp> for JulianDate {
    fn from(value: UnixTimestamp) -> Self {
        let jd = value.get_offset().as_seconds_f64() / SECONDS_IN_DAY as f64 + UNIX_TS_JD_OFFSET;
        JulianDate::new(JULIAN_EPOCH, jd)
    }
}
impl From<JulianDate> for UnixTimestamp {
    fn from(value: JulianDate) -> Self {
        let ts = (value.day_number - UNIX_TS_JD_OFFSET) * SECONDS_IN_DAY as f64;
        UnixTimestamp::from_seconds_f64(ts)
    }
}

impl_julian!(ReducedJulianDate, REDUCED_JULIAN_EPOCH, REDUCED_JD_OFFSET);
impl_julian!(ModifiedJulianDate, REDUCED_JULIAN_EPOCH, MODIFIED_JD_OFFSET);
impl_julian!(
    TruncatedJulianDate,
    TRUNCATED_JULIAN_EPOCH,
    TRUNCATED_JD_OFFSET
);
impl_julian!(LilianDate, GREGORIAN_EPOCH, LILIAN_JD_OFFSET);
impl_julian!(RataDieDate, COMMON_ERA_EPOCH, RATA_DIE_JD_OFFSET);
