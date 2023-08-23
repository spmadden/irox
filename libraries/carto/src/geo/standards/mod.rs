// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use crate::geo::ellipse::Ellipse;
use crate::geo::ellipsoid::Ellipsoid;
use crate::geo::standards::airy::AIRY_PARAMS;
use crate::geo::standards::grs80::GRS80_PARAMS;
use crate::geo::standards::hayford_international::INTERNATIONAL_PARAMS;
use crate::geo::standards::wgs84::{
    WGS84_EQUAL_AREA_SPHERE_PARAMS, WGS84_EQUAL_VOLUME_SPHERE_PARAMS, WGS84_MEAN_RADIUS_PARAMS,
    WGS84_PARAMS,
};
use crate::geo::EllipticalShape;

pub mod airy;
pub mod grs80;
pub mod hayford_international;
pub mod wgs84;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum StandardShapes {
    /// Standard WGS84
    WGS84,

    /// An average of the WGS84 axes
    WGS84_MeanRadius,

    /// A sphere with the same surface area as WGS84
    WGS84_EqualAreaSphere,

    /// A sphere with the same volume as WGS84
    WGS84_EqualVolumeSphere,

    /// Hayford Ellipsoid ca 1924
    Hayford_International,

    /// ITRS GRS80 Ellipsoid ca 1979
    GRS80,

    /// Airy Ellipsoid ca 1830
    Airy,
}

impl From<&StandardShapes> for Ellipse {
    fn from(value: &StandardShapes) -> Self {
        match value {
            StandardShapes::WGS84 => WGS84_PARAMS,
            StandardShapes::WGS84_MeanRadius => WGS84_MEAN_RADIUS_PARAMS,
            StandardShapes::WGS84_EqualAreaSphere => WGS84_EQUAL_AREA_SPHERE_PARAMS,
            StandardShapes::WGS84_EqualVolumeSphere => WGS84_EQUAL_VOLUME_SPHERE_PARAMS,

            StandardShapes::Hayford_International => INTERNATIONAL_PARAMS,
            StandardShapes::Airy => AIRY_PARAMS,
            StandardShapes::GRS80 => GRS80_PARAMS,
        }
    }
}

impl From<&StandardShapes> for EllipticalShape {
    fn from(value: &StandardShapes) -> Self {
        EllipticalShape::Ellipse(value.as_ellipse())
    }
}
impl From<StandardShapes> for EllipticalShape {
    fn from(value: StandardShapes) -> Self {
        EllipticalShape::Ellipse(value.as_ellipse())
    }
}

impl StandardShapes {
    #[must_use]
    pub fn as_ellipse(&self) -> Ellipse {
        self.into()
    }

    #[must_use]
    pub fn as_ellipsoid(&self) -> Ellipsoid {
        self.as_ellipse().into()
    }
}
