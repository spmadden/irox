// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! This module has extras around the [`egui::style`] module
//!

use std::sync::Arc;

use eframe::{CreationContext, Frame, Storage};
use egui::{Context, Style};

///
/// Implementation of `eframe::App` that automatically saves the state of the
/// `egui::Style` when it changes in the background.
pub struct StylePersistingApp {
    style: Arc<Style>,
}

impl StylePersistingApp {
    #[must_use]
    pub fn new(cc: &CreationContext) -> StylePersistingApp {
        let mut style = cc.egui_ctx.style().clone();

        if let Some(storage) = cc.storage {
            if let Some(style_str) = storage.get_string("style") {
                if let Ok(parsed) = ron::from_str::<Style>(style_str.as_str()) {
                    cc.egui_ctx.set_style(parsed);
                    style = cc.egui_ctx.style();
                }
            }
        }
        StylePersistingApp { style }
    }
}

impl eframe::App for StylePersistingApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.style = ctx.style();
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        if let Ok(enc) = ron::to_string(&self.style) {
            storage.set_string("style", enc);
        }
    }
}
