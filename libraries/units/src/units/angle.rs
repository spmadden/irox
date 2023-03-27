//!
//! This module contains the basic types and conversions for the SI coherent derived "Planar Angle"
//! quantity
use crate::units::{FromUnits, Unit};

///
/// Represents a specific Planar Angle unit - SI or otherwise
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub enum AngleUnits {
    /// SI Base Unit for Planar Angle - Radians, the unit radius of a circle.
    /// There are `tau` Radians across a full circle
    #[default]
    Radians,

    /// Derived unit for Planar Angle - Degrees.  
    /// There are 360 degrees across a full circle
    Degrees,

    /// Derived unit for Planar Angle - Minutes - the first division of a degree
    /// There are 60 minutes in a degree.
    Minutes,

    /// Derived unit for Planar Angle - Seconds - the second division of a degree
    /// There are 60 seconds in a minute.
    Seconds,

    /// Derived unit for Planar Angle - Revolution (turn) - a full circuit of a circle
    /// There are 360 degrees in a revolution
    Revolutions,

    /// Derived unit for Planar Angle - NATO Mil
    /// There are 6400 mils in a turn/revolution
    Mils,
}

macro_rules! from_units_angle {
    ($type:ident) => {
        impl crate::units::FromUnits<$type> for AngleUnits {
            fn from(&self, value: $type, units: Self) -> $type {
                match self {
                    AngleUnits::Degrees => match units {
                        AngleUnits::Radians => value * DEG_2_RAD as $type,
                        AngleUnits::Degrees => value as $type,
                        AngleUnits::Minutes => value * DEG_2_MIN as $type,
                        AngleUnits::Seconds => value * DEG_2_SEC as $type,
                        AngleUnits::Revolutions => value / REV_2_DEG as $type,
                        AngleUnits::Mils => value * DEG_2_MIL as $type,
                    },
                    AngleUnits::Radians => match units {
                        AngleUnits::Degrees => value * RAD_2_DEG as $type,
                        AngleUnits::Radians => value as $type,
                        AngleUnits::Minutes => value * RAD_2_DEG as $type * DEG_2_MIN as $type,
                        AngleUnits::Seconds => value * RAD_2_DEG as $type * DEG_2_SEC as $type,
                        AngleUnits::Revolutions => value / REV_2_RAD as $type,
                        AngleUnits::Mils => value * RAD_2_MIL as $type,
                    },
                    _ => todo!(),
                }
            }
        }
    };
}

basic_unit!(Angle, AngleUnits, Degrees);
from_units_angle!(f32);
from_units_angle!(f64);

impl Unit<AngleUnits> for Angle {
    fn as_unit(&self, units: AngleUnits) -> Self
    where
        Self: Sized,
    {
        Angle {
            value: units.from(self.value, self.units),
            units,
        }
    }
}

impl Angle {
    pub const fn new_radians(value: f64) -> Angle {
        Self::new(value, AngleUnits::Radians)
    }

    pub const fn new_degrees(value: f64) -> Angle {
        Self::new(value, AngleUnits::Degrees)
    }

    pub fn as_degrees(&self) -> Angle {
        self.as_unit(AngleUnits::Degrees)
    }

    pub fn as_radians(&self) -> Angle {
        self.as_unit(AngleUnits::Radians)
    }
}

pub const DEG_2_RAD: f64 = 0.017_453_292_519_943_295;
pub const RAD_2_DEG: f64 = 57.295_779_513_082_32;
pub const REV_2_DEG: f64 = 360.;
pub const REV_2_RAD: f64 = std::f64::consts::TAU;
pub const MIN_2_SEC: f64 = 60.;
pub const MIL_2_REV: f64 = 6400.;
pub const DEG_2_MIN: f64 = 60.;
pub const DEG_2_SEC: f64 = DEG_2_MIN * MIN_2_SEC;
pub const DEG_2_MIL: f64 = MIL_2_REV / REV_2_DEG;
pub const RAD_2_MIL: f64 = MIL_2_REV / REV_2_RAD;
