// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Dot Viewer
//!

#![forbid(unsafe_code)]

use irox_log::init_console_level;

pub fn main() {
    init_console_level(irox_log::log::Level::Info);
    #[cfg(not(target_arch = "wasm32"))]
    {
        use irox_dot::drawing::app::{App, InitData};
        extern crate alloc;
        use alloc::boxed::Box;
        use egui::{Vec2, ViewportBuilder};
        use irox_egui_extras::egui;
        use irox_egui_extras::toolframe::ToolFrame;
        use irox_log::log::error;

        let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

        let native_options = eframe::NativeOptions {
            viewport,
            multisampling: 0,

            ..Default::default()
        };
        let init = InitData {
            dotjsondata: vec![],
            scale: 0.044,
            pos: [600., 1000.],
        };
        if let Err(e) = eframe::run_native(
            "draw-panels",
            native_options,
            Box::new(|cc| Ok(Box::new(ToolFrame::new(cc, Box::new(App::new(cc, init)))))),
        ) {
            error!("{e:?}");
        };
    }
}
