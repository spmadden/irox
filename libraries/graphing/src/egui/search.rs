// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::SharedNodeIdentifier;
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
