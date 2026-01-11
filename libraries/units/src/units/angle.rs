// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! This module contains the basic types and conversions for the SI coherent derived "Planar Angle"
//! quantity
use core::fmt::{Display, Formatter};

use crate::units::{FromUnits, Unit};

///
/// Represents a specific Planar Angle unit - SI or otherwise
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
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
                        AngleUnits::Radians => value * RAD_2_DEG as $type,
                        AngleUnits::Degrees => value as $type,

                        AngleUnits::Minutes => value * RAD_2_DEG as $type * DEG_2_MIN as $type,
                        AngleUnits::Seconds => value * RAD_2_DEG as $type * DEG_2_SEC as $type,
                        AngleUnits::Revolutions => value / REV_2_RAD as $type,
                        AngleUnits::Mils => value * RAD_2_MIL as $type,
                    },
                    AngleUnits::Radians => match units {
                        AngleUnits::Degrees => value * DEG_2_RAD as $type,
                        AngleUnits::Radians => value as $type,

                        AngleUnits::Minutes => value * DEG_2_MIN as $type,
                        AngleUnits::Seconds => value * DEG_2_SEC as $type,
                        AngleUnits::Revolutions => value / REV_2_DEG as $type,
                        AngleUnits::Mils => value * DEG_2_MIL as $type,
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

impl Angle {
    #[must_use]
    pub const fn new_radians(value: f64) -> Angle {
        Self::new(value, AngleUnits::Radians)
    }

    #[must_use]
    pub const fn new_degrees(value: f64) -> Angle {
        Self::new(value, AngleUnits::Degrees)
    }

    #[must_use]
    pub fn new_dms(degrees: i16, minutes: u8, seconds: f64) -> Angle {
        let mult: f64 = match degrees {
            ..=0 => -1.0,
            _ => 1.0,
        };
        let minutes: f64 = f64::from(minutes) * mult;
        let seconds: f64 = seconds * mult;
        let value = f64::from(degrees) + minutes / 60. + seconds / 3600.;
        Self::new_degrees(value)
    }

    #[must_use]
    #[allow(unused_imports)]
    pub fn new_dm(degrees: i16, minutes: f64) -> Angle {
        use irox_tools::f64::FloatExt;
        let seconds = minutes.fract() * 60.;
        Self::new_dms(degrees, minutes.trunc() as u8, seconds)
    }

    #[must_use]
    pub fn as_degrees(&self) -> Angle {
        self.as_unit(AngleUnits::Degrees)
    }

    #[must_use]
    pub fn as_radians(&self) -> Angle {
        self.as_unit(AngleUnits::Radians)
    }

    #[must_use]
    pub fn as_dms(&self) -> (i16, u8, f64) {
        let (deg, val) = self.as_deg_min();

        let min = val as u8;
        let sec = (val - min as f64) * 60.;
        (deg, min, sec)
    }

    #[must_use]
    #[allow(unused_imports)]
    pub fn as_deg_min(&self) -> (i16, f64) {
        use irox_tools::f64::FloatExt;
        let val = self.as_degrees().value;
        let sign = val.signum() as i16;
        let val = val.abs();

        let deg = val as i16;
        let min = (val - deg as f64) * 60.;
        (deg * sign, min)
    }

    pub fn sin(&self) -> f64 {
        use irox_tools::f64::FloatExt;
        let v = self.as_radians().value;
        FloatExt::sin(v)
    }
    #[cfg(feature = "std")]
    pub fn asin(&self) -> f64 {
        self.as_radians().value.asin()
    }
    pub fn cos(&self) -> f64 {
        use irox_tools::f64::FloatExt;
        let v = self.as_radians().value;
        FloatExt::cos(v)
    }
    #[cfg(feature = "std")]
    pub fn acos(&self) -> f64 {
        self.as_radians().value.acos()
    }

    #[cfg(feature = "std")]
    pub fn tan(&self) -> f64 {
        self.as_radians().value.tan()
    }

    #[must_use]
    pub const fn min_value() -> Self {
        Angle::new_degrees(0.0)
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:03.3}\u{00B0}", self.as_degrees().value)
    }
}

/// Degree to Radians factor
pub const DEG_2_RAD: f64 = 0.017_453_292_519_943_295;
/// Radians to Degrees factor
pub const RAD_2_DEG: f64 = 57.295_779_513_082_32;
/// Revolutions to Degrees factor
pub const REV_2_DEG: f64 = 360.;
/// Revolutions to Radians factor
pub const REV_2_RAD: f64 = core::f64::consts::TAU;
/// Minutes to Seconds factor
pub const MIN_2_SEC: f64 = 60.;
/// Mils to Revolutions factor
pub const MIL_2_REV: f64 = 6400.;
/// Degrees to Minutes factor
pub const DEG_2_MIN: f64 = 60.;
/// Degrees to Seconds factor
pub const DEG_2_SEC: f64 = DEG_2_MIN * MIN_2_SEC;
/// Degrees to Mils factor
pub const DEG_2_MIL: f64 = MIL_2_REV / REV_2_DEG;
pub const RAD_2_MIL: f64 = MIL_2_REV / REV_2_RAD;
