// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_units::shapes::circular::CircularDimension;
use irox_units::shapes::Ellipse;
use irox_units::units::compass::Azimuth;

use crate::{
    geo::{EllipticalShape, standards},
    units::{
        angle::{Angle, AngleUnits},
        length::Length,
    },
};
use crate::altitude::Altitude;
use crate::error::ConvertError;

/// A generic coordinate type that does not distinguish between a [RelativeCoordinateType] or an
/// [AbsoluteCoordinateType].
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CoordinateType {
    Elliptical(EllipticalCoordinate),
    Cartesian(CartesianCoordinate),
    Horizontal(HorizontalCoordinate),
}

/// A "Absolute Coordinate" is a coordinate that has no variable/dependent element.  It
/// does not require a context to be meaningful.  Contrast with [RelativeCoordinateType]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AbsoluteCoordinateType {
    Elliptical(EllipticalCoordinate),
    Cartesian(CartesianCoordinate),
}

/// A "Relative Coordinate" is a coordinate that has a variable or dependent element.  It's
/// absolute coordinate is dependent on another absolute coordinate as a reference point.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RelativeCoordinateType {
    Cartesian(CartesianCoordinate),
    Horizontal(HorizontalCoordinate),
}

/// Forcing type for Latitude
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Latitude(pub Angle);

/// Forcing type for Longitude
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Longitude(pub Angle);

/// Forcing type for Elevation, the angle above the local horizontal
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Elevation(pub Angle);

/// Represents a Latitude, Longitude, and Altitude on a Elliptical Shape
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct EllipticalCoordinate {
    latitude: Latitude,
    longitude: Longitude,
    reference_frame: EllipticalShape,
    altitude: Option<Altitude>,
    altitude_uncertainty: Option<Length>,
    position_uncertainty: Option<PositionUncertainty>,
    timestamp: Option<f64>,
}

impl EllipticalCoordinate {
    ///
    /// Constructs a new `EllipticalCoordinate` object
    #[must_use]
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
            altitude_uncertainty: None,
            position_uncertainty: None,
            timestamp: None,
        }
    }

    ///
    /// Constructs a new `EllipticalCoordinate` object assuming [`AngleUnits::Degrees`] and [`standards::WGS84_SHAPE`]
    #[must_use]
    pub const fn new_degrees_wgs84(latitude: f64, longitude: f64) -> EllipticalCoordinate {
        Self::new(
            Latitude(Angle::new(latitude, AngleUnits::Degrees)),
            Longitude(Angle::new(longitude, AngleUnits::Degrees)),
            standards::wgs84::WGS84_SHAPE,
        )
    }

    #[must_use]
    pub fn get_latitude(&self) -> &Latitude {
        &self.latitude
    }

    #[must_use]
    pub fn get_longitude(&self) -> &Longitude {
        &self.longitude
    }

    #[must_use]
    pub fn get_reference_frame(&self) -> &EllipticalShape {
        &self.reference_frame
    }

    #[must_use]
    pub fn get_altitude(&self) -> &Option<Altitude> {
        &self.altitude
    }

    #[must_use]
    pub fn get_altitude_uncertainty(&self) -> &Option<Length> {
        &self.altitude_uncertainty
    }

    #[must_use]
    pub fn get_timestamp(&self) -> &Option<f64> {
        &self.timestamp
    }

    #[must_use]
    pub fn with_altitude(self, altitude: Altitude) -> EllipticalCoordinate {
        EllipticalCoordinate {
            altitude: Some(altitude),
            ..self
        }
    }

    #[must_use]
    pub fn with_timestamp(self, timestamp: f64) -> EllipticalCoordinate {
        EllipticalCoordinate {
            timestamp: Some(timestamp),
            ..self
        }
    }

    #[must_use]
    pub fn position_uncertainty(&self) -> &Option<PositionUncertainty> {
        &self.position_uncertainty
    }
}

///
/// Allows the incremental building of an elliptical coordinate
#[derive(Debug, Default, Clone)]
pub struct EllipticalCoordinateBuilder {
    latitude: Option<Latitude>,
    longitude: Option<Longitude>,
    reference_frame: Option<EllipticalShape>,
    altitude: Option<Altitude>,
    altitude_uncertainty: Option<Length>,
    position_uncertainty: Option<PositionUncertainty>,
    timestamp: Option<f64>,
}

impl EllipticalCoordinateBuilder {
    #[must_use]
    pub fn new() -> EllipticalCoordinateBuilder {
        Default::default()
    }

    pub fn with_latitude(&mut self, latitude: Latitude) -> &mut EllipticalCoordinateBuilder {
        self.latitude = Some(latitude);
        self
    }
    pub fn with_longitude(&mut self, longitude: Longitude) -> &mut EllipticalCoordinateBuilder {
        self.longitude = Some(longitude);
        self
    }
    pub fn with_reference_frame(
        &mut self,
        frame: EllipticalShape,
    ) -> &mut EllipticalCoordinateBuilder {
        self.reference_frame = Some(frame);
        self
    }
    pub fn with_altitude(&mut self, alt: Altitude) -> &mut EllipticalCoordinateBuilder {
        self.altitude = Some(alt);
        self
    }
    pub fn with_altitude_uncertainty(
        &mut self,
        alt_unk: Length,
    ) -> &mut EllipticalCoordinateBuilder {
        self.altitude_uncertainty = Some(alt_unk);
        self
    }
    pub fn with_position_uncertainty(
        &mut self,
        pos_unk: PositionUncertainty,
    ) -> &mut EllipticalCoordinateBuilder {
        self.position_uncertainty = Some(pos_unk);
        self
    }

    pub fn with_timestamp(&mut self, timestamp: f64) -> &mut EllipticalCoordinateBuilder {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn build(self) -> Result<EllipticalCoordinate, ConvertError> {
        let Some(latitude) = self.latitude else {
            return Err(ConvertError::MissingValue("Missing latitude".to_string()))
        };
        let Some(longitude) = self.longitude else {
            return Err(ConvertError::MissingValue("Missing longitude".to_string()))
        };
        let Some(reference_frame) = self.reference_frame else {
            return Err(ConvertError::MissingValue("Missing reference frame".to_string()))
        };
        Ok(EllipticalCoordinate {
            latitude,
            longitude,
            reference_frame,
            altitude: self.altitude,
            altitude_uncertainty: self.altitude_uncertainty,
            position_uncertainty: self.position_uncertainty,
            timestamp: self.timestamp,
        })
    }
}

///
/// Represents a coordinate in 3D Cartesian Space (X, Y, Z)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CartesianCoordinate {
    x: Length,
    y: Length,
    z: Length,
}

impl CartesianCoordinate {
    #[must_use]
    pub fn new(x: Length, y: Length, z: Length) -> CartesianCoordinate {
        CartesianCoordinate { x, y, z }
    }

    #[must_use]
    pub fn new_meters(x_meters: f64, y_meters: f64, z_meters: f64) -> CartesianCoordinate {
        Self::new(
            Length::new_meters(x_meters),
            Length::new_meters(y_meters),
            Length::new_meters(z_meters),
        )
    }

    #[must_use]
    pub fn get_x(&self) -> &Length {
        &self.x
    }

    #[must_use]
    pub fn get_y(&self) -> &Length {
        &self.y
    }

    #[must_use]
    pub fn get_z(&self) -> &Length {
        &self.z
    }
}

///
/// An uncertainty type for a position.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PositionUncertainty {
    /// Represents a uncertainty represented as a perfect circle, either radius or diameter.
    CircularUncertainty(CircularDimension),

    /// Represents an uncertainty represented as an ellipse, optionally oriented
    EllipticalUncertainty(Ellipse),
}

///
/// A coordinate type that represents an Azimuth/Elevation look angle from a particular
/// refernece point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HorizontalCoordinate {
    reference: AbsoluteCoordinateType,
    azimuth: Azimuth,
    elevation: Elevation,
}

#[cfg(target_os = "windows")]
pub mod windows_conv {
    use windows::Devices::Geolocation::Geocoordinate;

    use irox_units::shapes::CircularDimension;
    use irox_units::units::angle::Angle;
    use irox_units::units::length::Length;

    use crate::altitude::{Altitude, AltitudeReferenceFrame};
    use crate::coordinate::{EllipticalCoordinate, EllipticalCoordinateBuilder, Latitude, Longitude, PositionUncertainty};
    use crate::error::ConvertError;
    use crate::geo::EllipticalShape;
    use crate::geo::standards::wgs84::{WGS84_EPSG_CODE, WGS84_SHAPE};

    impl TryFrom<&Geocoordinate> for EllipticalCoordinate {
        type Error = ConvertError;

        fn try_from(value: &Geocoordinate) -> Result<Self, Self::Error> {
            let mut bld = EllipticalCoordinateBuilder::new();

            let Ok(point) = value.Point() else {
                return Err(ConvertError::MissingValue("Missing point value".to_string()));
            };
            let Ok(pos) = point.Position() else {
                return Err(ConvertError::MissingValue("Missing position value".to_string()))
            };
            bld.with_latitude(Latitude(Angle::new_degrees(pos.Latitude)));
            bld.with_longitude(Longitude(Angle::new_degrees(pos.Longitude)));

            let alt = Length::new_meters(pos.Altitude);
            let alt_frame = match point.AltitudeReferenceSystem() {
                Ok(frame) => match frame.0 {
                    1 => AltitudeReferenceFrame::Terrain,
                    2 => AltitudeReferenceFrame::Ellipsoid,
                    3 => AltitudeReferenceFrame::Geoid,
                    4 => AltitudeReferenceFrame::SurfaceFeatures,
                    _ => AltitudeReferenceFrame::Unspecified,
                },
                Err(_) => AltitudeReferenceFrame::Unspecified,
            };
            bld.with_altitude(Altitude::new(alt, alt_frame));

            bld.with_reference_frame(match point.SpatialReferenceId(){
                Ok(epsg) => match epsg {
                    WGS84_EPSG_CODE => WGS84_SHAPE,
                    e => EllipticalShape::EpsgDatum(e)
                },
                Err(_) => {
                    // assume wgs84.
                    WGS84_SHAPE
                }
            });

            if let Ok(acc) = value.Accuracy() {
                let length = Length::new_meters(acc);
                let rad = CircularDimension::new_radius(length);
                bld.with_position_uncertainty(PositionUncertainty::CircularUncertainty(rad));
            }
            if let Ok(acc) = value.AltitudeAccuracy() {
                if let Ok(acc) = acc.GetDouble() {
                    bld.with_altitude_uncertainty(Length::new_meters(acc));
                }
            }

            if let Ok(ts) = value.PositionSourceTimestamp() {
                if let Ok(ts) = ts.GetDateTime() {
                    bld.with_timestamp(ts.UniversalTime as f64);
                }
            }

            bld.build()
        }
    }
}
