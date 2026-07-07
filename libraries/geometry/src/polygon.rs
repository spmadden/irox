// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;
use crate::geometry::{Centroid, Geometry};
use crate::line::LineSegment;
use crate::point::Point;
use crate::rectangle::Rectangle;
use crate::{Point2D, Vector2D};
use alloc::vec::Vec;
use core::slice::Iter;
use irox_tools::iterators::{LendingIterator, Windows};
use irox_tools::FloatIsh;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Polygon<T: FloatIsh> {
    points: Vec<Point<T>>,
    bounding_box: Rectangle<T>,
}

impl<T: FloatIsh> Polygon<T> {
    pub fn empty() -> Self {
        Self {
            points: Vec::new(),
            bounding_box: Rectangle::EMPTY,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            points: Vec::with_capacity(capacity),
            bounding_box: Rectangle::EMPTY,
        }
    }
    pub fn add_point(&mut self, point: Point<T>) {
        if let Some(last) = self.points.last() {
            if *last == point {
                return;
            }
        }
        self.points.push(point);
        self.bounding_box.add_point(point);
    }
    pub fn iter_points(&self) -> Iter<'_, Point<T>> {
        self.points.iter()
    }
    pub fn iter_segments(&self) -> PolygonSegmentIter<'_, T> {
        let wins = Windows::<2, Point<T>>::new(self.points.iter());
        let first = self.points.first();
        let mut last_iter = self.points.iter().rev();
        let last = if let Some(first) = first {
            loop {
                let Some(last) = last_iter.next() else {
                    break None;
                };
                if *last == *first {
                    continue;
                }
                break Some(LineSegment {
                    start: *last,
                    end: *first,
                });
            }
        } else {
            None
        };
        PolygonSegmentIter { iter: wins, last }
    }

    pub fn is_clockwise(&self) -> bool {
        false
    }
    #[must_use]
    pub fn clip(&self, _other: &Self) -> Self {
        todo!()
    }
}

pub struct PolygonSegmentIter<'a, T: FloatIsh> {
    iter: Windows<'a, 2, Point<T>>,
    last: Option<LineSegment<T>>,
}
impl<T: FloatIsh> Iterator for PolygonSegmentIter<'_, T> {
    type Item = LineSegment<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some([a, b]) = self.iter.next_ref() {
            if a == b {
                return self.next();
            }
            Some(LineSegment { start: *a, end: *b })
        } else {
            self.last.take()
        }
    }
}
impl<T: FloatIsh> Centroid<T> for Polygon<T> {
    fn centroid(&self) -> Point<T> {
        let mut out = Point::default().to_vector();
        for p in &self.points {
            out += p.to_vector();
        }
        out /= T::from_f64(self.points.len() as f64);

        out.to_point()
    }
}

impl<T: FloatIsh> Geometry<T> for Polygon<T> {
    ///
    /// Basic implementation of the ray casting algorithm.
    fn contains(&self, point: &Point<T>) -> bool {
        // check square bbox first for speed
        if !self.bounding_box.contains(point) {
            return false;
        }
        // generate a ray from the test point, out twice as wide as the bounding box
        let y = point.y;
        let farx = self.bounding_box.far_x().x * T::from_f64(2.0);
        let ray = LineSegment {
            start: *point,
            end: Point::new_point(farx, y),
        };

        // now count the ray crossings.
        let mut crossings = 0;
        for line in self.iter_segments() {
            if line.start == *point || line.end == *point {
                return true;
            }
            let intersect = ray.intersect(&line);
            if intersect.is_some() {
                crossings += 1;
            }
        }
        crossings % 2 == 1
    }

    fn distance_to(&self, _point: &Point<T>) -> T {
        todo!()
    }

    fn intersects(&self, _point: &Point<T>) -> bool {
        todo!()
    }

    fn bounding_rectangle(&self) -> Rectangle<T> {
        self.bounding_box
    }
}

#[cfg(test)]
mod tests {
    use crate::{Geometry, Point, Point2D, Polygon, Rectangle, Vector};

    #[test]
    pub fn contains_test1() {
        let poly = Rectangle {
            min: Point::new_point(-1.0, -1.0),
            size: Vector::new(2.0, 2.0),
        }
        .to_polygon();
        assert!(poly.contains(&Point::new_point(0.0, 0.0)));
    }
    #[test]
    pub fn contains_test2() {
        let poly = Rectangle {
            min: Point::new_point(-1.0, -1.0),
            size: Vector::new(2.0, 2.0),
        }
        .to_polygon();
        assert!(!poly.contains(&Point::new_point(2.0, 2.0)));
    }
    #[test]
    pub fn contains_test3() {
        let poly = Rectangle {
            min: Point::new_point(-1.0, -1.0),
            size: Vector::new(2.0, 2.0),
        }
        .to_polygon();
        assert!(!poly.contains(&Point::new_point(-2.0, -2.0)));
    }
    #[test]
    pub fn contains_test4() {
        let poly = Rectangle {
            min: Point::new_point(-1.0, -1.0),
            size: Vector::new(2.0, 2.0),
        }
        .to_polygon();
        assert!(poly.contains(&Point::new_point(-1.0, -1.0)));
    }
    #[test]
    pub fn contains_test5() {
        let poly = Rectangle {
            min: Point::new_point(-1.0, -1.0),
            size: Vector::new(2.0, 2.0),
        }
        .to_polygon();
        assert!(poly.contains(&Point::new_point(1.0, 1.0)));
    }
    #[test]
    pub fn contains_test6() {
        let mut poly = Polygon::empty();
        poly.add_point(Point::new_point(-1.0, -1.0));
        poly.add_point(Point::new_point(-1.0, 1.0));
        poly.add_point(Point::new_point(0.0, 0.0));
        poly.add_point(Point::new_point(1.0, 1.0));
        poly.add_point(Point::new_point(1.0, -1.0));
        assert!(poly.contains(&Point::new_point(0.5, 0.49)));
        assert!(poly.contains(&Point::new_point(-0.5, 0.5)));
        assert!(poly.contains(&Point::new_point(0.0, -0.5)));
    }
}
