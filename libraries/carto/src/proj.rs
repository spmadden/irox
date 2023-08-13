// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate};

///
/// Allows a projection from Elliptical to Cartesian coordinates
pub trait Projection {
    /// Returns the center elliptical coordinate of this projection
    fn get_center_coords(&self) -> &EllipticalCoordinate;

    /// Projects the elliptical coordinate to an equivalent cartesian coordinate
    fn project_to_cartesian(&self, coord: &EllipticalCoordinate) -> CartesianCoordinate;

    /// Projects the cartesian coordinate to an equivalent elliptical coordinate
    fn project_to_elliptical(&self, coord: &CartesianCoordinate) -> EllipticalCoordinate;
}
