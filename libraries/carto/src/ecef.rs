// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::altitude::{Altitude, AltitudeReferenceFrame};
use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate, Latitude, Longitude};
use crate::error::ConvertError;
use crate::geo::standards::wgs84::WGS84_ELLIPSOID;
use crate::geo::EllipticalShape;
use crate::position_type::{ECEFPosition, WGS84Position};
use irox_units::units::angle::Angle;
use irox_units::units::length::Length;

pub struct ECEF;
impl ECEF {
    pub fn coord_to_ecef(coord: &EllipticalCoordinate) -> Result<ECEFPosition, ConvertError> {
        let shape = coord.get_reference_frame().as_ellipsoid()?;

        let v = shape.radius_curvature_prime_vertical(coord.get_latitude());

        let cosphi = coord.get_latitude().cos();
        let sinphi = coord.get_latitude().sin();
        let coslam = coord.get_longitude().cos();
        let sinlam = coord.get_longitude().sin();

        let mut h = Length::default();
        if let Some(ht) = coord.get_altitude() {
            let arf = ht.reference_frame();
            if arf != AltitudeReferenceFrame::Ellipsoid {
                return Err(ConvertError::MismatchedReferenceFrame(format!(
                    "Expecting altitude reference frame to be Ellipsoid, but was {arf:?}",
                )));
            };
            h += ht.value();
        }
        let e2 = shape.first_eccentricity_squared();

        let x = (v + h) * cosphi * coslam;
        let y = (v + h) * cosphi * sinlam;
        let z = (v * (1. - e2) + h) * sinphi;

        let cc = CartesianCoordinate::new(x, y, z);
        let ecef = ECEFPosition(cc);
        Ok(ecef)
    }

    pub fn ecef_to_coord(
        ecef: &ECEFPosition,
        shape: EllipticalShape,
    ) -> Result<EllipticalCoordinate, ConvertError> {
        let s = shape.as_ellipsoid()?;
        let coord = ecef.0;
        let x = coord.get_x().as_meters().value();
        let y = coord.get_y().as_meters().value();
        let z = coord.get_z().as_meters().value();
        let lam = y.atan2(x);

        let e2 = s.first_eccentricity_squared();
        let eps = e2 / (1. - e2);
        let b = s.semi_minor_axis_b().as_meters().value();
        let a = s.semi_major_axis_a().as_meters().value();
        let p = (x * x + y * y).sqrt();
        let q = (z * a).atan2(p * b);

        let s3 = q.sin().powi(3);
        let c3 = q.cos().powi(3);
        let phi = (z + eps * b * s3).atan2(p - e2 * a * c3);

        let lat = Latitude(Angle::new_radians(phi));
        let v = s.radius_curvature_prime_vertical(&lat);
        let ht = (p / phi.cos()) - v.as_meters().value();

        let lon = Longitude(Angle::new_radians(lam));
        let out = EllipticalCoordinate::new(lat, lon, shape);
        let out = out.with_altitude(Altitude::new(
            Length::new_meters(ht),
            AltitudeReferenceFrame::Ellipsoid,
        ));
        Ok(out)
    }
}

pub struct WGS84ECEF;
impl WGS84ECEF {
    pub fn ecef_to_coord(ecef: &ECEFPosition) -> WGS84Position {
        let s = WGS84_ELLIPSOID;
        let coord = ecef.0;
        let x = coord.get_x().as_meters().value();
        let y = coord.get_y().as_meters().value();
        let z = coord.get_z().as_meters().value();
        let lam = y.atan2(x);

        let e2 = s.first_eccentricity_squared();
        let eps = e2 / (1. - e2);
        let b = s.semi_minor_axis_b().as_meters().value();
        let a = s.semi_major_axis_a().as_meters().value();
        let p = (x * x + y * y).sqrt();
        let q = (z * a).atan2(p * b);

        let s3 = q.sin().powi(3);
        let c3 = q.cos().powi(3);
        let phi = (z + eps * b * s3).atan2(p - e2 * a * c3);

        let lat = Latitude(Angle::new_radians(phi));
        let v = s.radius_curvature_prime_vertical(&lat);
        let ht = (p / phi.cos()) - v.as_meters().value();

        let lon = Longitude(Angle::new_radians(lam));
        let out = EllipticalCoordinate::new(lat, lon, s.as_elliptical_shape());
        let out = out.with_altitude(Altitude::new(
            Length::new_meters(ht),
            AltitudeReferenceFrame::Ellipsoid,
        ));
        WGS84Position(out)
    }
    pub fn coord_to_ecef(coord: WGS84Position) -> Result<ECEFPosition, ConvertError> {
        let shape = WGS84_ELLIPSOID;
        let coord = coord.0;
        let v = shape.radius_curvature_prime_vertical(coord.get_latitude());

        let cosphi = coord.get_latitude().cos();
        let sinphi = coord.get_latitude().sin();
        let coslam = coord.get_longitude().cos();
        let sinlam = coord.get_longitude().sin();

        let mut h = Length::default();
        if let Some(ht) = coord.get_altitude() {
            let arf = ht.reference_frame();
            if arf != AltitudeReferenceFrame::Ellipsoid {
                return Err(ConvertError::MismatchedReferenceFrame(format!(
                    "Expecting altitude reference frame to be Ellipsoid, but was {arf:?}",
                )));
            };
            h += ht.value();
        }
        let e2 = shape.first_eccentricity_squared();

        let x = (v + h) * cosphi * coslam;
        let y = (v + h) * cosphi * sinlam;
        let z = (v * (1. - e2) + h) * sinphi;

        let cc = CartesianCoordinate::new(x, y, z);
        let ecef = ECEFPosition(cc);
        Ok(ecef)
    }
}

#[cfg(test)]
mod test {
    use crate::altitude::{Altitude, AltitudeReferenceFrame};
    use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate, Latitude, Longitude};
    use crate::ecef::ECEF;
    use crate::error::ConvertError;
    use crate::geo::standards::StandardShapes;
    use crate::position_type::ECEFPosition;
    use irox_tools::assert_eq_eps;
    use irox_units::units::angle::Angle;
    use irox_units::units::length::Length;

    #[test]
    pub fn test1() -> Result<(), ConvertError> {
        let coord = EllipticalCoordinate::new(
            Latitude(Angle::new_dms(41, 21, 12.99487)),
            Longitude(Angle::new_dms(-72, 01, 25.04041)),
            StandardShapes::NAD83.into(),
        )
        .with_altitude(Altitude::new(
            Length::new_meters(635.478),
            AltitudeReferenceFrame::Ellipsoid,
        ));

        let exp = ECEFPosition(CartesianCoordinate::new_meters(
            1479921.8391126473,
            -4561128.807626251,
            4192401.5311963623,
        ));
        assert_eq!(exp, ECEF::coord_to_ecef(&coord)?);

        let lla = ECEF::ecef_to_coord(&exp, StandardShapes::NAD83.into())?;
        assert_eq_eps!(
            coord.get_latitude().as_degrees().value(),
            lla.get_latitude().as_degrees().value(),
            1e-13
        );
        assert_eq_eps!(
            coord.get_longitude().as_degrees().value(),
            lla.get_longitude().as_degrees().value(),
            1e-13
        );
        assert_eq_eps!(
            coord.get_altitude().unwrap_or_default().value().value(),
            lla.get_altitude().unwrap_or_default().value().value(),
            1e-5
        );

        Ok(())
    }

    #[test]
    pub fn test_gigs() -> Result<(), ConvertError> {
        let tests = [
            (-962479.592, 555687.852, 6260738.653, 80., 150., 1214.137),
            (-962297.006, 555582.435, 6259542.961, 80., 150., 0.),
            (
                -1598248.169,
                2768777.623,
                5501278.468,
                60.00475191,
                119.9952454,
                619.6317,
            ),
            (
                -1598023.169,
                2768387.912,
                5500499.045,
                60.00475258,
                119.9952447,
                -280.3683,
            ),
            (2764210.405, 4787752.865, 3170468.52, 30., 60., 189.569),
            (2764128.32, 4787610.688, 3170373.735, 30., 60., 0.),
            (6377934.396, -112., 434., 0.00392509, -0.00100615, -202.5882),
            (
                6374934.396,
                -112.,
                434.,
                0.00392695,
                -0.00100662,
                -3202.5881,
            ),
            (
                6367934.396,
                -112.,
                434.,
                0.00393129,
                -0.00100773,
                -10202.5881,
            ),
            (2764128.32, -4787610.688, -3170373.735, -30., -60., 0.),
            (
                2763900.349,
                -4787215.831,
                -3170110.497,
                -30.,
                -60.,
                -526.476,
            ),
            (
                2763880.863,
                -4787182.081,
                -3170087.997,
                -30.,
                -60.,
                -571.476,
            ),
            (
                -1598023.169,
                -2768611.912,
                -5499631.045,
                -59.99934884,
                -119.9932376,
                -935.0995,
            ),
            (
                -1597798.169,
                -2768222.201,
                -5498851.622,
                -59.99934874,
                -119.9932366,
                -1835.0995,
            ),
            (-962297.006, -555582.435, -6259542.961, -80., -150., 0.),
            (
                -962150.945,
                -555498.107,
                -6258586.462,
                -80.,
                -150.,
                -971.255,
            ),
            (
                -961798.295,
                -555294.505,
                -6256277.087,
                -80.,
                -150.,
                -3316.255,
            ),
            (
                -2187336.719,
                -112.,
                5971017.093,
                70.00490733,
                -179.9970662,
                -223.6178,
            ),
            (-2904698.555, -2904698.555, 4862789.038, 50., -135., 0.),
            (
                371.,
                -5783593.614,
                2679326.11,
                25.00366329,
                -89.99632465,
                -274.7286,
            ),
            (6378137., 0., 0., 0., 0., 0.),
            (
                -4087095.478,
                2977467.559,
                -3875457.429,
                -37.65282217,
                143.9264925,
                737.7182,
            ),
            (
                -4085919.959,
                2976611.233,
                -3874335.274,
                -37.65282206,
                143.9264921,
                -1099.2288,
            ),
            (
                -4084000.165,
                2975212.729,
                -3872502.631,
                -37.65282187,
                143.9264914,
                -4099.2288,
            ),
            (
                -4079520.647,
                2971949.553,
                -3868226.465,
                -37.65282143,
                143.9264898,
                -11099.2288,
            ),
            (-2904698.555, 2904698.555, -4862789.038, -50., 135., 0.),
            (
                -2187336.719,
                -112.,
                -5970149.093,
                -70.00224647,
                -179.9970662,
                -1039.2896,
            ),
        ];
        for (_idx, test) in tests.iter().enumerate() {
            let (x, y, z, lat, lon, ht) = *test;
            let exp_ecef = ECEFPosition(CartesianCoordinate::new_meters(x, y, z));
            let exp_lla = EllipticalCoordinate::new(
                Latitude(Angle::new_degrees(lat)),
                Longitude(Angle::new_degrees(lon)),
                StandardShapes::WGS84.into(),
            )
            .with_altitude(Altitude::new(
                Length::new_meters(ht),
                AltitudeReferenceFrame::Ellipsoid,
            ));

            let ecef = ECEF::coord_to_ecef(&exp_lla)?;
            assert_eq_eps!(exp_ecef.0.get_x().value(), ecef.0.get_x().value(), 3e-3);
            assert_eq_eps!(exp_ecef.0.get_y().value(), ecef.0.get_y().value(), 3e-3);
            assert_eq_eps!(exp_ecef.0.get_z().value(), ecef.0.get_z().value(), 3e-3);

            let coord = ECEF::ecef_to_coord(&exp_ecef, StandardShapes::WGS84.into())?;
            assert_eq_eps!(
                exp_lla.get_latitude().as_radians().value(),
                coord.get_latitude().as_radians().value(),
                1e-9
            );
            assert_eq_eps!(
                exp_lla.get_longitude().as_radians().value(),
                coord.get_longitude().as_radians().value(),
                1e-9
            );
            assert_eq_eps!(
                exp_lla.get_altitude().unwrap_or_default().value().value(),
                coord.get_altitude().unwrap_or_default().value().value(),
                1e-3
            );
        }

        Ok(())
    }
}
