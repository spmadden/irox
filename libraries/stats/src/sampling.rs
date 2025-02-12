// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use irox_time::epoch::Timestamp;
use irox_time::Time64;
use irox_tools::StrWrapper;
use irox_tools::ToU64;

macro_rules! sample64 {
    ($name:ident, $prim:ty) => {
        /// A sample with a time resolution of 64 bits and a value resolution of 64 bits. 128b total.
        #[derive(Default, Debug, Copy, Clone)]
        pub struct $name {
            pub time: Time64,
            pub value: $prim,
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.time == other.time && self.value == other.value
            }
        }
        impl Eq for $name {}
        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                self.time.cmp(&other.time)
            }
        }
        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.time.hash(state);
                self.value.to_u64().hash(state);
            }
        }

        impl $name {
            #[must_use]
            pub const fn new(time: Time64, value: $prim) -> Self {
                Self { time, value }
            }
            #[must_use]
            pub const fn value(&self) -> $prim {
                self.value
            }
            #[must_use]
            pub const fn time(&self) -> Time64 {
                self.time
            }
            pub fn set_time(&mut self, time: Time64) {
                self.time = time;
            }
            pub fn set_value(&mut self, value: $prim) {
                self.value = value;
            }
        }
    };
}
sample64!(Sample64, f64);
sample64!(IntSample64, u64);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StrSample64<'a> {
    pub time: Time64,
    pub value: StrWrapper<'a>,
}
impl PartialOrd for StrSample64<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for StrSample64<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}
impl<'a> StrSample64<'a> {
    #[must_use]
    pub const fn new(time: Time64, value: StrWrapper<'a>) -> Self {
        Self { time, value }
    }
    #[must_use]
    pub const fn value(&self) -> &StrWrapper<'a> {
        &self.value
    }
    #[must_use]
    pub const fn time(&self) -> Time64 {
        self.time
    }
    pub fn set_time(&mut self, time: Time64) {
        self.time = time;
    }
    pub fn set_value(&mut self, value: StrWrapper<'a>) {
        self.value = value;
    }
}

///
/// A more generic sample that uses [`Timestamp<T>`] rather than [`Time64`]
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

#[derive(Debug, Clone)]
pub enum SampleValue<'a> {
    String(StrWrapper<'a>),
    Float(f64),
    Int(u64),
}
impl Default for SampleValue<'_> {
    fn default() -> Self {
        SampleValue::Int(0)
    }
}
impl PartialEq for SampleValue<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SampleValue::String(a), SampleValue::String(b)) => a == b,
            (SampleValue::Float(a), SampleValue::Float(b)) => a == b,
            (SampleValue::Int(a), SampleValue::Int(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for SampleValue<'_> {}
impl Hash for SampleValue<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            SampleValue::String(s) => s.hash(state),
            SampleValue::Int(i) => i.hash(state),
            SampleValue::Float(f) => f.to_bits().hash(state),
        }
    }
}
