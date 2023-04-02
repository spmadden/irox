use crate::geo::ellipsoid::Ellipsoid;
use irox_units::units::length::{Length, LengthUnits};

#[derive(Debug, Clone, Copy)]
pub struct Ellipse {
    semi_major_axis: Length,
    inverse_flattening: f64,
}

impl Ellipse {
    pub const fn new(semi_major_axis: Length, inverse_flattening: f64) -> Ellipse {
        Ellipse {
            semi_major_axis,
            inverse_flattening,
        }
    }

    pub const fn new_meters(semi_major_axis_meters: f64, inverse_flattening: f64) -> Ellipse {
        Self::new(
            Length::new(semi_major_axis_meters, LengthUnits::Meters),
            inverse_flattening,
        )
    }

    pub const fn new_sphere(radius: Length) -> Ellipse {
        Ellipse {
            semi_major_axis: radius,
            inverse_flattening: 0.0,
        }
    }

    pub const fn new_sphere_meters(radius_meters: f64) -> Ellipse {
        Self::new_sphere(Length::new(radius_meters, LengthUnits::Meters))
    }

    pub const fn semi_major_axis_a(&self) -> Length {
        self.semi_major_axis
    }

    pub const fn inverse_flattening(&self) -> f64 {
        self.inverse_flattening
    }

    pub fn semi_minor_axis_b(&self) -> Length {
        self.semi_major_axis * (1.0 - self.flattening_f())
    }

    pub fn flattening_f(&self) -> f64 {
        1.0 / self.inverse_flattening
    }

    pub fn first_eccentricity_squared(&self) -> f64 {
        let f = self.flattening_f();
        f * (2.0 - f)
    }

    pub fn first_eccentricity(&self) -> f64 {
        self.first_eccentricity_squared().sqrt()
    }

    pub fn second_eccentricity_squared(&self) -> f64 {
        self.first_eccentricity_squared() / (1.0 - self.first_eccentricity_squared())
    }

    pub fn second_eccentricity(&self) -> f64 {
        self.first_eccentricity_squared().sqrt()
    }
}

impl From<Ellipsoid> for Ellipse {
    fn from(value: Ellipsoid) -> Self {
        Ellipse::new(value.semi_major_axis, value.inverse_flattening)
    }
}
