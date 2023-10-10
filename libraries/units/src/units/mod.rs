// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

///
/// Matches (struct, units, default) to make a new basic struct

#[macro_export]
macro_rules! basic_unit {
    ($struct_type:ident, $units_type: ident, $default_units: ident) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq)]
        pub struct $struct_type {
            value: f64,
            units: $units_type,
        }

        impl $struct_type {
            #[must_use]
            pub const fn new(value: f64, units: $units_type) -> Self {
                Self { value, units }
            }

            #[must_use]
            pub fn value(&self) -> f64 {
                self.value
            }

            #[must_use]
            pub fn units(&self) -> $units_type {
                self.units
            }
        }

        impl $crate::units::UnitStruct<$units_type> for $struct_type {
            fn new(value: f64, units: $units_type) -> Self {
                Self { value, units }
            }

            fn value(&self) -> f64 {
                self.value
            }

            fn units(&self) -> $units_type {
                self.units
            }
        }

        impl std::ops::Add for $struct_type {
            type Output = $struct_type;

            fn add(self, rhs: Self) -> Self::Output {
                let val = $crate::units::Unit::<$units_type>::as_unit(&rhs, self.units()).value();
                $crate::units::UnitStruct::<$units_type>::new(self.value() + val, self.units())
            }
        }

        impl std::ops::Add for &$struct_type {
            type Output = $struct_type;

            fn add(self, rhs: Self) -> Self::Output {
                let val = $crate::units::Unit::<$units_type>::as_unit(rhs, self.units()).value();
                $crate::units::UnitStruct::<$units_type>::new(self.value() + val, self.units())
            }
        }

        impl std::ops::Add<&Self> for &$struct_type {
            type Output = $struct_type;

            fn add(self, rhs: &Self) -> Self::Output {
                let val = $crate::units::Unit::<$units_type>::as_unit(*rhs, self.units()).value();
                $crate::units::UnitStruct::<$units_type>::new(self.value() + val, self.units())
            }
        }

        impl std::ops::AddAssign for $struct_type {
            fn add_assign(&mut self, rhs: Self) {
                let val = $crate::units::Unit::<$units_type>::as_unit(&rhs, self.units()).value();
                self.value += val;
            }
        }

        impl std::ops::Sub for $struct_type {
            type Output = $struct_type;

            fn sub(self, rhs: Self) -> Self::Output {
                let val = $crate::units::Unit::<$units_type>::as_unit(&rhs, self.units()).value();
                $crate::units::UnitStruct::<$units_type>::new(self.value() - val, self.units())
            }
        }

        impl<'a> std::ops::Sub<&'a $struct_type> for &'a $struct_type {
            type Output = $struct_type;

            fn sub(self, rhs: Self) -> Self::Output {
                let val = $crate::units::Unit::<$units_type>::as_unit(rhs, self.units()).value();
                $crate::units::UnitStruct::<$units_type>::new(self.value() - val, self.units())
            }
        }

        impl std::ops::Div<f64> for $struct_type {
            type Output = $struct_type;

            fn div(self, rhs: f64) -> Self::Output {
                $crate::units::UnitStruct::<$units_type>::new(self.value() / rhs, self.units())
            }
        }

        impl std::ops::Div for $struct_type {
            type Output = f64;

            fn div(self, rhs: Self) -> Self::Output {
                let upper = self.value();
                let lower = $crate::units::Unit::<$units_type>::as_unit(&rhs, self.units()).value();
                upper / lower
            }
        }

        impl std::ops::DivAssign<f64> for $struct_type {
            fn div_assign(&mut self, rhs: f64) {
                self.value /= rhs
            }
        }

        impl std::ops::Mul<f64> for $struct_type {
            type Output = $struct_type;

            fn mul(self, rhs: f64) -> Self::Output {
                $crate::units::UnitStruct::<$units_type>::new(self.value() * rhs, self.units())
            }
        }

        impl std::ops::Mul<f64> for &$struct_type {
            type Output = $struct_type;

            fn mul(self, rhs: f64) -> Self::Output {
                $crate::units::UnitStruct::<$units_type>::new(self.value() * rhs, self.units())
            }
        }
        impl std::ops::Mul<&$struct_type> for f64 {
            type Output = $struct_type;

            fn mul(self, rhs: &$struct_type) -> Self::Output {
                rhs * self
            }
        }
        impl std::ops::Mul<$struct_type> for f64 {
            type Output = $struct_type;

            fn mul(self, rhs: $struct_type) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::MulAssign<f64> for $struct_type {
            fn mul_assign(&mut self, rhs: f64) {
                self.value *= rhs
            }
        }

        impl std::cmp::PartialOrd<$struct_type> for $struct_type {
            fn partial_cmp(&self, rhs: &$struct_type) -> Option<std::cmp::Ordering> {
                let val = $crate::units::Unit::<$units_type>::as_unit(rhs, self.units()).value();
                self.value().partial_cmp(&val)
            }
        }

        pub const ZERO: $struct_type = $struct_type::new(0.0, $units_type::$default_units);
    };
}

///
/// Trait to allow the direct conversion of a unit scalar to another unit scalar
pub trait FromUnits<T> {
    /// Converts the value with unit units into this (self) unit
    fn from(&self, value: T, units: Self) -> T;
}

///
/// Represents a Value/Unit pairing
pub trait UnitStruct<T>: Unit<T> {
    /// Creates a new type
    fn new(value: f64, units: T) -> Self
    where
        Self: Sized;

    /// Returns the value of this struct
    fn value(&self) -> f64;

    /// Returns the unit type of this struct
    fn units(&self) -> T;
}

///
/// Trait to provide access to unit conversions
pub trait Unit<T> {
    #[must_use]
    fn as_unit(&self, unit: T) -> Self
    where
        Self: Sized;
}

pub mod angle;
pub mod compass;
pub mod datasize;
pub mod length;
pub mod speed;
pub mod temperature;
