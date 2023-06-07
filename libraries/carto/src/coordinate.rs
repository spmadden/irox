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

/// Forcing type for Latitude
#[derive(Debug, Clone, Copy, Default)]
pub struct Latitude(pub Angle);

/// Forcing type for Longitude
#[derive(Debug, Clone, Copy, Default)]
pub struct Longitude(pub Angle);

/// Represents a Latitude and Longitude on a Elliptical Shape
#[derive(Debug, Clone, Default)]
pub struct EllipticalCoordinate {
    latitude: Latitude,
    longitude: Longitude,
    altitude: Option<Length>,
    timestamp: Option<f64>,
    reference_frame: EllipticalShape,
}

impl EllipticalCoordinate {
    ///
    /// Constructs a new EllipticalCoordinate object
    pub const fn new(
        latitude: Latitude,
        longitude: Longitude,
        reference_frame: EllipticalShape,
    ) -> EllipticalCoordinate {
        EllipticalCoordinate {
            latitude,
            longitude,
            reference_frame,
            altitude: None,
            timestamp: None,
        }
    }

    ///
    /// Constructs a new EllipticalCoordinate object assuming [`AngleUnits::Degrees`] and [`standards::WGS84_SHAPE`]
    pub const fn new_degrees_wgs84(latitude: f64, longitude: f64) -> EllipticalCoordinate {
        Self::new(
            Latitude(Angle::new(latitude, AngleUnits::Degrees)),
            Longitude(Angle::new(longitude, AngleUnits::Degrees)),
            standards::wgs84::WGS84_SHAPE,
        )
    }

    pub fn get_latitude(&self) -> &Latitude {
        &self.latitude
    }

    pub fn get_longitude(&self) -> &Longitude {
        &self.longitude
    }

    pub fn get_reference_frame(&self) -> &EllipticalShape {
        &self.reference_frame
    }

    pub fn with_altitude(self, altitude: Length) -> EllipticalCoordinate {
        EllipticalCoordinate {
            altitude: Some(altitude),
            ..self
        }
    }

    pub fn with_timestamp(self, timestamp: f64) -> EllipticalCoordinate {
        EllipticalCoordinate {
            timestamp: Some(timestamp),
            ..self
        }
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
