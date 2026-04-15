// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Edge, Node};
use irox_egui_extras::egui::Pos2;
use irox_egui_extras::egui::Shape;
use irox_egui_extras::egui::{Color32, Stroke};
use irox_geometry::Vector;

pub type NodeRendererProvider = Box<dyn FnMut(&Node) -> &dyn NodeRenderer>;
pub trait NodeRenderer {
    fn get_shapes(&self, node: &Node, center: Vector<f64>) -> Vec<Shape> {
        let mut out = Vec::new();
        self.add_shapes_to(node, center, &mut out);
        out
    }
    fn add_shapes_to(&self, node: &Node, center: Vector<f64>, shapes: &mut Vec<Shape>);
}
pub static DEFAULT_NODE_RENDERER: DefaultNodeRenderer = DefaultNodeRenderer;
pub struct DefaultNodeRenderer;
impl NodeRenderer for DefaultNodeRenderer {
    fn add_shapes_to(&self, _node: &Node, _center: Vector<f64>, _shapes: &mut Vec<Shape>) {

    }
}

pub type EdgeRendererProvider = Box<dyn FnMut(&Edge) -> &dyn EdgeRenderer>;
pub trait EdgeRenderer {
    fn get_shapes(&self, edge: &Edge, left: Vector<f64>, right: Vector<f64>) -> Vec<Shape> {
        let mut out = Vec::new();
        self.add_shapes_to(edge, left, right, &mut out);
        out
    }
    fn add_shapes_to(&self, edge: &Edge, left: Vector<f64>, right: Vector<f64>, shapes: &mut Vec<Shape>);
}

pub static DEFAULT_EDGE_RENDERER: DefaultEdgeRenderer = DefaultEdgeRenderer;
pub struct DefaultEdgeRenderer;
impl EdgeRenderer for DefaultEdgeRenderer {
    fn add_shapes_to(&self, _edge: &Edge, left: Vector<f64>, right: Vector<f64>, shapes: &mut Vec<Shape>) {
        let pts = [
            Pos2::new(left.vx as f32, left.vy as f32),
            Pos2::new(right.vx as f32, right.vy as f32),
        ];
        // shapes.push(line_to_bezier(pts));
        shapes.push(Shape::LineSegment {
            points: pts,
            stroke: Stroke::new(1.0, Color32::BLACK),
        });
    }
}
