pub mod standards;

use crate::units::length::{Length, LengthUnits};

#[derive(Debug, Clone)]
pub enum EllipticalShape {
    EPSG(String),
    Ellipse(Ellipse),
}

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

    pub fn semi_major_axis(&self) -> Length {
        self.semi_major_axis
    }

    pub fn inverse_flattening(&self) -> f64 {
        self.inverse_flattening
    }
}

pub struct Ellipsoid {
    pub(crate) semi_major_axis: Length,
    pub(crate) inverse_flattening: f64,

    pub(crate) semi_minor_axis: Length,

    pub(crate) first_eccentricity: f64,
    pub(crate) first_eccentricity_squared: f64,

    pub(crate) second_eccentricity: f64,
    pub(crate) second_eccentricity_squared: f64,
}
