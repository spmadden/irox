// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Ellipsoids, and calculators therefor

use irox_units::units::length::Length;

use crate::geo::ellipse::Ellipse;
use crate::geo::standards::wgs84::WGS84_ELLIPSOID;
use crate::geo::EllipticalShape;

use irox_tools::cfg_feature_std;

cfg_feature_std! {
    use crate::coordinate::Latitude;
    use irox_units::units::compass::{Azimuth, Compass, CompassReference, RotationDirection};

}

impl From<Ellipse> for Ellipsoid {
    fn from(value: Ellipse) -> Self {
        let semi_major_axis = value.semi_major_axis_a();
        let inverse_flattening = value.inverse_flattening();
        let semi_minor_axis = value.semi_minor_axis_b();
        let first_eccentricity = value.first_eccentricity();
        let first_eccentricity_squared = value.first_eccentricity_squared();
        let second_eccentricity = value.second_eccentricity();
        let second_eccentricity_squared = value.second_eccentricity_squared();

        Ellipsoid {
            semi_major_axis,
            inverse_flattening,
            semi_minor_axis,
            first_eccentricity,
            first_eccentricity_squared,
            second_eccentricity,
            second_eccentricity_squared,
        }
    }
}

impl From<&Ellipsoid> for Ellipse {
    fn from(value: &Ellipsoid) -> Self {
        Ellipse::new(value.semi_major_axis, value.inverse_flattening)
    }
}

///
/// Ellipsoid calculation values with radius calculation methods.
#[derive(Debug, Copy, Clone)]
pub struct Ellipsoid {
    /// The semi major axis is the axis with the longer length, typically the "equator"
    /// of an ellipsoid
    pub(crate) semi_major_axis: Length,

    /// Inverse flattening is the inverted ratio of the length of the semi-major axis to the length
    /// of the semi-minor axis
    pub(crate) inverse_flattening: f64,

    /// The semi minor axis is the axis with the shorter length, typically the "polar" axis of an
    /// ellipsoid
    pub(crate) semi_minor_axis: Length,

    /// First derivation of the eccentricity
    pub(crate) first_eccentricity: f64,
    /// First derivation of the eccentricity squared
    pub(crate) first_eccentricity_squared: f64,

    /// Second derivation of the eccentricity
    pub(crate) second_eccentricity: f64,

    /// Second derivation of the eccentricity squared
    pub(crate) second_eccentricity_squared: f64,
}

impl Default for Ellipsoid {
    fn default() -> Self {
        WGS84_ELLIPSOID
    }
}

impl From<Ellipsoid> for EllipticalShape {
    fn from(value: Ellipsoid) -> Self {
        EllipticalShape::Ellipse(value.into())
    }
}

impl From<&Ellipsoid> for EllipticalShape {
    fn from(value: &Ellipsoid) -> Self {
        EllipticalShape::Ellipse(value.into())
    }
}

impl Ellipsoid {
    /// Returns the Semi-Major axis of the Ellipsoid (a)
    #[must_use]
    pub const fn semi_major_axis_a(&self) -> Length {
        self.semi_major_axis
    }

    /// Returns the Semi-Minor axis of the Ellipsoid (b)
    #[must_use]
    pub const fn semi_minor_axis_b(&self) -> Length {
        self.semi_minor_axis
    }

    /// Returns the inverse flattening (1 / f)
    #[must_use]
    pub const fn inverse_flattening(&self) -> f64 {
        self.inverse_flattening
    }

    /// Returns e - the first eccentricity
    #[must_use]
    pub const fn first_eccentricity(&self) -> f64 {
        self.first_eccentricity
    }

    /// Returns e^2 - the first eccentricity squared
    #[must_use]
    pub const fn first_eccentricity_squared(&self) -> f64 {
        self.first_eccentricity_squared
    }

    /// Returns e' - the second eccentricity (e prime)
    #[must_use]
    pub const fn second_eccentricity(&self) -> f64 {
        self.second_eccentricity
    }

    /// Returns e'^2 - the second eccentricity (e prime) squared
    #[must_use]
    pub const fn second_eccentricity_sq(&self) -> f64 {
        self.second_eccentricity_squared
    }

    /// Returns the flattening (f) parameter
    #[must_use]
    pub fn flattening_f(&self) -> f64 {
        1.0 / self.inverse_flattening
    }

    /// n - Rapp Vol1 3.19
    #[must_use]
    pub fn third_flattening_n_eta(&self) -> f64 {
        let a = self.semi_major_axis;
        let b = self.semi_minor_axis;
        (a - b) / (a + b)
    }

    cfg_feature_std! {
        ///
        /// Computes the radius of curvature in the meridian (north-south) direction at the indicated
        /// latitude
        /// Rapp Vol1 - 3.87
        #[must_use]
        pub fn radius_curvature_meridian(&self, latitude: &Latitude) -> Length {
            let upper = self.semi_major_axis * (1. - self.first_eccentricity_squared);
            let sin2 = latitude.0.as_radians().value().sin().powi(2);
            let lower = (1. - self.first_eccentricity_squared * sin2).powf(3. / 2.);

            upper / lower
        }

        ///
        /// Computes the radius of curvature in the prime meridian (east-west) direction at the indicated
        /// latitude
        /// Rapp Vol1 - 3.99
        #[must_use]
        pub fn radius_curvature_prime_vertical(&self, latitude: &Latitude) -> Length {
            let sin2 = latitude.0.as_radians().value().sin().powi(2);
            let lower = (1. - self.first_eccentricity_squared * sin2).sqrt();
            self.semi_major_axis / lower
        }

        ///
        /// Computes the radius of curvature in the normal section azimuth at the indicated latitude
        /// Rapp Vol1 - 3.104
        #[must_use]
        pub fn radius_curvature_azimuthal(
            &self,
            latitude: &Latitude,
            azimuth: &Compass<Azimuth>,
        ) -> Length {
            let azimuth = azimuth.as_direction_reference(
                RotationDirection::PositiveClockwise,
                CompassReference::TrueNorth,
            );
            let az_rad = azimuth.angle().as_radians().value();
            let cos2az = az_rad.cos().powi(2);
            let cos2la = latitude.0.as_radians().value().cos().powi(2);

            let n = self.radius_curvature_prime_vertical(latitude);
            let lower = 1.0 + self.second_eccentricity_squared * cos2az * cos2la;

            n / lower
        }

        ///
        /// Computes the average radius of curvature at the indicated latitude
        /// Rapp Vol1 - 3.140
        #[must_use]
        pub fn radius_curvature_average(&self, latitude: &Latitude) -> Length {
            let sin2 = latitude.0.as_radians().value().sin().powi(2);
            let upper = self.semi_major_axis * (1.0 - self.first_eccentricity_squared).sqrt();
            let lower = 1.0 - self.first_eccentricity_squared * sin2;
            upper / lower
        }
    }

    ///
    /// Computes the radius of a sphere that has the same surface area of this ellipsoid
    /// Rapp Vol1 - 3.144
    #[must_use]
    pub fn spherical_radius_equal_area_approximation(&self) -> Length {
        let e2 = self.first_eccentricity_squared;
        let e4 = e2 * e2;
        let e6 = e4 * e2;
        let offset = 1. - e2 / 6. - 17. / 360. * e4 - 67. / 3024. * e6;
        self.semi_major_axis * offset
    }

    ///
    /// Computes the radius of a sphere that has the same interior volume of this ellipsoid
    /// Rapp Vol1 - 3.149
    #[must_use]
    pub fn spherical_radius_equal_volume_approximation(&self) -> Length {
        let e2 = self.first_eccentricity_squared;
        let e4 = e2 * e2;
        let e6 = e4 * e2;
        let offset = 1. - e2 / 6. - 5. / 72. * e4 - 55. / 1296. * e6;
        self.semi_major_axis * offset
    }

    #[must_use]
    pub fn as_ellipse(&self) -> Ellipse {
        Ellipse::from(*self)
    }
    #[must_use]
    pub fn as_elliptical_shape(&self) -> EllipticalShape {
        EllipticalShape::Ellipse(self.as_ellipse())
    }
}
