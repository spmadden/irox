// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Hayford's international ellipsoid ca. 1924
//!
use irox_units::units::length::Length;

use crate::geo::ellipse::Ellipse;

/// International
pub const INTERNATIONAL_SEMI_MAJOR_LENGTH: Length = Length::new_meters(6_378_388.);
pub const INTERNATIONAL_INVERSE_FLATTENING: f64 = 297.;
pub const INTERNATIONAL_PARAMS: Ellipse = Ellipse::named(
    "International",
    INTERNATIONAL_SEMI_MAJOR_LENGTH,
    INTERNATIONAL_INVERSE_FLATTENING,
);
