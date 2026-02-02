// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::rectangle::Rectangle;
use crate::{LineSegment, Point, Polygon};
use irox_tools::FloatIsh;

#[derive(Debug, Clone, PartialEq)]
pub enum GeometryType<T: FloatIsh> {
    Point(Point<T>),
    Line(LineSegment<T>),
    Polygon(Polygon<T>),
    Rectangle(Rectangle<T>),
}
impl<T: FloatIsh> Centroid<T> for GeometryType<T> {
    fn centroid(&self) -> Point<T> {
        todo!()
    }
}
impl<T: FloatIsh> Geometry<T> for GeometryType<T> {
    fn contains(&self, point: &Point<T>) -> bool {
        match self {
            GeometryType::Point(p) => p.contains(point),
            GeometryType::Line(l) => l.contains(point),
            GeometryType::Polygon(p) => p.contains(point),
            GeometryType::Rectangle(r) => r.contains(point),
        }
    }

    fn distance_to(&self, point: &Point<T>) -> T {
        match self {
            GeometryType::Point(p) => p.distance_to(point),
            GeometryType::Line(l) => l.distance_to(point),
            GeometryType::Polygon(p) => p.distance_to(point),
            GeometryType::Rectangle(r) => r.distance_to(point),
        }
    }

    fn intersects(&self, point: &Point<T>) -> bool {
        match self {
            GeometryType::Point(p) => p.intersects(point),
            GeometryType::Line(l) => l.intersects(point),
            GeometryType::Polygon(p) => p.intersects(point),
            GeometryType::Rectangle(r) => r.intersects(point),
        }
    }

    fn bounding_rectangle(&self) -> Rectangle<T> {
        match self {
            GeometryType::Point(p) => p.bounding_rectangle(),
            GeometryType::Line(l) => l.bounding_rectangle(),
            GeometryType::Polygon(p) => p.bounding_rectangle(),
            GeometryType::Rectangle(r) => r.bounding_rectangle(),
        }
    }
}

pub trait Geometry<T: FloatIsh>: Centroid<T> {
    fn contains(&self, point: &Point<T>) -> bool;
    fn distance_to(&self, point: &Point<T>) -> T;
    fn intersects(&self, point: &Point<T>) -> bool;
    fn bounding_rectangle(&self) -> Rectangle<T>;
}

pub trait Centroid<T: FloatIsh> {
    fn centroid(&self) -> Point<T>;
}
