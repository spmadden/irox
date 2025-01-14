// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Different associated variants of the same position.  ECEF, ENU, LLA

use crate::altitude::Altitude;
use crate::coordinate::{
    AbsoluteCoordinateType, CartesianCoordinate, EllipticalCoordinate, EllipticalCoordinateBuilder,
    Latitude, Longitude, PositionUncertainty,
};
use crate::error::ConvertError;
use crate::geo::standards::wgs84::WGS84_SHAPE;
use core::ops::Deref;
use irox_units::units::length::Length;

///
/// Different variants of the same position.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PositionType {
    EarthCenteredEarthFixed(ECEFPosition),
    WGS84(WGS84Position),
    EastNorthUp(ENUPosition),
    NorthEastDown(NEDPosition),
}

///
/// Represents a position in ECEF `EarthCenteredEarthFixed` coordinate space
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ECEFPosition(pub CartesianCoordinate);
impl ECEFPosition {
    pub fn as_position_type(&self) -> PositionType {
        PositionType::EarthCenteredEarthFixed(*self)
    }
}
impl Deref for ECEFPosition {
    type Target = CartesianCoordinate;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

///
/// Represents a position in WGS84 Latitude/Longitude
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WGS84Position(pub EllipticalCoordinate);
impl WGS84Position {
    pub fn as_position_type(&self) -> PositionType {
        PositionType::WGS84(*self)
    }
}
impl Deref for WGS84Position {
    type Target = EllipticalCoordinate;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct WGS84PositionBuilder {
    builder: EllipticalCoordinateBuilder,
}

impl Default for WGS84PositionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WGS84PositionBuilder {
    #[must_use]
    pub fn new() -> Self {
        let mut builder = EllipticalCoordinateBuilder::new();
        builder.with_reference_frame(WGS84_SHAPE);
        WGS84PositionBuilder { builder }
    }

    #[must_use]
    pub fn with_latitude(mut self, latitude: Latitude) -> Self {
        self.builder.with_latitude(latitude);
        self
    }

    #[must_use]
    pub fn with_longitude(mut self, longitude: Longitude) -> Self {
        self.builder.with_longitude(longitude);
        self
    }

    #[must_use]
    pub fn with_altitude(mut self, altitude: Altitude) -> Self {
        self.builder.with_altitude(altitude);
        self
    }

    #[must_use]
    pub fn with_altitude_uncertainty(mut self, altitude_uncertainty: Length) -> Self {
        self.builder.with_altitude_uncertainty(altitude_uncertainty);
        self
    }

    #[must_use]
    pub fn with_position_uncertainty(mut self, position_uncertainty: PositionUncertainty) -> Self {
        self.builder.with_position_uncertainty(position_uncertainty);
        self
    }

    pub fn build(self) -> Result<WGS84Position, ConvertError> {
        Ok(WGS84Position(self.builder.build()?))
    }
}

///
/// Represents a position in `EastNorthUp` coordinate space
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ENUPosition {
    base_position: AbsoluteCoordinateType,
    coordinate: CartesianCoordinate,
}
impl ENUPosition {
    pub fn new(base_position: AbsoluteCoordinateType, coordinate: CartesianCoordinate) -> Self {
        Self {
            base_position,
            coordinate,
        }
    }
    pub fn as_position_type(&self) -> PositionType {
        PositionType::EastNorthUp(*self)
    }
    pub fn base_position(&self) -> &AbsoluteCoordinateType {
        &self.base_position
    }
    pub fn coordinate(&self) -> &CartesianCoordinate {
        &self.coordinate
    }
}

///
/// Represents a position in `NorthEastDown` coordinate space
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NEDPosition {
    base_position: AbsoluteCoordinateType,
    coordinate: CartesianCoordinate,
}
impl NEDPosition {
    pub fn as_position_type(&self) -> PositionType {
        PositionType::NorthEastDown(*self)
    }
}

///
/// Represents a set of positions that represent the exact same point in space, in varying different
/// formats/encodings/projections.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Positions {
    pub ecef: Option<ECEFPosition>,
    pub latlon: Option<WGS84Position>,
    pub enu: Option<ENUPosition>,
    pub ned: Option<NEDPosition>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PositionsBuilder {
    positions: Positions,
}

impl PositionsBuilder {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    #[must_use]
    pub fn with_ecef(mut self, ecef: ECEFPosition) -> Self {
        self.positions.ecef = Some(ecef);
        self
    }

    #[must_use]
    pub fn with_latlon(mut self, latlon: WGS84Position) -> Self {
        self.positions.latlon = Some(latlon);
        self
    }

    #[must_use]
    pub fn with_enu(mut self, enu: ENUPosition) -> Self {
        self.positions.enu = Some(enu);
        self
    }

    #[must_use]
    pub fn with_ned(mut self, ned: NEDPosition) -> Self {
        self.positions.ned = Some(ned);
        self
    }

    #[must_use]
    pub fn build(self) -> Positions {
        self.positions
    }
}
