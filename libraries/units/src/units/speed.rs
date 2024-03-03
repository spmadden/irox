// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! This module contains the basic types and conversions for the SI "Speed" quantity
use core::fmt::{Display, Formatter};

use crate::units::{FromUnits, Unit};

///
/// Represents a specific speed unit - SI or otherwise
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum SpeedUnits {
    /// SI Base Unit for Speed - MetersPerSecond
    #[default]
    MetersPerSecond,

    /// Miles Per Hour
    MilesPerHour,

    /// Kilometers Per Hour
    KilometersPerHour,

    /// Nautical Mile per hour
    Knots,
}

macro_rules! from_units_speed {
    ($type:ident) => {
        impl crate::units::FromUnits<$type> for SpeedUnits {
            fn from(&self, value: $type, units: Self) -> $type {
                match self {
                    // target
                    SpeedUnits::MetersPerSecond => match units {
                        // source
                        SpeedUnits::MetersPerSecond => value as $type,
                        SpeedUnits::MilesPerHour => value * MPH_TO_MPS as $type,
                        SpeedUnits::KilometersPerHour => value * KPH_TO_MPS as $type,
                        SpeedUnits::Knots => value * KNOT_TO_MPS as $type,
                    },
                    _ => todo!(),
                }
            }
        }
    };
}
basic_unit!(Speed, SpeedUnits, MetersPerSecond);
from_units_speed!(f32);
from_units_speed!(f64);

impl Unit<SpeedUnits> for Speed {
    fn as_unit(&self, units: SpeedUnits) -> Self {
        Speed {
            value: units.from(self.value, self.units),
            units,
        }
    }
}

impl Speed {
    #[must_use]
    pub fn as_meters_per_second(&self) -> Speed {
        self.as_unit(SpeedUnits::MetersPerSecond)
    }

    #[must_use]
    pub fn new_meters_per_second(value: f64) -> Speed {
        Speed::new(value, SpeedUnits::MetersPerSecond)
    }
}

impl Display for Speed {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:0.3}m/s", self.as_meters_per_second().value)
    }
}

pub const FPS_TO_MPS: f64 = 8.466_667E-5;
pub const MPS_TO_FPS: f64 = 1.0 / FPS_TO_MPS;
pub const KPH_TO_MPS: f64 = 2.777_778E-1;
pub const MPS_TO_KPH: f64 = 1.0 / KPH_TO_MPS;
pub const KNOT_TO_MPS: f64 = 5.144_444E-1;
pub const MPS_TO_KNOT: f64 = 1.0 / KNOT_TO_MPS;
pub const MPH_TO_MPS: f64 = 4.4704E-1;
pub const MPS_TO_MPH: f64 = 1.0 / MPH_TO_MPS;
