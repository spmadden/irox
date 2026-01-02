// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Point, Point2D};
use irox_tools::FloatIsh;

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

    pub fn intersect(&self, other: &Self) -> Option<Point<T>> {
        let m1 = self.slope();
        let m2 = other.slope();
        let b1 = self.intercept();
        let b2 = other.intercept();

        match (m1, m2) {
            (Some(m1), Some(m2)) => {
                let x = (b2 - b1) / (m2 - m1);
                let y = m1 * x + b1;
                Some(Point::new_point(x, y))
            }
            (Some(m1), None) => {
                let x = other.start.x;
                let y = m1 * x + b1;
                Some(Point::new_point(x, y))
            }
            (None, Some(m2)) => {
                let x = self.start.x;
                let y = m2 * x + b1;
                Some(Point::new_point(x, y))
            }
            (None, None) => None,
        }
    }

    pub fn is_clockwise(&self, point: &Point<T>) -> bool {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let px = point.x - self.start.x;
        let py = point.y - self.start.y;
        (dx * py - dy * px) <= T::ZERO
    }
}

#[cfg(test)]
mod tests {
    use crate::{LineSegment, Point, Point2D};

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
}
