// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

pub mod renderer;
pub mod treelist;

use crate::egui::renderer::{
    EdgeRendererProvider, NodeRendererProvider, RenderingContext, DEFAULT_EDGE_RENDERER,
    DEFAULT_NODE_RENDERER,
};
use crate::fdp::{
    Centering, DefaultNodePlacement, EdgeForce, Force, Repulsive, Shared, Simulation,
    SimulationParams,
};
use crate::{Graph, SharedEdgeIdentifier, SharedNodeIdentifier};
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::Write;
use irox_egui_extras::drawpanel::{DrawPanel, LayerCommand, LayerOpts, ScaleMode};
use irox_egui_extras::eframe::epaint::{RectShape, TextShape};
use irox_egui_extras::egui::text::LayoutJob;
use irox_egui_extras::egui::{
    Align, Context, CornerRadius, FontId, Pos2, Response, Shape, Slider, Ui, Vec2, Widget, Window,
};
use irox_egui_extras::{profile_scope, WithAlpha};
use irox_geometry::{LineSegment, Point, Point2D, Vector, Vector2D};
use std::sync::mpsc::Sender;

const EDGE_MOUSEOVER_MAX_DISTANCE: f32 = 5.;
const NODE_MOUSEOVER_MAX_DISTANCE: f32 = 20.;

pub struct ParamsWindow<'a> {
    pub params: &'a mut SimulationParams,
    pub forces: &'a mut Vec<Force>,
}
impl ParamsWindow<'_> {
    pub fn show(&mut self, ui: &mut Ui) {
        Slider::new(&mut self.params.alpha, 0.0..=1.0)
            .text("Simulation Energy (alpha)")
            .ui(ui);
        if Slider::new(&mut self.params.target_iterations, 100..=600)
            .text("Target Iterations")
            .ui(ui)
            .changed()
        {
            self.params.alpha = 1.0;
            self.params.update_target_iterations();
        }
        if Slider::new(&mut self.params.alpha_min, 0.1..=0.75)
            .text("Energy Density (alpha_min)")
            .ui(ui)
            .changed()
        {
            self.params.update_target_iterations();
            self.params.alpha = 1.0;
        }
        if Slider::new(&mut self.params.velocity_decay, 0.1..=0.75)
            .text("Particule Energy (velocity decay)")
            .ui(ui)
            .changed()
        {
            self.params.alpha = 1.0;
        }

        let mut centering_force = None;
        let mut edge_force = None;
        let mut node_force = None;
        let mut boundary_force = None;

        for f in self.forces.iter_mut() {
            match f {
                Force::Centering(p) => centering_force = Some(p),
                Force::Edge(e) => edge_force = Some(e),
                Force::Repulsive(r) => node_force = Some(r),
                Force::Collision(c) => boundary_force = Some(c),
            }
        }
        if let Some(cf) = centering_force {
            if Slider::new(&mut cf.strength, 0.0..=1.)
                .text("Centering Strength")
                .ui(ui)
                .changed()
            {
                self.params.alpha = 1.0;
            };
        }
        if let Some(ef) = edge_force {
            if Slider::new(&mut ef.distance, 1.0..=100.)
                .text("Desired Edge Distance")
                .ui(ui)
                .changed()
            {
                self.params.alpha = 1.0;
            };
            if Slider::new(&mut ef.iterations, 1..=10)
                .text("Edge Strength")
                .ui(ui)
                .changed()
            {
                self.params.alpha = 1.0;
            };
        }
        if let Some(nf) = node_force {
            if Slider::new(&mut nf.strength, 10.0..=-100.)
                .text("Node Strength")
                .ui(ui)
                .changed()
            {
                self.params.alpha = 1.0;
            };
        }
        if let Some(cf) = boundary_force {
            if Slider::new(&mut cf.strength, 0.1..=100.)
                .text("Collision Force")
                .ui(ui)
                .changed()
            {
                self.params.alpha = 1.0;
            };
            if Slider::new(&mut cf.iterations, 1..=10)
                .text("Collision Strength")
                .ui(ui)
                .changed()
            {
                self.params.alpha = 1.0;
            };
        }

        if ui.button("Defaults").clicked() {
            *self.params = SimulationParams::default();
        }
    }
}

pub struct FDPSimulationWidget {
    pub sim: Simulation,
    pub panel: DrawPanel,
    pub graph_layer: Sender<LayerCommand>,
    pub tt_layer: Sender<LayerCommand>,
    pub drag_subject: Option<SharedNodeIdentifier>,
    pub play: bool,
    pub last_tick: f64,
    pub show_tick_controls: bool,
    pub draw_id: bool,

    pub sim_params_window: bool,
    pub response: Shared<Option<Response>>,
    pub node_renderer: NodeRendererProvider,
    pub edge_renderer: EdgeRendererProvider,
}
impl FDPSimulationWidget {
    pub fn with_simulation(sim: Simulation) -> Self {
        let mut panel = DrawPanel::new("graph");
        panel.draw_cursor_crosshairs = false;
        let graph_layer = panel.add_layer(
            "graph".to_string(),
            LayerOpts {
                visible: true,
                scale_mode: ScaleMode::ScaleOnlyPosition,
            },
        );
        let tt_layer = panel.add_layer(
            "tooltip".to_string(),
            LayerOpts {
                visible: true,
                scale_mode: ScaleMode::ScaleOnlyPosition,
            },
        );
        let response: Rc<RefCell<Option<Response>>> = Rc::new(RefCell::new(None));
        let r2 = response.clone();
        panel.interactions.push(Box::new(move |resp| {
            r2.replace(Some(resp.clone()));
        }));

        FDPSimulationWidget {
            sim,
            panel,
            graph_layer,
            play: true,
            last_tick: 0.0,
            show_tick_controls: false,
            tt_layer,
            drag_subject: None,
            sim_params_window: false,
            draw_id: true,
            response,
            node_renderer: Box::new(|_| &DEFAULT_NODE_RENDERER),
            edge_renderer: Box::new(|_| &DEFAULT_EDGE_RENDERER),
        }
    }
    pub fn new(graph: Shared<Graph>) -> Self {
        let alpha_min = 0.2f64;
        let alpha_decay = 1. - alpha_min.powf(1. / 300.);
        let params = SimulationParams {
            alpha_min,
            alpha_decay,
            velocity_decay: 0.6,
            ..Default::default()
        };
        let sim = Simulation::new(
            params,
            vec![
                Force::Centering(Centering::default()),
                Force::Edge(EdgeForce::default()),
                Force::Repulsive(Repulsive::default()),
            ],
            graph,
            Box::new(DefaultNodePlacement::default()),
        );
        Self::with_simulation(sim)
    }
    pub fn tick(&mut self, ctx: &Context) {
        self.sim.tick();
        if !self.sim.is_done() {
            ctx.request_repaint();
        }
    }
    pub fn play_tick(&mut self, ctx: &Context) {
        profile_scope!("FDPWidget::play_tick");
        if !self.play {
            return;
        }
        let time = ctx.input(|i| i.time);
        if time - self.last_tick > 0.0 {
            self.last_tick = time;
            self.tick(ctx);
        }
    }
    pub fn show(&mut self, ui: &mut Ui) {
        profile_scope!("FDPWidget::show");
        if self.sim_params_window {
            Window::new("Simulation Params").show(ui.ctx(), |ui| {
                ParamsWindow {
                    params: &mut self.sim.params,
                    forces: &mut self.sim.forces,
                }
                .show(ui);
            });
        }
        self.play_tick(ui.ctx());
        if self.show_tick_controls {
            ui.label(format!("Tick: {}", self.sim.params.tick));
            ui.horizontal(|ui| {
                if ui.button("Tick!").clicked() {
                    self.tick(ui.ctx());
                }
                let text = if self.play { "\u{23F8}" } else { "\u{23F5}" };
                ui.checkbox(&mut self.play, text);
            });
        }
        let current_transform = self.panel.transform;
        let current_world_area = self.panel.world_area;
        let last_window_area = self.panel.last_window_area;
        let rendering_context = RenderingContext {
            current_transform,
            current_world_area,
            last_window_area,
            ui,
        };
        let mut shapes = Vec::new();
        self.sim.iter_edges(|edge, sim| {
            let mut left = Vector::<f64>::default();
            let mut right = Vector::<f64>::default();
            sim.node_mut(&edge.left, |n| {
                left = n.current_position;
            });
            sim.node_mut(&edge.right, |n| {
                right = n.current_position;
            });
            if let Some(edge) = sim.graph.borrow().edges.get(&edge.id) {
                edge.get(|edge| {
                    profile_scope!("Edge Renderer: {}", edge.id().to_string());
                    (self.edge_renderer)(edge).add_shapes_to(
                        &rendering_context,
                        edge,
                        left,
                        right,
                        &mut shapes,
                    );
                });
            }
        });
        self.sim.iter_nodes(|_id, node, working| {
            let ctr = working.current_position;
            node.get(|node| {
                profile_scope!("Node Renderer: {}", node.descriptor.id.to_string());
                (self.node_renderer)(node).add_shapes_to(
                    &rendering_context,
                    node,
                    ctr,
                    &mut shapes,
                );
            });
        });

        let _ = self.graph_layer.send(LayerCommand::ClearSetShapes(shapes));
        self.find_hover(ui);
        self.panel.show(ui);
    }
    pub fn find_closest_edge_to(&mut self, pos: Pos2) -> Option<SharedEdgeIdentifier> {
        profile_scope!("FDPWidget::find_closest_edge_to");
        let xfm = self.panel.transform;
        let mut closest_edge: Option<SharedEdgeIdentifier> = None;
        let mut closest_edge_dist = f32::MAX;
        let translate = Vector::new(xfm.translation.x as f64, xfm.translation.y as f64);
        let mouse = Point::new_point(pos.x as f64, pos.y as f64);
        self.sim.iter_edges(|e, sim| {
            let id = e.id.clone();
            let mut p1 = Vector::<f64>::default();
            sim.node(&e.left, |v| {
                p1 = v.current_position;
            });
            let mut p2 = Vector::<f64>::default();
            sim.node(&e.right, |v| {
                p2 = v.current_position;
            });
            let p1 = p1 * xfm.scaling as f64 + translate;
            let p2 = p2 * xfm.scaling as f64 + translate;
            let line = LineSegment {
                start: p1.to_point(),
                end: p2.to_point(),
            };
            let distance = line.distance_to(&mouse) as f32;

            if distance < EDGE_MOUSEOVER_MAX_DISTANCE && distance <= closest_edge_dist {
                closest_edge = Some(id);
                closest_edge_dist = distance;
            }
        });
        closest_edge
    }
    pub fn find_closest_node_to(&mut self, pos: Pos2) -> Option<SharedNodeIdentifier> {
        profile_scope!("FDPWidget::find_closest_node_to");

        let xfm = self.panel.transform;

        let mut closest_node_dist = f32::MAX;
        let mut closest_node: Option<SharedNodeIdentifier> = None;
        self.sim.iter_nodes(|id, _node, sim| {
            let np = sim.current_position;
            let np = Pos2::new(np.vx as f32, np.vy as f32);
            let npw = xfm * np;
            let dp = pos - npw;
            let distance = dp.length();
            if distance < NODE_MOUSEOVER_MAX_DISTANCE && distance <= closest_node_dist {
                closest_node = Some(id.clone());
                closest_node_dist = distance;
            }
        });
        closest_node
    }

    pub fn find_hover(&mut self, ui: &mut Ui) {
        profile_scope!("FDPWidget::find_hover");
        let response = self.response.borrow().clone();
        if let Some(response) = &response {
            if let Some(mut pos) = response.hover_pos() {
                let _ = self.tt_layer.send(LayerCommand::ClearShapes);
                if let Some(area) = self.panel.last_window_area {
                    pos -= area.min.to_vec2();
                }
                let closest_node = self.find_closest_node_to(pos);
                let closest_edge = self.find_closest_edge_to(pos);

                let contains = if let Some(d) = closest_node {
                    let mut out = String::new();
                    let _ = writeln!(&mut out, "id: {d}");

                    // if let Some(desc) = &d.description {
                    //     let _ = writeln!(&mut out, "desc: {}", desc);
                    // }
                    // for (k, v) in &d.attrs {
                    //     let _ = writeln!(&mut out, "{k}: {v}");
                    // }
                    let clicked = response.clicked();

                    if clicked {
                        // self.spawn_panel_window(d.id.to_string());
                    }

                    out.trim().to_string()
                } else if let Some(d) = closest_edge {
                    let mut out = String::new();
                    let _ = writeln!(&mut out, "id: {d}");

                    // if let Some(desc) = &d.description {
                    //     let _ = writeln!(&mut out, "desc: {}", desc);
                    // }
                    // for (k, v) in &d.attrs {
                    //     let _ = writeln!(&mut out, "{k}: {v}");
                    // }
                    out.trim().to_string()
                } else {
                    String::default()
                };
                if !contains.is_empty() {
                    let fgc = ui.visuals().widgets.active.fg_stroke.color;
                    let bgc = ui.visuals().widgets.active.bg_fill.with_alpha(160);
                    let galley = ui.ctx().fonts_mut(|f| {
                        let mut job = LayoutJob::simple(
                            contains.clone(),
                            FontId::monospace(14.),
                            fgc,
                            f32::INFINITY,
                        );
                        job.halign = Align::LEFT;
                        f.layout_job(job)
                    });
                    let pos = self.panel.transform.inverse() * (pos + Vec2::new(20., 5.));
                    let rect = galley.rect.translate(pos.to_vec2());
                    let txt = Shape::Text(TextShape::new(pos, galley, fgc));
                    let rect = RectShape::filled(rect, CornerRadius::default(), bgc);
                    let _ = self
                        .tt_layer
                        .send(LayerCommand::ClearSetShapes(vec![Shape::Rect(rect), txt]));
                }
            }
        }
    }
}
