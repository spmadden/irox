use crate::geo::ellipse::Ellipse;
use irox_units::units::length::Length;

pub const HAYFORD_SEMI_MAJOR_LENGTH: Length = Length::new_meters(6378388.);
pub const HAYFORD_INVERSE_FLATTENING: f64 = 297.;
pub const HAYFORD_PARAMS: Ellipse =
    Ellipse::new(HAYFORD_SEMI_MAJOR_LENGTH, HAYFORD_INVERSE_FLATTENING);
