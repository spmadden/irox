// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::geo::ellipse::Ellipse;
use crate::geo::ellipsoid::Ellipsoid;
use crate::geo::standards::airy::AIRY_PARAMS;
use crate::geo::standards::grs80::GRS80_PARAMS;
use crate::geo::standards::hayford_international::INTERNATIONAL_PARAMS;
use crate::geo::standards::nad::CLARKE_1866_PARAMS;
use crate::geo::standards::wgs84::{
    WGS84_EQUAL_AREA_SPHERE_PARAMS, WGS84_EQUAL_VOLUME_SPHERE_PARAMS, WGS84_MEAN_RADIUS_PARAMS,
    WGS84_PARAMS,
};
use crate::geo::EllipticalShape;

pub mod airy;
pub mod grs80;
pub mod hayford_international;
pub mod nad;
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

    /// Navioncs uses the Hayford International ellipsoid
    NavionicsMercator,

    Clarke1866,
    NAD27, // same as Clarke1866
    NAD83, // same as GRS80
}

impl From<&StandardShapes> for Ellipse {
    fn from(value: &StandardShapes) -> Self {
        match value {
            StandardShapes::WGS84 => WGS84_PARAMS,
            StandardShapes::WGS84_MeanRadius => WGS84_MEAN_RADIUS_PARAMS,
            StandardShapes::WGS84_EqualAreaSphere => WGS84_EQUAL_AREA_SPHERE_PARAMS,
            StandardShapes::WGS84_EqualVolumeSphere => WGS84_EQUAL_VOLUME_SPHERE_PARAMS,

            StandardShapes::Hayford_International | StandardShapes::NavionicsMercator => {
                INTERNATIONAL_PARAMS
            }
            StandardShapes::Airy => AIRY_PARAMS,
            StandardShapes::GRS80 | StandardShapes::NAD83 => GRS80_PARAMS,
            StandardShapes::Clarke1866 | StandardShapes::NAD27 => CLARKE_1866_PARAMS,
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

    #[must_use]
    pub fn lookup_epsg(epsg: u32) -> Option<StandardShapes> {
        match epsg {
            4326 => Some(StandardShapes::WGS84),
            _ => None,
        }
    }
}
