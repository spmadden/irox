use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate, Latitude, Longitude};
use crate::geo::ellipsoid::{Ellipsoid, MeridianCalculators};
use crate::geo::standards::StandardShapes;
use crate::proj::Projection;
use irox_units::units::angle::Angle;
use irox_units::units::length;
use irox_units::units::length::Length;

#[derive(Debug, Clone)]
pub struct TransverseMercator {
    center: EllipticalCoordinate,
    shape: Ellipsoid,

    false_northing: Length,
    false_easting: Length,

    scale_factor: f64,
}

impl TransverseMercator {
    pub fn builder() -> TMBuilder {
        TMBuilder {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TMBuilder {
    tm: TransverseMercator,
    fe_set: bool,
    fn_set: bool,
}
impl TMBuilder {
    pub fn with_scale_factor(mut self, scale_factor: f64) -> Self {
        self.tm.scale_factor = scale_factor;
        self
    }
    pub fn with_shape(mut self, shape: Ellipsoid) -> Self {
        self.tm.shape = shape;
        self
    }
    pub fn with_center(mut self, center: EllipticalCoordinate) -> Self {
        if !self.fn_set {
            let false_northing = match center.get_latitude().0.value() {
                0.0.. => 0.0,
                ..=0.0 => 10_000_000.,
                _ => unreachable!(),
            };
            self.tm.false_northing = Length::new_meters(false_northing);
        }
        if !self.fe_set {
            self.tm.false_easting = Length::new_meters(500_000.);
        }
        self.tm.center = center;
        self
    }
    pub fn with_false_northing(mut self, false_northing: Length) -> Self {
        self.tm.false_northing = false_northing.as_meters();
        self.fn_set = true;
        self
    }
    pub fn with_false_easting(mut self, false_easting: Length) -> Self {
        self.tm.false_easting = false_easting.as_meters();
        self.fe_set = true;
        self
    }
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
        let cos4_phi = cos_phi.powi(4);
        let cos5_phi = cos_phi.powi(5);
        let cos6_phi = cos_phi.powi(6);
        let cos7_phi = cos_phi.powi(7);
        let tan2_phi = tan_phi.powi(2);
        let tan4_phi = tan_phi.powi(4);
        let tan6_phi = tan_phi.powi(6);

        let ep2 = self.shape.second_eccentricity_squared;
        let ep2cos2 = ep2 * cos2_phi;
        let ep4cos4 = ep2 * ep2 * cos4_phi;
        let ep6cos6 = ep2cos2 * ep4cos4;
        let ep8cos8 = ep4cos4.powi(2);

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

    fn project_to_elliptical(&self, coord: &CartesianCoordinate) -> EllipticalCoordinate {
        let phi_eps = 1e-10;

        let northing = coord.get_y();
        let easting = coord.get_x();

        let mut phi0 = self.center.get_latitude().0.as_radians().value();
        let mut phi_prime = (northing - &self.false_northing) / self.shape.semi_major_axis + phi0;
        loop {
            let dphi = phi_prime - phi0;
            let m = MeridianCalculators::DeakinHunterKarney
                .get(&self.shape)
                .meridional_arc_distance(&Angle::new_radians(dphi));

            let phip_eps = northing - &self.false_northing - m;
            if phip_eps.as_meters().value() < phi_eps {
                break;
            }
            phi0 = phi_prime;
            phi_prime = (northing - &self.false_northing - m) / self.shape.semi_major_axis + phi0;
        }

        let phiplat = Latitude(Angle::new_radians(phi_prime));

        let v = self
            .shape
            .radius_curvature_prime_vertical(&phiplat)
            .as_meters()
            .value();
        let v3 = v.powi(3);
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

        let cos_phip = phi_prime.cos();
        let cos2_phip = cos_phip.powi(2);

        let k0 = self.scale_factor;
        let k02 = k0.powi(2);
        let k04 = k0.powi(4);

        let e2cos2 = self.shape.second_eccentricity_squared * cos2_phip;
        let e4cos4 = e2cos2.powi(2);

        let t10 = tan_phip / (2. * p * v * k02);

        let t11a = tan_phip / (24. * p * v3 * k04);
        let t11b = 5. + 3. * tan2_phip + e2cos2 - 4. * e4cos4 - 9. * tan2_phip * e2cos2;
        let t11 = t11a * t11b;

        let t12 = 0.0;
        let t13 = 0.0;
        let t14 = 0.0;
        let t15 = 0.0;
        let t16 = 0.0;
        let t17 = 0.0;

        let phi = phi_prime - de2 * t10 + de4 * t11 - de6 * t12 + de8 * t13;

        let lam0 = self.center.get_longitude().0.as_radians().value();
        let lam = lam0 + de * t14 - de3 * t15 + de5 * t16 - de7 * t17;

        EllipticalCoordinate::new(
            Latitude(Angle::new_radians(phi)),
            Longitude(Angle::new_radians(lam)),
            self.shape.into(),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::coordinate::{EllipticalCoordinate, Latitude, Longitude};
    use crate::geo::standards::StandardShapes;
    use crate::proj::Projection;
    use crate::tm::TransverseMercator;

    use crate::geo::ellipsoid::Ellipsoid;
    use irox_tools::assert_eq_eps;
    use irox_units::units::angle::Angle;

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
                500000.0,
                8100702.9,
            ),
            TestPoint::new(
                StandardShapes::Hayford_International,
                47,
                30.,
                102.,
                789422.07,
                3322624.35,
            ),
            TestPoint::new(
                StandardShapes::Hayford_International,
                48,
                30.,
                102.,
                210577.93,
                3322624.35,
            ),
            TestPoint::new(
                StandardShapes::Hayford_International,
                12,
                Angle::new_dms(72, 4, 32.110).value(),
                Angle::new_dms(-113, 54, 43.321).value(),
                400000.00,
                8000000.01,
            ),
            TestPoint::new(
                StandardShapes::Hayford_International,
                11,
                Angle::new_dms(72, 4, 32.110).value(),
                Angle::new_dms(-113, 54, 43.321).value(),
                606036.97,
                8000301.04,
            ),
        ];

        for point in points {
            let zone_lon = (point.zone - 1) as f64 * 6. - 177.;
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

            println!("Delta (x, y) = ({}, {})", deltax, deltay);
        }
    }
}
