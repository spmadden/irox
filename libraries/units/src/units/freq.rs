// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::units::{FromUnits, Unit};
use core::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FrequencyUnit {
    #[default]
    Hertz,
    Kilohertz,
    Megahertz,
    Gigahertz,
}
impl FrequencyUnit {
    pub fn abbreviation(&self) -> &'static str {
        match self {
            FrequencyUnit::Hertz => "Hz",
            FrequencyUnit::Kilohertz => "KHz",
            FrequencyUnit::Megahertz => "MHz",
            FrequencyUnit::Gigahertz => "GHz",
        }
    }
}

macro_rules! from_units_freq {
    ($type:ident) => {
        impl crate::units::FromUnits<$type> for FrequencyUnit {
            fn from(&self, value: $type, units: Self) -> $type {
                match self {
                    // target
                    FrequencyUnit::Hertz => {
                        // source
                        match units {
                            FrequencyUnit::Hertz => value as $type,
                            FrequencyUnit::Kilohertz => value * KHZ_TO_HZ as $type,
                            FrequencyUnit::Megahertz => value * MHZ_TO_HZ as $type,
                            FrequencyUnit::Gigahertz => value * GHZ_TO_HZ as $type,
                        }
                    }
                    FrequencyUnit::Kilohertz => {
                        // source
                        match units {
                            FrequencyUnit::Hertz => value * KHZ_TO_HZ as $type,
                            FrequencyUnit::Kilohertz => value as $type,
                            FrequencyUnit::Megahertz => value * MHZ_TO_HZ as $type,
                            FrequencyUnit::Gigahertz => value * GHZ_TO_HZ as $type,
                        }
                    }
                    FrequencyUnit::Megahertz => {
                        // source
                        match units {
                            FrequencyUnit::Hertz => value * HZ_TO_MHZ as $type,
                            FrequencyUnit::Kilohertz => value * KHZ_TO_MHZ as $type,
                            FrequencyUnit::Megahertz => value as $type,
                            FrequencyUnit::Gigahertz => value * GHZ_TO_MHZ as $type,
                        }
                    }
                    FrequencyUnit::Gigahertz => {
                        // source
                        match units {
                            FrequencyUnit::Hertz => value * HZ_TO_GHZ as $type,
                            FrequencyUnit::Kilohertz => value * KHZ_TO_GHZ as $type,
                            FrequencyUnit::Megahertz => value * MHZ_TO_GHZ as $type,
                            FrequencyUnit::Gigahertz => value as $type,
                        }
                    }
                }
            }
        }
    };
}
basic_unit!(Frequency, FrequencyUnit, Hertz);
from_units_freq!(f64);

impl Frequency {
    pub fn new_hz(hz: u64) -> Self {
        Self::new(hz as f64, FrequencyUnit::Hertz)
    }
    pub fn new_hz_f64(hz: f64) -> Self {
        Self::new(hz, FrequencyUnit::Hertz)
    }
    pub fn new_khz(khz: f64) -> Self {
        Self::new(khz, FrequencyUnit::Kilohertz)
    }
}
impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.3} {}", self.value, self.units.abbreviation())
    }
}

pub const HZ_TO_KHZ: f64 = 1e-3;
pub const HZ_TO_MHZ: f64 = 1e-6;
pub const HZ_TO_GHZ: f64 = 1e-9;

pub const GHZ_TO_HZ: f64 = 1e9;
pub const MHZ_TO_HZ: f64 = 1e6;
pub const KHZ_TO_HZ: f64 = 1e3;

pub const KHZ_TO_MHZ: f64 = 1e-3;
pub const MHZ_TO_KHZ: f64 = 1e3;

pub const MHZ_TO_GHZ: f64 = 1e-3;
pub const GHZ_TO_MHZ: f64 = 1e3;

pub const GHZ_TO_KHZ: f64 = 1e6;
pub const KHZ_TO_GHZ: f64 = 1e-6;
