// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! This module contains the basic types and conversions for the SI "Length" quantity
use core::fmt::{Display, Formatter};

use crate::units::{FromUnits, Unit};

///
/// Represents a specific length unit - SI or otherwise
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum LengthUnits {
    /// SI Base Unit for Length - Meters
    #[default]
    Meters,

    /// SI Derived unit kilometers
    Kilometers,

    /// US Imperial "Foot"
    Feet,

    /// US Imperial "Mile"
    Mile,

    /// Nautical Mile, classically 1 arcminute on the Earth
    NauticalMile,
}
macro_rules! from_units_length {
    ($type:ident) => {
        impl crate::units::FromUnits<$type> for LengthUnits {
            fn from(&self, value: $type, source_unit: Self) -> $type {
                match self {
                    // target
                    LengthUnits::Meters => match source_unit {
                        // source
                        LengthUnits::Meters => value as $type,
                        LengthUnits::Feet => value * FEET_TO_METERS as $type,
                        LengthUnits::Kilometers => value * KILOMETERS_TO_METERS as $type,
                        LengthUnits::Mile => value * MILES_TO_METERS as $type,
                        LengthUnits::NauticalMile => value * NAUTICAL_MILES_TO_METERS as $type,
                    },
                    LengthUnits::Feet => match source_unit {
                        LengthUnits::Meters => value * METERS_TO_FEET as $type,
                        LengthUnits::Feet => value as $type,
                        LengthUnits::Kilometers => {
                            FromUnits::<$type>::from(&LengthUnits::Meters, value, source_unit)
                                * METERS_TO_KILOMETERS as $type
                        }
                        LengthUnits::Mile => {
                            FromUnits::<$type>::from(&LengthUnits::Meters, value, source_unit)
                                * METERS_TO_MILES as $type
                        }
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            }
        }
    };
}
basic_unit!(Length, LengthUnits, Meters);
from_units_length!(f32);
from_units_length!(f64);

impl LengthUnits {
    pub const fn short_name(&self) -> &'static str {
        match self {
            LengthUnits::Meters => "m",
            LengthUnits::Kilometers => "km",
            LengthUnits::Feet => "ft",
            LengthUnits::Mile => "mi",
            LengthUnits::NauticalMile => "nmi",
        }
    }
}

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
    #[must_use]
    pub const fn new_meters(value: f64) -> Length {
        Self {
            value,
            units: LengthUnits::Meters,
        }
    }

    #[must_use]
    pub const fn new_feet(value: f64) -> Length {
        Self {
            value,
            units: LengthUnits::Feet,
        }
    }

    #[must_use]
    pub fn as_meters(&self) -> Length {
        self.as_unit(LengthUnits::Meters)
    }

    #[must_use]
    pub fn as_feet(&self) -> Length {
        self.as_unit(LengthUnits::Feet)
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{:02.3}{}",
            self.value,
            self.units.short_name()
        ))
    }
}

pub const FEET_TO_METERS: f64 = 3.048E-01; // Exact, as per NIST 811.2008
pub const METERS_TO_FEET: f64 = 1. / FEET_TO_METERS;
pub const MILES_TO_METERS: f64 = 1.609_344E3; // Exact, as per NIST 811.2008
pub const METERS_TO_MILES: f64 = 1. / MILES_TO_METERS;
pub const KILOMETERS_TO_METERS: f64 = 1000.;
pub const METERS_TO_KILOMETERS: f64 = 1. / KILOMETERS_TO_METERS;
pub const NAUTICAL_MILES_TO_METERS: f64 = 1.852E3;
pub const METERS_TO_NAUTICAL_MILE: f64 = 1. / NAUTICAL_MILES_TO_METERS;

#[cfg(test)]
mod tests {
    use crate::units::length::LengthUnits;
    use crate::units::FromUnits;

    #[test]
    pub fn test_feet_meters() {
        assert_eq!(
            LengthUnits::Feet.from(1.0, LengthUnits::Meters),
            1.0 / 0.3048
        );
        assert_eq!(LengthUnits::Meters.from(1.0, LengthUnits::Feet), 0.3048);
    }
}
