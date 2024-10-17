// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::Sample;
use irox_time::Time64;
use irox_types::PrimitiveValue;
use std::sync::Arc;

/// A trait to provide access to the current time.  This could be driven by
/// a monotonic clock, a RTC, or a wall-clock time.
pub trait TimeProvider {
    fn get_current_time(&self) -> Time64;
}

pub struct TimeWrapper {
    timer: Arc<Box<dyn TimeProvider + 'static>>,
}
impl TimeWrapper {
    pub fn new(timer: Arc<Box<dyn TimeProvider + 'static>>) -> TimeWrapper {
        TimeWrapper { timer: timer }
    }

    pub fn wrap_infallible<V: Into<PrimitiveValue>, F: FnMut() -> V>(&self, mut func: F) -> Sample {
        let time = self.timer.get_current_time();
        Sample::new(func(), time)
    }
}
