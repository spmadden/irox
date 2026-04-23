// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::algorithms::roots::Roots;
use crate::{Graph, SharedNodeIdentifier};
use irox_egui_extras::egui::{CollapsingHeader, Response, Ui, Vec2, Widget};

pub struct TreeListWidget<'a> {
    pub graph: &'a Graph,
    pub roots: Vec<SharedNodeIdentifier>,
}
impl<'a> TreeListWidget<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let roots = Roots::run(graph);
        Self { graph, roots }
    }
}

impl Widget for TreeListWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.allocate_ui(Vec2::default(), |ui| {
            for root in &self.roots {
                CollapsingHeader::new(root.to_string()).show(ui, |_ui| {});
            }
        })
        .response
    }
}
