// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_egui_extras::composite::CompositeApp;
use irox_egui_extras::eframe;
use irox_egui_extras::eframe::App;
use irox_egui_extras::egui::{CentralPanel, Ui, Vec2, ViewportBuilder};
use irox_egui_extras::fonts::{load_fonts, FontSet};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame, ToolFrameOptions};
use irox_graphing::egui::FDPSimulationWidget;
use irox_graphing::fdp::magnetic::Magnetic;
use irox_graphing::fdp::{Centering, EdgeForce, Force, Repulsive, Shared};
use irox_graphing::{Descriptor, Edge, EdgeDescriptor, Graph, Node, NodeDescriptor};
use irox_log::log::{error, Level};
use irox_tools::hash::bytewords::words_to_string;
use irox_tools::identifier::Identifier;
use irox_units::units::angle::Angle;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MNode {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub group: String,
}
fn get_identifier(id: &str) -> Identifier {
    let id = irox_tools::hash::murmur3_128(id.as_bytes()) as u32;
    let out = words_to_string(&id.to_be_bytes(), "-");
    Identifier::String(out)
}
impl From<MNode> for Node {
    fn from(value: MNode) -> Self {
        Node {
            descriptor: NodeDescriptor(Descriptor {
                id: get_identifier(&value.id).into(),
                description: None,
                attrs: BTreeMap::from_iter([("group".into(), value.group.clone())]),
            }),
            navigable_edges: vec![],
            all_edges: vec![],
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MEdge {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub value: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MGraph {
    #[serde(default)]
    pub nodes: Vec<MNode>,
    #[serde(default)]
    pub links: Vec<MEdge>,
}
impl TryFrom<MGraph> for Graph {
    type Error = String;

    fn try_from(mut value: MGraph) -> Result<Self, Self::Error> {
        let mut g = Graph::default();

        for n in value.nodes {
            let node: Node = n.into();
            g.add_node(node.into())?;
        }
        g.add_node(
            Node {
                descriptor: NodeDescriptor(Descriptor {
                    id: Identifier::random_string().into(),
                    description: None,
                    attrs: BTreeMap::new(),
                }),
                navigable_edges: vec![],
                all_edges: vec![],
            }
            .into(),
        )?;
        for (idx, e) in value.links.drain(..).enumerate() {
            let from = get_identifier(&e.source).into();
            let to = get_identifier(&e.target).into();

            let descriptor = EdgeDescriptor(Identifier::Integer(idx as u64).into());
            let e = Edge::Directed {
                descriptor,
                from,
                to,
            };
            g.add_edge(e.into())?;
        }

        Ok(g)
    }
}

pub fn main() -> Result<(), String> {
    let graph: MGraph =
        serde_json::from_str(include_str!("disjoint.json")).map_err(|e| e.to_string())?;

    let graph: Graph = graph.try_into()?;
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
        FDPSimulationApp { widget }
    }
}

impl eframe::App for FDPSimulationApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
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
