// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Airy ellipsoid 1830
//!
use irox_units::units::length::Length;

use crate::geo::ellipse::Ellipse;

/// Airy semi-major axis
pub const AIRY_SEMI_MAJOR: Length = Length::new_meters(6_377_563.396);
/// Airy inverse flattening
pub const AIRY_INVERSE_FLATTENING: f64 = 299.324_964_6;
/// Airy ellipse parameters
pub const AIRY_PARAMS: Ellipse =
    Ellipse::named("Airy1830", AIRY_SEMI_MAJOR, AIRY_INVERSE_FLATTENING);
