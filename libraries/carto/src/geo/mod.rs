// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Geodesy types and math, Ellipses, Ellipsoids, Elliptical Shapes

use ellipse::Ellipse;

use crate::geo::standards::wgs84::{WGS84_EPSG_SHAPE, WGS84_SHAPE};

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

impl EllipticalShape {
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            EllipticalShape::EpsgDatum(d) => {
                format!("EPSG({d})")
            }
            EllipticalShape::Ellipse(e) => String::from(e.name()),
        }
    }

    #[must_use]
    pub fn is_wgs84(&self) -> bool {
        *self == WGS84_SHAPE || *self == WGS84_EPSG_SHAPE
    }
}
