// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::f64::consts::{PI, TAU};

use irox_units::units::angle::{self, Angle};

use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate, Latitude, Longitude};
use crate::geo::standards::wgs84::WGS84_SHAPE;
use crate::geo::EllipticalShape;
use crate::proj::Projection;

pub const SPHERICAL_MERCATOR_SHAPE: EllipticalShape = EllipticalShape::EpsgDatum(3857);

pub struct SphericalMercatorProjection {
    zoom_level: u8,
}

impl SphericalMercatorProjection {
    #[must_use]
    pub fn new(zoom_level: u8) -> SphericalMercatorProjection {
        SphericalMercatorProjection { zoom_level }
    }

    #[must_use]
    pub fn tile_x_index(&self, coordinate: &EllipticalCoordinate) -> f64 {
        let lon_deg = coordinate.get_longitude().0.as_degrees().value();
        let offset = (lon_deg + 180.) / 360.;
        let max_tile = f64::from(1 << self.zoom_level);
        offset * max_tile
    }

    #[must_use]
    pub fn tile_y_index(&self, coordinate: &EllipticalCoordinate) -> f64 {
        let lat_rad = coordinate.get_latitude().0.as_radians().value();

        let y = lat_rad.tan().asinh();
        let y = (1. - (y / PI)) / 2.;
        let max_tile = f64::from(1 << self.zoom_level);
        max_tile * y
    }

    #[must_use]
    pub fn latitude(&self, tile_y: f64) -> Latitude {
        let offset = 1. - (2. * tile_y) / f64::from(1 << self.zoom_level);
        Latitude(Angle::new_radians((PI * offset).sinh().atan()))
    }

    #[must_use]
    pub fn longitude(&self, tile_x: f64) -> Longitude {
        let offset = tile_x / f64::from(1 << self.zoom_level);
        Longitude(Angle::new_radians(offset * TAU - PI))
    }

    #[must_use]
    pub fn max_tile_index(&self) -> u64 {
        (1 << self.zoom_level) - 1
    }
}

impl Projection for SphericalMercatorProjection {
    fn get_center_coords(&self) -> &EllipticalCoordinate {
        &CENTER_COORDS
    }

    fn project_to_cartesian(&self, coord: &EllipticalCoordinate) -> CartesianCoordinate {
        let x = self.tile_x_index(coord) * TILE_TO_PIXEL;
        let y = self.tile_y_index(coord) * TILE_TO_PIXEL;
        let z = f64::from(self.zoom_level);

        CartesianCoordinate::new_meters(x, y, z)
    }

    fn project_to_elliptical(&self, coord: &CartesianCoordinate) -> EllipticalCoordinate {
        let lat = self.latitude(coord.get_y().as_meters().value());
        let lon = self.longitude(coord.get_x().as_meters().value());

        EllipticalCoordinate::new(lat, lon, WGS84_SHAPE)
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
    EllipticalCoordinate::new(Latitude(angle::ZERO), Longitude(angle::ZERO), WGS84_SHAPE);

// pub const BOUNDS: Bounds<CartesianCoordinate> = Bounds::new()

const TILE_TO_PIXEL: f64 = 40.743_665_431_525_21;

#[cfg(test)]
mod test {
    use crate::coordinate::EllipticalCoordinate;

    use super::SphericalMercatorProjection;

    #[test]
    pub fn test1() {
        let sm1 = SphericalMercatorProjection::new(1);

        assert_eq!(0.0, sm1.latitude(1.0).0.as_degrees().value());
        assert_eq!(0.0, sm1.longitude(1.0).0.as_degrees().value());
    }

    #[test]
    pub fn test2() {
        let sm = SphericalMercatorProjection::new(10);

        let coord = EllipticalCoordinate::new_degrees_wgs84(24.846_562, -81.914);

        assert_eq!(439, sm.tile_y_index(&coord) as u64);
        assert_eq!(279, sm.tile_x_index(&coord) as u64);

        let max_tile = 1 << sm.zoom_level;
        assert_eq!(2_u64.pow(sm.zoom_level.into()), max_tile as u64);

        let invy = max_tile - 439 - 1;
        assert_eq!(invy, 584);
    }
}
