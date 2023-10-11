// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use crate::time::{Date, Duration};

///
/// The "epoch" serves as a reference point from which time is measured.
pub struct Epoch {
    date: Date,
}
impl Epoch {
    ///
    /// The Gregorian Date of this particular Epoch.
    pub fn get_gregorian_date(&self) -> Date {
        self.date
    }
}

///
/// Represents a duration offset from a particular [`Epoch`]
pub trait Timestamp {
    ///
    /// Returns the Epoch associated with this timestamp.
    fn get_epoch(&self) -> Epoch;

    /// Returns the duration offset of the epoch associated with this timestamp.
    fn get_duration(&self) -> Duration;
}

///
/// The Unix Epoch, 1970-01-01, 00:00:00
pub const UNIX_EPOCH: Epoch = Epoch {
    date: Date {
        year: 1970,
        day_of_year: 1,
    },
};

///
/// Represents a duration offset from the [`UNIX_EPOCH`].
pub struct UnixTimestamp(Duration);
impl Timestamp for UnixTimestamp {
    fn get_epoch(&self) -> Epoch {
        UNIX_EPOCH
    }

    fn get_duration(&self) -> Duration {
        self.0
    }
}

///
/// The GPS Epoch, 1980-01-06, 00:00:00
pub const GPS_EPOCH: Epoch = Epoch {
    date: Date {
        year: 1980,
        day_of_year: 6,
    },
};

///
/// Represents a duration offset from the [`GPS_EPOCH`]
pub struct GPSTimestamp(Duration);
impl Timestamp for GPSTimestamp {
    fn get_epoch(&self) -> Epoch {
        GPS_EPOCH
    }

    fn get_duration(&self) -> Duration {
        self.0
    }
}

///
/// The Gregorian Epoch, 15-OCT-1582
pub const GREGORIAN_EPOCH: Epoch = Epoch {
    date: Date {
        year: 1582,
        day_of_year: 288,
    },
};

///
/// Represents a duration offset from the [`GREGORIAN_EPOCH`]
pub struct GregorianTimestamp(Duration);
impl Timestamp for GregorianTimestamp {
    fn get_epoch(&self) -> Epoch {
        GREGORIAN_EPOCH
    }

    fn get_duration(&self) -> Duration {
        self.0
    }
}

///
/// The Windows NT Epoch, 01-JAN-1601.
///
/// Why this date?  The Gregorian calendar operates on a 400-year cycle, and
/// 1601 is the first year of the cycle that was active at the time Windows NT
/// was being designed. In other words, it was chosen to make the math come out
/// nicely.
pub const WINDOWS_NT_EPOCH: Epoch = Epoch {
    date: Date {
        year: 1601,
        day_of_year: 1,
    },
};

///
/// Represents a duration offset from the [`WINDOWS_NT_EPOCH`]
///
/// Note: when a duration is actually retrieved from the windows FILETIME
/// routines, it comes back in 100-nanosecond increments from this epoch.
pub struct WindowsNTTimestamp(Duration);
impl Timestamp for WindowsNTTimestamp {
    fn get_epoch(&self) -> Epoch {
        WINDOWS_NT_EPOCH
    }

    fn get_duration(&self) -> Duration {
        self.0
    }
}
