use irox_units::coordinate::{CartesianCoordinate, EllipticalCoordinate};

pub trait Projection {
    fn get_center_coords(&self) -> &EllipticalCoordinate;

    fn project_to_cartesian(&self, coord: &EllipticalCoordinate) -> CartesianCoordinate;

    fn project_to_elliptical(&self, coord: &CartesianCoordinate) -> EllipticalCoordinate;
}
