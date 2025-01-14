// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::coordinate::{AbsoluteCoordinateType, CartesianCoordinate};
use crate::ecef::ECEF;
use crate::error::ConvertError;
use crate::geo::standards::StandardShapes;
use crate::position_type::{ECEFPosition, ENUPosition, WGS84Position};
use irox_tools::math::{AsMatrix, Matrix};
use irox_units::units::length::Length;

///
/// Local East-North-Up coordinate frame (X=East, Y=North, Z=Up)
pub struct LocalENU {
    to_enu: Matrix<3, 3, f64>,
    to_ecef: Matrix<3, 3, f64>,
    base_wgs84: WGS84Position,
    base_ecef: ECEFPosition,
}

///
/// Local North-East-Down coordinate frame (X=North, Y=East, Z=Down)
pub struct LocalNED;

impl LocalENU {
    pub fn new(base_position: AbsoluteCoordinateType) -> Result<LocalENU, ConvertError> {
        let (base, base_ecef) = match base_position {
            AbsoluteCoordinateType::Elliptical(e) => (e, ECEF::coord_to_ecef(&e)?),
            AbsoluteCoordinateType::ECEF(e) => {
                (ECEF::ecef_to_coord(&e, StandardShapes::WGS84.into())?, e)
            }
        };
        let base_wgs84 = WGS84Position(base);
        let lat = base.get_latitude().0.as_radians().value();
        let lon = base.get_longitude().0.as_radians().value();
        let to_enu = Matrix::<3, 3, f64>::rotated_y(lat).rotate_z(lon);
        let to_ecef = to_enu.transpose();
        Ok(Self {
            to_ecef,
            to_enu,
            base_wgs84,
            base_ecef,
        })
    }

    pub fn enu_to_ecef(&self, enu: &ENUPosition) -> Result<ECEFPosition, ConvertError> {
        let coord = enu.coordinate();
        let base = *enu.base_position();
        if base != AbsoluteCoordinateType::ECEF(self.base_ecef)
            && base != AbsoluteCoordinateType::Elliptical(self.base_wgs84.0)
        {
            // can't use cached because it's not in this local ENU reference frame.
            return LocalENU::new(base)?.enu_to_ecef(enu);
        }

        let x = coord.get_x().as_meters().value();
        let y = coord.get_y().as_meters().value();
        let z = coord.get_z().as_meters().value();

        let [[x], [y], [z]] = *self.to_ecef.mul([[z], [x], [y]].as_matrix());
        let base = self.base_ecef.0;
        let x = Length::new_meters(x) + base.get_x();
        let y = Length::new_meters(y) + base.get_y();
        let z = Length::new_meters(z) + base.get_z();

        Ok(ECEFPosition(CartesianCoordinate::new(x, y, z)))
    }

    pub fn ecef_to_enu(&self, ecef: &ECEFPosition) -> Result<ENUPosition, ConvertError> {
        let coord = ecef.0;
        let base = self.base_ecef.0;
        let x = coord.get_x().as_meters().value() - base.get_x().as_meters().value();
        let y = coord.get_y().as_meters().value() - base.get_y().as_meters().value();
        let z = coord.get_z().as_meters().value() - base.get_z().as_meters().value();
        let [[z], [x], [y]] = *self.to_enu.mul([[x], [y], [z]].as_matrix());

        Ok(ENUPosition::new(
            AbsoluteCoordinateType::Elliptical(self.base_wgs84.0),
            CartesianCoordinate::new_meters(x, y, z),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::assert_coordinate_eq_eps;
    use crate::coordinate::{AbsoluteCoordinateType, CartesianCoordinate};
    use crate::ecef::WGS84ECEF;
    use crate::error::ConvertError;
    use crate::local::LocalENU;
    use crate::position_type::{ECEFPosition, ENUPosition};
    use irox_units::units::length::Length;

    #[test]
    pub fn test1() -> Result<(), ConvertError> {
        let base = ECEFPosition(CartesianCoordinate::new(
            Length::new_meters(3652755.3048),
            Length::new_meters(319574.6799),
            Length::new_meters(5201547.3536),
        ));
        let basewgs84 = WGS84ECEF::ecef_to_coord(&base);
        let absbase = AbsoluteCoordinateType::Elliptical(basewgs84.0);
        let enu = LocalENU::new(absbase)?;
        let exp = ENUPosition::new(
            absbase,
            CartesianCoordinate::new(
                Length::new_meters(-189013.869),
                Length::new_meters(-128642.040),
                Length::new_meters(-4220.171),
            ),
        );

        let test = ECEFPosition(CartesianCoordinate::new(
            Length::new_meters(3771793.968),
            Length::new_meters(140253.342),
            Length::new_meters(5124304.349),
        ));

        let res = enu.ecef_to_enu(&test)?;
        assert_eq!(*res.base_position(), absbase);
        let p = res.coordinate();
        assert_coordinate_eq_eps!(*exp.coordinate(), *p, 2e-3);

        let res = enu.enu_to_ecef(&res)?;
        assert_coordinate_eq_eps!(test, res, 1e-10);
        Ok(())
    }
}
