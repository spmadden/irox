// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Geodesy types and math, Ellipses, Ellipsoids, Elliptical Shapes

use crate::error::ConvertError;
use crate::geo::ellipsoid::Ellipsoid;
use crate::geo::standards::wgs84::{WGS84_EPSG_SHAPE, WGS84_SHAPE};
use crate::geo::standards::StandardShapes;
use ellipse::Ellipse;

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

    pub fn as_ellipse(&self) -> Result<Ellipse, ConvertError> {
        Ellipse::try_from(self)
    }

    pub fn as_ellipsoid(&self) -> Result<Ellipsoid, ConvertError> {
        self.as_ellipse().map(Into::into)
    }
}

impl TryFrom<&EllipticalShape> for Ellipse {
    type Error = ConvertError;

    fn try_from(value: &EllipticalShape) -> Result<Self, Self::Error> {
        match value {
            EllipticalShape::EpsgDatum(d) => StandardShapes::lookup_epsg(*d)
                .ok_or_else(|| ConvertError::MissingProjection(format!("Unknown EPSG code: {d}")))
                .map(|v| v.as_ellipse()),
            EllipticalShape::Ellipse(e) => Ok(*e),
        }
    }
}
