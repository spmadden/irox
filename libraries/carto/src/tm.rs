// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Transverse Mercator Map Projection.

use irox_units::units::angle::Angle;
use irox_units::units::length;
use irox_units::units::length::Length;

use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate, Latitude, Longitude};
use crate::geo::ellipsoid::{Ellipsoid, MeridianCalculators};
use crate::geo::standards::StandardShapes;
use crate::proj::Projection;

///
/// An implementation of the Transverse Mercator map projection.
///
/// This implementation uses the expansions contained within `DMA TM 8358.2`, however, 8358.2 has
/// issues with how the 'Meridional Distance' is calculated.  As such, to get full proper nanometer
/// accuracy, the [`crate::geo::ellipsoid::DeakinHunterKarneyMeridianCalculator`] is used to calculate meridian arc length.
///
/// Using the map center returned by Default is not recommended, as it is unlikely to be useful
/// to your specific application.  Recommend ALWAYS setting a map projection center to be within
/// +/- 6 degrees of your chosen area of interest.  6 degrees gives an excellent error factor, and
/// as you get further from the map center, the error increases exponentially.
#[derive(Debug, Clone)]
pub struct TransverseMercator {
    /// Center [0,0] coordinate of the map projection, defaults, to [0 lat, 0 lon]
    center: EllipticalCoordinate,
    /// Shape of the Ellipsoid in use, defaults to WGS84
    shape: Ellipsoid,

    /// The false northing offset on the X axis, defaults to `0` in the northern hemisphere, and
    /// `10_000_000m` in the southern hemisphere
    false_northing: Length,
    /// The false easting offset on the Y axis, defaults to `500_000m`
    false_easting: Length,

    /// The scaling factor, defaults to `0.9996` as per TM 8358.2
    scale_factor: f64,
}

impl TransverseMercator {
    #[must_use]
    pub fn builder() -> TMBuilder {
        TMBuilder {
            ..Default::default()
        }
    }
}

///
/// Builds a transverse mercator map projection
#[derive(Debug, Clone, Default)]
pub struct TMBuilder {
    tm: TransverseMercator,
    fe_set: bool,
    fn_set: bool,
}
impl TMBuilder {
    ///
    /// Opt for a specific scale factor
    #[must_use]
    pub fn with_scale_factor(mut self, scale_factor: f64) -> Self {
        self.tm.scale_factor = scale_factor;
        self
    }

    ///
    /// Opt for a custom, non-WGS84 Ellipsoid
    #[must_use]
    pub fn with_shape(mut self, shape: Ellipsoid) -> Self {
        self.tm.shape = shape;
        self
    }

    /// Opt for a specific center of map projection, the 0,0 coordinates
    #[must_use]
    pub fn with_center(mut self, center: EllipticalCoordinate) -> Self {
        if !self.fn_set {
            let lat = center.get_latitude().0.value();
            let mut false_northing = 0.0;
            if lat < 0.0 {
                false_northing = 10_000_000.;
            }
            self.tm.false_northing = Length::new_meters(false_northing);
        }
        if !self.fe_set {
            self.tm.false_easting = Length::new_meters(500_000.);
        }
        self.tm.center = center;
        self
    }

    ///
    /// Opt for a specific 'False Northing' offset of the Y-axis.
    #[must_use]
    pub fn with_false_northing(mut self, false_northing: Length) -> Self {
        self.tm.false_northing = false_northing.as_meters();
        self.fn_set = true;
        self
    }

    ///
    /// Opt for a specific 'False Easting' offset of the X-axis.
    #[must_use]
    pub fn with_false_easting(mut self, false_easting: Length) -> Self {
        self.tm.false_easting = false_easting.as_meters();
        self.fe_set = true;
        self
    }
    #[must_use]
    pub fn build(self) -> TransverseMercator {
        self.tm
    }
}

impl Default for TransverseMercator {
    fn default() -> Self {
        TransverseMercator {
            scale_factor: 0.9996,
            shape: StandardShapes::WGS84.as_ellipsoid(),

            false_easting: Length::default(),
            false_northing: Length::default(),
            center: EllipticalCoordinate::new_degrees_wgs84(0.0, 0.0),
        }
    }
}

impl Projection for TransverseMercator {
    fn get_center_coords(&self) -> &EllipticalCoordinate {
        &self.center
    }

    ///
    /// Projects (Lat, Lon, Alt) into TM (X-East, Y-North, Z-Up)
    fn project_to_cartesian(&self, coord: &EllipticalCoordinate) -> CartesianCoordinate {
        let w = (coord.get_longitude().0 - self.center.get_longitude().0)
            .as_radians()
            .value();
        let w2 = w.powi(2);
        let w3 = w.powi(3);
        let w4 = w.powi(4);
        let w5 = w.powi(5);
        let w6 = w.powi(6);
        let w7 = w.powi(7);
        let w8 = w.powi(8);

        let latitude = coord.get_latitude();
        let delta_lat = latitude.0 - self.center.get_latitude().0;

        let v = self.shape.radius_curvature_prime_vertical(latitude);

        let phi = latitude.0.as_radians().value();
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        let tan_phi = phi.tan();
        let cos2_phi = cos_phi.powi(2);
        let cos3_phi = cos_phi.powi(3);
        let cos5_phi = cos_phi.powi(5);
        let cos7_phi = cos_phi.powi(7);

        let tan2_phi = tan_phi.powi(2);
        let tan4_phi = tan_phi.powi(4);
        let tan6_phi = tan_phi.powi(6);

        let ep2 = self.shape.second_eccentricity_squared;
        let ep2cos2 = ep2 * cos2_phi;
        let ep4cos4 = ep2cos2.powi(2);
        let ep6cos6 = ep2cos2.powi(3);
        let ep8cos8 = ep2cos2.powi(4);

        let t1 = self.scale_factor
            * MeridianCalculators::DeakinHunterKarney
                .get(&self.shape)
                .meridional_arc_distance(&delta_lat);

        let t2 = v * sin_phi * cos_phi * self.scale_factor / 2.0;

        let t3a = v * sin_phi * cos3_phi * self.scale_factor / 24.0;
        let t3b = 5. - tan2_phi + 9. * ep2cos2 + 4. * ep4cos4;
        let t3 = t3a * t3b;

        let t4a = v * sin_phi * cos5_phi * self.scale_factor / 720.;
        let t4b = 61. - 58. * tan2_phi + tan4_phi + 270. * ep2cos2 - 330. * tan2_phi * ep2cos2;
        let t4c = 445. * ep4cos4 + 324. * ep6cos6 - 680. * tan2_phi * ep4cos4;
        let t4d = 88. * ep8cos8 - 600. * tan2_phi * ep6cos6 - 192. * tan2_phi * ep8cos8;
        let t4 = t4a * (t4b + t4c + t4d);

        let t5a = v * sin_phi * cos7_phi * self.scale_factor / 40320.;
        let t5b = 1385. - 3111. * tan2_phi + 543. * tan4_phi - tan6_phi;
        let t5 = t5a * t5b;

        let northing = self.false_northing + t1 + w2 * t2 + w4 * t3 + w6 * t4 + w8 * t5;

        let t6 = v * cos_phi * self.scale_factor;

        let t7a = v * cos3_phi * self.scale_factor / 6.;
        let t7b = 1. - tan2_phi + ep2cos2;
        let t7 = t7a * t7b;

        let t8a = v * cos5_phi * self.scale_factor / 120.;
        let t8b = 5. - 18. * tan2_phi + tan4_phi + 14. * ep2cos2 - 58. * tan2_phi * ep2cos2
            + 13. * ep4cos4;
        let t8c = 4. * ep6cos6 - 64. * tan2_phi * ep4cos4 - 24. * tan2_phi * ep6cos6;
        let t8 = t8a * (t8b + t8c);

        let t9a = v * cos7_phi * self.scale_factor / 5040.;
        let t9b = 61. - 479. * tan2_phi + 179. * tan4_phi - tan6_phi;
        let t9 = t9a * t9b;

        let easting = self.false_easting + w * t6 + w3 * t7 + w5 * t8 + w7 * t9;
        CartesianCoordinate::new(easting, northing, length::ZERO)
    }

    ///
    /// Projects TM (X-East, Y-North, Z-Up) into (Lat, Lon, Alt)
    fn project_to_elliptical(&self, coord: &CartesianCoordinate) -> EllipticalCoordinate {
        let phi_eps = 1e-9;

        let northing = coord.get_y();
        let easting = coord.get_x();
        let k0 = self.scale_factor;

        let scaled_axis = self.shape.semi_major_axis;
        let phi0 = self.center.get_latitude().0.as_radians().value();
        let mut phi_prime = (northing - &self.false_northing) / scaled_axis + phi0;
        loop {
            let dphi = phi_prime - phi0;
            let m = MeridianCalculators::DeakinHunterKarney
                .get(&self.shape)
                .meridional_arc_distance(&Angle::new_radians(dphi))
                * k0;

            let phip_eps = northing - &self.false_northing - m;
            if phip_eps.as_meters().value().abs() < phi_eps {
                break;
            }
            phi_prime += (northing - &self.false_northing - m) / scaled_axis;
        }

        let phiplat = Latitude(Angle::new_radians(phi_prime));

        let v = self
            .shape
            .radius_curvature_prime_vertical(&phiplat)
            .as_meters()
            .value();
        let v3 = v.powi(3);
        let v5 = v.powi(5);
        let v7 = v.powi(7);
        let p = self
            .shape
            .radius_curvature_meridian(&phiplat)
            .as_meters()
            .value();

        let de = (easting - &self.false_easting).as_meters().value();
        let de2 = de.powi(2);
        let de3 = de.powi(3);
        let de4 = de.powi(4);
        let de5 = de.powi(5);
        let de6 = de.powi(6);
        let de7 = de.powi(7);
        let de8 = de.powi(8);

        let tan_phip = phi_prime.tan();
        let tan2_phip = tan_phip.powi(2);
        let tan4_phip = tan_phip.powi(4);
        let tan6_phip = tan_phip.powi(6);

        let cos_phip = phi_prime.cos();
        let cos2_phip = cos_phip.powi(2);

        let k02 = k0.powi(2);
        let k03 = k0.powi(3);
        let k04 = k0.powi(4);
        let k05 = k0.powi(5);
        let k06 = k0.powi(6);
        let k07 = k0.powi(7);
        let k08 = k0.powi(8);

        let e2cos2 = self.shape.second_eccentricity_squared * cos2_phip;
        let e4cos4 = e2cos2.powi(2);
        let e6cos6 = e2cos2.powi(4);
        let e8cos8 = e2cos2.powi(6);

        let t10 = tan_phip / (2. * p * v * k02);

        let t11a = tan_phip / (24. * p * v3 * k04);
        let t11b = 5. + 3. * tan2_phip + e2cos2 - 4. * e4cos4 - 9. * tan2_phip * e2cos2;
        let t11 = t11a * t11b;

        let t12a = tan_phip / (720. * p * v5 * k06);
        let t12b =
            61. + 90. * tan2_phip + 46. * e2cos2 + 45. * tan4_phip - 252. * tan2_phip * e2cos2;
        let t12c = 3. * e4cos4 + 100. * e6cos6 - 66. * tan2_phip * e4cos4;
        let t12d = 90. * tan4_phip * e2cos2 + 88. * e8cos8 + 225. * tan4_phip * e4cos4;
        let t12e = 84. * tan2_phip * e6cos6 - 192. * tan2_phip * e8cos8;
        let t12 = t12a * (t12b - t12c - t12d + t12e);

        let t13a = tan_phip / (40320. * p * v7 * k08);
        let t13b = 1385. + 3633. * tan2_phip + 4095. * tan4_phip + 1575. * tan6_phip;
        let t13 = t13a * t13b;

        let t14 = v * cos_phip * k0;

        let t15a = 6. * v3 * cos_phip * k03;
        let t15b = 1. + 2. * tan2_phip + e2cos2;
        let t15 = t15b / t15a;

        let t16a = 120. * v5 * cos_phip * k05;
        let t16b = 5. + 6. * e2cos2 + 28. * tan2_phip - 3. * e4cos4 + 8. * tan2_phip * e2cos2;
        let t16c =
            24. * tan4_phip - 4. * e6cos6 + 4. * tan2_phip * e4cos4 + 24. * tan2_phip * e6cos6;
        let t16 = (t16b + t16c) / t16a;

        let t17a = 5040. * v7 * cos_phip * k07;
        let t17b = 61. + 622. * tan2_phip + 1320. * tan4_phip + 720. * tan6_phip;
        let t17 = t17b / t17a;

        let phi = phi_prime - de2 * t10 + de4 * t11 - de6 * t12 + de8 * t13;

        let lam0 = self.center.get_longitude().0.as_radians().value();
        let lam = lam0 + de / t14 - de3 * t15 + de5 * t16 - de7 * t17;

        EllipticalCoordinate::new(
            Latitude(Angle::new_radians(phi)),
            Longitude(Angle::new_radians(lam)),
            self.shape.into(),
        )
    }
}

#[cfg(test)]
mod test {
    use irox_tools::assert_eq_eps;
    use irox_units::units::angle::Angle;

    use crate::coordinate::{EllipticalCoordinate, Latitude, Longitude};
    use crate::geo::standards::StandardShapes;
    use crate::proj::Projection;
    use crate::tm::TransverseMercator;

    struct TestPoint {
        shape: StandardShapes,
        zone: u8,
        test_lat: f64,
        test_lon: f64,
        x: f64,
        y: f64,
    }

    impl TestPoint {
        pub fn new(
            shape: StandardShapes,
            zone: u8,
            test_lat: f64,
            test_lon: f64,
            x: f64,
            y: f64,
        ) -> TestPoint {
            TestPoint {
                shape,
                zone,
                test_lat,
                test_lon,
                x,
                y,
            }
        }
    }

    #[test]
    pub fn dmatm8358_points() {
        let points = [
            TestPoint::new(
                StandardShapes::Hayford_International,
                38,
                73.,
                45.,
                500_000.0,
                8_100_702.9,
            ),
            TestPoint::new(
                StandardShapes::Hayford_International,
                47,
                30.,
                102.,
                789_422.07,
                3_322_624.35,
            ),
            TestPoint::new(
                StandardShapes::Hayford_International,
                48,
                30.,
                102.,
                210_577.93,
                3_322_624.35,
            ),
            TestPoint::new(
                StandardShapes::Hayford_International,
                12,
                Angle::new_dms(72, 4, 32.110).value(),
                Angle::new_dms(-113, 54, 43.321).value(),
                400_000.00,
                8_000_000.01,
            ),
            TestPoint::new(
                StandardShapes::Hayford_International,
                11,
                Angle::new_dms(72, 4, 32.110).value(),
                Angle::new_dms(-113, 54, 43.321).value(),
                606_036.97,
                8_000_301.04,
            ),
        ];

        for point in points {
            let zone_lon = f64::from(point.zone - 1) * 6. - 177.;
            let latitude = Latitude(Angle::new_degrees(0.));
            let longitude = Longitude(Angle::new_degrees(zone_lon));
            let center = EllipticalCoordinate::new(latitude, longitude, point.shape.into());
            let tm = TransverseMercator::builder()
                .with_center(center)
                .with_shape(point.shape.as_ellipsoid())
                .build();

            let test_point = EllipticalCoordinate::new(
                Latitude(Angle::new_degrees(point.test_lat)),
                Longitude(Angle::new_degrees(point.test_lon)),
                point.shape.into(),
            );
            let result = tm.project_to_cartesian(&test_point);
            println!("{result:?}");

            let deltay = point.y - result.get_y().as_meters().value();
            assert_eq_eps!(point.y, result.get_y().as_meters().value(), 4e-3);
            let deltax = point.x - result.get_x().as_meters().value();
            assert_eq_eps!(point.x, result.get_x().as_meters().value(), 4e-3);

            println!("Delta (x, y) = ({deltax}, {deltay})");

            let elli = tm.project_to_elliptical(&result);
            let deltalat = elli.get_latitude().0.as_degrees().value() - point.test_lat;
            let deltalon = elli.get_longitude().0.as_degrees().value() - point.test_lon;
            println!("{elli:?}");
            println!("Delta (lat,lon) = ({deltalat}, {deltalon})");
            assert!(deltalat.abs() < 1e-10);
            assert!(deltalon.abs() < 1e-10)
        }
    }
}
