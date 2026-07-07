// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::geometry::{Centroid, Geometry};
use crate::rectangle::Rectangle;
use crate::{Point, Point2D, Vector, Vector2D};
use irox_tools::FloatIsh;
use irox_units::units::angle::Angle;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct LineSegment<T: FloatIsh> {
    pub start: Point<T>,
    pub end: Point<T>,
}

impl<T: FloatIsh> LineSegment<T> {
    pub fn slope(&self) -> Option<T> {
        let dx = self.end.x - self.start.x;
        if dx == T::ZERO {
            return None;
        }
        Some((self.end.y - self.start.y) / dx)
    }

    pub fn intercept(&self) -> T {
        let Some(slope) = self.slope() else {
            return self.start.x;
        };
        self.start.y - slope * self.start.x
    }

    pub fn angle(&self) -> Angle {
        let vec = self.end - self.start;
        let ang = vec.vy.atan2(vec.vx).to_f64();
        Angle::new_radians(ang)
    }

    pub fn intersect(&self, other: &Self) -> Option<Point<T>> {
        let x1 = self.start.x;
        let x2 = self.end.x;
        let y1 = self.start.y;
        let y2 = self.end.y;
        let x3 = other.start.x;
        let x4 = other.end.x;
        let y3 = other.start.y;
        let y4 = other.end.y;

        let a = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
        let b = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let t = a / b;

        let a = (x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3);
        let b = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let u = T::ZERO - a / b;

        if t < T::ZERO || t > T::ONE || u < T::ZERO || u > T::ONE {
            None
        } else {
            Some(Point::new_point(x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
        }
    }

    pub fn is_clockwise(&self, point: &Point<T>) -> bool {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let px = point.x - self.start.x;
        let py = point.y - self.start.y;
        (dx * py - dy * px) <= T::ZERO
    }

    pub fn length(&self) -> T {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn point_along_length(&self, pct: T) -> Point<T> {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let proj = Vector::new(dx * pct, dy * pct);
        self.start + proj
    }

    pub fn distance_to(&self, point: &Point<T>) -> T {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let px = point.x - self.start.x;
        let py = point.y - self.start.y;
        let len = self.length();
        if len == T::ZERO {
            return T::ZERO;
        }
        let pct = ((px * dx) + (py * dy)) / (len * len);
        let pct = pct.clamp(T::ZERO, T::ONE);
        let point_on_segment = self.point_along_length(pct);
        let v = *point - point_on_segment;
        v.magnitude()
    }
}

impl<T: FloatIsh> Centroid<T> for LineSegment<T> {
    fn centroid(&self) -> Point<T> {
        self.point_along_length(T::from_f64(0.5))
    }
}

impl<T: FloatIsh> Geometry<T> for LineSegment<T> {
    fn contains(&self, _point: &Point<T>) -> bool {
        todo!()
    }

    fn distance_to(&self, point: &Point<T>) -> T {
        LineSegment::distance_to(self, point)
    }

    fn intersects(&self, _point: &Point<T>) -> bool {
        todo!()
    }

    fn bounding_rectangle(&self) -> Rectangle<T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{LineSegment, Point, Point2D};
    use irox_tools::assert_eq_eps;

    #[test]
    pub fn test1() {
        let line = LineSegment {
            start: Point::new_point(5., 5.),
            end: Point::new_point(5., 20.),
        };
        assert_eq!(None, line.slope());
        assert!(line.is_clockwise(&Point::new_point(5., 10.)));
    }
    #[test]
    pub fn test2() {
        let line = LineSegment {
            start: Point::new_point(1., 1.),
            end: Point::new_point(10., 10.),
        };
        assert_eq!(Some(1.), line.slope());
        assert!(!line.is_clockwise(&Point::new_point(5., 10.)));
        assert!(line.is_clockwise(&Point::new_point(5., 5.)));
        assert!(line.is_clockwise(&Point::new_point(10., 5.)));
    }

    #[test]
    pub fn test_distance() {
        let line = LineSegment {
            start: Point::new_point(50., 80.),
            end: Point::new_point(50., -800.),
        };
        let d = line.distance_to(&Point::new_point(20., 1000.));
        assert_eq_eps!(920.48900047746360889268014185024f64, d, 1e-7);

        let line = LineSegment {
            start: Point::new_point(0., 0.),
            end: Point::new_point(10., 0.),
        };
        let pnt = line.point_along_length(0.5);
        assert_eq!(pnt, Point::new_point(5.0, 0.0));
        let d = line.distance_to(&Point::new_point(0., 10.));
        assert_eq_eps!(10f64, d, 1e-13);
        let d = line.distance_to(&Point::new_point(5., 10.));
        assert_eq_eps!(10f64, d, 1e-13);
        let d = line.distance_to(&Point::new_point(10., 10.));
        assert_eq_eps!(10f64, d, 1e-13);
        let d = line.distance_to(&Point::new_point(10., -10.));
        assert_eq_eps!(10f64, d, 1e-13);
        let d = line.distance_to(&Point::new_point(5., -10.));
        assert_eq_eps!(10f64, d, 1e-13);
        let d = line.distance_to(&Point::new_point(0., -10.));
        assert_eq_eps!(10f64, d, 1e-13);
        let d = line.distance_to(&Point::new_point(-10., 0.));
        assert_eq_eps!(10f64, d, 1e-13);
        let d = line.distance_to(&Point::new_point(20., 0.));
        assert_eq_eps!(10f64, d, 1e-13);
    }

    #[test]
    pub fn test_intersection1() {
        let line1 = LineSegment {
            start: Point::new_point(2.0, 1.0),
            end: Point::new_point(2.0, 3.0),
        };
        let line2 = LineSegment {
            start: Point::new_point(1.0, 2.0),
            end: Point::new_point(3.0, 2.0),
        };
        let intersection = line1.intersect(&line2);
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();
        assert_eq!(intersection, Point::new_point(2.0, 2.0));
    }
    #[test]
    pub fn test_intersection2() {
        let line1 = LineSegment {
            start: Point::new_point(2.0, 1.0),
            end: Point::new_point(2.0, 3.0),
        };
        let line2 = LineSegment {
            end: Point::new_point(1.0, 2.0),
            start: Point::new_point(3.0, 2.0),
        };
        let intersection = line1.intersect(&line2);
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();
        assert_eq!(intersection, Point::new_point(2.0, 2.0));
    }
}
