// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use eframe::emath::Pos2;
use eframe::epaint::PathShape;
use egui::{Color32, Shape, Stroke};
use irox_geometry::{Vector, Vector2D};
use irox_units::units::angle::Angle;

pub const DEFAULT_HEAD_ANGLE: Angle = Angle::new_degrees(30.);
pub const DEFAULT_FILL: Color32 = Color32::TRANSPARENT;
pub struct Arrow {
    pub fill_color: Option<Color32>,
    pub point_angle: Angle,
    pub head_angle: Option<Angle>,
    pub head_length: f32,
    pub stroke: Stroke,
}
impl Arrow {
    pub fn to_shape(self) -> Shape {
        let head_angle = self.head_angle.unwrap_or(DEFAULT_HEAD_ANGLE);
        let fill_color = self.fill_color.unwrap_or(DEFAULT_FILL);
        let left = Vector::new(-1.0, 0.0)
            .rotate_clockwise(head_angle)
            .rotate(self.point_angle)
            * self.head_length;
        let right = Vector::new(-1.0, 0.0)
            .rotate(head_angle)
            .rotate(self.point_angle)
            * self.head_length;
        let tri = vec![
            Pos2::new(0.0, 0.0),
            Pos2::new(left.vx, left.vy),
            Pos2::new(right.vx, right.vy),
            Pos2::new(0.0, 0.0),
        ];
        Shape::Path(PathShape::convex_polygon(tri, fill_color, self.stroke))
    }
}

pub fn create_arrow() {}
