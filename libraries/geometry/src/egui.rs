// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::{Point, Rectangle, Vector};
use crate::transform::LinearTransform;

impl From<egui::Pos2> for Point<f32> {
    fn from(value: egui::Pos2) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: None,
            m: None,
        }
    }
}
impl From<Point<f32>> for egui::Pos2 {
    fn from(value: Point<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<egui::Vec2> for Vector<f32> {
    fn from(value: egui::Vec2) -> Self {
        Self {
            vx: value.x,
            vy: value.y,
        }
    }
}
impl From<Vector<f32>> for egui::Vec2 {
    fn from(value: Vector<f32>) -> Self {
        Self {
            x: value.vx,
            y: value.vy,
        }
    }
}
impl From<egui::Rect> for Rectangle<f32> {
    fn from(value: egui::Rect) -> Self {
        let size = value.max - value.min;
        Self {
            min: value.min.into(),
            size: size.into(),
        }
    }
}
impl From<Rectangle<f32>> for egui::Rect {
    fn from(value: Rectangle<f32>) -> Self {
        Self {
            min: value.min.into(),
            max: value.far_point().into(),
        }
    }
}
impl From<egui::emath::TSTransform> for LinearTransform<f32> {
    fn from(value: egui::emath::TSTransform) -> Self {
        Self {
            scale: value.scaling,
            translate: value.translation.into(),
        }
    }
}
impl From<LinearTransform<f32>> for egui::emath::TSTransform {
    fn from(value: LinearTransform<f32>) -> Self {
        Self {
            scaling: value.scale,
            translation: value.translate.into(),
        }
    }
}