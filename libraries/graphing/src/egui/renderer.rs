// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use crate::fdp::SimulationWorkingNode;
use crate::{Edge, Node};
use core::fmt::Write;
use core::ops::Deref;
use egui::emath::TSTransform;
use egui::{Rect, Response, Vec2};
use irox_egui_extras::arrows::Arrow;
use irox_egui_extras::drawpanel::Layer;
use irox_egui_extras::egui;
use irox_egui_extras::egui::epaint::{CircleShape, RectShape, TextShape};
use irox_egui_extras::egui::text::LayoutJob;
use irox_egui_extras::egui::{Align, Color32, CornerRadius, FontId, Pos2, Shape, Stroke, Ui};
use irox_egui_extras::WithAlpha;
use irox_geometry::transform::ModelPoint;
use irox_geometry::{LineSegment, Vector, Vector2D};

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
        sim_node: &SimulationWorkingNode,
        center: ModelPoint,
    ) -> Vec<Shape> {
        let mut out = Vec::new();
        self.add_shapes_to(context, node, sim_node, center, &mut out);
        out
    }
    fn add_shapes_to(
        &self,
        context: &RenderingContext,
        node: &Node,
        sim_node: &SimulationWorkingNode,
        center: ModelPoint,
        shapes: &mut Vec<Shape>,
    );
    fn on_response(
        &self,
        _context: &RenderingContext,
        _node: &Node,
        _response: &Response,
        _shapes: &mut Vec<Shape>,
    ) {
    }
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
        _sim_node: &SimulationWorkingNode,
        center: ModelPoint,
        shapes: &mut Vec<Shape>,
    ) {
        let id = node.descriptor.id.to_string();
        let p : Pos2 = (*center.deref()).into();
        let ui = context.ui;
        let fgc = ui.visuals().widgets.active.fg_stroke.color;
        let bgc = ui.visuals().widgets.active.bg_fill.with_alpha(160);

        let mut circle = Shape::Circle(CircleShape::filled(p, 2., fgc));
        circle.transform(context.current_transform);
        shapes.push(circle);
        if self.draw_id {
            let mut shps = Vec::new();
            TextBoxRenderer::add_shapes_to(&id, context, fgc, bgc, p, &mut shps);
            for mut shp in shps.drain(..) {
                // Layer::scale_position(&mut shp, context.current_transform);
                shp.transform(context.current_transform);
                shapes.push(shp);
            }
        }
    }

    fn on_response(
        &self,
        context: &RenderingContext,
        node: &Node,
        response: &Response,
        shapes: &mut Vec<Shape>,
    ) {
        if response.hovered() {
            let Some(p) = response.hover_pos() else {
                return;
            };
            // let p = context.current_transform.inverse().mul_pos(p);
            let mut out = String::new();
            let d = &node.descriptor.id.to_string();
            let _ = writeln!(&mut out, "id: {d}");

            if let Some(desc) = &node.descriptor.description {
                let _ = writeln!(&mut out, "desc: {desc}");
            }
            for (k, v) in &node.descriptor.attrs {
                let _ = writeln!(&mut out, "{k}: {v}");
            }
            let ui = context.ui;
            let fgc = ui.visuals().widgets.active.fg_stroke.color;
            let bgc = ui.visuals().widgets.active.bg_fill.with_alpha(160);

            let galley = ui.ctx().fonts_mut(|f| {
                let mut job =
                    LayoutJob::simple(out.clone(), FontId::monospace(14.), fgc, f32::INFINITY);
                job.halign = Align::LEFT;
                f.layout_job(job)
            });
            let pos = context.current_transform.inverse() * (p + Vec2::new(20., 5.));
            let rect = galley.rect.translate(pos.to_vec2());
            let mut txt = Shape::Text(TextShape::new(pos, galley, fgc));
            Layer::scale_position(&mut txt, context.current_transform);
            let mut rect = Shape::Rect(RectShape::filled(rect, CornerRadius::default(), bgc));
            Layer::scale_position(&mut rect, context.current_transform);
            shapes.push(txt);
            shapes.push(rect);
        }
    }
}

pub struct CompositeNodeRenderer {
    pub renderers: Vec<Box<dyn NodeRenderer + Send + Sync>>,
}
impl NodeRenderer for CompositeNodeRenderer {
    fn add_shapes_to(
        &self,
        context: &RenderingContext,
        node: &Node,
        sim_node: &SimulationWorkingNode,
        center: ModelPoint,
        shapes: &mut Vec<Shape>,
    ) {
        for renderer in &self.renderers {
            renderer.add_shapes_to(context, node, sim_node, center, shapes);
        }
    }
}
pub struct DebugForceNodeRenderer;
impl NodeRenderer for DebugForceNodeRenderer {
    fn add_shapes_to(
        &self,
        _context: &RenderingContext,
        _node: &Node,
        sim_node: &SimulationWorkingNode,
        start: ModelPoint,
        shapes: &mut Vec<Shape>,
    ) {
        let cvel = sim_node.current_velocity * 100.;
        let end = *start.deref() + cvel;
        let stroke = Stroke::new(1.5, Color32::RED);
        shapes.push(Shape::line_segment(
            [
                Pos2::new(start.x as f32, start.y as f32),
                Pos2::new(end.x as f32, end.y as f32),
            ],
            stroke,
        ));
        let line = LineSegment { start: *start.deref(), end };
        let angle = line.angle();
        let mut shp = Arrow {
            fill_color: None,
            point_angle: angle,
            head_angle: None,
            head_length: 10.0,
            stroke,
        }
        .to_shape();
        let end = Vec2::new(end.x as f32, end.y as f32);
        shp.transform(TSTransform::new(end, 1.));
        shapes.push(shp);
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
        context: &RenderingContext,
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
        let stroke = context.ui.visuals().widgets.active.fg_stroke;
        shapes.push(Shape::LineSegment {
            points: pts,
            stroke,
        });

        if edge.is_directed() {
            let bgc = context.ui.visuals().widgets.active.bg_fill.with_alpha(160);

            let angle = LineSegment {
                start: left.to_point(),
                end: right.to_point(),
            }
            .angle();
            let mut shp = Arrow {
                fill_color: Some(bgc),
                point_angle: angle,
                head_angle: None,
                head_length: 10.0,
                stroke,
            }
            .to_shape();
            shp.transform(TSTransform::new(
                pts[1].to_vec2(),
                1. / context.current_transform.scaling,
            ));
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
        adj.x += ctr.x;

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
