// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Different associated variants of the same position.  ECEF, ENU, LLA

use crate::altitude::Altitude;
use crate::coordinate::{
    AbsoluteCoordinateType, CartesianCoordinate, EllipticalCoordinate, EllipticalCoordinateBuilder,
    Latitude, Longitude, PositionUncertainty, RelativeCoordinateType,
};
use crate::error::ConvertError;
use crate::geo::standards::wgs84::WGS84_SHAPE;
use irox_units::units::length::Length;

///
/// Different variants of the same position.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PositionType {
    EarthCenteredEarthFixed(ECEFPosition),
    WGS84(WGS84Position),
    EastNorthUp(RelativeCoordinateType),
}

///
/// Represents a position in ECEF `EarthCenteredEarthFixed` coordinate space
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ECEFPosition(pub CartesianCoordinate);

///
/// Represents a position in WGS84 Latitude/Longitude
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WGS84Position(pub EllipticalCoordinate);

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

///
/// Represents a set of positions that represent the exact same point in space, in varying different
/// formats/encodings/projections.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Positions {
    pub ecef: Option<ECEFPosition>,
    pub latlon: Option<WGS84Position>,
    pub enu: Option<ENUPosition>,
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
    pub fn build(self) -> Positions {
        self.positions
    }
}
