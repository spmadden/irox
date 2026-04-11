// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use egui::*;
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
        "test-image",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.0);
            // cc.egui_ctx.set_zoom_factor(1. / 1.25);
            cc.egui_ctx.tessellation_options_mut(|v| {
                v.feathering = false;
                v.round_rects_to_pixels = false;
                v.round_line_segments_to_pixels = false;
                v.round_text_to_pixels = false;
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
    init: bool,
    img: Vec<TextureHandle>,
    shapes: Vec<Shape>,
}
impl TestApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        load_fonts(FontSet::basics(), &cc.egui_ctx);
        TestApp {
            init: false,
            img: Default::default(),
            shapes: Vec::new(),
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
        self.shapes = shapes;
    }
}
impl eframe::App for TestApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.init(ui.ctx());
        CentralPanel::default().show_inside(ui, |ui| {
            let (resp, painter) = ui.allocate_painter(Vec2::new(1024., 1024.), Sense::empty());
            let rect = resp.rect;
            for shp in &self.shapes {
                let mut shp = shp.clone();
                shp.translate(rect.min.to_vec2());
                painter.add(shp);
            }
        });
    }
}

impl ToolApp for TestApp {}
