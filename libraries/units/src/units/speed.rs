//!
//! This module contains the basic types and conversions for the SI "Speed" quantity
use crate::units::{FromUnits, Unit};

///
/// Represents a specific speed unit - SI or otherwise
#[derive(Debug, Clone, Copy, Default)]
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

macro_rules! from_units_length {
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
from_units_length!(f32);
from_units_length!(f64);

impl Unit<SpeedUnits> for Speed {
    fn as_unit(&self, units: SpeedUnits) -> Self {
        Speed {
            value: units.from(self.value, self.units),
            units,
        }
    }
}

impl Speed {
    pub fn as_meters_per_second(&self) -> Speed {
        self.as_unit(SpeedUnits::MetersPerSecond)
    }

    pub fn new_meters_per_second(value: f64) -> Speed {
        Speed::new(value, SpeedUnits::MetersPerSecond)
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
