// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Structures to allow the composition of various apps
//!

use std::time::Duration;

use eframe::emath::Vec2;
use eframe::{App, Frame, Storage};
use egui::Context;

///
/// An implementation of `eframe::App` that allows the composition of multiple sub-apps.
/// Each app is called in the order added to this structure
pub struct CompositeApp {
    apps: Vec<Box<dyn App>>,

    persist_native_window: bool,
    persist_egui_memory: bool,
    warm_up_enabled: bool,
}

impl Default for CompositeApp {
    fn default() -> Self {
        CompositeApp {
            apps: Vec::new(),
            persist_egui_memory: true,
            persist_native_window: true,
            warm_up_enabled: false,
        }
    }
}

impl CompositeApp {
    pub fn add(&mut self, app: Box<dyn App>) {
        self.persist_egui_memory &= app.persist_egui_memory();
        self.persist_native_window &= app.persist_native_window();

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

    fn on_close_event(&mut self) -> bool {
        let mut ret = true;
        for app in &mut self.apps {
            ret &= app.on_close_event();
        }
        ret
    }

    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        for app in &mut self.apps {
            app.on_exit(gl)
        }
    }

    fn auto_save_interval(&self) -> Duration {
        let min = self.apps.iter().map(|app| app.auto_save_interval()).min();
        if let Some(min) = min {
            return min;
        }
        Duration::from_secs(30)
    }

    fn max_size_points(&self) -> Vec2 {
        let mut x: f32 = f32::INFINITY;
        let mut y: f32 = f32::INFINITY;

        for app in &self.apps {
            let pts = app.max_size_points();
            x = x.min(pts.x);
            y = y.min(pts.y);
        }

        Vec2::new(x, y)
    }

    fn persist_native_window(&self) -> bool {
        self.persist_native_window
    }

    fn persist_egui_memory(&self) -> bool {
        self.persist_egui_memory
    }

    fn warm_up_enabled(&self) -> bool {
        self.warm_up_enabled
    }

    fn post_rendering(&mut self, window_size_px: [u32; 2], frame: &Frame) {
        for app in &mut self.apps {
            app.post_rendering(window_size_px, frame)
        }
    }
}
