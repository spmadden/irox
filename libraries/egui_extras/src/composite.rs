// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Structures to allow the composition of various apps
//!

use std::time::Duration;

use eframe::{App, Frame, Storage};
use egui::{Context, RawInput, Visuals};

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
            persist_egui_memory: false,
        }
    }
}

impl CompositeApp {
    pub fn add(&mut self, app: Box<dyn App>) {
        self.persist_egui_memory |= app.persist_egui_memory();

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

    #[cfg(feature = "glow")]
    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        for app in &mut self.apps {
            app.on_exit(gl)
        }
    }

    #[cfg(not(feature = "glow"))]
    fn on_exit(&mut self) {
        for app in &mut self.apps {
            app.on_exit()
        }
    }

    fn auto_save_interval(&self) -> Duration {
        let min = self.apps.iter().map(|app| app.auto_save_interval()).min();
        if let Some(min) = min {
            return min;
        }
        Duration::from_secs(30)
    }

    fn clear_color(&self, visuals: &Visuals) -> [f32; 4] {
        if let Some(f) = self.apps.first() {
            f.clear_color(visuals)
        } else {
            egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()
        }
    }

    fn persist_egui_memory(&self) -> bool {
        self.persist_egui_memory
    }

    fn raw_input_hook(&mut self, ctx: &Context, raw_input: &mut RawInput) {
        for app in &mut self.apps {
            app.raw_input_hook(ctx, raw_input)
        }
    }
}
