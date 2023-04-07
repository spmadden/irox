//!
//! Hayford's international ellipsoid ca. 1924
//!
use crate::geo::ellipse::Ellipse;
use irox_units::units::length::Length;

/// International
pub const INTERNATIONAL_SEMI_MAJOR_LENGTH: Length = Length::new_meters(6378388.);
pub const INTERNATIONAL_INVERSE_FLATTENING: f64 = 297.;
pub const INTERNATIONAL_PARAMS: Ellipse = Ellipse::named(
    "International",
    INTERNATIONAL_SEMI_MAJOR_LENGTH,
    INTERNATIONAL_INVERSE_FLATTENING,
);
