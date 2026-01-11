// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use egui::*;
use irox_egui_extras::drawpanel::{DrawPanel, LayerCommand, LayerOpts};
use irox_egui_extras::fonts::{load_fonts, FontSet};
use irox_egui_extras::testimage::TestImage;
use irox_egui_extras::toolframe::{ToolApp, ToolFrame, ToolFrameOptions};
use log::{error, Level};

pub fn main() {
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
                Box::new(TestApp::new(cc)),
                ToolFrameOptions {
                    show_rendering_stats: true,
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
}
pub struct TestApp {
    panel: DrawPanel,
    init: bool,
    img: Vec<TextureHandle>,
}
impl TestApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        load_fonts(FontSet::all(), &cc.egui_ctx);
        TestApp {
            panel: DrawPanel::default(),
            init: false,
            img: Default::default(),
        }
    }
    pub fn init(&mut self, ctx: &Context) {
        if self.init {
            return;
        }
        self.init = true;
        let TestImage {
            shapes, handles, ..
        } = TestImage::new(ctx);
        self.img = handles;
        let sender = self
            .panel
            .add_layer("test".to_string(), LayerOpts::default());
        let _ = sender.send(LayerCommand::ClearSetShapes(shapes));
    }
}
impl eframe::App for TestApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.init(ctx);
        CentralPanel::default().show(ctx, |ui| {
            self.panel.show(ui);
        });
    }
}

impl ToolApp for TestApp {}
