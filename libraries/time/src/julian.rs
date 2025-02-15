// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Contains [`JulianDate`] and others - ways of measuring a discrete amount of days from a specific
//! Julian [`Epoch`]
//!

use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use irox_units::units::duration::{Duration, DurationUnit};

use crate::epoch::{PrimeEpoch, PRIME_EPOCH, UNIX_EPOCH, VICINTIPOCH, Y2K_EPOCH};
use crate::{
    epoch::{Epoch, UnixTimestamp, COMMON_ERA_EPOCH, GREGORIAN_EPOCH},
    gregorian::Date,
    SECONDS_IN_DAY,
};

//
/// The Julian Epoch, 01-JAN 4713 BC (Gregorian)
pub const JULIAN_EPOCH: JulianEpoch = JulianEpoch::new(
    JULIAN_JD_OFFSET,
    Epoch(Date {
        year: -4712,
        day_of_year: 0,
    }),
);
///
/// The Julian Date is the number of days since the [`JULIAN_EPOCH`]
///
/// Noon 12:00 on 01-JAN, 4713 BC
pub type JulianDate = JulianDayNumber<JulianEpoch>;
impl JulianDate {
    pub const fn new(day_number: f64) -> Self {
        Self {
            epoch: JULIAN_EPOCH,
            day_number,
            _phantom: PhantomData,
        }
    }
}

pub const JULIAN_JD_OFFSET: f64 = 0.0_f64;

pub trait AsJulianEpoch {
    fn get_julian_epoch() -> JulianEpoch;
}

///
/// A Julian Date represents a number of days (86400 seconds) since a particular
/// Epoch.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct JulianDayNumber<T: AsJulianEpoch> {
    epoch: JulianEpoch,
    day_number: f64,

    _phantom: PhantomData<T>,
}

impl<T: AsJulianEpoch> JulianDayNumber<T> {
    pub(crate) const fn new_internal(epoch: JulianEpoch, day_number: f64) -> Self {
        JulianDayNumber {
            epoch,
            day_number,
            _phantom: PhantomData,
        }
    }
    /// Returns the julian day number (number of days since the epoch) of this date
    pub const fn get_day_number(&self) -> f64 {
        self.day_number
    }
    /// Returns the epoch of this date
    pub const fn get_julian_epoch(&self) -> JulianEpoch {
        self.epoch
    }

    /// Converts the specified JDN into the local JDN Epoch
    pub fn from_jdn<O: AsJulianEpoch>(other: &JulianDayNumber<O>) -> Self {
        let jdn = other.as_julian_date();
        let je = T::get_julian_epoch();
        Self {
            epoch: je,
            day_number: jdn.day_number - je.day_offset_from_julian_epoch,
            _phantom: PhantomData,
        }
    }

    /// Returns the julian date equivalient of this JDN
    pub const fn as_julian_date(&self) -> JulianDate {
        JulianDate::new_internal(
            JULIAN_EPOCH,
            self.day_number + self.epoch.day_offset_from_julian_epoch,
        )
    }
    /// Returns the [`ReducedJulianDate`] equivalent of this JDN
    pub fn as_reduced_date(&self) -> ReducedJulianDate {
        ReducedJulianDate::from_jdn(self)
    }
    pub fn as_modified_date(&self) -> ModifiedJulianDate {
        ModifiedJulianDate::from_jdn(self)
    }
    pub fn as_truncated_date(&self) -> TruncatedJulianDate {
        TruncatedJulianDate::from_jdn(self)
    }
    pub fn as_lilian_date(&self) -> LilianDate {
        LilianDate::from_jdn(self)
    }
    pub fn as_rata_die_date(&self) -> RataDieDate {
        RataDieDate::from_jdn(self)
    }
    pub fn as_prime_date(&self) -> PrimeDate {
        PrimeDate::from_jdn(self)
    }
}

/// No functionality, used as a static compile-time type check
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct JulianEpoch {
    day_offset_from_julian_epoch: f64,
    epoch: Epoch,
}
impl AsJulianEpoch for JulianEpoch {
    fn get_julian_epoch() -> JulianEpoch {
        JULIAN_EPOCH
    }
}
impl Eq for JulianEpoch {}
impl JulianEpoch {
    pub const fn new(day_offset_from_julian_epoch: f64, epoch: Epoch) -> Self {
        JulianEpoch {
            day_offset_from_julian_epoch,
            epoch,
        }
    }
    pub fn get_day_offset_from_julian_epoch(&self) -> f64 {
        self.day_offset_from_julian_epoch
    }
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }
}

macro_rules! impl_julian {
    ($date:ident,$epoch:ident) => {
        impl $date {
            pub const fn new(day_number: f64) -> Self {
                Self {
                    epoch: $epoch,
                    day_number,
                    _phantom: PhantomData,
                }
            }
        }
        // impl From<JulianDate> for $date {
        //     fn from(value: JulianDate) -> Self {
        //         $date::new(value.day_number - $epoch.day_offset_from_julian_epoch)
        //     }
        // }
        impl From<$date> for JulianDate {
            fn from(value: $date) -> Self {
                JulianDate::new(value.day_number + $epoch.day_offset_from_julian_epoch)
            }
        }
        impl From<&JulianDate> for $date {
            fn from(value: &JulianDate) -> Self {
                $date::new(value.day_number - $epoch.day_offset_from_julian_epoch)
            }
        }
        impl From<&$date> for JulianDate {
            fn from(value: &$date) -> Self {
                JulianDate::new(value.day_number + $epoch.day_offset_from_julian_epoch)
            }
        }
    };
}

/// No functionality, used as a static compile-time type check
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ReducedJulianEpoch;
impl AsJulianEpoch for ReducedJulianEpoch {
    fn get_julian_epoch() -> JulianEpoch {
        REDUCED_JULIAN_EPOCH
    }
}
///
/// The Reduced Julian Epoch, 16-NOV-1858
///
/// 2400000 JD after the [`JULIAN_EPOCH`]
pub const REDUCED_JULIAN_EPOCH: JulianEpoch = JulianEpoch::new(
    REDUCED_JD_OFFSET,
    Epoch(Date {
        year: 1858,
        day_of_year: 320,
    }),
);

///
/// The Reduced Julian Date is the number of days since the [`REDUCED_JULIAN_EPOCH`]
/// or 2400000 days after the [`JULIAN_EPOCH`]
///
/// Noon 12:00 on 16-NOV-1858
pub type ReducedJulianDate = JulianDayNumber<ReducedJulianEpoch>;

/// The offset from the [`JULIAN_EPOCH`] for the [`ReducedJulianDate`]
pub const REDUCED_JD_OFFSET: f64 = 2400000_f64;
impl_julian!(ReducedJulianDate, REDUCED_JULIAN_EPOCH);

/// No functionality, used as a static compile-time type check
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ModifiedJulianEpoch;
impl AsJulianEpoch for ModifiedJulianEpoch {
    fn get_julian_epoch() -> JulianEpoch {
        MODIFIED_JULIAN_EPOCH
    }
}

pub const MODIFIED_JULIAN_EPOCH: JulianEpoch = JulianEpoch::new(
    MODIFIED_JD_OFFSET,
    Epoch(Date {
        year: 1858,
        day_of_year: 320,
    }),
);

///
/// The Modified Julian Date shifts the Reduced Julian Date by 12 hours forward,
/// or 2400000.5 days after the [`JULIAN_EPOCH`]
///
/// Midnight on 17-NOV-1858
pub type ModifiedJulianDate = JulianDayNumber<ModifiedJulianEpoch>;

/// The offset from the [`JULIAN_EPOCH`] for the [`ModifiedJulianDate`]
pub const MODIFIED_JD_OFFSET: f64 = 2400000.5_f64;
impl_julian!(ModifiedJulianDate, MODIFIED_JULIAN_EPOCH);

/// No functionality, used as a static compile-time type check
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct TruncatedJulianEpoch;
impl AsJulianEpoch for TruncatedJulianEpoch {
    fn get_julian_epoch() -> JulianEpoch {
        TRUNCATED_JULIAN_EPOCH
    }
}
///
/// The Truncated Julian Epoch, used by NASA, 24-MAY-1968
///
/// 2440000.5 JD after the [`JULIAN_EPOCH`]
pub const TRUNCATED_JULIAN_EPOCH: JulianEpoch = JulianEpoch::new(
    TRUNCATED_JD_OFFSET,
    Epoch(Date {
        year: 1968,
        day_of_year: 145,
    }),
);

///
/// The Truncated Julian Date uses the [`TRUNCATED_JULIAN_EPOCH`] as a round
/// offset of 2440000.5 after the [`JULIAN_EPOCH`]
///
/// Midnight on 24-MAY-1968
pub type TruncatedJulianDate = JulianDayNumber<TruncatedJulianEpoch>;

/// The offset from the [`JULIAN_EPOCH`] for the [`TruncatedJulianDate`]
pub const TRUNCATED_JD_OFFSET: f64 = 2440000.5_f64;
impl_julian!(TruncatedJulianDate, TRUNCATED_JULIAN_EPOCH);

/// No functionality, used as a static compile-time type check
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct LilianEpoch;
impl AsJulianEpoch for LilianEpoch {
    fn get_julian_epoch() -> JulianEpoch {
        LILIAN_EPOCH
    }
}

///
/// The Lilian Date is the day number offset from the [`GREGORIAN_EPOCH`],
/// 2299159.5 JD after the [`JULIAN_EPOCH`]
///
/// Midnight on 15-OCT-1582
pub type LilianDate = JulianDayNumber<LilianEpoch>;

/// The offset from the [`JULIAN_EPOCH`] for the [`LilianDate`]
pub const LILIAN_JD_OFFSET: f64 = 2299159.5_f64;

pub const LILIAN_EPOCH: JulianEpoch = JulianEpoch::new(LILIAN_JD_OFFSET, GREGORIAN_EPOCH);
impl_julian!(LilianDate, LILIAN_EPOCH);

/// No functionality, used as a static compile-time type check
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct RataDieEpoch;
impl AsJulianEpoch for RataDieEpoch {
    fn get_julian_epoch() -> JulianEpoch {
        RATA_DIE_EPOCH
    }
}

///
/// The Rata Die (Latin: "Fixed Date") is the fixed number of days in the Common
/// Era, since Midnight 01-01-0001 AD, 1721424.5 after [`JULIAN_EPOCH`]
pub type RataDieDate = JulianDayNumber<RataDieEpoch>;

/// The offset from the [`JULIAN_EPOCH`] for the [`RataDieDate`]
pub const RATA_DIE_JD_OFFSET: f64 = 1721424.5_f64;
pub const RATA_DIE_EPOCH: JulianEpoch = JulianEpoch::new(RATA_DIE_JD_OFFSET, COMMON_ERA_EPOCH);
impl_julian!(RataDieDate, RATA_DIE_EPOCH);

/// The offset from the [`JULIAN_EPOCH`] for the [`UnixTimestamp`]
pub const UNIX_TS_JD_OFFSET: f64 = 2240587.5_f64;
pub const UNIX_JD_EPOCH: JulianEpoch = JulianEpoch::new(UNIX_TS_JD_OFFSET, UNIX_EPOCH);

///
/// The Prime Date is the fixed number of days since 01-JAN-1900.
pub type PrimeDate = JulianDayNumber<PrimeEpoch>;
pub const PRIME_JD_OFFSET: f64 = 2415020.5_f64;
pub const PRIME_JD_EPOCH: JulianEpoch = JulianEpoch::new(PRIME_JD_OFFSET, PRIME_EPOCH);
impl AsJulianEpoch for PrimeEpoch {
    fn get_julian_epoch() -> JulianEpoch {
        PRIME_JD_EPOCH
    }
}
impl_julian!(PrimeDate, PRIME_JD_EPOCH);

pub const MJD2000_OFFSET: f64 = 2451544.5;
pub const MJD2000_EPOCH: JulianEpoch = JulianEpoch::new(MJD2000_OFFSET, Y2K_EPOCH);
pub struct MJD2000Epoch;
impl AsJulianEpoch for MJD2000Epoch {
    fn get_julian_epoch() -> JulianEpoch {
        MJD2000_EPOCH
    }
}
pub type MJD2000 = JulianDayNumber<MJD2000Epoch>;
impl_julian!(MJD2000, MJD2000_EPOCH);

pub const VICINTI_OFFSET: f64 = 2458849.5;
pub const VICINTI_JD_EPOCH: JulianEpoch = JulianEpoch::new(VICINTI_OFFSET, VICINTIPOCH);

impl<T: AsJulianEpoch> Add<Duration> for JulianDayNumber<T> {
    type Output = JulianDayNumber<T>;

    fn add(self, rhs: Duration) -> Self::Output {
        let day_number = self.day_number + rhs.as_seconds_f64() / SECONDS_IN_DAY as f64;
        Self::new_internal(self.epoch, day_number)
    }
}

impl<T: AsJulianEpoch> Add<&Duration> for JulianDayNumber<T> {
    type Output = JulianDayNumber<T>;

    fn add(self, rhs: &Duration) -> Self::Output {
        let day_number = self.day_number + rhs.as_seconds_f64() / SECONDS_IN_DAY as f64;
        Self::new_internal(self.epoch, day_number)
    }
}

impl<T: AsJulianEpoch> Sub<Duration> for JulianDayNumber<T> {
    type Output = JulianDayNumber<T>;

    fn sub(self, rhs: Duration) -> Self::Output {
        let day_number = self.day_number - rhs.as_seconds_f64() / SECONDS_IN_DAY as f64;
        Self::new_internal(self.epoch, day_number)
    }
}

impl<T: AsJulianEpoch> Sub<&Duration> for JulianDayNumber<T> {
    type Output = JulianDayNumber<T>;

    fn sub(self, rhs: &Duration) -> Self::Output {
        let day_number = self.day_number - rhs.as_seconds_f64() / SECONDS_IN_DAY as f64;
        Self::new_internal(self.epoch, day_number)
    }
}

impl<T: AsJulianEpoch> AddAssign<Duration> for JulianDayNumber<T> {
    fn add_assign(&mut self, rhs: Duration) {
        self.day_number += rhs.as_seconds_f64() / SECONDS_IN_DAY as f64;
    }
}

impl<T: AsJulianEpoch> AddAssign<&Duration> for JulianDayNumber<T> {
    fn add_assign(&mut self, rhs: &Duration) {
        self.day_number += rhs.as_seconds_f64() / SECONDS_IN_DAY as f64;
    }
}

impl<T: AsJulianEpoch> SubAssign<Duration> for JulianDayNumber<T> {
    fn sub_assign(&mut self, rhs: Duration) {
        self.day_number -= rhs.as_seconds_f64() / SECONDS_IN_DAY as f64;
    }
}

impl<T: AsJulianEpoch> SubAssign<&Duration> for JulianDayNumber<T> {
    fn sub_assign(&mut self, rhs: &Duration) {
        self.day_number -= rhs.as_seconds_f64() / SECONDS_IN_DAY as f64;
    }
}

impl<T: AsJulianEpoch> Sub<JulianDayNumber<T>> for JulianDayNumber<T> {
    type Output = Duration;

    fn sub(self, rhs: JulianDayNumber<T>) -> Self::Output {
        let dx = self.day_number - rhs.day_number;
        Duration::new(dx, DurationUnit::Day)
    }
}
impl<T: AsJulianEpoch> Sub<&JulianDayNumber<T>> for JulianDayNumber<T> {
    type Output = Duration;

    fn sub(self, rhs: &JulianDayNumber<T>) -> Self::Output {
        let dx = self.day_number - rhs.day_number;
        Duration::new(dx, DurationUnit::Day)
    }
}

impl From<UnixTimestamp> for JulianDate {
    fn from(value: UnixTimestamp) -> Self {
        let jd = value.get_offset().as_seconds_f64() / SECONDS_IN_DAY as f64 + UNIX_TS_JD_OFFSET;
        JulianDate::new(jd)
    }
}

impl From<JulianDate> for UnixTimestamp {
    fn from(value: JulianDate) -> Self {
        let ts = (value.day_number - UNIX_TS_JD_OFFSET) * SECONDS_IN_DAY as f64;
        UnixTimestamp::from_seconds_f64(ts)
    }
}
