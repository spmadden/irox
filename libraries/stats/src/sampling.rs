// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use irox_time::epoch::Timestamp;
use irox_time::Time64;

#[derive(Default, Debug, Copy, Clone)]
pub struct Sample64 {
    time: Time64,
    value: f64,
}
impl PartialEq for Sample64 {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.value == other.value
    }
}
impl Eq for Sample64 {}
impl PartialOrd for Sample64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Sample64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}
impl Hash for Sample64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.time.hash(state);
        self.value.to_bits().hash(state);
    }
}

impl Sample64 {
    #[must_use]
    pub const fn new(value: f64, time: Time64) -> Self {
        Sample64 { time, value }
    }
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }
    #[must_use]
    pub const fn time(&self) -> Time64 {
        self.time
    }
    pub fn set_time(&mut self, time: Time64) {
        self.time = time;
    }
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sample<T: Copy> {
    pub time: Timestamp<T>,
    pub value: f64,
}
impl<T: Copy> Sample<T> {
    #[must_use]
    pub const fn new(value: f64, time: Timestamp<T>) -> Self {
        Sample { time, value }
    }
    #[must_use]
    pub const fn time(&self) -> Timestamp<T> {
        self.time
    }
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }
}
impl<T: Copy> Ord for Sample<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}
impl<T: Copy> PartialOrd for Sample<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Copy> Eq for Sample<T> {}
impl<T: Copy> PartialEq for Sample<T> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.value == other.value
    }
}
