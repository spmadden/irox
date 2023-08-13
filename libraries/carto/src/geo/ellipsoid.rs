// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_units::units::angle::Angle;
use irox_units::units::compass::{Azimuth, Compass, CompassReference, RotationDirection};
use irox_units::units::length::Length;

use crate::coordinate::Latitude;
use crate::geo::ellipse::Ellipse;
use crate::geo::EllipticalShape;

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
    pub(crate) semi_major_axis: Length,
    pub(crate) inverse_flattening: f64,

    pub(crate) semi_minor_axis: Length,

    pub(crate) first_eccentricity: f64,
    pub(crate) first_eccentricity_squared: f64,

    pub(crate) second_eccentricity: f64,
    pub(crate) second_eccentricity_squared: f64,
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
    pub const fn semi_major_axis_a(&self) -> Length {
        self.semi_major_axis
    }

    /// Returns the Semi-Minor axis of the Ellipsoid (b)
    pub const fn semi_minor_axis_b(&self) -> Length {
        self.semi_minor_axis
    }

    /// Returns the inverse flattening (1 / f)
    pub const fn inverse_flattening(&self) -> f64 {
        self.inverse_flattening
    }

    /// Returns e - the first eccentricity
    pub const fn first_eccentricity(&self) -> f64 {
        self.first_eccentricity
    }

    /// Returns e^2 - the first eccentricity squared
    pub const fn first_eccentricity_squared(&self) -> f64 {
        self.first_eccentricity_squared
    }

    /// Returns e' - the second eccentricity (e prime)
    pub const fn second_eccentricity(&self) -> f64 {
        self.second_eccentricity
    }

    /// Returns e'^2 - the second eccentricity (e prime) squared
    pub const fn second_eccentricity_sq(&self) -> f64 {
        self.second_eccentricity_squared
    }

    /// Returns the flattening (f) parameter
    pub fn flattening_f(&self) -> f64 {
        1.0 / self.inverse_flattening
    }

    /// n - Rapp Vol1 3.19
    pub fn third_flattening_n_eta(&self) -> f64 {
        let a = self.semi_major_axis;
        let b = self.semi_minor_axis;
        (a - b) / (a + b)
    }

    ///
    /// Computes the radius of curvature in the meridian (north-south) direction at the indicated
    /// latitude
    /// Rapp Vol1 - 3.87
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
    pub fn radius_curvature_prime_vertical(&self, latitude: &Latitude) -> Length {
        let sin2 = latitude.0.as_radians().value().sin().powi(2);
        let lower = (1. - self.first_eccentricity_squared * sin2).sqrt();
        self.semi_major_axis / lower
    }

    ///
    /// Computes the radius of curvature in the normal section azimuth at the indicated latitude
    /// Rapp Vol1 - 3.104
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
    pub fn radius_curvature_average(&self, latitude: &Latitude) -> Length {
        let sin2 = latitude.0.as_radians().value().sin().powi(2);
        let upper = self.semi_major_axis * (1.0 - self.first_eccentricity_squared).sqrt();
        let lower = 1.0 - self.first_eccentricity_squared * sin2;
        upper / lower
    }

    ///
    /// Computes the radius of a sphere that has the same surface area of this ellipsoid
    /// Rapp Vol1 - 3.144
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
    pub fn spherical_radius_equal_volume_approximation(&self) -> Length {
        let e2 = self.first_eccentricity_squared;
        let e4 = e2 * e2;
        let e6 = e4 * e2;
        let offset = 1. - e2 / 6. - 5. / 72. * e4 - 55. / 1296. * e6;
        self.semi_major_axis * offset
    }
}

pub enum MeridianCalculators {
    DeakinHunterKarney,
    Bessel,
}

impl MeridianCalculators {
    pub fn get(&self, ellipsoid: &Ellipsoid) -> Box<dyn MeridianCalculator> {
        let out: Box<dyn MeridianCalculator> = match self {
            MeridianCalculators::DeakinHunterKarney => {
                Box::new(DeakinHunterKarneyMeridianCalculator::new(ellipsoid))
            }
            MeridianCalculators::Bessel => Box::new(BesselMeridianCalculator::new(ellipsoid)),
        };
        out
    }
}

pub trait MeridianCalculator {
    fn meridional_arc_distance(&self, delta_lat: &Angle) -> Length;
}

pub struct DeakinHunterKarneyMeridianCalculator {
    third_flattening_n: f64,
    coefficients: [f64; 9],
    semi_major_axis_a: Length,
}

impl DeakinHunterKarneyMeridianCalculator {
    pub fn new(ellipsoid: &Ellipsoid) -> DeakinHunterKarneyMeridianCalculator {
        let third_flattening_n = ellipsoid.third_flattening_n_eta();
        let semi_major_axis_a = ellipsoid.semi_major_axis;

        let n = third_flattening_n;
        let n2 = n * n;
        let n3 = n2 * n;
        let n4 = n3 * n;
        let n5 = n4 * n;
        let n6 = n5 * n;
        let n7 = n6 * n;
        let n8 = n7 * n;

        let c0 = 1. + n2 / 4. + n4 / 64. + n6 / 256. + (25. / 16384.) * n8;
        let c1 = (-3. / 2.) * n + (3. / 16.) * n3 + (3. / 128.) * n5 + (15. / 2048.) * n7;
        let c2 = (15. / 16.) * n2 + (15. / 64.) * n4 - (75. / 2048.) * n6 - (105. / 8192.) * n8;
        let c3 = (-35. / 48.) * n3 + (175. / 768.) * n5 + (245. / 6144.) * n7;
        let c4 = (315. / 512.) * n4 - (441. / 2048.) * n6 - (1323. / 32768.) * n8;
        let c5 = (-693. / 1280.) * n5 + (2079. / 10240.) * n7;
        let c6 = (1001. / 2048.) * n6 - (1573. / 8192.) * n8;
        let c7 = -(6435. / 14336.) * n7;
        let c8 = (109395. / 262144.) * n8;

        let coefficients = [c0, c1, c2, c3, c4, c5, c6, c7, c8];

        DeakinHunterKarneyMeridianCalculator {
            third_flattening_n,
            coefficients,
            semi_major_axis_a,
        }
    }
}
impl MeridianCalculator for DeakinHunterKarneyMeridianCalculator {
    fn meridional_arc_distance(&self, delta_lat: &Angle) -> Length {
        let lat = delta_lat.as_radians().value();
        let mut val = self.coefficients[0] * lat;

        for idx in 1..=8 {
            let base = (2. * lat * idx as f64).sin();
            let adj = self.coefficients[idx] * base;
            val += adj;
        }
        val * self.semi_major_axis_a / (1. + self.third_flattening_n)
    }
}

pub struct BesselMeridianCalculator {
    coefficients: [f64; 4],
    prefix: Length,
}

impl BesselMeridianCalculator {
    pub fn new(ellipsoid: &Ellipsoid) -> BesselMeridianCalculator {
        let n = ellipsoid.third_flattening_n_eta();
        let n2 = n.powi(2);
        let n3 = n.powi(3);
        let n4 = n.powi(4);
        let n5 = n.powi(5);

        let a = 1. + (9. / 4.) * n2 + (225. / 64.) * n4;
        let b = (3. / 2.) * (n + (15. / 8.) * n3 + (175. / 64.) * n5);
        let c = (15. / 16.) * (n2 + (7. / 4.) * n4);
        let d = (35. / 48.) * (n3 + (27. / 16.) * n5);
        let coefficients = [a, b, c, d];

        let prefix = ellipsoid.semi_major_axis * (1. - n).powi(2) * (1. + n);

        BesselMeridianCalculator {
            coefficients,
            prefix,
        }
    }
}

impl MeridianCalculator for BesselMeridianCalculator {
    fn meridional_arc_distance(&self, delta_lat: &Angle) -> Length {
        let phi = delta_lat.as_radians().value();

        let a = self.coefficients[0] * phi;
        let b = self.coefficients[1] * (2. * phi).sin();
        let c = self.coefficients[2] * (4. * phi).sin();
        let d = self.coefficients[3] * (6. * phi).sin();

        self.prefix * (a - b + c - d)
    }
}
