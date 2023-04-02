//!
//! This module contains the basic types and conversions for the SI "Length" quantity
use crate::units::{FromUnits, Unit};

///
/// Represents a specific length unit - SI or otherwise
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub enum LengthUnits {
    /// SI Base Unit for Length - Meters
    #[default]
    Meters,

    /// US Imperial "Foot"
    Feet,
}
macro_rules! from_units_length {
    ($type:ident) => {
        impl crate::units::FromUnits<$type> for LengthUnits {
            fn from(&self, value: $type, units: Self) -> $type {
                match self {
                    LengthUnits::Meters => match units {
                        LengthUnits::Meters => value as $type,
                        LengthUnits::Feet => value * METERS_TO_FEET as $type,
                    },
                    LengthUnits::Feet => match units {
                        LengthUnits::Meters => value * FEET_TO_METERS as $type,
                        LengthUnits::Feet => value as $type,
                    },
                }
            }
        }
    };
}
basic_unit!(Length, LengthUnits, Meters);
from_units_length!(f32);
from_units_length!(f64);

impl Unit<LengthUnits> for Length {
    fn as_unit(&self, units: LengthUnits) -> Self {
        Length {
            value: units.from(self.value, self.units),
            units,
        }
    }
}

///
/// Represents a discrete quantity of 'Length' as defined in NIST 811.2008
impl Length {
    pub const fn new_meters(value: f64) -> Length {
        Self {
            value,
            units: LengthUnits::Meters,
        }
    }

    pub const fn new_feet(value: f64) -> Length {
        Self {
            value,
            units: LengthUnits::Feet,
        }
    }

    pub fn as_meters(&self) -> Length {
        self.as_unit(LengthUnits::Meters)
    }

    pub fn as_feet(&self) -> Length {
        self.as_unit(LengthUnits::Feet)
    }
}

pub const FEET_TO_METERS: f64 = 3.048E-01; // Exact, as per NIST 811.2008
pub const METERS_TO_FEET: f64 = 1. / FEET_TO_METERS;
