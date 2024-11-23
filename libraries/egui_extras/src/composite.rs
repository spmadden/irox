// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Structures to allow the composition of various apps
//!

use std::time::Duration;

use eframe::{App, Frame, Storage};
use egui::Context;

///
/// An implementation of `eframe::App` that allows the composition of multiple sub-apps.
/// Each app is called in the order added to this structure
pub struct CompositeApp {
    apps: Vec<Box<dyn App>>,

    persist_egui_memory: bool,
}

impl Default for CompositeApp {
    fn default() -> Self {
        CompositeApp {
            apps: Vec::new(),
            persist_egui_memory: true,
        }
    }
}

impl CompositeApp {
    pub fn add(&mut self, app: Box<dyn App>) {
        self.persist_egui_memory &= app.persist_egui_memory();

        self.apps.push(app);
    }
}

impl App for CompositeApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        for app in &mut self.apps {
            app.update(ctx, frame)
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        for app in &mut self.apps {
            app.save(storage)
        }
    }

    fn auto_save_interval(&self) -> Duration {
        let min = self.apps.iter().map(|app| app.auto_save_interval()).min();
        if let Some(min) = min {
            return min;
        }
        Duration::from_secs(30)
    }

    fn persist_egui_memory(&self) -> bool {
        self.persist_egui_memory
    }
}
