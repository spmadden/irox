// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Contains the concept of an [`Epoch`] - a specific Proleptic Gregorian [`Date`] from which a
//! [`Timestamp`] is measured against.
//!
//! A [`Timestamp`] is a [`Duration`], a physical amount of time measured against an [`Epoch`]
//!
//! Epochs
//! -------
//! | Epoch     | JDN         | Year    | DOY   | Julian Epoch               | Gregorian Epoch               | Timestamp              |
//! |-----------|-------------|---------|-------|----------------------------|-------------------------------|------------------------|
//! | Julian    | `0.0`       | `-4712` | `0`   | [`JULIAN_EPOCH`]           |                               |
//! | Rata Die  | `1721424.5` | `0001`  | `0`   | [`RATA_DIE_EPOCH`]         | [`COMMON_ERA_EPOCH`]          |
//! | Lilian    | `2299159.5` | `1582`  | `257` | [`LILIAN_EPOCH`]           | [`GREGORIAN_EPOCH`]           | [`GregorianTimestamp`] |
//! | Windows   | `2305813.5` | `1601`  | `0`   |                            | [`WINDOWS_NT_EPOCH`]          | [`WindowsNTTimestamp`] |
//! | Reduced   | `2400000`   | `1858`  | `320` | [`REDUCED_JULIAN_EPOCH`]   |                               |
//! | Modified  | `2400000.5` | `1858`  | `320` | [`MODIFIED_JULIAN_EPOCH`]  |                               |
//! | Prime     | `2415020.5` | `1900`  | `0`   | [`PRIME_JD_EPOCH`]         | [`PRIME_EPOCH`] [`NTP_EPOCH`] | [`PrimeTimestamp`]     |
//! | Truncated | `2440000.5` | `1968`  | `145` | [`TRUNCATED_JULIAN_EPOCH`] |                               |
//! | Unix      | `2440587.5` | `1970`  | `0`   | [`UNIX_JD_EPOCH`]          | [`UNIX_EPOCH`]                | [`UnixTimestamp`]      |
//! | GPS       | `2444244.5` | `1980`  | `5`   |                            | [`GPS_EPOCH`]                 | [`GPSTimestamp`]       |
//! | MJD2000   | `2451544.5` | `2000`  | `0`   | [`MJD2000_EPOCH`]          | [`Y2K_EPOCH`]                 |
//! | Leapoch   | `2451604.5` | `2000`  | `60`  |                            | [`LEAPOCH`]                   | [`LeapochTimestamp`]   |
//! | Vicinti   | `2458849.5` | `2020`  | `0`   | [`VICINTI_JD_EPOCH`]       | [`VICINTIPOCH`]               | [`VicintiTimestamp`]   |

use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use irox_tools::{cfg_docs, cfg_feature_serde};
use irox_units::units::duration::Duration;

use crate::gregorian::Date;
use crate::julian::JulianDate;

cfg_docs! {
    use crate::julian::{JULIAN_EPOCH, RATA_DIE_EPOCH, LILIAN_EPOCH, REDUCED_JULIAN_EPOCH, MODIFIED_JULIAN_EPOCH, PRIME_JD_EPOCH, TRUNCATED_JULIAN_EPOCH, UNIX_JD_EPOCH, MJD2000_EPOCH, VICINTI_JD_EPOCH};
}

///
/// An `Epoch` serves as a reference point from which time is measured.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Epoch(pub Date);

impl Epoch {
    ///
    /// The Gregorian Date of this particular Epoch.
    pub fn get_gregorian_date(&self) -> Date {
        self.0
    }
}

///
/// Represents a [`Duration`] offset from a particular [`Epoch`]
#[derive(Debug, Copy, Clone)]
pub struct Timestamp<T> {
    epoch: Epoch,
    offset: Duration,

    _phantom: PhantomData<T>,
}

impl<T> Timestamp<T> {
    pub const fn new(epoch: Epoch, duration: Duration) -> Self {
        Self {
            epoch,
            offset: duration,
            _phantom: PhantomData,
        }
    }

    ///
    /// Returns the base epoch for this timestamp
    #[must_use]
    pub const fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    ///
    /// Returns the relative offset of this timestamp from the specified epoch.
    #[must_use]
    pub const fn get_offset(&self) -> Duration {
        self.offset
    }
}
impl<T> PartialEq for Timestamp<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.epoch != other.epoch {
            return false;
        }
        self.offset.eq(&other.offset)
    }
}
impl<T> Eq for Timestamp<T> {}

impl<T> PartialOrd for Timestamp<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.offset.cmp(&other.offset))
    }
}
impl<T> Ord for Timestamp<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset)
    }
}

///
/// The Unix Epoch, 1970-01-01, 00:00:00
pub const UNIX_EPOCH: Epoch = Epoch(Date {
    year: 1970,
    day_of_year: 0,
});

///
/// Represents a duration offset from the [`UNIX_EPOCH`].
pub type UnixTimestamp = Timestamp<UnixEpoch>;

/// `UnixEpoch` is a compile-time check for [`UnixTimestamp`] = [`Timestamp<UnixEpoch>`]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct UnixEpoch;

pub trait FromTimestamp<T> {
    fn from_timestamp(other: &Timestamp<T>) -> Self;
}

macro_rules! derive_timestamp_impl {
    ($epoch:ident,$name:ident) => {
        impl $name {
            ///
            /// Creates a new timestamp given the specified offset
            pub const fn from_offset(offset: Duration) -> $name {
                $name::new($epoch, offset)
            }

            ///
            /// Creates a new timestamp given the specified number of seconds
            pub const fn from_seconds(seconds: u32) -> $name {
                $name::from_seconds_f64(seconds as f64)
            }

            ///
            /// Creates a new timestamp given the specified number of fractional seconds
            pub const fn from_seconds_f64(seconds: f64) -> $name {
                $name::from_offset(Duration::new_seconds(seconds))
            }
        }
        impl Default for $name {
            fn default() -> Self {
                $name::from_offset(Duration::default())
            }
        }
        impl From<Duration> for $name {
            fn from(value: Duration) -> Self {
                $name::from_offset(value)
            }
        }

        impl<T> FromTimestamp<T> for $name {
            fn from_timestamp(other: &Timestamp<T>) -> Self {
                let epoch_offset = $epoch.0 - other.epoch.0;
                let new_duration = other.offset - epoch_offset;
                $name::from_offset(new_duration)
            }
        }
    };
}

impl<T> AddAssign<Duration> for Timestamp<T> {
    fn add_assign(&mut self, rhs: Duration) {
        self.offset += rhs;
    }
}

impl<T> SubAssign<Duration> for Timestamp<T> {
    fn sub_assign(&mut self, rhs: Duration) {
        self.offset -= rhs;
    }
}

impl<T> AddAssign<&Duration> for Timestamp<T> {
    fn add_assign(&mut self, rhs: &Duration) {
        self.offset += *rhs;
    }
}

impl<T> SubAssign<&Duration> for Timestamp<T> {
    fn sub_assign(&mut self, rhs: &Duration) {
        self.offset -= *rhs;
    }
}
impl<T> SubAssign<&mut Duration> for Timestamp<T> {
    fn sub_assign(&mut self, rhs: &mut Duration) {
        self.offset -= *rhs;
    }
}
macro_rules! impl_sub_timestamp {
    ($($sub:ty)+, $($slf:ty)+) => {
        impl<T> Sub<$($sub)+> for $($slf)+ {
            type Output = Duration;

            fn sub(self, rhs: $($sub)+) -> Self::Output {
                self.offset - rhs.offset
            }
        }
    };
}
macro_rules! impl_sub_duration {
    ($($sub:ty)+, $($slf:ty)+) => {
        impl<T> Sub<$($sub)+> for $($slf)+ {
            type Output = Timestamp<T>;

            fn sub(self, rhs: $($sub)+) -> Self::Output {
                let offset = self.offset - rhs;
                Timestamp::new(self.epoch, offset)
            }
        }
    };
}
macro_rules! impl_add_timestamp {
    ($($sub:ty)+, $($slf:ty)+) => {
        impl<T> Add<$($sub)+> for $($slf)+ {
            type Output = Timestamp<T>;

            fn add(self, rhs: $($sub)+) -> Self::Output {
                let offset = self.offset + rhs;
                Timestamp::new(self.epoch, offset)
            }
        }
    };
}
macro_rules! impl_op {
    ($op:ident, $($operand:ty)+) => {
        $op!($($operand)+, Timestamp<T>);
        $op!($($operand)+, &Timestamp<T>);
        $op!($($operand)+, &mut Timestamp<T>);
        $op!(&$($operand)+, Timestamp<T>);
        $op!(&$($operand)+, &Timestamp<T>);
        $op!(&$($operand)+, &mut Timestamp<T>);
        $op!(&mut $($operand)+, Timestamp<T>);
        $op!(&mut $($operand)+, &Timestamp<T>);
        $op!(&mut $($operand)+, &mut Timestamp<T>);
    };
}
impl_op!(impl_sub_timestamp, Timestamp<T>);
impl_op!(impl_add_timestamp, Duration);
impl_op!(impl_sub_duration, Duration);

impl UnixTimestamp {
    ///
    /// Returns the local system clock equivalent of the unix timestamp
    #[must_use]
    #[cfg(feature = "std")]
    pub fn now() -> UnixTimestamp {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(t) => UnixTimestamp::from_offset(t.into()),
            Err(t) => {
                UnixTimestamp::from_offset(Duration::new_seconds(-1.0 * t.duration().as_secs_f64()))
            }
        }
    }

    ///
    /// Returns the local system clock duration since the timestamp.  MAY BE NEGATIVE if the clock
    /// has changed since the last call.
    #[must_use]
    #[cfg(feature = "std")]
    pub fn elapsed(&self) -> Duration {
        Self::now().offset - self.offset
    }

    ///
    /// Returns this timestamp as a Date
    #[must_use]
    pub fn as_date(&self) -> Date {
        self.into()
    }

    #[must_use]
    pub fn as_julian(&self) -> JulianDate {
        (*self).into()
    }
}
derive_timestamp_impl!(UNIX_EPOCH, UnixTimestamp);

///
/// The GPS Epoch, 1980-01-06, 00:00:00
pub const GPS_EPOCH: Epoch = Epoch(Date {
    year: 1980,
    day_of_year: 5,
});

///
/// Represents a duration offset from the [`GPS_EPOCH`]
pub type GPSTimestamp = Timestamp<GPSEpoch>;

/// `GPSEpoch` is a compile-time check for [`GPSTimestamp`] = [`Timestamp<GPSEpoch>`]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct GPSEpoch;
derive_timestamp_impl!(GPS_EPOCH, GPSTimestamp);

///
/// The Gregorian Epoch, 15-OCT-1582
pub const GREGORIAN_EPOCH: Epoch = Epoch(Date {
    year: 1582,
    day_of_year: 287,
});

///
/// Represents a duration offset from the [`GREGORIAN_EPOCH`]
pub type GregorianTimestamp = Timestamp<GregorianEpoch>;

/// `GregorianEpoch` is a compile-time check for [`GregorianTimestamp`] = [`Timestamp<GregorianEpoch>`]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct GregorianEpoch;
derive_timestamp_impl!(GREGORIAN_EPOCH, GregorianTimestamp);

///
/// The Windows NT Epoch, 01-JAN-1601.
///
/// Why this date?  The Gregorian calendar operates on a 400-year cycle, and
/// 1601 is the first year of the cycle that was active at the time Windows NT
/// was being designed. In other words, it was chosen to make the math come out
/// nicely.
pub const WINDOWS_NT_EPOCH: Epoch = Epoch(Date {
    year: 1601,
    day_of_year: 0,
});

///
/// Represents a duration offset from the [`WINDOWS_NT_EPOCH`]
///
/// Note: when a duration is actually retrieved from the windows FILETIME
/// routines, it comes back in 100-nanosecond increments from this epoch.
pub type WindowsNTTimestamp = Timestamp<WindowsEpoch>;

/// `WindowsEpoch` is a compile-time check for [`WindowsNTTimestamp`] = [`Timestamp<WindowsEpoch>`]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct WindowsEpoch;
derive_timestamp_impl!(WINDOWS_NT_EPOCH, WindowsNTTimestamp);

///
/// The Common Era Epoch, 01-JAN-0001 AD
pub const COMMON_ERA_EPOCH: Epoch = Epoch(Date {
    year: 1,
    day_of_year: 0,
});

///
/// The Prime Epoch, 01-JAN-1900
pub const PRIME_EPOCH: Epoch = Epoch(Date {
    year: 1900,
    day_of_year: 0,
});
///
/// Represents a duration offset from the [`WINDOWS_NT_EPOCH`]
///
/// Note: when a duration is actually retrieved from the windows FILETIME
/// routines, it comes back in 100-nanosecond increments from this epoch.
pub type PrimeTimestamp = Timestamp<PrimeEpoch>;
/// `PrimeEpoch` is a compile-time check for [`PrimeTimestamp`] = [`Timestamp<crate::epoch::PrimeEpoch>`]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct PrimeEpoch;
derive_timestamp_impl!(PRIME_EPOCH, PrimeTimestamp);
///
/// The NTP epoch is the same as the [`PRIME_EPOCH`]
pub const NTP_EPOCH: Epoch = PRIME_EPOCH;

/// 01-MAR-2000, a mod400 year after the leap day.
pub const LEAPOCH_TIMESTAMP: UnixTimestamp = UnixTimestamp::from_seconds(951868800);
/// 01-MAR-2000, a mod400 year after the leap day.
pub const LEAPOCH: Epoch = Epoch(Date {
    year: 2000,
    day_of_year: 60,
});
/// `Leapoch` is a compile-time check for [`LeapochTimestamp`] = [`Timestamp<crate::epoch::Leapoch>`]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Leapoch;

///
/// Represents a duration offset from the [`LEAPOCH`]
pub type LeapochTimestamp = Timestamp<Leapoch>;
derive_timestamp_impl!(LEAPOCH, LeapochTimestamp);

pub const JULIAN_DAY_1_JAN_YR0: f64 = 1721059.5;

/// The Year 2000 (Y2K) Epoch 01-JAN-2000
pub const Y2K_EPOCH: Epoch = Epoch(Date {
    year: 2000,
    day_of_year: 0,
});

/// The Vigintipoch (Viginti-poch, 'twenty') 01-JAN-2020
pub const VICINTIPOCH: Epoch = Epoch(Date {
    year: 2020,
    day_of_year: 0,
});
/// `Vicintipoch` is a compile-time check for [`VicintiTimestamp`] = [`Timestamp<crate::epoch::Vicintipoch>`]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vicintipoch;
pub type VicintiTimestamp = Timestamp<Vicintipoch>;
derive_timestamp_impl!(VICINTIPOCH, VicintiTimestamp);

cfg_feature_serde! {
    struct UnixTimestampVisitor;
    impl serde::de::Visitor<'_> for UnixTimestampVisitor {
        type Value = UnixTimestamp;

        fn expecting(&self, fmt: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            write!(fmt, "The visitor expects to receive a uint64 representing number of seconds since")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: serde::de::Error {
            Ok(UnixTimestamp::from_offset(Duration::from_nanos(v)))
        }

    }
    impl<'de> serde::Deserialize<'de> for UnixTimestamp {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
            deserializer.deserialize_u64(UnixTimestampVisitor)
        }
    }
    impl serde::Serialize for UnixTimestamp {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            let nanos = self.offset.as_nanos();
            serializer.serialize_u64(nanos)
        }
    }
}
#[cfg(all(test, feature = "serde", feature = "std"))]
mod tests {
    use crate::epoch::UnixTimestamp;
    use irox_units::units::duration::Duration;

    #[test]
    #[cfg(all(feature = "serde", feature = "std"))]
    pub fn serde_test() -> Result<(), crate::FormatError> {
        #[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Debug)]
        struct Test {
            a: UnixTimestamp,
        }
        impl Default for Test {
            fn default() -> Self {
                Self {
                    a: UnixTimestamp::default(),
                }
            }
        }
        let a = Test {
            a: UnixTimestamp::from_offset(Duration::from_days(1)),
        };
        let s = serde_json::to_string(&a).unwrap_or_default();
        assert_eq!(
            s,
            format!("{{\"a\":{}}}", Duration::from_days(1).as_nanos())
        );
        let b: Test = serde_json::from_str(&s).unwrap();
        assert_eq!(a, b);
        Ok(())
    }
}
