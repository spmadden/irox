// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! This module contains the basic types and conversions for the SI "Temperature" quantity
use crate::units::{FromUnits, Unit};

///
/// Represents a specific temperature unit - SI or otherwise
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum TemperatureUnits {
    #[default]
    Kelvin,

    Celsius,

    Centigrade,

    Farenheit,

    Rankine,
}

macro_rules! from_units_length {
    ($type:ident) => {
        impl crate::units::FromUnits<$type> for TemperatureUnits {
            fn from(&self, value: $type, source_unit: Self) -> $type {
                match self {
                    // target
                    TemperatureUnits::Kelvin => match source_unit {
                        // source
                        TemperatureUnits::Kelvin => value,
                        TemperatureUnits::Celsius => {
                            (value + CELSIUS_KELVIN_OFFSET as $type) as $type
                        }
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            }
        }
    };
}

basic_unit!(Temperature, TemperatureUnits, Kelvin);
from_units_length!(f32);
from_units_length!(f64);

impl Unit<TemperatureUnits> for Temperature {
    fn as_unit(&self, units: TemperatureUnits) -> Self
    where
        Self: Sized,
    {
        Temperature {
            value: units.from(self.value, self.units),
            units,
        }
    }
}

impl Temperature {
    #[must_use]
    pub fn new_celsius(value: f64) -> Temperature {
        Self::new(value, TemperatureUnits::Celsius)
    }
}

pub const CELSIUS_KELVIN_OFFSET: f64 = 273.15;
pub const CELSIUS_FARENHEIT_OFFSET: f64 = 32.;
#[must_use]
pub fn celsius2kelvin(cel: f64) -> f64 {
    cel + CELSIUS_KELVIN_OFFSET
}
#[must_use]
pub fn kelvin2celsius(kel: f64) -> f64 {
    kel - CELSIUS_KELVIN_OFFSET
}
#[must_use]
pub fn celsius2farenheit(cel: f64) -> f64 {
    cel * 1.8 - CELSIUS_KELVIN_OFFSET
}
#[must_use]
pub fn fahrenheit2celsius(far: f64) -> f64 {
    (far - CELSIUS_KELVIN_OFFSET) / 1.8
}
// pub fn rankine2kelvin()

#[cfg(test)]
mod test {
    use crate::units::temperature::{celsius2kelvin, CELSIUS_KELVIN_OFFSET};

    #[test]
    pub fn test() {
        let val: f32 = 0.0;
        let out = celsius2kelvin(f64::from(val));
        assert_eq!(out, CELSIUS_KELVIN_OFFSET)
    }
}
