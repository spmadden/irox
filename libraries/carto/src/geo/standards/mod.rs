use crate::geo::ellipsoid::Ellipsoid;
use crate::geo::standards::hayford_international::HAYFORD_PARAMS;
use crate::geo::standards::wgs84::*;
use crate::geo::EllipticalShape;

use crate::geo::ellipse::Ellipse;

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
}

impl From<&StandardShapes> for Ellipse {
    fn from(value: &StandardShapes) -> Self {
        match value {
            StandardShapes::WGS84 => WGS84_PARAMS,
            StandardShapes::WGS84_MeanRadius => WGS84_MEAN_RADIUS_PARAMS,
            StandardShapes::WGS84_EqualAreaSphere => WGS84_EQUAL_AREA_SPHERE_PARAMS,
            StandardShapes::WGS84_EqualVolumeSphere => WGS84_EQUAL_VOLUME_SPHERE_PARAMS,

            StandardShapes::Hayford_International => HAYFORD_PARAMS,
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
    pub fn as_ellipse(&self) -> Ellipse {
        self.into()
    }

    pub fn as_ellipsoid(&self) -> Ellipsoid {
        self.as_ellipse().into()
    }
}
