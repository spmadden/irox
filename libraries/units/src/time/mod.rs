// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

pub use duration::*;
pub use epoch::*;
pub use gregorian::*;

use crate::bounds::{GreaterThanEqualToValueError, LessThanValue, Range};

mod duration;
mod epoch;
mod gregorian;

///
/// Represents a time of the day, an offset into the day from midnight.
pub struct Time {
    second_of_day: u32,
}

impl Time {
    pub fn new(second_of_day: u32) -> Result<Time, GreaterThanEqualToValueError<u32>> {
        LessThanValue::new(86401).check_value_is_valid(&second_of_day)?;
        Ok(Time { second_of_day })
    }
}

pub const HOURS_IN_DAY: u8 = 24;
pub const MINUTES_IN_HOUR: u8 = 60;
pub const SECONDS_IN_MINUTE: u8 = 60;
pub const MINUTES_IN_DAY: u16 = 1440;

///
/// Generally 86400, but occasionally 86401 for leap seconds.
pub const SECONDS_IN_DAY: u32 = 86400;
