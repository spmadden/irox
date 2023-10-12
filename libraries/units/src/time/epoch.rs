// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::marker::PhantomData;

use crate::time::{Date, Duration};

///
/// The "epoch" serves as a reference point from which time is measured.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Epoch(pub Date);
impl Epoch {
    ///
    /// The Gregorian Date of this particular Epoch.
    pub fn get_gregorian_date(&self) -> Date {
        self.0
    }
}

///
/// Represents a duration offset from a particular [`Epoch`]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Timestamp<T> {
    epoch: Epoch,
    offset: Duration,

    _phantom: PhantomData<T>,
}
impl<T> Timestamp<T> {
    ///
    /// Returns the base epoch for this timestamp
    #[must_use]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    ///
    /// Returns the relative offset of this timestamp from the specified epoch.
    #[must_use]
    pub fn get_offset(&self) -> Duration {
        self.offset
    }
}

///
/// The Unix Epoch, 1970-01-01, 00:00:00
pub const UNIX_EPOCH: Epoch = Epoch(Date {
    year: 1970,
    day_of_year: 1,
});

///
/// Represents a duration offset from the [`UNIX_EPOCH`].
pub type UnixTimestamp = Timestamp<UnixEpoch>;
pub struct UnixEpoch;

macro_rules! derive_timestamp_impl {
    ($epoch:ident,$name:ident) => {
        impl $name {
            ///
            /// Creates a new timestamp given the specified offset
            pub fn from_offset(offset: Duration) -> $name {
                $name {
                    epoch: $epoch,
                    offset,
                    ..$name::default()
                }
            }

            ///
            /// Creates a new timestamp given the specified number of seconds
            pub fn from_seconds(seconds: u32) -> $name {
                $name::from_offset(Duration::new_seconds(seconds as f64))
            }
        }
        impl Default for $name {
            fn default() -> Self {
                $name {
                    epoch: $epoch,
                    offset: Duration::default(),

                    _phantom: Default::default(),
                }
            }
        }
        impl From<Duration> for $name {
            fn from(value: Duration) -> Self {
                $name::from_offset(value)
            }
        }
    };
}

impl UnixTimestamp {
    ///
    /// Returns the local system clock equivalent of the unix timestamp
    pub fn now() -> UnixTimestamp {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(t) => UnixTimestamp::from_offset(t.into()),
            Err(t) => {
                UnixTimestamp::from_offset(Duration::new_seconds(-1.0 * t.duration().as_secs_f64()))
            }
        }
    }
}
derive_timestamp_impl!(UNIX_EPOCH, UnixTimestamp);

///
/// The GPS Epoch, 1980-01-06, 00:00:00
pub const GPS_EPOCH: Epoch = Epoch(Date {
    year: 1980,
    day_of_year: 6,
});

///
/// Represents a duration offset from the [`GPS_EPOCH`]
pub type GPSTimestamp = Timestamp<GPSEpoch>;
pub struct GPSEpoch;
derive_timestamp_impl!(GPS_EPOCH, GPSTimestamp);

///
/// The Gregorian Epoch, 15-OCT-1582
pub const GREGORIAN_EPOCH: Epoch = Epoch(Date {
    year: 1582,
    day_of_year: 288,
});

///
/// Represents a duration offset from the [`GREGORIAN_EPOCH`]
pub type GregorianTimestamp = Timestamp<GregorianEpoch>;
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
    day_of_year: 1,
});

///
/// Represents a duration offset from the [`WINDOWS_NT_EPOCH`]
///
/// Note: when a duration is actually retrieved from the windows FILETIME
/// routines, it comes back in 100-nanosecond increments from this epoch.
pub type WindowsNTTimestamp = Timestamp<WindowsEpoch>;
pub struct WindowsEpoch;
derive_timestamp_impl!(WINDOWS_NT_EPOCH, WindowsNTTimestamp);

///
/// The Common Era Epoch, 01-JAN-0001 AD
pub const COMMON_ERA_EPOCH: Epoch = Epoch(Date {
    year: 1,
    day_of_year: 1,
});
