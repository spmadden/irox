// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

extern crate alloc;

pub mod renderer;
pub mod search;
pub mod treelist;

use crate::egui::renderer::{
    EdgeRendererProvider, NodeRendererProvider, RenderingContext, DEFAULT_EDGE_RENDERER,
    DEFAULT_NODE_RENDERER,
};
use crate::fdp::{
    Centering, DefaultNodePlacement, EdgeForce, Force, Repulsive, Shared, Simulation,
    SimulationParams,
};
use crate::{Graph, SharedNodeIdentifier};
use alloc::rc::Rc;
use core::cell::RefCell;
use irox_egui_extras::drawpanel::{DrawPanel, LayerCommand, LayerOpts, ScaleMode};
use irox_egui_extras::eframe::epaint::RectShape;
use irox_egui_extras::egui::{
    Color32, Context, CornerRadius, Id, Rect, Response, Sense, Shape, Slider, Stroke, StrokeKind,
    Ui, Widget, Window,
};
use irox_egui_extras::{profile_scope};
use irox_geometry::transform::LinearTransform;
use irox_geometry::{Vector, Vector2D};
use std::sync::mpsc::Sender;

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
                _ => {}
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
        self.panel.show(ui);
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
                    let mut eshapes = Vec::new();
                    (self.edge_renderer)(edge).add_shapes_to(
                        &rendering_context,
                        edge,
                        left,
                        right,
                        &mut eshapes,
                    );
                    let painter = ui.painter();
                    let mut bbox = Rect::NOTHING;
                    for mut shp in eshapes {
                        bbox |= shp.visual_bounding_rect();
                        shp.transform(current_transform);
                        painter.add(shp);
                    }
                });
            }
        });
        let xfm = LinearTransform::<f64>::from(current_transform);
        self.sim.iter_nodes(|id, node, working| {
            let ctr = xfm.new_model_point(&working.current_position.to_point());
            node.get(|node| {
                profile_scope!("Node Renderer: {}", node.descriptor.id.to_string());
                let mut eshapes = Vec::new();
                (self.node_renderer)(node).add_shapes_to(
                    &rendering_context,
                    node,
                    working,
                    ctr,
                    &mut eshapes,
                );
                let mut bbox = Rect::NOTHING;
                for shp in &mut eshapes {
                    bbox |= shp.visual_bounding_rect();
                }
                if cfg!(debug_assertions) {
                    eshapes.push(Shape::Rect(RectShape::new(
                        bbox,
                        CornerRadius::default(),
                        Color32::TRANSPARENT,
                        Stroke::new(1.0, Color32::RED),
                        StrokeKind::Middle,
                    )));
                }
                let painter = ui.painter();
                eshapes.drain(..).for_each(|shp| {
                    painter.add(shp);
                });
                let resp = ui.interact(bbox, Id::new(id.to_string()), Sense::hover());
                (self.node_renderer)(node).on_response(
                    &rendering_context,
                    node,
                    &resp,
                    &mut shapes,
                );
            });
        });
        let painter = ui.painter();
        shapes.drain(..).for_each(|shp| {
            painter.add(shp);
        });
    }
}
