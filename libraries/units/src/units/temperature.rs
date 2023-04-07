//!
//! This module contains the basic types and conversions for the SI "Temperature" quantity
use crate::units::{FromUnits, Unit};
use std::ops::{Add, Div, Mul};

///
/// Represents a specific temperature unit - SI or otherwise
#[derive(Debug, Clone, Copy, Default)]
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
            fn from(&self, value: $type, units: Self) -> $type {
                match self {
                    // target
                    TemperatureUnits::Kelvin => match units {
                        // source
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
    fn as_unit(&self, unit: TemperatureUnits) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Temperature {
    pub fn new_celsius(value: f64) -> Temperature {
        Self::new(value, TemperatureUnits::Celsius)
    }
}

pub const CELSIUS_KELVIN_OFFSET: f64 = 273.15;
pub const CELSIUS_FARENHEIT_OFFSET: f64 = 32.;
pub fn celsius2kelvin(cel: f64) -> f64 {
    cel + CELSIUS_KELVIN_OFFSET
}
pub fn kelvin2celsius(kel: f64) -> f64 {
    kel - CELSIUS_KELVIN_OFFSET
}
pub fn celsius2farenheit(cel: f64) -> f64 {
    cel * 1.8 - CELSIUS_KELVIN_OFFSET
}
pub fn fahrenheit2celsius(far: f64) -> f64 {
    (far - CELSIUS_KELVIN_OFFSET) / 1.8
}
// pub fn rankine2kelvin()

#[cfg(test)]
mod test {
    use crate::units::temperature::celsius2kelvin;

    #[test]
    pub fn test() {
        let val: f32 = 0.0;
        let out = celsius2kelvin(val as f64);
    }
}
