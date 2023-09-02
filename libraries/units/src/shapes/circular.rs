// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Div, DivAssign, Mul, MulAssign};

use crate::units::length::Length;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum CircularAspect {
    #[default]
    Radius,
    Diameter,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CircularDimension {
    dimension_type: CircularAspect,
    dimension: Length,
}

impl Display for CircularDimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "+/- {}m", self.as_radius().dimension.as_meters().value())
    }
}

impl Default for CircularDimension {
    fn default() -> Self {
        CircularDimension {
            dimension: Length::new_meters(1.0),
            dimension_type: CircularAspect::Radius,
        }
    }
}

impl CircularDimension {
    #[must_use]
    pub const fn new(dimension_type: CircularAspect, dimension: Length) -> CircularDimension {
        CircularDimension {
            dimension,
            dimension_type,
        }
    }
    #[must_use]
    pub const fn new_radius(radius: Length) -> CircularDimension {
        CircularDimension::new(CircularAspect::Radius, radius)
    }
    #[must_use]
    pub const fn new_diameter(diameter: Length) -> CircularDimension {
        CircularDimension::new(CircularAspect::Diameter, diameter)
    }

    #[must_use]
    pub fn get_aspect(&self) -> CircularAspect {
        self.dimension_type
    }

    #[must_use]
    pub fn get_dimension(&self) -> Length {
        self.dimension
    }

    #[must_use]
    pub fn as_radius(self) -> CircularDimension {
        match self.dimension_type {
            CircularAspect::Radius => self,
            CircularAspect::Diameter => CircularDimension::new_radius(self.dimension / 2.0),
        }
    }

    #[must_use]
    pub fn as_diameter(self) -> CircularDimension {
        match self.dimension_type {
            CircularAspect::Radius => CircularDimension::new_diameter(self.dimension * 2.0),
            CircularAspect::Diameter => self,
        }
    }

    #[must_use]
    pub fn as_aspect(self, aspect: CircularAspect) -> CircularDimension {
        match aspect {
            CircularAspect::Radius => self.as_radius(),
            CircularAspect::Diameter => self.as_diameter(),
        }
    }
}

impl Mul<f64> for CircularDimension {
    type Output = CircularDimension;

    fn mul(self, rhs: f64) -> Self::Output {
        CircularDimension {
            dimension_type: self.dimension_type,
            dimension: self.dimension * rhs,
        }
    }
}

impl Mul<f64> for &CircularDimension {
    type Output = CircularDimension;

    fn mul(self, rhs: f64) -> Self::Output {
        CircularDimension {
            dimension_type: self.dimension_type,
            dimension: self.dimension * rhs,
        }
    }
}

impl MulAssign<f64> for CircularDimension {
    fn mul_assign(&mut self, rhs: f64) {
        self.dimension *= rhs
    }
}

impl Div<f64> for CircularDimension {
    type Output = CircularDimension;

    fn div(self, rhs: f64) -> Self::Output {
        CircularDimension {
            dimension_type: self.dimension_type,
            dimension: self.dimension / rhs,
        }
    }
}

impl Div<f64> for &CircularDimension {
    type Output = CircularDimension;

    fn div(self, rhs: f64) -> Self::Output {
        CircularDimension {
            dimension_type: self.dimension_type,
            dimension: self.dimension / rhs,
        }
    }
}

impl DivAssign<f64> for CircularDimension {
    fn div_assign(&mut self, rhs: f64) {
        self.dimension /= rhs
    }
}

impl PartialOrd<CircularDimension> for CircularDimension {
    fn partial_cmp(&self, other: &CircularDimension) -> Option<Ordering> {
        let dim = other.as_aspect(self.dimension_type).dimension;
        self.dimension.partial_cmp(&dim)
    }
}
