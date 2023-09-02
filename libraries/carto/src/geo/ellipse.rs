// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_units::units::length::{Length, LengthUnits};

use crate::geo::ellipsoid::Ellipsoid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    semi_major_axis: Length,
    inverse_flattening: f64,
    name: &'static str,
}

impl Ellipse {
    #[must_use]
    pub const fn named(
        name: &'static str,
        semi_major_axis: Length,
        inverse_flattening: f64,
    ) -> Ellipse {
        Ellipse {
            semi_major_axis,
            inverse_flattening,
            name,
        }
    }
    #[must_use]
    pub const fn new(semi_major_axis: Length, inverse_flattening: f64) -> Ellipse {
        Ellipse {
            semi_major_axis,
            inverse_flattening,
            name: "unnamed",
        }
    }

    #[must_use]
    pub const fn new_meters(semi_major_axis_meters: f64, inverse_flattening: f64) -> Ellipse {
        Self::new(
            Length::new(semi_major_axis_meters, LengthUnits::Meters),
            inverse_flattening,
        )
    }

    #[must_use]
    pub const fn new_sphere(radius: Length) -> Ellipse {
        Ellipse {
            semi_major_axis: radius,
            inverse_flattening: 0.0,
            name: "unnamed",
        }
    }

    #[must_use]
    pub const fn new_sphere_meters(radius_meters: f64) -> Ellipse {
        Self::new_sphere(Length::new(radius_meters, LengthUnits::Meters))
    }

    #[must_use]
    pub const fn semi_major_axis_a(&self) -> Length {
        self.semi_major_axis
    }

    #[must_use]
    pub const fn inverse_flattening(&self) -> f64 {
        self.inverse_flattening
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub fn semi_minor_axis_b(&self) -> Length {
        self.semi_major_axis * (1.0 - self.flattening_f())
    }

    #[must_use]
    pub fn flattening_f(&self) -> f64 {
        1.0 / self.inverse_flattening
    }

    #[must_use]
    pub fn first_eccentricity_squared(&self) -> f64 {
        let f = self.flattening_f();
        f * (2.0 - f)
    }

    #[must_use]
    pub fn first_eccentricity(&self) -> f64 {
        self.first_eccentricity_squared().sqrt()
    }

    #[must_use]
    pub fn second_eccentricity_squared(&self) -> f64 {
        self.first_eccentricity_squared() / (1.0 - self.first_eccentricity_squared())
    }

    #[must_use]
    pub fn second_eccentricity(&self) -> f64 {
        self.first_eccentricity_squared().sqrt()
    }
}

impl From<Ellipsoid> for Ellipse {
    fn from(value: Ellipsoid) -> Self {
        Ellipse::new(value.semi_major_axis, value.inverse_flattening)
    }
}
