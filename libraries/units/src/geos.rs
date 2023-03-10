use crate::units::length::Length;

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
    pub fn new(semi_major_axis: Length, inverse_flattening: f64) -> Ellipse {
        Ellipse {
            semi_major_axis,
            inverse_flattening,
        }
    }

    pub fn semi_major_axis(&self) -> Length {
        self.semi_major_axis
    }

    pub fn inverse_flattening(&self) -> f64 {
        self.inverse_flattening
    }
}
