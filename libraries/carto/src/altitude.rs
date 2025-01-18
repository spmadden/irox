// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Altitude and Altitude Reference Frames

use core::fmt::{Display, Formatter};

use irox_units::units::length::Length;

/// The reference or zero point for a particular Altitude value
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum AltitudeReferenceFrame {
    /// The frame is unknown or unspecified
    #[default]
    Unspecified,

    /// The frame is "above the specified ellipsoidal model" - most commonly WGS84
    Ellipsoid,

    /// The frame is "above the specified geoidal model" - most commonly EGM/MSL - Mean Sea Level
    Geoid,

    /// The frame is "above the local terrain" - Above Ground Level (AGL)
    Terrain,

    /// The frame is "the center of mass" of the Earth
    Geocentric,

    /// The frame is "above the tallest local surface features"
    /// Examples include trees, buildings, mountains, towers, etc.
    /// Most commonly used by aircraft as a "hard deck", positive values imply an aircraft
    /// will not collide with a structure
    SurfaceFeatures,

    /// Elevation above a standard datum air-pressure level
    PressureAltitude,

    /// The altitude as indicated by a altimeter pressure measuring instrument
    IndicatedAltitude,

    /// The altitude
    DensityAltitude,
}

impl AltitudeReferenceFrame {
    pub fn short_name(&self) -> &'static str {
        match self {
            AltitudeReferenceFrame::Unspecified => "UNK",
            AltitudeReferenceFrame::Ellipsoid => "ELL",
            AltitudeReferenceFrame::Geoid => "MSL",
            AltitudeReferenceFrame::Terrain => "AGL",
            AltitudeReferenceFrame::Geocentric => "GEO",
            AltitudeReferenceFrame::SurfaceFeatures => "MSA",
            AltitudeReferenceFrame::PressureAltitude => "PA",
            AltitudeReferenceFrame::IndicatedAltitude => "IA",
            AltitudeReferenceFrame::DensityAltitude => "DA",
        }
    }
}

/// A distance above a particular reference point
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Altitude {
    value: Length,
    reference_frame: AltitudeReferenceFrame,
}

impl Altitude {
    #[must_use]
    pub const fn new(value: Length, reference_frame: AltitudeReferenceFrame) -> Altitude {
        Altitude {
            value,
            reference_frame,
        }
    }

    #[must_use]
    pub const fn new_unknown(value: Length) -> Altitude {
        Altitude::new(value, AltitudeReferenceFrame::Unspecified)
    }

    #[must_use]
    pub fn value(&self) -> Length {
        self.value
    }

    #[must_use]
    pub fn reference_frame(&self) -> AltitudeReferenceFrame {
        self.reference_frame
    }
}

impl Display for Altitude {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}",
            self.value,
            self.reference_frame.short_name()
        ))
    }
}
