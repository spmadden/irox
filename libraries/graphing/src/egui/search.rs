// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::egui::renderer::NodeRenderingState;
use crate::{Graph, SharedNode, SharedNodeIdentifier};
use irox_egui_extras::egui;

#[derive(Debug, Clone)]
pub enum SearchResult {
    QueryChanged(String),
    SelectedNode(SharedNodeIdentifier),
}

pub struct SearchWidget {
    pub query: String,
    pub selected_index: Option<usize>,
    pub results: Vec<SharedNodeIdentifier>,
    pub operation_result: Vec<SearchResult>,
}
impl Default for SearchWidget {
    fn default() -> Self {
        Self::new()
    }
}
impl SearchWidget {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            selected_index: None,
            results: Vec::new(),
            operation_result: Vec::new(),
        }
    }
    fn mark_node_highlighted(node: &mut SharedNode, highlighted: bool) {
        node.memory_mut(|mem| {
            let Some(mem) = mem else {
                return;
            };
            let mut mem = mem.borrow_mut();
            let state = mem.get_mut_or_default::<_, NodeRenderingState>("NodeRenderingState");
            let Some(state) = state else {
                return;
            };
            state.highlighted = highlighted;
        });
    }
    fn mark_highlighted(node: &SharedNodeIdentifier, graph: &mut Graph, highlighted: bool) {
        if let Some(node) = graph.nodes.get_mut(node) {
            Self::mark_node_highlighted(node, highlighted);
        }
    }
    pub fn process(&mut self, graph: &mut Graph) {
        self.operation_result.retain(|op| {
            if let SearchResult::QueryChanged(query) = op {
                for res in self.results.drain(..) {
                    Self::mark_highlighted(&res, graph, false);
                }
                self.results.clear();
                self.selected_index = None;
                if query.is_empty() {
                    return false;
                }
                for node in graph.nodes.values_mut() {
                    let Some(id) = node.id() else {
                        continue;
                    };
                    let matched = node.descriptor(|d| {
                        let Some(d) = d else {
                            return false;
                        };
                        if d.id.to_string().to_lowercase().contains(query) {
                            self.results.push(id.clone().into());
                            return true;
                        }
                        if let Some(desc) = &d.description {
                            if desc.to_lowercase().contains(query) {
                                self.results.push(id.clone().into());
                                return true;
                            }
                        }
                        for val in d.attrs.values() {
                            if val.to_lowercase().contains(query) {
                                self.results.push(id.clone().into());
                                return true;
                            }
                        }
                        false
                    });
                    Self::mark_node_highlighted(node, matched);
                }
            }
            matches!(op, SearchResult::SelectedNode(_))
        });
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        if ui.text_edit_singleline(&mut self.query).changed() {
            self.selected_index = None;
            self.operation_result
                .push(SearchResult::QueryChanged(self.query.clone()));
        }
        if !self.results.is_empty() {
            let sel = self.selected_index.map(|v| v + 1).unwrap_or(0);
            let max = self.results.len();
            ui.label(format!("{sel} of {max}"));
            if ui.button("Next").clicked() {
                let current_index = if let Some(prev_idx) = self.selected_index {
                    if prev_idx + 1 < self.results.len() {
                        prev_idx + 1
                    } else {
                        0
                    }
                } else {
                    0
                };
                if let Some(selected) = self.results.get(current_index) {
                    self.operation_result
                        .push(SearchResult::SelectedNode(selected.clone()));
                }
                self.selected_index = Some(current_index);
            }
        }
    }
}
