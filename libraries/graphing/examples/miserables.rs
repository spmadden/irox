// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_egui_extras::eframe;
use irox_egui_extras::egui::{CentralPanel, Context, Vec2, ViewportBuilder};
use irox_egui_extras::fonts::{load_fonts, FontSet};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame, ToolFrameOptions};
use irox_graphing::egui::FDPSimulationWidget;
use irox_graphing::{Descriptor, Edge, EdgeDescriptor, Graph, Node, NodeDescriptor};
use irox_log::log::{error, Level};
use irox_tools::identifier::Identifier;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MNode {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub group: u32,
}
impl From<MNode> for Node {
    fn from(value: MNode) -> Self {
        Node {
            descriptor: NodeDescriptor(Descriptor {
                id: Identifier::String(value.id),
                description: None,
                attrs: BTreeMap::from_iter([("group".into(), format!("{}", value.group))]),
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
        for (idx, e) in value.links.drain(..).enumerate() {
            let from: Descriptor = Identifier::String(e.source).into();
            let to: Descriptor = Identifier::String(e.target).into();

            let Some(from) = g.nodes.get(&NodeDescriptor(from)).cloned() else {
                return Err("Can't find from node".to_string());
            };
            let Some(to) = g.nodes.get(&NodeDescriptor(to)).cloned() else {
                return Err("Can't find to node".to_string());
            };
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
        serde_json::from_str(include_str!("miserables.json")).map_err(|e| e.to_string())?;

    let graph: Graph = graph.try_into()?;

    irox_log::init_console_level(Level::Info);
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 1024.));

    let native_options = eframe::NativeOptions {
        viewport,
        multisampling: 0,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "draw-panels",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.0);
            cc.egui_ctx.set_zoom_factor(1. / 1.25);
            cc.egui_ctx.tessellation_options_mut(|v| {
                v.feathering = false;
                v.round_rects_to_pixels = false;
            });
            Ok(Box::new(ToolFrame::new_opts(
                cc,
                Box::new(FDPSimulationApp::new(&graph, cc)),
                ToolFrameOptions {
                    show_rendering_stats: true,
                    full_speed: true,
                    enable_memory_ui: true,
                    enable_texture_ui: true,
                    enable_inspection_ui: true,
                    enable_settings_ui: true,

                    ..Default::default()
                },
            )))
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
    pub fn new(graph: &Graph, cc: &eframe::CreationContext) -> Self {
        load_fonts(FontSet::all_as_defaults(), &cc.egui_ctx);

        FDPSimulationApp {
            widget: FDPSimulationWidget::new(graph),
        }
    }
}

impl eframe::App for FDPSimulationApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.widget.show(ui);
        });
    }
}

impl ToolApp for FDPSimulationApp {}
