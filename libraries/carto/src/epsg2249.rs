// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate, Latitude, Longitude};
use crate::geo::standards::StandardShapes;
use crate::geo::EllipticalShape;
use crate::lcc::{LambertConformalConic, LambertConformalConicBuilder};
use crate::proj::Projection;
use irox_units::units::angle::Angle;
use irox_units::units::length::{Length, LengthUnits};
use irox_units::units::Unit;

const LAT1: Latitude = Latitude(Angle::new_degrees(42.6833333333336));
const LAT2: Latitude = Latitude(Angle::new_degrees(41.7166666666669));
const FE: Length = Length::new_meters(200_000.);
const FN: Length = Length::new_meters(750_000.);
const LAT0: Latitude = Latitude(Angle::new_degrees(41.));
const LON0: Longitude = Longitude(Angle::new_degrees(-71.5));
const SHP: StandardShapes = StandardShapes::GRS80;
const CTR: EllipticalCoordinate =
    EllipticalCoordinate::new(LAT0, LON0, EllipticalShape::Ellipse(SHP.as_ellipse()));
pub struct Epsg2249 {
    proj: LambertConformalConic,
}
impl Default for Epsg2249 {
    fn default() -> Self {
        Self::new()
    }
}
impl Epsg2249 {
    pub fn new() -> Self {
        let b = LambertConformalConicBuilder::default()
            .with_first_parallel(LAT1)
            .with_second_parallel(LAT2)
            .with_center(CTR)
            .with_shape(SHP.as_ellipsoid())
            .with_false_easting(FE)
            .with_false_northing(FN);

        Self { proj: b.build() }
    }
}
impl Projection for Epsg2249 {
    fn get_center_coords(&self) -> &EllipticalCoordinate {
        &CTR
    }

    fn project_to_cartesian(&self, coord: &EllipticalCoordinate) -> CartesianCoordinate {
        let v = self.proj.project_to_cartesian(coord);
        CartesianCoordinate::new(
            v.get_x().as_unit(LengthUnits::USSurveyFoot),
            v.get_y().as_unit(LengthUnits::USSurveyFoot),
            v.get_z().as_unit(LengthUnits::USSurveyFoot),
        )
    }

    fn project_to_elliptical(&self, coord: &CartesianCoordinate) -> EllipticalCoordinate {
        self.proj.project_to_elliptical(coord)
    }
}

#[cfg(test)]
mod tests {
    use crate::coordinate::{CartesianCoordinate, EllipticalCoordinate, Latitude, Longitude};
    use crate::epsg2249::Epsg2249;
    use crate::geo::standards::StandardShapes;
    use crate::proj::Projection;
    use crate::spcs::MASS_MAINLAND;
    use irox_tools::irox_bits::Error;
    use irox_units::units::angle::{Angle, AngleUnits};
    use std::fs::File;
    use std::io::BufReader;
    use std::str::FromStr;

    macro_rules! get {
        ($map:ident,$key:literal) => {
            $map.get($key)
                .map(|v| f64::from_str(v).unwrap_or_default())
                .unwrap_or_default()
        };
    }
    #[test]
    pub fn test() -> Result<(), Error> {
        let file = File::open("data\\macontrol.csv")?;
        let file = BufReader::new(file);
        let mut reader = irox_csv::CSVMapReader::new(file).unwrap();
        let proj = Epsg2249::new();
        let proj = MASS_MAINLAND;
        let mut xdiffs = irox_stats::streaming::Summary::<f64>::default();
        let mut ydiffs = irox_stats::streaming::Summary::<f64>::default();
        let mut latdiffs = irox_stats::streaming::Summary::<f64>::default();
        let mut londiffs = irox_stats::streaming::Summary::<f64>::default();

        while let Ok(Some(row)) = reader.next_row() {
            let map = row.into_map_lossy();
            let x = get!(map, "SHP_X_COORD");
            let y = get!(map, "SHP_Y_COORD");
            let c = CartesianCoordinate::new_meters(x, y, 0.);
            let lat = get!(map, "Y");
            let lon = get!(map, "X");
            let e = EllipticalCoordinate::new(
                Latitude(Angle::new_degrees(lat)),
                Longitude(Angle::new_degrees(lon)),
                StandardShapes::NAD83.into(),
            );
            println!("{c} // {e} // {map:?}");

            let ce = proj.project_to_cartesian(&e);
            let xdiff = c.get_x() - ce.get_x();
            let ydiff = c.get_y() - ce.get_y();
            println!("xdiff: {}", xdiff);
            println!("ydiff: {}", ydiff);
            // assert_eq!(c, ce);
            let ec = proj.project_to_elliptical(&c).as_unit(AngleUnits::Degrees);
            // assert_eq!(e, ec);
            let latdiff = e.get_latitude().0 - ec.get_latitude().0;
            let londiff = e.get_longitude().0 - ec.get_longitude().0;
            println!("latdiff: {}", latdiff);
            println!("londiff: {}", londiff);

            xdiffs.add_sample(xdiff.value());
            ydiffs.add_sample(ydiff.value());
            latdiffs.add_sample(latdiff.value());
            londiffs.add_sample(londiff.value());
        }
        println!("xstats: {xdiffs}");
        println!("ystats: {ydiffs}");
        println!("latstats: {latdiffs}");
        println!("lonstats: {londiffs}");
        Ok(())
    }
}
