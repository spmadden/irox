// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

///
/// Matches (struct, units, default) to make a new basic struct
macro_rules! impl_unitstruct {
    ($struct_type:ident, $units_type: ident, $($slf:ty)+) => {
        impl $crate::units::UnitStruct<$units_type> for $($slf)+ {
            type Output = $struct_type;

            fn new(value: f64, units: $units_type) -> $struct_type {
                $struct_type { value, units }
            }

            fn value(&self) -> f64 {
                self.value
            }

            fn units(&self) -> $units_type {
                self.units
            }
        }
        impl Unit<$units_type> for $($slf)+ {
            type Output = $struct_type;
            fn as_unit(&self, units: $units_type) -> $struct_type
            where
                Self: Sized,
            {
                $struct_type {
                    value: units.from(self.value, self.units),
                    units,
                }
            }
        }
    };
}

macro_rules! impl_sub {
    ($($out:ty)+, $units_type:ident, $($sub:ty)+, $($slf:ty)+) => {
        impl<'a> core::ops::Sub<$($sub)+> for $($slf)+ {
            type Output = $($out)+;

            fn sub(self, rhs: $($sub)+) -> $($out)+ {
                let val = <$($sub)+ as $crate::units::Unit::<$units_type>>::as_unit(&rhs, self.units()).value();
                <$($slf)+ as $crate::units::UnitStruct::<$units_type>>::new(self.value() - val, self.units())
            }
        }
    };
}
macro_rules! impl_add {
    ($($out:ty)+, $units_type:ident, $($add:ty)+, $($slf:ty)+) => {
        impl<'a> core::ops::Add<$($add)+> for $($slf)+ {
            type Output = $($out)+;

            fn add(self, rhs: $($add)+) -> $($out)+ {
                let val = <$($add)+ as $crate::units::Unit::<$units_type>>::as_unit(&rhs, self.units()).value();
                <$($slf)+ as $crate::units::UnitStruct::<$units_type>>::new(self.value() + val, self.units())
            }
        }
    };
}
macro_rules! impl_subassign {
    ($($out:ty)+, $units_type:ident, $($sub:ty)+, $($slf:ty)+) => {
        impl<'a> core::ops::SubAssign<$($sub)+> for $($slf)+ {
            fn sub_assign(&mut self, rhs: $($sub)+) {
                let val = <$($sub)+ as $crate::units::Unit::<$units_type>>::as_unit(&rhs, self.units()).value();
                self.value -= val;
            }

        }
    };
}
macro_rules! impl_addassign {
    ($($out:ty)+, $units_type:ident, $($add:ty)+, $($slf:ty)+) => {
        impl<'a> core::ops::AddAssign<$($add)+> for $($slf)+ {
            fn add_assign(&mut self, rhs: $($add)+) {
                let val = <$($add)+ as $crate::units::Unit::<$units_type>>::as_unit(&rhs, self.units()).value();
                self.value += val;
            }
        }
    };
}
macro_rules! impl_div {
    ($($out:ty)+, $units_type:ident, $($div:ty)+, $($slf:ty)+) => {
        impl<'a> core::ops::Div<$($div)+> for $($slf)+ {
            type Output = $($out)+ ;
            fn div(self, rhs: $($div)+) -> $($out)+  {
                <$($slf)+ as $crate::units::UnitStruct::<$units_type>>::new(self.value() / rhs, self.units())
            }
        }
        impl<'a> core::ops::Div<$($slf)+> for $($slf)+ {
            type Output = f64;
            fn div(self, rhs: $($slf)+) -> Self::Output {
                let upper = self.value();
                let lower = <$($slf)+ as $crate::units::Unit::<$units_type>>::as_unit(&rhs, self.units()).value();
                upper / lower
            }
        }
    };
}

macro_rules! impl_divassign {
    ($($out:ty)+, $units_type:ident, $($div:ty)+, $($slf:ty)+) => {
        impl<'a> core::ops::DivAssign<$($div)+> for $($slf)+ {
            fn div_assign(&mut self, rhs: $($div)+) {
                self.value /= rhs;
            }
        }
    };
}

macro_rules! impl_mul {
    ($($out:ty)+, $units_type:ident, $($mul:ty)+, $($slf:ty)+) => {
         impl<'a> core::ops::Mul<$($mul)+> for $($slf)+ {
            type Output = $($out)+ ;
            fn mul(self, rhs: $($mul)+) -> $($out)+  {
                <$($slf)+ as $crate::units::UnitStruct::<$units_type>>::new(self.value() * rhs, self.units())
            }
        }
        impl<'a> core::ops::Mul<$($slf)+> for $($slf)+ {
            type Output = $($mul)+;
            fn mul(self, rhs: $($slf)+) -> Self::Output {
                let upper = self.value();
                let lower = <$($slf)+ as $crate::units::Unit::<$units_type>>::as_unit(&rhs, self.units()).value();
                upper * lower
            }
        }
        impl<'a> core::ops::Mul<$($slf)+> for f64 {
            type Output = $($out)+;
            fn mul(self, rhs: $($slf)+) -> Self::Output {
                rhs * self
            }
        }
    };
}
macro_rules! impl_mulassign {
    ($($out:ty)+, $units_type:ident, $($mul:ty)+, $($slf:ty)+) => {
        impl<'a> core::ops::MulAssign<$($mul)+> for $($slf)+ {
            fn mul_assign(&mut self, rhs: $($mul)+) {
                self.value *= rhs;
            }
        }
        // impl core::ops::MulAssign<f64> for $struct_type {
        //     fn mul_assign(&mut self, rhs: f64) {
        //         self.value *= rhs
        //     }
        // }
    };
}

macro_rules! impl_op {
    ($op:ident, $units_type:ident, $($operand:ty)+) => {
        $op!($($operand)+, $units_type, $($operand)+, $($operand)+);
        $op!($($operand)+, $units_type, $($operand)+, &$($operand)+);
        $op!($($operand)+, $units_type, $($operand)+, &mut $($operand)+);
        $op!($($operand)+, $units_type, &$($operand)+, $($operand)+);
        $op!($($operand)+, $units_type, &$($operand)+, &$($operand)+);
        $op!($($operand)+, $units_type, &$($operand)+, &mut $($operand)+);
        $op!($($operand)+, $units_type, &mut $($operand)+, $($operand)+);
        $op!($($operand)+, $units_type, &mut $($operand)+, &$($operand)+);
        $op!($($operand)+, $units_type, &mut $($operand)+, &mut $($operand)+);
    };
}
macro_rules! impl_mutop {
    ($op:ident, $units_type:ident, $($operand:ty)+) => {
        $op!($($operand)+, $units_type, $($operand)+, $($operand)+);
        $op!($($operand)+, $units_type, $($operand)+, &mut $($operand)+);
        $op!($($operand)+, $units_type, &$($operand)+, $($operand)+);
        $op!($($operand)+, $units_type, &$($operand)+, &mut $($operand)+);
        $op!($($operand)+, $units_type, &mut $($operand)+, $($operand)+);
        $op!($($operand)+, $units_type, &mut $($operand)+, &mut $($operand)+);
    };
}

#[macro_export]
macro_rules! basic_unit {
    ($struct_type:ident, $units_type: ident, $default_units: ident) => {
        #[derive(Debug, Clone, Copy, Default)]
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

        impl_unitstruct!($struct_type, $units_type, $struct_type);
        impl_unitstruct!($struct_type, $units_type, &$struct_type);
        impl_unitstruct!($struct_type, $units_type, &mut $struct_type);

        impl_op!(impl_sub, $units_type, $struct_type);
        impl_mutop!(impl_subassign, $units_type, $struct_type);
        impl_op!(impl_add, $units_type, $struct_type);
        impl_mutop!(impl_addassign, $units_type, $struct_type);
        impl_div!($struct_type, $units_type, f64, $struct_type);
        impl_div!($struct_type, $units_type, f64, &$struct_type);
        impl_div!($struct_type, $units_type, f64, &mut $struct_type);
        impl_divassign!($struct_type, $units_type, f64, $struct_type);
        impl_divassign!($struct_type, $units_type, f64, &mut $struct_type);
        impl_mul!($struct_type, $units_type, f64, $struct_type);
        impl_mul!($struct_type, $units_type, f64, &$struct_type);
        impl_mul!($struct_type, $units_type, f64, &mut $struct_type);
        impl_mulassign!($struct_type, $units_type, f64, $struct_type);
        impl_mulassign!($struct_type, $units_type, f64, &mut $struct_type);

        impl core::cmp::PartialOrd<$struct_type> for $struct_type {
            fn partial_cmp(&self, rhs: &$struct_type) -> Option<core::cmp::Ordering> {
                Some(self.cmp(rhs))
            }
        }
        impl core::cmp::Ord for $struct_type {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.value.total_cmp(&other.as_unit(self.units).value)
            }
        }
        impl core::cmp::Eq for $struct_type {}

        impl core::cmp::PartialEq<$struct_type> for $struct_type {
            fn eq(&self, rhs: &$struct_type) -> bool {
                if let Some(val) = self.partial_cmp(rhs) {
                    return val == core::cmp::Ordering::Equal;
                }
                false
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
    type Output;

    /// Creates a new type
    fn new(value: f64, units: T) -> <Self as UnitStruct<T>>::Output;

    /// Returns the value of this struct
    fn value(&self) -> f64;

    /// Returns the unit type of this struct
    fn units(&self) -> T;
}

///
/// Trait to provide access to unit conversions
pub trait Unit<T> {
    type Output;
    #[must_use]
    fn as_unit(&self, unit: T) -> Self::Output
    where
        Self: Sized;
}

pub mod angle;
pub mod compass;
pub mod datasize;
pub mod drift;
pub mod duration;
pub mod freq;
pub mod length;
pub mod speed;
pub mod temperature;
