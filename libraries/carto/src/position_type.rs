// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Different associated variants of the same position.  ECEF, ENU, LLA

use crate::coordinate::{
    AbsoluteCoordinateType, CartesianCoordinate, EllipticalCoordinate, RelativeCoordinateType,
};

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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Positions {
    pub ecef: Option<ECEFPosition>,
    pub latlon: Option<WGS84Position>,
    pub enu: Option<ENUPosition>,
}
