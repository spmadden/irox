// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate, Latitude, Longitude};
use crate::geo::ellipsoid::Ellipsoid;
use crate::proj::Projection;
use core::f64::consts::FRAC_PI_4;
use irox_units::units::angle::Angle;
use irox_units::units::length::Length;
use std::f64::consts::FRAC_PI_2;

fn m(shape: &Ellipsoid, phi: &Angle) -> f64 {
    let phi = phi.as_radians().value();
    let sin2 = phi.sin().powi(2);
    let e2 = shape.first_eccentricity_squared();
    let cos = phi.cos();
    cos / (1. - e2 * sin2).sqrt()
}
fn t(shape: &Ellipsoid, phi: &Angle) -> f64 {
    let phi = phi.as_radians().value();
    let u = (FRAC_PI_4 - phi / 2.).tan();
    let e = shape.first_eccentricity();
    let sinphi = phi.sin();
    let low = ((1. - e * sinphi) / (1. + e * sinphi)).powf(e / 2.);
    u / low
}
#[derive(Default)]
pub struct LambertConformalConicBuilder {
    center: EllipticalCoordinate,
    shape: Ellipsoid,
    first_parallel: Latitude,
    second_parallel: Latitude,
    false_northing: Length,
    false_easting: Length,
}
impl LambertConformalConicBuilder {
    #[must_use]
    pub fn with_center(mut self, center: EllipticalCoordinate) -> Self {
        self.center = center;
        self
    }
    #[must_use]
    pub fn with_shape(mut self, shape: Ellipsoid) -> Self {
        self.shape = shape;
        self
    }
    #[must_use]
    pub fn with_first_parallel(mut self, first_parallel: Latitude) -> Self {
        self.first_parallel = first_parallel;
        self
    }
    #[must_use]
    pub fn with_second_parallel(mut self, second_parallel: Latitude) -> Self {
        self.second_parallel = second_parallel;
        self
    }
    #[must_use]
    pub fn with_false_northing(mut self, false_northing: Length) -> Self {
        self.false_northing = false_northing;
        self
    }
    #[must_use]
    pub fn with_false_easting(mut self, false_easting: Length) -> Self {
        self.false_easting = false_easting;
        self
    }
    #[must_use]
    pub fn build(self) -> LambertConformalConic {
        let phi0 = self.center.get_latitude();
        let phi1 = self.first_parallel;
        let phi2 = self.second_parallel;

        let a = self.shape.semi_major_axis_a().as_meters().value();

        let t0 = t(&self.shape, phi0);
        let t1 = t(&self.shape, &phi1);
        let t2 = t(&self.shape, &phi2);

        let m1 = m(&self.shape, &phi1);
        let m2 = m(&self.shape, &phi2);

        let n = (m1.ln() - m2.ln()) / (t1.ln() - t2.ln());
        let f = m1 / (n * t1.powf(n));
        let p0 = a * f * t0.powf(n);
        LambertConformalConic {
            a,
            n,
            f,
            p0,
            center: self.center,
            false_easting: self.false_easting,
            false_northing: self.false_northing,
            shape: self.shape,
        }
    }
}

pub struct LambertConformalConic {
    center: EllipticalCoordinate,
    shape: Ellipsoid,
    false_northing: Length,
    false_easting: Length,
    p0: f64,
    a: f64,
    n: f64,
    f: f64,
}

impl Projection for LambertConformalConic {
    fn get_center_coords(&self) -> &EllipticalCoordinate {
        &self.center
    }

    fn project_to_cartesian(&self, coord: &EllipticalCoordinate) -> CartesianCoordinate {
        let n = self.n;
        let p0 = self.p0;
        let a = self.a;
        let f = self.f;
        let lam0 = self.center.get_longitude();
        let phi = coord.get_latitude();
        let lam = coord.get_longitude();
        let t = t(&self.shape, phi);
        let theta = n * (lam.as_radians() - lam0.as_radians());

        let p = a * f * t.powf(n);

        let x = Length::new_meters(p * theta.sin());
        let y = Length::new_meters(p0 - p * theta.cos());

        CartesianCoordinate::new(
            x + self.false_easting,
            y + self.false_northing,
            Length::ZERO,
        )
    }

    fn project_to_elliptical(&self, coord: &CartesianCoordinate) -> EllipticalCoordinate {
        let x = coord.get_x().as_meters().value();
        let y = coord.get_y().as_meters().value();

        let p = (x.powi(2) + (self.p0 - y).powi(2)).sqrt() * self.n.signum();
        let t = (p / (self.a * self.f)).powf(1.0 / self.n);
        let theta = x.atan2(self.p0 - y);
        let lam = theta / self.n + self.center.get_longitude().as_radians().value();
        let e = self.shape.first_eccentricity();

        let mut phi = FRAC_PI_2 - 2. * t.atan();
        loop {
            let phit = FRAC_PI_2
                - 2. * (t * ((1. - e * phi.sin()) / (1. + e * phi.sin())).powf(e / 2.)).atan();
            let eps = (phit - phi).abs();
            phi = phit;
            if eps < 1e-10 {
                break;
            }
        }

        let lat = Latitude(Angle::new_radians(phi));
        let lon = Longitude(Angle::new_radians(lam));

        EllipticalCoordinate::new(lat, lon, self.shape.into())
    }
}

#[cfg(test)]
mod test {
    use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate, Longitude};
    use crate::geo::standards::StandardShapes;
    use crate::lcc::{LambertConformalConicBuilder, Latitude};
    use crate::proj::Projection;
    use irox_tools::assert_eq_eps;
    use irox_units::units::angle::Angle;
    use irox_units::units::length::Length;

    #[test]
    pub fn test_lambert_conic() {
        let lcc = LambertConformalConicBuilder::default()
            .with_first_parallel(Latitude(Angle::new_dms(33, 00, 0.0)))
            .with_second_parallel(Latitude(Angle::new_dms(45, 00, 0.0)))
            .with_center(EllipticalCoordinate::new(
                Latitude(Angle::new_dms(23, 0, 0.0)),
                Longitude(Angle::new_dms(-96, 00, 0.0)),
                StandardShapes::NAD27.into(),
            ))
            .with_shape(StandardShapes::NAD27.as_ellipsoid())
            .build();
        let xyz = lcc.project_to_cartesian(&EllipticalCoordinate::new(
            Latitude(Angle::new_dms(35, 00, 0.0)),
            Longitude(Angle::new_dms(-75, 00, 0.0)),
            StandardShapes::NAD27.into(),
        ));
        assert_eq_eps!(1894410.9, xyz.get_x().as_meters().value(), 0.1);
        assert_eq_eps!(1564649.5, xyz.get_y().as_meters().value(), 0.1);

        let lla = lcc.project_to_elliptical(&CartesianCoordinate::new(
            Length::new_meters(1894410.9),
            Length::new_meters(1564649.5),
            Length::ZERO,
        ));
        assert_eq_eps!(35.0, lla.get_latitude().as_degrees().value(), 1e-6);
        assert_eq_eps!(-75.0, lla.get_longitude().as_degrees().value(), 1e-6);
    }
}
