// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::fdp::{Collision, EdgeForce, Force, PosForce, Repulsive, Simulation, SimulationParams};
use crate::Graph;
use irox_egui_extras::drawpanel::{DrawPanel, LayerCommand, LayerOpts, ScaleMode};
use irox_egui_extras::eframe::epaint::{CircleShape, RectShape, TextShape};
use irox_egui_extras::egui::text::LayoutJob;
use irox_egui_extras::egui::{Align, Context, CornerRadius, FontId, Pos2, Shape, Ui};
use irox_egui_extras::{line_to_bezier, WithAlpha};
use std::sync::mpsc::Sender;

pub struct FDPSimulationWidget {
    pub sim: Simulation,
    pub panel: DrawPanel,
    pub graph_layer: Sender<LayerCommand>,
    pub play: bool,
    pub last_tick: f64,
}
impl FDPSimulationWidget {
    pub fn new(graph: &Graph) -> Self {
        let sim = Simulation::new(
            SimulationParams::default(),
            &graph.nodes.values().cloned().collect::<Vec<_>>(),
            &graph.edges.values().cloned().collect::<Vec<_>>(),
            vec![
                Force::Position(PosForce::default()),
                Force::Edge(EdgeForce::default().with_distance(20.)), //.with_fixed_strength(Some(1.))),
                Force::Repulsive(Repulsive::default().with_strength(-1.)),
                Force::Collision(Collision::default().with_radius(10.)),
            ],
        );
        let mut panel = DrawPanel::new("graph");
        panel.draw_cursor_crosshairs = false;
        let sender = panel.add_layer(
            "graph".to_string(),
            LayerOpts {
                visible: true,
                scale_mode: ScaleMode::ScaleOnlyPosition,
            },
        );
        FDPSimulationWidget {
            sim,
            panel,
            graph_layer: sender,
            play: true,
            last_tick: 0.0,
        }
    }
    pub fn tick(&mut self) {
        self.sim.tick();
    }
    pub fn play_tick(&mut self, ctx: &Context) {
        if !self.play {
            return;
        }
        let time = ctx.input(|i| i.time);
        if time - self.last_tick > 0.0 {
            self.last_tick = time;
            self.tick();
        }
    }
    pub fn show(&mut self, ui: &mut Ui) {
        ui.label(format!("Tick: {}", self.sim.params.tick));
        self.play_tick(ui.ctx());
        ui.horizontal(|ui| {
            if ui.button("Tick!").clicked() {
                self.tick();
            }
            let text = if self.play { "\u{23F8}" } else { "\u{23F5}" };
            ui.checkbox(&mut self.play, text);
        });
        let fgc = ui.visuals().widgets.active.fg_stroke.color;
        let bgc = ui.visuals().widgets.active.bg_fill.with_alpha(160);
        let mut shapes = Vec::new();
        for e in self.sim.edges.borrow().iter() {
            let e = e.borrow();
            let left = e.left.borrow().current_position;
            let right = e.right.borrow().current_position;
            let pts = [
                Pos2::new(left.vx as f32, left.vy as f32),
                Pos2::new(right.vx as f32, right.vy as f32),
            ];
            shapes.push(line_to_bezier(pts));
        }
        for n in self.sim.nodes.borrow().iter() {
            let node = n.borrow();
            let id = node
                .node
                .descriptor(|v| v.cloned().map(|v| v.id.to_string()))
                .unwrap_or_default();
            let p = node.current_position;
            let p = Pos2::new(p.vx as f32, p.vy as f32);

            shapes.push(Shape::Circle(CircleShape::filled(p, 2., fgc)));
            let galley = ui.ctx().fonts(|f| {
                let mut job =
                    LayoutJob::simple(id.clone(), FontId::monospace(14.), fgc, f32::INFINITY);
                job.halign = Align::Center;
                f.layout_job(job)
            });
            let ctr = galley.rect.size() / 2.;
            let mut adj = galley.rect.left_top();
            adj.x += ctr.x / self.panel.transform.scaling;

            let rect = galley.rect.translate(-adj.to_vec2()).translate(p.to_vec2());

            let txt = Shape::Text(TextShape::new(p, galley, fgc));

            let rect = RectShape::filled(rect, CornerRadius::default(), bgc);
            shapes.push(Shape::Rect(rect));
            shapes.push(txt);
        }

        let _ = self.graph_layer.send(LayerCommand::ClearSetShapes(shapes));
        self.panel.show(ui);
    }
}
