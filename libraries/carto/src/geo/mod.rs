// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use ellipse::Ellipse;

use crate::geo::standards::wgs84::WGS84_SHAPE;

pub mod ellipse;
pub mod ellipsoid;
pub mod standards;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EllipticalShape {
    EpsgDatum(u32),
    Ellipse(Ellipse),
}

impl Default for EllipticalShape {
    fn default() -> Self {
        WGS84_SHAPE
    }
}
