use crate::geo::ellipse::Ellipse;
use crate::geo::ellipsoid::Ellipsoid;
use crate::geo::EllipticalShape;
use irox_units::units::length::{Length, LengthUnits};

pub const WGS84_SEMI_MAJOR_LENGTH: Length = Length::new(6_378_137.0, LengthUnits::Meters);
pub const WGS84_INVERSE_FLATTENING: f64 = 298.257_223_563;
pub const WGS84_SEMI_MINOR_LENGTH: Length = Length::new(6_356_752.314_2, LengthUnits::Meters);
pub const WGS84_FIRST_ECCENTRICITY: f64 = 0.081_819_180_842_6;
pub const WGS84_FIRST_ECCENTRICITY_SQUARED: f64 = 0.006_694_379_990_13;
pub const WGS84_SECOND_ECCENTRICITY: f64 = 0.082_094_437_949_6;
pub const WGS84_SECOND_ECCENTRICITY_SQUARED: f64 = 0.006_739_946_742_27;
pub const WGS84_PARAMS: Ellipse =
    Ellipse::named("WGS84", WGS84_SEMI_MAJOR_LENGTH, WGS84_INVERSE_FLATTENING);
pub const WGS84_SHAPE: EllipticalShape = EllipticalShape::Ellipse(WGS84_PARAMS);

pub static WGS84_ELLIPSOID: Ellipsoid = Ellipsoid {
    semi_major_axis: WGS84_SEMI_MAJOR_LENGTH,
    inverse_flattening: WGS84_INVERSE_FLATTENING,
    semi_minor_axis: WGS84_SEMI_MINOR_LENGTH,
    first_eccentricity: WGS84_FIRST_ECCENTRICITY,
    first_eccentricity_squared: WGS84_FIRST_ECCENTRICITY_SQUARED,
    second_eccentricity: WGS84_SECOND_ECCENTRICITY,
    second_eccentricity_squared: WGS84_SECOND_ECCENTRICITY_SQUARED,
};

pub static WGS84_MEAN_RADIUS_PARAMS: Ellipse = Ellipse::new_sphere_meters(6_371_008.771_4);
pub static WGS84_EQUAL_AREA_SPHERE_PARAMS: Ellipse = Ellipse::new_sphere_meters(6_317_007.180_9);
pub static WGS84_EQUAL_VOLUME_SPHERE_PARAMS: Ellipse = Ellipse::new_sphere_meters(6_371_000.790_0);
