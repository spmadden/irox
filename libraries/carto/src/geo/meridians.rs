// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::geo::ellipsoid::Ellipsoid;
use irox_units::units::angle::Angle;
use irox_units::units::length::Length;

///
/// Meridian Arc Length Calculators.  A Meridian Arc is an Arc that transits on a great circle path
/// from one pole to the other pole - either from South to North, or North to South.
pub enum MeridianCalculators {
    DeakinHunterKarney,
    Bessel,
}

impl MeridianCalculators {
    #[must_use]
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

///
/// Meridian Arc Length Calculators.  A Meridian Arc is an Arc that transits on a great circle path
/// from one pole to the other pole - either from South to North, or North to South.
pub trait MeridianCalculator {
    fn meridional_arc_distance(&self, delta_lat: &Angle) -> Length;
}

///
/// An implementation of a Meridian Arc Length calculator.
///
/// The best I've found to date of the Karney-Krueger equations, as described by Deakin, Hunter, and
/// Karney on p4, eq 38 of the cited paper.
///
/// Deakin, R.E., Hunter, M.N. and Karney, C.F.F., (2010).
/// 'A FRESH LOOK AT THE UTM PROJECTION: Karney-Krueger equations V2', Presented at the Surveying and
/// Spatial Sciences Institute (SSSI) Land Surveying Commission National Conference,
/// Melbourne, 18-21 April, 2012
pub struct DeakinHunterKarneyMeridianCalculator {
    third_flattening_n: f64,
    coefficients: [f64; 9],
    semi_major_axis_a: Length,
}

impl DeakinHunterKarneyMeridianCalculator {
    #[must_use]
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
        let c8 = (109_395. / 262_144.) * n8;

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

        for (idx, coeff) in self.coefficients.iter().enumerate() {
            let base = (2. * lat * idx as f64).sin();
            let adj = coeff * base;
            val += adj;
        }
        val * self.semi_major_axis_a / (1. + self.third_flattening_n)
    }
}

///
/// Bessel's formula for the Meridian Arc, as described on page 2, eq 5 of Kawase 2011:
///
/// Kawase, K. (2011): 'A General Formula for Calculating Meridian Arc Length and its Application to
/// Coordinate Conversion in the Gauss-Krüger Projection', Bulletin of the Geospatial
/// Information Authority of Japan, Vol.59 December, 2011
pub struct BesselMeridianCalculator {
    coefficients: [f64; 4],
    prefix: Length,
}

impl BesselMeridianCalculator {
    #[must_use]
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