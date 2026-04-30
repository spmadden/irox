// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use eframe::emath::Vec2;
use eframe::{App, CreationContext, Frame};
use egui::{Ui, ViewportBuilder};
use irox_egui_extras::composite::CompositeApp;
use irox_egui_extras::toolframe::{ToolApp, ToolFrame, ToolFrameOptions};
use irox_graphing::fdp::magnetic::Magnetic;
use irox_graphing::fdp::Simulation;
use irox_log::log::{error, Level};

#[test]
pub fn test() {
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
                    Box::new(ForcesApp::new(cc)),
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
}

pub struct ForcesApp {}
impl ForcesApp {
    pub fn new(cc: &CreationContext) -> Self {
        Self {}
    }
}
impl ToolApp for ForcesApp {}
impl App for ForcesApp {
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        let f = Magnetic {
            iterations: 1,
            strength: 1.0,
            field_angles: vec![],
        };

        // Simulation::new();
    }
}
