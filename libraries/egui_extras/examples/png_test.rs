// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use egui::{CentralPanel, Rect, Scene, Ui, Vec2, ViewportBuilder};
use irox_egui_extras::fonts::{load_fonts, FontSet};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame, ToolFrameOptions};
use irox_egui_extras::widgets::image::Image;
use irox_imagery::png::PNGFile;
use irox_tools::irox_bits::BitsWrapper;
use log::{error, Level};

pub fn main() {
    irox_log::init_console_level(Level::Info);
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 1024.));

    let native_options = eframe::NativeOptions {
        viewport,
        // multisampling: 0,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "png-image",
        native_options,
        Box::new(|cc| {
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
    img: Option<Image>,
    rect: Rect,
}
impl TestApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        load_fonts(FontSet::basics(), &cc.egui_ctx);
        let data = include_bytes!("../../imagery/assets/Ap5j5mb.png");
        let png = PNGFile::read_from(&mut BitsWrapper::Owned(&mut data.as_slice()))
            .ok()
            .and_then(|v| v.to_image().ok())
            .map(|v| Image::new("test image", v, &cc.egui_ctx));

        TestApp {
            img: png,
            rect: Rect::ZERO,
        }
    }
}
impl eframe::App for TestApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            if let Some(img) = &self.img {
                Scene::new()
                    .zoom_range(0.0..=f32::INFINITY)
                    .show(ui, &mut self.rect, |ui| {
                        img.show(ui);
                    });
            } else {
                ui.label("Failed to load image");
            }
        });
    }
}

impl ToolApp for TestApp {}
