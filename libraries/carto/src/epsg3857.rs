use irox_units::{
    coordinate::{CartesianCoordinate, EllipticalCoordinate},
    geo::standards::WGS84_SHAPE,
    units::angle,
};

use crate::proj::Projection;

pub struct SphericalMercatorProjection {
    zoom_level: u8,
}

impl SphericalMercatorProjection {
    pub fn new(zoom_level: u8) -> SphericalMercatorProjection {
        SphericalMercatorProjection { zoom_level }
    }
}

impl Projection for SphericalMercatorProjection {
    fn get_center_coords(&self) -> &irox_units::coordinate::EllipticalCoordinate {
        &CENTER_COORDS
    }

    fn project_to_cartesian(
        &self,
        coord: &irox_units::coordinate::EllipticalCoordinate,
    ) -> irox_units::coordinate::CartesianCoordinate {
        let lon = coord.get_longitude().as_degrees();
        let x = (lon.value() + 180.) / 360. * (2 << self.zoom_level) as f64;

        let y = 0.0;
        let z = self.zoom_level as f64;

        CartesianCoordinate::new_meters(x, y, z)
    }

    fn project_to_elliptical(
        &self,
        coord: &irox_units::coordinate::CartesianCoordinate,
    ) -> irox_units::coordinate::EllipticalCoordinate {
        todo!()
    }
}

pub const UPPER_LEFT_COORDINATE_X: f64 = -180.0;
pub const UPPER_LEFT_COORDINATE_Y: f64 = 85.051_128_779_806_59;

pub const LOWER_LEFT_COORDINATE_X: f64 = -180.0;
pub const LOWER_LEFT_COORDINATE_Y: f64 = -85.051_128_779_806_59;

pub const UPPER_RIGHT_COORDINATE_X: f64 = -UPPER_LEFT_COORDINATE_X;
pub const UPPER_RIGHT_COORDINATE_Y: f64 = UPPER_LEFT_COORDINATE_Y;

pub const LOWER_RIGHT_COORDINATE_X: f64 = -LOWER_LEFT_COORDINATE_X;
pub const LOWER_RIGHT_COORDINATE_Y: f64 = LOWER_LEFT_COORDINATE_Y;

pub static CENTER_COORDS: EllipticalCoordinate =
    EllipticalCoordinate::new(angle::ZERO, angle::ZERO, WGS84_SHAPE);

// pub const BOUNDS: Bounds<CartesianCoordinate> = Bounds::new()
