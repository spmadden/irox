// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! This module contains the basic types and conversions for the SI "Length" quantity
use crate::units::{FromUnits, Unit};
use core::fmt::{Display, Formatter};
use irox_fixedmath::FixedI64;

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

    /// The U.S. Survey Foot
    USSurveyFoot,
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
                        LengthUnits::Feet => value * FEET_TO_METERS_F64 as $type,
                        LengthUnits::Kilometers => value * KILOMETERS_TO_METERS_F64 as $type,
                        LengthUnits::Mile => value * MILES_TO_METERS_F64 as $type,
                        LengthUnits::NauticalMile => value * NAUTICAL_MILES_TO_METERS_F64 as $type,
                        LengthUnits::USSurveyFoot => value * SURVEYFOOT_TO_METER_F64 as $type,
                    },
                    LengthUnits::Feet => match source_unit {
                        LengthUnits::Meters => value * METERS_TO_FEET_F64 as $type,
                        LengthUnits::Feet => value as $type,
                        LengthUnits::Kilometers => {
                            FromUnits::<$type>::from(&LengthUnits::Meters, value, source_unit)
                                * METERS_TO_KILOMETERS_F64 as $type
                        }
                        LengthUnits::Mile => {
                            FromUnits::<$type>::from(&LengthUnits::Meters, value, source_unit)
                                * METERS_TO_MILES_F64 as $type
                        }
                        LengthUnits::NauticalMile => {
                            FromUnits::<$type>::from(&LengthUnits::Meters, value, source_unit)
                                * METERS_TO_NAUTICAL_MILE_F64 as $type
                        }
                        LengthUnits::USSurveyFoot => {
                            value * (SURVEYFOOT_TO_METER_F64 * METERS_TO_FEET_F64) as $type
                        }
                    },
                    LengthUnits::Kilometers => match source_unit {
                        LengthUnits::Meters => value * METERS_TO_KILOMETERS_F64 as $type,
                        LengthUnits::Kilometers => value,
                        LengthUnits::Feet => {
                            value * (FEET_TO_METERS_F64 * METERS_TO_KILOMETERS_F64) as $type
                        }
                        LengthUnits::Mile => {
                            value * (MILES_TO_METERS_F64 * METERS_TO_KILOMETERS_F64) as $type
                        }
                        LengthUnits::NauticalMile => {
                            value
                                * (NAUTICAL_MILES_TO_METERS_F64 * METERS_TO_KILOMETERS_F64) as $type
                        }
                        LengthUnits::USSurveyFoot => {
                            value * (SURVEYFOOT_TO_METER_F64 * METERS_TO_KILOMETERS_F64) as $type
                        }
                    },
                    LengthUnits::Mile => match source_unit {
                        LengthUnits::Meters => value * METERS_TO_MILES_F64 as $type,
                        LengthUnits::Kilometers => {
                            value * (KILOMETERS_TO_METERS_F64 * METERS_TO_MILES_F64) as $type
                        }
                        LengthUnits::Feet => {
                            value * (FEET_TO_METERS_F64 * METERS_TO_MILES_F64) as $type
                        }
                        LengthUnits::Mile => value,
                        LengthUnits::NauticalMile => {
                            value * (NAUTICAL_MILES_TO_METERS_F64 * METERS_TO_MILES_F64) as $type
                        }
                        LengthUnits::USSurveyFoot => {
                            value * (SURVEYFOOT_TO_METER_F64 * METERS_TO_MILES_F64) as $type
                        }
                    },
                    LengthUnits::NauticalMile => match source_unit {
                        LengthUnits::Meters => value * METERS_TO_NAUTICAL_MILE_F64 as $type,
                        LengthUnits::Kilometers => {
                            value
                                * (KILOMETERS_TO_METERS_F64 * METERS_TO_NAUTICAL_MILE_F64) as $type
                        }
                        LengthUnits::Feet => {
                            value * (FEET_TO_METERS_F64 * METERS_TO_NAUTICAL_MILE_F64) as $type
                        }
                        LengthUnits::Mile => {
                            value * (MILES_TO_METERS_F64 * METERS_TO_NAUTICAL_MILE_F64) as $type
                        }
                        LengthUnits::NauticalMile => value,
                        LengthUnits::USSurveyFoot => {
                            value * (SURVEYFOOT_TO_METER_F64 * METERS_TO_NAUTICAL_MILE_F64) as $type
                        }
                    },
                    LengthUnits::USSurveyFoot => match source_unit {
                        LengthUnits::Meters => value * METER_TO_SURVEYFOOT_F64 as $type,
                        LengthUnits::Kilometers => {
                            value * (KILOMETERS_TO_METERS_F64 * METER_TO_SURVEYFOOT_F64) as $type
                        }
                        LengthUnits::Feet => {
                            value * (FEET_TO_METERS_F64 * METER_TO_SURVEYFOOT_F64) as $type
                        }
                        LengthUnits::Mile => {
                            value * (MILES_TO_METERS_F64 * METER_TO_SURVEYFOOT_F64) as $type
                        }
                        LengthUnits::NauticalMile => {
                            value
                                * (NAUTICAL_MILES_TO_METERS_F64 * METER_TO_SURVEYFOOT_F64) as $type
                        }
                        LengthUnits::USSurveyFoot => value,
                    },
                }
            }
        }
    };
}
basic_unit!(Length, LengthUnits, Meters);
from_units_length!(f32);
from_units_length!(f64);

impl crate::units::FromUnits<FixedI64> for LengthUnits {
    fn from(&self, value: FixedI64, source_unit: Self) -> FixedI64 {
        match self {
            // target
            LengthUnits::Meters => match source_unit {
                // source
                LengthUnits::Meters => value,
                LengthUnits::Feet => value * FEET_TO_METERS,
                LengthUnits::Kilometers => value * KILOMETERS_TO_METERS,
                LengthUnits::Mile => value * MILES_TO_METERS,
                LengthUnits::NauticalMile => value * NAUTICAL_MILES_TO_METERS,
                LengthUnits::USSurveyFoot => value * SURVEYFOOT_TO_METER,
            },
            LengthUnits::Feet => match source_unit {
                LengthUnits::Meters => value * METERS_TO_FEET,
                LengthUnits::Feet => value,
                LengthUnits::Kilometers => {
                    FromUnits::<_>::from(&LengthUnits::Meters, value, source_unit)
                        * METERS_TO_KILOMETERS
                }
                LengthUnits::Mile => {
                    FromUnits::<_>::from(&LengthUnits::Meters, value, source_unit) * METERS_TO_MILES
                }
                LengthUnits::NauticalMile => {
                    FromUnits::<_>::from(&LengthUnits::Meters, value, source_unit)
                        * METERS_TO_NAUTICAL_MILE
                }
                LengthUnits::USSurveyFoot => value * (SURVEYFOOT_TO_METER * METERS_TO_FEET),
            },
            LengthUnits::Kilometers => match source_unit {
                LengthUnits::Meters => value * METERS_TO_KILOMETERS,
                LengthUnits::Kilometers => value,
                LengthUnits::Feet => value * (FEET_TO_METERS * METERS_TO_KILOMETERS),
                LengthUnits::Mile => value * (MILES_TO_METERS * METERS_TO_KILOMETERS),
                LengthUnits::NauticalMile => {
                    value * (NAUTICAL_MILES_TO_METERS * METERS_TO_KILOMETERS)
                }
                LengthUnits::USSurveyFoot => value * (SURVEYFOOT_TO_METER * METERS_TO_KILOMETERS),
            },
            LengthUnits::Mile => match source_unit {
                LengthUnits::Meters => value * METERS_TO_MILES,
                LengthUnits::Kilometers => value * (KILOMETERS_TO_METERS * METERS_TO_MILES),
                LengthUnits::Feet => value * (FEET_TO_METERS * METERS_TO_MILES),
                LengthUnits::Mile => value,
                LengthUnits::NauticalMile => value * (NAUTICAL_MILES_TO_METERS * METERS_TO_MILES),
                LengthUnits::USSurveyFoot => value * (SURVEYFOOT_TO_METER * METERS_TO_MILES),
            },
            LengthUnits::NauticalMile => match source_unit {
                LengthUnits::Meters => value * METERS_TO_NAUTICAL_MILE,
                LengthUnits::Kilometers => value * (KILOMETERS_TO_METERS * METERS_TO_NAUTICAL_MILE),
                LengthUnits::Feet => value * (FEET_TO_METERS * METERS_TO_NAUTICAL_MILE),
                LengthUnits::Mile => value * (MILES_TO_METERS * METERS_TO_NAUTICAL_MILE),
                LengthUnits::NauticalMile => value,
                LengthUnits::USSurveyFoot => {
                    value * (SURVEYFOOT_TO_METER * METERS_TO_NAUTICAL_MILE)
                }
            },
            LengthUnits::USSurveyFoot => match source_unit {
                LengthUnits::Meters => value * METER_TO_SURVEYFOOT,
                LengthUnits::Kilometers => value * (KILOMETERS_TO_METERS * METER_TO_SURVEYFOOT),
                LengthUnits::Feet => value * (FEET_TO_METERS * METER_TO_SURVEYFOOT),
                LengthUnits::Mile => value * (MILES_TO_METERS * METER_TO_SURVEYFOOT),
                LengthUnits::NauticalMile => {
                    value * (NAUTICAL_MILES_TO_METERS * METER_TO_SURVEYFOOT)
                }
                LengthUnits::USSurveyFoot => value,
            },
        }
    }
}

impl LengthUnits {
    pub const fn short_name(&self) -> &'static str {
        match self {
            LengthUnits::Meters => "m",
            LengthUnits::Kilometers => "km",
            LengthUnits::Feet => "ft",
            LengthUnits::Mile => "mi",
            LengthUnits::NauticalMile => "nmi",
            LengthUnits::USSurveyFoot => "ussft",
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
    pub fn new_meters(value: f64) -> Length {
        Self {
            value: value.into(),
            units: LengthUnits::Meters,
        }
    }

    #[must_use]
    pub fn new_feet(value: f64) -> Length {
        Self {
            value: value.into(),
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

pub const FEET_TO_METERS_F64: f64 = 3.048E-01; // Exact, as per NIST 811.2008
pub const FEET_TO_METERS: FixedI64 = FixedI64::from_parts(0, 1309106032);
pub const METERS_TO_FEET_F64: f64 = 1. / FEET_TO_METERS_F64;
pub const METERS_TO_FEET: FixedI64 = FixedI64::from_parts(3, 1206198164);
pub const MILES_TO_METERS_F64: f64 = 1.609_344E3; // Exact, as per NIST 811.2008
pub const MILES_TO_METERS: FixedI64 = FixedI64::from_parts(1609, 1477468749);
pub const METERS_TO_MILES_F64: f64 = 1. / MILES_TO_METERS_F64;
pub const METERS_TO_MILES: FixedI64 = FixedI64::from_parts(0, 2668769);
pub const KILOMETERS_TO_METERS_F64: f64 = 1000.;
pub const KILOMETERS_TO_METERS: FixedI64 = FixedI64::from_parts(1000, 0);
pub const METERS_TO_KILOMETERS_F64: f64 = 1. / KILOMETERS_TO_METERS_F64;
pub const METERS_TO_KILOMETERS: FixedI64 = FixedI64::from_parts(1000, 4294967);
pub const NAUTICAL_MILES_TO_METERS_F64: f64 = 1.852E3;
pub const NAUTICAL_MILES_TO_METERS: FixedI64 = FixedI64::from_parts(1852, 0);
pub const METERS_TO_NAUTICAL_MILE_F64: f64 = 1. / NAUTICAL_MILES_TO_METERS_F64;
pub const METERS_TO_NAUTICAL_MILE: FixedI64 = FixedI64::from_parts(0, 2319097);

pub const SURVEYFOOT_TO_METER_F64: f64 = 3.048006E-1;
pub const SURVEYFOOT_TO_METER: FixedI64 = FixedI64::from_parts(0, 1309108608);
pub const METER_TO_SURVEYFOOT_F64: f64 = 1. / SURVEYFOOT_TO_METER_F64;
pub const METER_TO_SURVEYFOOT: FixedI64 = FixedI64::from_parts(3, 14091072311);

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

    #[test]
    pub fn test_meters_kilometers() {
        assert_eq!(
            LengthUnits::Meters.from(1.0, LengthUnits::Kilometers),
            1000.
        );
        assert_eq!(
            LengthUnits::Kilometers.from(1000.0, LengthUnits::Meters),
            1.
        );
    }
}
