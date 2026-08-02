// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use egui::epaint::{PathShape, Vertex};
use egui::{Color32, Id, Mesh, Painter, Pos2, Sense, Shape, Stroke, Ui, Vec2};
use irox_geometry::{Geometry, Point, Polygon, Vector, Vector2D};
use irox_units::units::angle::Angle;
use std::sync::Arc;

pub struct Response {
    pub hovered: bool,
    pub clicked: bool,
}
impl Response {
    pub fn hovered(&self) -> bool {
        self.hovered
    }
    pub fn clicked(&self) -> bool {
        self.clicked
    }
}
pub struct ArcWedgeSet {
    pub identifier: Id,
    pub size: f32,
    pub wedges: Vec<ArcWedge>,
}
impl ArcWedgeSet {
    pub fn show(&self, ui: &mut Ui) {
        let (id, rect) = ui.allocate_space(Vec2::splat(self.size));
        let response = ui.interact(rect, id, Sense::click());
        let painter = ui.painter_at(rect);
        for wedge in &self.wedges {
            wedge.show(ui, &painter, &response);
        }
    }
}
pub struct ArcWedge {
    pub identifier: Id,
    pub start_angle: Angle,
    pub end_angle: Angle,
    pub pad_angle: Angle,
    pub inner_length: f32,
    pub outer_length: f32,
    pub pad_length: f32,
    pub stroke_color: Color32,
    pub fill_color: Color32,
    pub hovered_fill_color: Color32,
}
impl ArcWedge {
    pub fn show(&self, ui: &mut Ui, painter: &Painter, response: &egui::Response) -> Response {
        let painter_space = response.rect;
        let hovered: bool = ui.memory(|mem| mem.data.get_temp(self.identifier).unwrap_or_default());
        let ctr = painter_space.center();

        let mut polygon_intersection = Polygon::<f32>::empty();
        let mut mesh = Mesh::default();
        let fill_color = if hovered {
            self.hovered_fill_color
        } else {
            self.fill_color
        };
        let inner_length = self.inner_length + 0.5 * self.pad_length;
        let outer_length = self.outer_length - 0.5 * self.pad_length;
        let start_angle = self.start_angle + Angle::new_degrees(180.) + self.pad_angle / 2.;
        let end_angle = self.end_angle + Angle::new_degrees(180.) - self.pad_angle / 2.;
        {
            // start line
            let pos = ctr + Vector::new(0.0, inner_length).rotate(start_angle).into();
            mesh.vertices.push(Vertex::untextured(pos, fill_color));
            polygon_intersection.add_point(pos.into());
            let pos = ctr + Vector::new(0.0, outer_length).rotate(start_angle).into();
            mesh.vertices.push(Vertex::untextured(pos, fill_color));
            polygon_intersection.add_point(pos.into());
        }
        let mut remaining_points = Vec::new();
        let mut idx = 0;
        {
            let mut angle = start_angle;
            while angle <= end_angle {
                let pos = ctr + Vector::new(0.0, inner_length).rotate(angle).into();
                mesh.vertices.push(Vertex::untextured(pos, fill_color));
                remaining_points.push(pos);
                let pos = ctr + Vector::new(0.0, outer_length).rotate(angle).into();
                mesh.vertices.push(Vertex::untextured(pos, fill_color));
                polygon_intersection.add_point(pos.into());
                angle += Angle::new_degrees(0.5);

                mesh.add_triangle(idx, idx + 1, idx + 3);
                mesh.add_triangle(idx + 3, idx + 2, idx);
                idx += 2;
            }
        }
        while let Some(pos) = remaining_points.pop() {
            polygon_intersection.add_point(pos.into());
        }

        let mut points = Vec::<Pos2>::new();
        for pnt in polygon_intersection.iter_points() {
            points.push((*pnt).into());
        }
        let shp = Shape::Path(PathShape::closed_line(
            points,
            Stroke::new(4.0, self.stroke_color),
        ));
        painter.add(shp);

        let shp = Shape::Mesh(Arc::new(mesh));
        painter.add(shp);
        let hovered = if let Some(hover_pos) = response.hover_pos() {
            if response.hovered() {
                let hover_pos: Point<f32> = hover_pos.into();
                polygon_intersection.contains(&hover_pos)
            } else {
                false
            }
        } else {
            false
        };
        let clicked = if hovered { response.clicked() } else { false };
        ui.memory_mut(|mem| {
            mem.data.insert_temp(self.identifier, hovered);
        });

        Response { hovered, clicked }
    }
}
