// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Contains [`Temperature`] and [`TemperatureUnits`] - physical measurements of the SI "Temperature" quantity
//!
use crate::units::{FromUnits, Unit};

///
/// Represents a specific temperature unit - SI or otherwise
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum TemperatureUnits {
    /// Referenced `0` is Absolute Zero and counts up at the same rate as [`Celsius`].  0 Kelvin is
    /// -273.15 Celsius, -459.67 Fahrenheit, and 0 Rankine
    #[default]
    Kelvin,

    /// Referenced such that `0` is the melting point of Ice, and `100` is the boiling point of water.
    /// Increments at the same rate as `Kelvin` as a constant offset.  0 Celsius is 273.15 Kelvin,
    /// 32 Fahrenheit, and 491.67 Rankine.
    Celsius,

    /// Freedom unit referenced such that `0` is the freezing point of a salty bath, and `100` is
    /// the approximate value of a human body.  0 Fahrenheit is -17.78 Celsius, 255.37 Kelvin, and
    /// 459.67 Rankine
    Fahrenheit,

    /// Fahrenheit, except referenced to Absolute Zero.  0 Rankine is 0 Kelvin, -273.15 Celsius and
    /// -459.67 Fahrenheit.
    Rankine,
}
macro_rules! scale {
    ($val:expr,$factor:expr,$type:ident) => {
        $val * $factor as $type
    };
}
macro_rules! offset {
    ($val:expr,$shift:expr,$type:ident) => {
        $val + $shift as $type
    };
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
                        TemperatureUnits::Celsius => offset!(value, CELSIUS_KELVIN_OFFSET, $type),
                        TemperatureUnits::Fahrenheit => scale!(
                            offset!(value, FAHRENHEIT_RANKINE_OFFSET, $type),
                            (1.0 / CELSIUS_FAHRENHEIT_FACTOR),
                            $type
                        ),
                        TemperatureUnits::Rankine => {
                            scale!(value, (1.0 / CELSIUS_FAHRENHEIT_FACTOR), $type)
                        }
                    },
                    TemperatureUnits::Celsius => match source_unit {
                        TemperatureUnits::Kelvin => offset!(value, -CELSIUS_KELVIN_OFFSET, $type),
                        TemperatureUnits::Celsius => value,
                        TemperatureUnits::Fahrenheit => scale!(
                            offset!(value, -CELSIUS_FAHRENHEIT_OFFSET, $type),
                            (1.0 / CELSIUS_FAHRENHEIT_FACTOR),
                            $type
                        ),
                        TemperatureUnits::Rankine => offset!(
                            scale!(value, (1.0 / CELSIUS_FAHRENHEIT_FACTOR), $type),
                            -CELSIUS_KELVIN_OFFSET,
                            $type
                        ),
                    },
                    TemperatureUnits::Fahrenheit => match source_unit {
                        TemperatureUnits::Kelvin => offset!(
                            scale!(value, CELSIUS_FAHRENHEIT_FACTOR, $type),
                            -FAHRENHEIT_RANKINE_OFFSET,
                            $type
                        ),
                        TemperatureUnits::Celsius => offset!(
                            scale!(value, CELSIUS_FAHRENHEIT_FACTOR, $type),
                            CELSIUS_FAHRENHEIT_OFFSET,
                            $type
                        ),
                        TemperatureUnits::Fahrenheit => value,
                        TemperatureUnits::Rankine => {
                            offset!(value, -FAHRENHEIT_RANKINE_OFFSET, $type)
                        }
                    },
                    TemperatureUnits::Rankine => match source_unit {
                        TemperatureUnits::Kelvin => scale!(value, CELSIUS_FAHRENHEIT_FACTOR, $type),
                        TemperatureUnits::Celsius => scale!(
                            offset!(value, CELSIUS_KELVIN_OFFSET, $type),
                            CELSIUS_FAHRENHEIT_FACTOR,
                            $type
                        ),
                        TemperatureUnits::Fahrenheit => {
                            offset!(value, FAHRENHEIT_RANKINE_OFFSET, $type)
                        }
                        TemperatureUnits::Rankine => value,
                    },
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

    #[must_use]
    pub fn new_fahrenheit(value: f64) -> Temperature {
        Self::new(value, TemperatureUnits::Fahrenheit)
    }

    #[must_use]
    pub fn new_kelvin(value: f64) -> Temperature {
        Self::new(value, TemperatureUnits::Kelvin)
    }

    #[must_use]
    pub fn new_rankine(value: f64) -> Temperature {
        Self::new(value, TemperatureUnits::Rankine)
    }

    #[must_use]
    pub fn as_celsius(&self) -> Temperature {
        self.as_unit(TemperatureUnits::Celsius)
    }

    #[must_use]
    pub fn as_kelvin(&self) -> Temperature {
        self.as_unit(TemperatureUnits::Kelvin)
    }

    #[must_use]
    pub fn as_fahrenheit(&self) -> Temperature {
        self.as_unit(TemperatureUnits::Fahrenheit)
    }

    #[must_use]
    pub fn as_rankine(&self) -> Temperature {
        self.as_unit(TemperatureUnits::Rankine)
    }
}

pub const CELSIUS_KELVIN_OFFSET: f64 = 273.15;
pub const CELSIUS_FAHRENHEIT_OFFSET: f64 = 32.;
pub const CELSIUS_FAHRENHEIT_FACTOR: f64 = 1.8;
pub const FAHRENHEIT_RANKINE_OFFSET: f64 = 459.67;

#[cfg(test)]
mod test {
    use crate::units::temperature::Temperature;

    #[test]
    pub fn tests() {
        let abs_k = Temperature::new_kelvin(0.0);
        assert_eq!(abs_k.as_kelvin().value, 0.0);
        assert_eq!(abs_k.as_celsius().value, -273.15);
        assert_eq!(abs_k.as_fahrenheit().value, -459.67);
        assert_eq!(abs_k.as_rankine().value, 0.0);

        let zerof = Temperature::new_fahrenheit(0.0);
        assert_eq!(zerof.as_kelvin().value, 255.37222222222223);
        assert_eq!(zerof.as_celsius().value, -17.77777777777778);
        assert_eq!(zerof.as_fahrenheit().value, 0.0);
        assert_eq!(zerof.as_rankine().value, 459.67);

        let zeroc = Temperature::new_celsius(0.0);
        assert_eq!(zeroc.as_kelvin().value, 273.15);
        assert_eq!(zeroc.as_celsius().value, 0.0);
        assert_eq!(zeroc.as_fahrenheit().value, 32.0);
        assert_eq!(zeroc.as_rankine().value, 491.66999999999996);

        let zeror = Temperature::new_rankine(0.0);
        assert_eq!(zeror.as_kelvin().value, 0.0);
        assert_eq!(zeror.as_celsius().value, -273.15);
        assert_eq!(zeror.as_fahrenheit().value, -459.67);
        assert_eq!(zeror.as_rankine().value, 0.0);

        let stp = Temperature::new_celsius(15.0);
        assert_eq!(stp.as_kelvin().value, 288.15);
        assert_eq!(stp.as_celsius().value, 15.0);
        assert_eq!(stp.as_fahrenheit().value, 59.);
        assert_eq!(stp.as_rankine().value, 518.67);

        let boil = Temperature::new_kelvin(373.1339);
        assert_eq!(boil.as_kelvin().value, 373.1339);
        assert_eq!(boil.as_celsius().value, 99.9839);
        assert_eq!(boil.as_fahrenheit().value, 211.97102);
        assert_eq!(boil.as_rankine().value, 671.64102);

        let boil = Temperature::new_celsius(99.9839);
        assert_eq!(boil.as_kelvin().value, 373.1339);
        assert_eq!(boil.as_celsius().value, 99.9839);
        assert_eq!(boil.as_fahrenheit().value, 211.97102);
        assert_eq!(boil.as_rankine().value, 671.64102);

        let boil = Temperature::new_fahrenheit(211.97102);
        assert_eq!(boil.as_kelvin().value, 373.13390000000004);
        assert_eq!(boil.as_celsius().value, 99.9839);
        assert_eq!(boil.as_fahrenheit().value, 211.97102);
        assert_eq!(boil.as_rankine().value, 671.64102);

        let boil = Temperature::new_rankine(671.64102);
        assert_eq!(boil.as_kelvin().value, 373.13390000000004);
        assert_eq!(boil.as_celsius().value, 99.98390000000006);
        assert_eq!(boil.as_fahrenheit().value, 211.97102);
        assert_eq!(boil.as_rankine().value, 671.64102);
    }
}
