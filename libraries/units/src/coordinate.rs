use crate::{
    geo::{standards, EllipticalShape},
    units::{
        angle::{Angle, AngleUnits},
        length::Length,
    },
};

#[derive(Debug, Clone)]
pub enum CoordinateType {
    Elliptical(EllipticalCoordinate),
    Cartesian(CartesianCoordinate),
}

#[derive(Debug, Clone)]
pub struct EllipticalCoordinate {
    latitude: Angle,
    longitude: Angle,
    reference_frame: EllipticalShape,
}

impl EllipticalCoordinate {
    ///
    /// Constructs a new EllipticalCoordinate object
    pub const fn new(
        latitude: Angle,
        longitude: Angle,
        reference_frame: EllipticalShape,
    ) -> EllipticalCoordinate {
        EllipticalCoordinate {
            latitude,
            longitude,
            reference_frame,
        }
    }

    ///
    /// Constructs a new EllipticalCoordinate object assuming [`AngleUnits::Degrees`] and [`standards::WGS84_SHAPE`]
    pub const fn new_degrees_wgs84(latitude: f64, longitude: f64) -> EllipticalCoordinate {
        Self::new(
            Angle::new(latitude, AngleUnits::Degrees),
            Angle::new(longitude, AngleUnits::Degrees),
            standards::WGS84_SHAPE,
        )
    }

    pub fn get_latitude(&self) -> &Angle {
        &self.latitude
    }

    pub fn get_longitude(&self) -> &Angle {
        &self.longitude
    }

    pub fn get_reference_frame(&self) -> &EllipticalShape {
        &self.reference_frame
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CartesianCoordinate {
    x: Length,
    y: Length,
    z: Length,
}

impl CartesianCoordinate {
    pub fn new(x: Length, y: Length, z: Length) -> CartesianCoordinate {
        CartesianCoordinate { x, y, z }
    }

    pub fn new_meters(x_meters: f64, y_meters: f64, z_meters: f64) -> CartesianCoordinate {
        Self::new(
            Length::new_meters(x_meters),
            Length::new_meters(y_meters),
            Length::new_meters(z_meters),
        )
    }

    pub fn get_x(&self) -> &Length {
        &self.x
    }

    pub fn get_y(&self) -> &Length {
        &self.y
    }

    pub fn get_z(&self) -> &Length {
        &self.z
    }
}
