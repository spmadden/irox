// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::points::{Double2D, Vec2D};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Rect2D {
    pub origin: Double2D,
    pub size: Vec2D,
}
impl Rect2D {
    #[must_use]
    pub fn new(origin: Double2D, size: Vec2D) -> Self {
        Self { origin, size }
    }
    #[must_use]
    pub fn origin(&self) -> Double2D {
        self.origin
    }
    #[must_use]
    pub fn size(&self) -> Vec2D {
        self.size
    }

    #[must_use]
    pub fn far_point(&self) -> Double2D {
        self.origin.translate(&self.size)
    }

    #[must_use]
    pub fn far_horiz(&self) -> Double2D {
        Double2D {
            x: self.origin.x + self.size.x,
            y: self.origin.y,
        }
    }
    #[must_use]
    pub fn far_vert(&self) -> Double2D {
        Double2D {
            x: self.origin.x,
            y: self.origin.y + self.size.y,
        }
    }

    #[must_use]
    pub fn corners(&self) -> [Double2D; 4] {
        [
            self.origin,
            self.far_horiz(),
            self.far_point(),
            self.far_vert(),
        ]
    }

    #[must_use]
    pub fn contains(&self, point: Double2D) -> bool {
        let Double2D { x, y } = point;
        let Double2D { x: fx, y: fy } = self.far_point();
        x >= self.origin.x && x <= fx && y >= self.origin.y && y <= fy
    }
    #[must_use]
    pub fn intersects(&self, other: &Rect2D) -> bool {
        for mypt in self.corners() {
            if other.contains(mypt) {
                return true;
            }
        }
        for pt in other.corners() {
            if self.contains(pt) {
                return true;
            }
        }
        false
    }
}

#[cfg(feature = "emath")]
impl From<emath::Rect> for Rect2D {
    fn from(rect: emath::Rect) -> Self {
        Self {
            origin: rect.min.into(),
            size: rect.size().into(),
        }
    }
}
#[cfg(feature = "emath")]
impl From<Rect2D> for emath::Rect {
    fn from(rect: Rect2D) -> Self {
        Self {
            min: rect.origin.into(),
            max: rect.far_point().into(),
        }
    }
}
