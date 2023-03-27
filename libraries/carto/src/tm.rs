use crate::coordinate::EllipticalCoordinate;
use crate::geo::EllipticalShape;

pub struct TransverseMercator {
    center: EllipticalCoordinate,
    shape: EllipticalShape,

}