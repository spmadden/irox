// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! `Ellipse` struct, describes an ellipse using two `CircularDimension`
//! axes and an optional `CompassDirection` orientation of the first axis
//!

use core::fmt::{Display, Formatter};

use crate::shapes::CircularDimension;
use crate::units::compass::CompassDirection;

///
/// A discrete measurement of an Ellipse.  An Ellipse is a circle with two
/// [`CircularDimension`]s offset by 90° to each other.
/// The `Ellipse::first_axis` orientation is indicated by `Ellipse::orientation`
/// and the `Ellipse::second_axis` is oriented orthogonally to the first.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Ellipse {
    first_axis: CircularDimension,
    second_axis: CircularDimension,
    orientation: Option<CompassDirection>,
}

impl Display for Ellipse {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.orientation {
            Some(o) => {
                write!(f, "{} / {} {}", self.first_axis, self.second_axis, o)
            }
            None => {
                write!(f, "{} / {}", self.first_axis, self.second_axis)
            }
        }
    }
}

impl Ellipse {
    #[must_use]
    pub fn new(first_axis: CircularDimension, second_axis: CircularDimension) -> Ellipse {
        Ellipse {
            first_axis,
            second_axis,
            orientation: None,
        }
    }

    #[must_use]
    pub fn semi_major_axis(&self) -> CircularDimension {
        if self.first_axis > self.second_axis {
            return self.first_axis.as_radius();
        }
        self.second_axis.as_radius()
    }

    #[must_use]
    pub fn semi_minor_axis(&self) -> CircularDimension {
        if self.first_axis > self.second_axis {
            return self.second_axis.as_radius();
        }
        self.first_axis.as_radius()
    }

    #[must_use]
    pub fn major_axis(&self) -> CircularDimension {
        if self.first_axis > self.second_axis {
            return self.first_axis.as_diameter();
        }
        self.second_axis.as_diameter()
    }

    #[must_use]
    pub fn minor_axis(&self) -> CircularDimension {
        if self.first_axis > self.second_axis {
            return self.second_axis.as_diameter();
        }
        self.first_axis.as_diameter()
    }

    #[must_use]
    pub fn orientation(&self) -> Option<CompassDirection> {
        self.orientation
    }

    #[must_use]
    pub fn with_orientation(self, orientation: CompassDirection) -> Ellipse {
        Ellipse {
            first_axis: self.first_axis,
            second_axis: self.second_axis,
            orientation: Some(orientation),
        }
    }
}

impl From<CircularDimension> for Ellipse {
    fn from(value: CircularDimension) -> Self {
        Ellipse {
            first_axis: value,
            second_axis: value,
            orientation: None,
        }
    }
}

impl From<&CircularDimension> for Ellipse {
    fn from(value: &CircularDimension) -> Self {
        Ellipse {
            first_axis: *value,
            second_axis: *value,
            orientation: None,
        }
    }
}
