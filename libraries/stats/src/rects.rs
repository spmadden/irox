// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::points::{Double2D, Vec2D};

#[allow(unused_imports)]
use irox_tools::f64::FloatExt;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Rect2D {
    pub origin: Double2D,
    pub far_point: Double2D,
}
impl Rect2D {
    pub const fn empty() -> Rect2D {
        Self {
            origin: Double2D::new(f64::INFINITY, f64::INFINITY),
            far_point: Double2D::new(f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }
    #[must_use]
    pub fn new(origin: Double2D, size: Vec2D) -> Self {
        Self {
            origin,
            far_point: origin.translate(&size),
        }
    }
    #[must_use]
    pub fn origin(&self) -> Double2D {
        self.origin
    }
    #[must_use]
    pub fn size(&self) -> Vec2D {
        self.far_point - self.origin
    }

    #[must_use]
    pub fn far_point(&self) -> Double2D {
        self.far_point
    }

    #[must_use]
    pub fn far_horiz(&self) -> Double2D {
        Double2D {
            x: self.far_point.x,
            y: self.origin.y,
        }
    }
    #[must_use]
    pub fn far_vert(&self) -> Double2D {
        Double2D {
            x: self.origin.x,
            y: self.far_point.y,
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

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.origin.x = self.origin.x.min(x);
        self.origin.y = self.origin.y.min(y);
        self.far_point.x = self.far_point.x.max(x);
        self.far_point.y = self.far_point.y.max(y);
    }

    #[must_use]
    pub fn height(&self) -> f64 {
        (self.far_point.y - self.origin.y).abs()
    }
    #[must_use]
    pub fn width(&self) -> f64 {
        (self.far_point.x - self.origin.x).abs()
    }
    #[must_use]
    pub fn center(&self) -> Double2D {
        let y = self.height() / 2. + self.origin.y;
        let x = self.width() / 2. + self.origin.x;
        Double2D::new(x, y)
    }
    #[must_use]
    pub fn lower_left_quadrant(&self) -> Rect2D {
        Rect2D {
            origin: self.origin,
            far_point: self.center(),
        }
    }
    #[must_use]
    pub fn upper_left_quadrant(&self) -> Rect2D {
        let ctr = self.center();
        Rect2D {
            origin: Double2D::new(self.origin.x, ctr.y),
            far_point: Double2D::new(ctr.x, self.far_point.y),
        }
    }
    #[must_use]
    pub fn upper_right_quadrant(&self) -> Rect2D {
        Rect2D {
            origin: self.center(),
            far_point: self.far_point,
        }
    }
    #[must_use]
    pub fn lower_right_quadrant(&self) -> Rect2D {
        let ctr = self.center();
        Rect2D {
            origin: Double2D::new(ctr.x, self.origin.y),
            far_point: Double2D::new(self.far_point.x, ctr.y),
        }
    }
}

#[cfg(feature = "emath")]
impl From<emath::Rect> for Rect2D {
    fn from(rect: emath::Rect) -> Self {
        Self {
            origin: rect.min.into(),
            far_point: rect.max.into(),
        }
    }
}
#[cfg(feature = "emath")]
impl From<Rect2D> for emath::Rect {
    fn from(rect: Rect2D) -> Self {
        Self {
            min: rect.origin.into(),
            max: rect.far_point.into(),
        }
    }
}
