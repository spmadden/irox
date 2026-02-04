// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;
use crate::geometry::{Centroid, Geometry};
use crate::line::LineSegment;
use crate::point::Point;
use crate::rectangle::Rectangle;
use crate::Vector2D;
use core::slice::Iter;
use alloc::vec::Vec;
use irox_tools::iterators::{LendingIterator, Windows};
use irox_tools::FloatIsh;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Polygon<T: FloatIsh> {
    pub points: Vec<Point<T>>,
}

impl<T: FloatIsh> Polygon<T> {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
    pub fn iter_points(&self) -> Iter<'_, Point<T>> {
        self.points.iter()
    }
    pub fn iter_segments(&self) -> PolygonSegmentIter<'_, T> {
        let wins = Windows::<2, Point<T>>::new(self.points.iter());
        let first = self.points.first();
        let last = self.points.last();
        let last = match (first, last) {
            (Some(first), Some(last)) => Some(LineSegment {
                start: *last,
                end: *first,
            }),
            _ => None,
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
    fn contains(&self, _point: &Point<T>) -> bool {
        todo!()
    }

    fn distance_to(&self, _point: &Point<T>) -> T {
        todo!()
    }

    fn intersects(&self, _point: &Point<T>) -> bool {
        todo!()
    }

    fn bounding_rectangle(&self) -> Rectangle<T> {
        todo!()
    }
}
