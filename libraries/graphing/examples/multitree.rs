// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::print_stdout)]

use eframe::emath::Vec2;
use eframe::App;
use egui::{CentralPanel, Panel, Ui, ViewportBuilder, Widget};
use irox_egui_extras::composite::CompositeApp;
use irox_egui_extras::fonts::{load_fonts, FontSet};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame, ToolFrameOptions};
use irox_graphing::algorithms::roots::Roots;
use irox_graphing::egui::renderer::{
    CompositeNodeRenderer, DebugForceNodeRenderer, DEFAULT_NODE_RENDERER,
};
use irox_graphing::egui::treelist::TreeListWidget;
use irox_graphing::egui::FDPSimulationWidget;
use irox_graphing::fdp::magnetic::Magnetic;
use irox_graphing::fdp::{Centering, EdgeForce, Force, Repulsive, Shared};
use irox_graphing::{Edge, EdgeDescriptor, Graph, Node};
use irox_log::log::{error, Level};
use irox_tools::identifier::Identifier;
use irox_tools::static_init;
use irox_units::units::angle::Angle;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub fn main() -> Result<(), String> {
    let mut graph = Graph::new();

    for n in ["G", "H", "E", "B", "D", "F", "A", "C", "I"] {
        let _ = graph.add_node(Node::from_id(n.into()).into());
    }
    for (l, r) in [
        ("D", "G"),
        ("B", "E"),
        ("A", "D"),
        ("D", "F"),
        ("B", "D"),
        ("A", "C"),
        ("G", "I"),
        ("D", "G"),
        ("H", "I"),
    ] {
        let _ = graph.add_edge(
            Edge::Directed {
                descriptor: EdgeDescriptor(Identifier::random_string().into()),
                from: Identifier::from(l).into(),
                to: Identifier::from(r).into(),
            }
            .into(),
        );
    }

    let roots = Roots::run(&graph);
    println!("roots: {roots:?}");

    let graph: Shared<Graph> = Rc::new(RefCell::new(graph));

    irox_log::init_console_level(Level::Info);
    let viewport = ViewportBuilder::default()
        .with_inner_size(Vec2::new(1024., 1024.))
        // .with_maximized(true)
        ;

    let native_options = eframe::NativeOptions {
        viewport,
        multisampling: 0,
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "irox-example-graphing-disjoint",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(CompositeApp::from(Vec::from([
                // Box::new(StylePersistingApp::new(cc)) as Box<dyn App>,
                Box::new(ToolFrame::new_opts(
                    cc,
                    Box::new(FDPSimulationApp::new(graph, cc)),
                    ToolFrameOptions {
                        show_rendering_stats: true,
                        full_speed: false,
                        enable_memory_ui: true,
                        enable_texture_ui: true,
                        enable_inspection_ui: true,
                        enable_settings_ui: true,

                        ..Default::default()
                    },
                )),
            ]
                as [Box<dyn App>; _]))))
        }),
    ) {
        error!("{e:?}");
    };
    Ok(())
}

static_init!(renderer, CompositeNodeRenderer, {
    CompositeNodeRenderer {
        renderers: vec![
            Box::new(DEFAULT_NODE_RENDERER),
            Box::new(DebugForceNodeRenderer),
        ],
    }
});
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    InitialNoMag,
    RunningMag,
    Done,
}
impl State {
    #[must_use]
    pub fn next(&self, app: &mut FDPSimulationApp) -> Self {
        match self {
            State::InitialNoMag => {
                app.set_forces_mag();
                app.widget.sim.restart();
                State::RunningMag
            }
            State::RunningMag => State::Done,
            State::Done => *self,
        }
    }
}
pub struct FDPSimulationApp {
    widget: FDPSimulationWidget,
    is_expanded: bool,
    state: State,
}
impl FDPSimulationApp {
    pub fn set_forces_mag(&mut self) {
        self.widget.sim.forces = vec![
            Force::Centering(Centering::new(0.01)),
            Force::Repulsive(
                Repulsive::default()
                    .with_strength(-100.)
                    .with_edge_distance(100.),
            ),
            Force::Edge(EdgeForce::default().with_distance(100.)),
            Force::Magnetic(Magnetic {
                iterations: 1,
                strength: 10.,
                field_angles: vec![
                    Angle::new_degrees(30.0),
                    Angle::new_degrees(45.0),
                    Angle::new_degrees(60.0),
                    Angle::new_degrees(90.0),
                    Angle::new_degrees(105.0),
                    Angle::new_degrees(135.0),
                    Angle::new_degrees(150.0),
                ],
            }),
        ];
    }
    pub fn set_forces_nomag(&mut self) {
        self.widget.sim.forces = vec![
            Force::Centering(Centering::new(0.01)),
            Force::Repulsive(
                Repulsive::default()
                    .with_strength(-100.)
                    .with_edge_distance(100.),
            ),
            Force::Edge(EdgeForce::default().with_distance(100.)),
        ];
    }
    pub fn new(graph: Shared<Graph>, cc: &eframe::CreationContext) -> Self {
        load_fonts(FontSet::basics(), &cc.egui_ctx);
        let mut widget = FDPSimulationWidget::new(graph);
        widget.draw_id = false;
        widget.sim_params_window = false;
        widget.node_renderer = Box::new(|_| renderer());
        widget.play = false;
        widget.show_tick_controls = true;
        // widget.sim.params.halt_on_energy = true;

        let mut app = FDPSimulationApp {
            widget,
            is_expanded: true,
            state: State::InitialNoMag,
        };
        app.set_forces_nomag();
        app
    }
}

impl eframe::App for FDPSimulationApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        Panel::left("left")
            .resizable(true)
            .show_animated_inside(ui, self.is_expanded, |ui| {
                let graph = self.widget.sim.graph.borrow();
                TreeListWidget::new(graph.deref()).ui(ui);
            });
        CentralPanel::default().show_inside(ui, |ui| {
            self.widget.show(ui);
        });
    }
}

impl ToolApp for FDPSimulationApp {
    fn bottom_bar(&mut self, ui: &mut Ui) {
        if self.widget.sim.is_done() {
            let current = self.state;
            let next = current.next(self);
            self.state = next;

            if ui.button("\u{21BA}").clicked() {
                self.widget.sim.params.tick = 0;
                // self.state = State::RunningMag;
                // self.set_forces_nomag();
                self.widget.sim.restart();
            }
        } else {
            ui.label(format!("{}", self.widget.sim.params.tick as u32));
        }
        #[cfg(debug_assertions)]
        ui.with_layout(
            egui::Layout::default().with_main_align(egui::Align::RIGHT),
            |ui| {
                let xfm = self.widget.panel.transform;
                ui.label(format!(
                    "Position: {} // Scale: {} // Energy: {:0.03} // Alpha: {:0.03}",
                    xfm.translation,
                    xfm.scaling,
                    self.widget.sim.params.average_energy,
                    self.widget.sim.params.alpha
                ));
            },
        );
    }
}
