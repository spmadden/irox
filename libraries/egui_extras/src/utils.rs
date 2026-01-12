// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use eframe::emath::Pos2;
use eframe::epaint::{Color32, QuadraticBezierShape, Shape, Stroke};
use std::f32::consts::FRAC_PI_2;

pub fn line_to_bezier(pts: [Pos2; 2]) -> Shape {
    let [a, b] = pts;
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let d = 1. - (20. - (dx * dx + dy * dy).sqrt()) / 20.;
    // let d = d*20.;
    let s = (dy / dx).atan();
    let pm = Pos2::new(dx / 2., dy / 2.) + a.to_vec2();
    let ang = s - FRAC_PI_2;
    let x2 = pm.x + d * ang.cos();
    let y2 = pm.y + d * ang.sin();
    let c = Pos2::new(x2, y2);
    Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke(
        [a, c, b],
        false,
        Color32::TRANSPARENT,
        Stroke::new(1.0, Color32::BLACK),
    ))
}
