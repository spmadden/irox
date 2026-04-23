// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Edge, Node};
use egui::emath::TSTransform;
use egui::epaint::PathShape;
use egui::Rect;
use irox_egui_extras::egui;
use irox_egui_extras::egui::epaint::{CircleShape, RectShape, TextShape};
use irox_egui_extras::egui::text::LayoutJob;
use irox_egui_extras::egui::{Align, Color32, CornerRadius, FontId, Pos2, Shape, Stroke, Ui};
use irox_egui_extras::WithAlpha;
use irox_geometry::{LineSegment, Vector, Vector2D};
use irox_units::units::angle::Angle;

pub struct RenderingContext<'a> {
    pub current_transform: TSTransform,
    pub current_world_area: Rect,
    pub last_window_area: Option<Rect>,
    pub ui: &'a Ui,
}

pub type NodeRendererProvider = Box<dyn FnMut(&Node) -> &dyn NodeRenderer>;
pub trait NodeRenderer {
    fn get_shapes(
        &self,
        context: &RenderingContext,
        node: &Node,
        center: Vector<f64>,
    ) -> Vec<Shape> {
        let mut out = Vec::new();
        self.add_shapes_to(context, node, center, &mut out);
        out
    }
    fn add_shapes_to(
        &self,
        context: &RenderingContext,
        node: &Node,
        center: Vector<f64>,
        shapes: &mut Vec<Shape>,
    );
}
pub static DEFAULT_NODE_RENDERER: DefaultNodeRenderer = DefaultNodeRenderer { draw_id: true };
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DefaultNodeRenderer {
    pub draw_id: bool,
}
impl Default for DefaultNodeRenderer {
    fn default() -> Self {
        Self { draw_id: true }
    }
}
impl NodeRenderer for DefaultNodeRenderer {
    fn add_shapes_to(
        &self,
        context: &RenderingContext,
        node: &Node,
        center: Vector<f64>,
        shapes: &mut Vec<Shape>,
    ) {
        let id = node.descriptor.id.to_string();
        let p = Pos2::new(center.vx as f32, center.vy as f32);
        let ui = context.ui;
        let fgc = ui.visuals().widgets.active.fg_stroke.color;
        let bgc = ui.visuals().widgets.active.bg_fill.with_alpha(160);

        shapes.push(Shape::Circle(CircleShape::filled(p, 2., fgc)));
        if self.draw_id {
            TextBoxRenderer::add_shapes_to(&id, context, fgc, bgc, p, shapes);
        }
    }
}

pub type EdgeRendererProvider = Box<dyn FnMut(&Edge) -> &dyn EdgeRenderer>;
pub trait EdgeRenderer {
    fn get_shapes(
        &self,
        context: &RenderingContext,
        edge: &Edge,
        left: Vector<f64>,
        right: Vector<f64>,
    ) -> Vec<Shape> {
        let mut out = Vec::new();
        self.add_shapes_to(context, edge, left, right, &mut out);
        out
    }
    fn add_shapes_to(
        &self,
        context: &RenderingContext,
        edge: &Edge,
        left: Vector<f64>,
        right: Vector<f64>,
        shapes: &mut Vec<Shape>,
    );
}

pub static DEFAULT_EDGE_RENDERER: DefaultEdgeRenderer = DefaultEdgeRenderer;
pub struct DefaultEdgeRenderer;
impl EdgeRenderer for DefaultEdgeRenderer {
    fn add_shapes_to(
        &self,
        _context: &RenderingContext,
        edge: &Edge,
        left: Vector<f64>,
        right: Vector<f64>,
        shapes: &mut Vec<Shape>,
    ) {
        let pts = [
            Pos2::new(left.vx as f32, left.vy as f32),
            Pos2::new(right.vx as f32, right.vy as f32),
        ];
        // shapes.push(line_to_bezier(pts));
        shapes.push(Shape::LineSegment {
            points: pts,
            stroke: Stroke::new(1.0, Color32::BLACK),
        });

        if edge.is_directed() {
            let bgc = _context.ui.visuals().widgets.active.bg_fill.with_alpha(160);

            let angle = LineSegment {
                start: left.to_point(),
                end: right.to_point(),
            }
            .angle();
            let stroke = Stroke::new(1.0, Color32::BLACK);
            let left = Vector::new(-1.0, 0.0)
                .rotate_clockwise(Angle::new_degrees(30.))
                .rotate(angle)
                * 10.
                / _context.current_transform.scaling;
            let right = Vector::new(-1.0, 0.0)
                .rotate(Angle::new_degrees(30.))
                .rotate(angle)
                * 10.
                / _context.current_transform.scaling;
            let pos = pts[1].to_vec2();
            let tri = vec![
                Pos2::new(0.0, 0.0) + pos,
                Pos2::new(left.vx, left.vy) + pos,
                Pos2::new(right.vx, right.vy) + pos,
                Pos2::new(0.0, 0.0) + pos,
            ];
            let shp = Shape::Path(PathShape::convex_polygon(tri, bgc, stroke));
            shapes.push(shp);
        }
    }
}

pub struct TextBoxRenderer;
impl TextBoxRenderer {
    pub fn add_shapes_to(
        text: &str,
        context: &RenderingContext,
        fgc: Color32,
        bgc: Color32,
        center: Pos2,
        shapes: &mut Vec<Shape>,
    ) {
        let galley = context.ui.ctx().fonts_mut(|f| {
            let mut job =
                LayoutJob::simple(text.to_string(), FontId::monospace(14.), fgc, f32::INFINITY);
            job.halign = Align::Center;
            f.layout_job(job)
        });
        let ctr = galley.rect.size() / 2.;
        let mut adj = galley.rect.left_top();
        adj.x += ctr.x / context.current_transform.scaling;

        let rect = galley
            .rect
            .translate(-adj.to_vec2())
            .translate(center.to_vec2());
        let txt = Shape::Text(TextShape::new(center, galley, fgc));
        let rect = RectShape::filled(rect, CornerRadius::default(), bgc);
        shapes.push(Shape::Rect(rect));
        shapes.push(txt);
    }
}
