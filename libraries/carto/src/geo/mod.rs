pub mod ellipse;
pub mod ellipsoid;
pub mod standards;

use crate::coordinate::Latitude;
use crate::units::compass::{Azimuth, Compass, CompassReference, RotationDirection};
use crate::units::length::{Length, LengthUnits};
use ellipse::Ellipse;
use ellipsoid::Ellipsoid;

#[derive(Debug, Clone)]
pub enum EllipticalShape {
    EPSG(String),
    Ellipse(Ellipse),
}
