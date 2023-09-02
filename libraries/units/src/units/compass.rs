// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::marker::PhantomData;

use crate::units::angle::Angle;
use crate::units::FromUnits;

///
/// The direction that a compass needle moves for "positive" increases
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum RotationDirection {
    /// Positive-Clockwise is the standard direction a compass needle moves, positive in a
    /// clockwise direction, towards the right, usually with the zero point at 'North'
    #[default]
    PositiveClockwise,

    /// Positive-Counter-Clockwise is the standard direction of rotation on a cartesian
    /// coordinate plane - mostly used for trigonometric convenience (sin/cos/tan/etc) where the
    /// needle moves towards the left when positive, usually with the zero point at 'East'
    PositiveCounterClockwise,
}

impl FromUnits<Angle> for RotationDirection {
    fn from(&self, value: Angle, units: Self) -> Angle {
        value
            * match self {
                RotationDirection::PositiveClockwise => match units {
                    RotationDirection::PositiveClockwise => 1.0,
                    RotationDirection::PositiveCounterClockwise => -1.0,
                },
                RotationDirection::PositiveCounterClockwise => match units {
                    RotationDirection::PositiveClockwise => -1.0,
                    RotationDirection::PositiveCounterClockwise => 1.0,
                },
            }
    }
}

///
/// The "zero" reference point for a compass needle
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum CompassReference {
    /// 0 is True North - the geometric north pole (axis of rotation)
    #[default]
    TrueNorth,

    /// 0 is Magnetic North - the direction that a compass needle points
    MagneticNorth,

    /// 0 is East - Used mostly for X/Y cartesian planes where angles are 0 to the right
    East,
}

impl FromUnits<Angle> for CompassReference {
    fn from(&self, value: Angle, units: Self) -> Angle {
        match self {
            CompassReference::TrueNorth => match units {
                CompassReference::TrueNorth => value,
                _ => todo!(),
            },
            CompassReference::MagneticNorth => match units {
                CompassReference::MagneticNorth => value,
                _ => todo!(),
            },
            CompassReference::East => match units {
                CompassReference::East => value,
                _ => todo!(),
            },
        }
    }
}

/// Represents a heading - the compass direction that the entity is pointing
pub type Heading = Compass<HeadingType>;

/// Type with no functions, used only as a compile-time type-check
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct HeadingType;

/// Represents a track - the compass direction that the entity is travelling
pub type Track = Compass<TrackType>;

/// Type with no functions, used only as a compile-time type-check
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct TrackType;

/// Represents a bearing - the compass direction of your desired destination
pub type Bearing = Compass<BearingType>;

/// Type with no functions, used only as a compile-time type-check
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct BearingType;

/// Represents a course - the compass direction of your desired track
pub type Course = Compass<CourseType>;

/// Type with no functions, used only as a compile-time type-check
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct CourseType;

/// Represents a azimuth - the compass direction of a generic pointing angle
pub type Azimuth = Compass<AzimuthType>;

/// Type with no functions, used only as a compile-time type-check
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct AzimuthType;

/// Represents a compass needle and the direction that it's pointing
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Compass<T> {
    angle: Angle,
    direction: RotationDirection,
    reference: CompassReference,
    _ign: PhantomData<T>,
}

impl<T> Compass<T> {
    ///
    /// Creates a new heading type - see [`HeadingType`] for details.
    #[must_use]
    pub const fn new_heading(
        angle: Angle,
        direction: RotationDirection,
        reference: CompassReference,
    ) -> Compass<HeadingType> {
        Compass {
            angle,
            direction,
            reference,
            _ign: PhantomData,
        }
    }

    ///
    /// Creates a new track type - see ['Track'] for details.
    #[must_use]
    pub const fn new_track(
        angle: Angle,
        direction: RotationDirection,
        reference: CompassReference,
    ) -> Compass<TrackType> {
        Compass {
            angle,
            direction,
            reference,
            _ign: PhantomData,
        }
    }

    ///
    /// Creates a new bearing type - see ['Bearing'] for details.
    #[must_use]
    pub const fn new_bearing(
        angle: Angle,
        direction: RotationDirection,
        reference: CompassReference,
    ) -> Compass<BearingType> {
        Compass {
            angle,
            direction,
            reference,
            _ign: PhantomData,
        }
    }

    ///
    /// Creates a new course type - see ['Course'] for details.
    #[must_use]
    pub const fn new_course(
        angle: Angle,
        direction: RotationDirection,
        reference: CompassReference,
    ) -> Compass<CourseType> {
        Compass {
            angle,
            direction,
            reference,
            _ign: PhantomData,
        }
    }

    ///
    /// Creates a new azimuth type - see ['Azimuth'] for details.
    #[must_use]
    pub const fn new_azimuth(
        angle: Angle,
        direction: RotationDirection,
        reference: CompassReference,
    ) -> Compass<AzimuthType> {
        Compass {
            angle,
            direction,
            reference,
            _ign: PhantomData,
        }
    }

    #[must_use]
    pub const fn angle(&self) -> &Angle {
        &self.angle
    }

    #[must_use]
    pub const fn direction(&self) -> &RotationDirection {
        &self.direction
    }

    #[must_use]
    pub const fn reference(&self) -> &CompassReference {
        &self.reference
    }

    #[must_use]
    pub fn as_direction_reference(
        &self,
        direction: RotationDirection,
        reference: CompassReference,
    ) -> Compass<T> {
        let angle = direction.from(self.angle, self.direction);
        let angle = reference.from(angle, self.reference);
        Compass {
            angle,
            direction,
            reference,
            _ign: PhantomData,
        }
    }
}

///
/// Represents a relative angle from a particular zero point that's not a standard reference like
/// North or East.  Used for "relative bearings" and the like where the angle is referenced to the
/// heading of an entity (like, 10 degrees to the right)
#[derive(Debug, Clone, PartialEq)]
pub struct CompassOffset<T, B> {
    compass: Compass<T>,
    offset: Angle,
    direction: RotationDirection,
    _ign: PhantomData<B>,
}

impl<T, B> CompassOffset<T, B> {
    #[must_use]
    pub fn compass(&self) -> &Compass<T> {
        &self.compass
    }

    #[must_use]
    pub fn offset(&self) -> &Angle {
        &self.offset
    }

    #[must_use]
    pub fn direction(&self) -> &RotationDirection {
        &self.direction
    }
}

/// Represents the relative angle from a particular entities heading
pub type RelativeBearing = CompassOffset<HeadingType, BearingType>;

impl Compass<HeadingType> {
    ///
    /// Converts this heading into a relative bearing using the specified offset and direction
    #[must_use]
    pub fn relative_bearing(self, direction: RotationDirection, offset: Angle) -> RelativeBearing {
        CompassOffset {
            compass: self,
            offset,
            direction,
            _ign: PhantomData,
        }
    }
}
