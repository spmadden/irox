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
use irox_graphing::egui::treelist::TreeListWidget;
use irox_graphing::egui::FDPSimulationWidget;
use irox_graphing::fdp::{Centering, EdgeForce, Force, Repulsive, Shared};
use irox_graphing::{Edge, EdgeDescriptor, Graph, Node};
use irox_log::log::{error, Level};
use irox_tools::identifier::Identifier;
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

pub struct FDPSimulationApp {
    widget: FDPSimulationWidget,
    is_expanded: bool,
}
impl FDPSimulationApp {
    pub fn new(graph: Shared<Graph>, cc: &eframe::CreationContext) -> Self {
        load_fonts(FontSet::basics(), &cc.egui_ctx);

        let mut widget = FDPSimulationWidget::new(graph);
        widget.draw_id = false;
        widget.sim_params_window = false;
        widget.sim.forces = vec![
            Force::Centering(Centering::new(0.01)),
            Force::Repulsive(
                Repulsive::default()
                    .with_strength(-100.)
                    .with_edge_distance(100.),
            ),
            Force::Edge(EdgeForce::default().with_distance(100.)),
        ];
        FDPSimulationApp {
            widget,
            is_expanded: true,
        }
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
            if ui.button("\u{21BA}").clicked() {
                self.widget.sim.params.tick = 0;
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
                    "Position: {} // Scale: {}",
                    xfm.translation, xfm.scaling
                ));
            },
        );
    }
}
