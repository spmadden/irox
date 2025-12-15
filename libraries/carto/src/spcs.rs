// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate, Latitude, Longitude};
use crate::geo::standards::wgs84::WGS84_SHAPE;
use crate::proj::Projection;
use irox_units::units::angle::Angle;
use irox_units::units::length::Length;

pub struct SPCSPolyParams {
    pub ctr: EllipticalCoordinate,
    pub offset: CartesianCoordinate,
    /// L: forward parameters
    pub l: [f64; 5],
    /// G: inverse parameters
    pub g: [f64; 5],
    /// F: grid scale factors
    pub f: [f64; 3],
    pub r0: f64,
}
#[allow(clippy::upper_case_acronyms)]
pub struct SPCS {
    params: SPCSPolyParams,
}
impl Projection for SPCS {
    fn get_center_coords(&self) -> &EllipticalCoordinate {
        todo!()
    }

    fn project_to_cartesian(&self, coord: &EllipticalCoordinate) -> CartesianCoordinate {
        let b0 = self.params.ctr.get_latitude();
        let l0 = self.params.ctr.get_longitude();
        let dphi = coord.get_latitude().0.as_degrees() - b0.0.as_degrees();
        let dphi = dphi.as_degrees().value();
        let [l1, l2, l3, l4, l5] = self.params.l;
        let u = l5 * dphi + l4;
        let u = u * dphi + l3;
        let u = u * dphi + l2;
        let u = u * dphi + l1;
        let u = u * dphi;
        let r = self.params.r0 - u;
        let gamma = l0.0 - coord.get_longitude().0;
        let gamma = gamma * b0.sin();

        let e = r * gamma.sin();
        let n = u + e * (gamma / 2.).tan();
        let e = Length::new_meters(e);
        let n = Length::new_meters(n);
        CartesianCoordinate::new(
            e + self.params.offset.get_x(),
            n + self.params.offset.get_y(),
            Length::new_meters(0.0),
        )
    }

    fn project_to_elliptical(&self, coord: &CartesianCoordinate) -> EllipticalCoordinate {
        let e = (coord.get_x() - self.params.offset.get_x())
            .as_meters()
            .value();
        let n = (coord.get_y() - self.params.offset.get_y())
            .as_meters()
            .value();
        let r = self.params.r0 - n;
        let gamma = (e / r).atan();
        let b0 = self.params.ctr.get_latitude();
        let l0 = self.params.ctr.get_longitude();
        let lon = Longitude(Angle::new_radians(
            l0.0.as_radians().value() - gamma / b0.sin(),
        ));
        let u = n - e * (gamma / 2.).tan();
        let [g1, g2, g3, g4, g5] = self.params.g;
        let dphi = g5 * u + g4;
        let dphi = dphi * u + g3;
        let dphi = dphi * u + g2;
        let dphi = dphi * u + g1;
        let dphi = dphi * u;
        let phi = Latitude(Angle::new_degrees(dphi) + b0.0);
        EllipticalCoordinate::new(phi, lon, WGS84_SHAPE)
    }
}

pub const MASS_MAINLAND: SPCS = SPCS {
    params: SPCSPolyParams {
        ctr: EllipticalCoordinate::new_degrees_wgs84(42.200_625_287_2, -71.5),
        offset: CartesianCoordinate::new_meters(200_000.0, 883353.0384, 0.0),
        l: [111073.2431, 9.71650, 5.63098, 0.021759, 0.0],
        g: [9.003068344e-6, -7.09026e-15, -3.69789e-20, -1.1855e-27, 0.0],
        f: [0.999964550086, 1.23003e-14, 5.69e-22],
        r0: 7044348.7021,
    },
};
