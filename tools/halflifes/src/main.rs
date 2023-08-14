// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use eframe::{self, Frame, NativeOptions};
use egui::{menu, CentralPanel, Context, Id, TopBottomPanel, Window};

use egui_irox_extras::composite::CompositeApp;
use egui_irox_extras::frame_history::FrameHistory;
use egui_irox_extras::styles::StylePersistingApp;

fn main() {
    let native_options = NativeOptions {
        multisampling: 2,
        ..Default::default()
    };
    eframe::run_native(
        "irox-halflifes",
        native_options,
        Box::new(|cc| {
            let mut comp = CompositeApp::default();
            comp.add(Box::new(StylePersistingApp::new(cc)));
            comp.add(Box::new(HalflifesApp::new(cc)));

            Box::new(comp)
        }),
    )
    .expect("Error running");
}

#[derive(Default)]
struct HalflifesApp {
    style_ui: bool,
    full_speed: bool,

    frame_history: FrameHistory,
}

impl HalflifesApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        HalflifesApp {
            ..Default::default()
        }
    }
}

impl eframe::App for HalflifesApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);

        TopBottomPanel::top(Id::new("top_panel")).show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        frame.close();
                    }
                });
                ui.menu_button("Settings", |ui| {
                    ui.checkbox(&mut self.full_speed, "Continuous Render");

                    if ui.button("Style").clicked() {
                        self.style_ui = true;
                        ui.close_menu();
                    }
                });
            });
        });
        if self.style_ui {
            Window::new("style")
                .open(&mut self.style_ui)
                .show(ctx, |ui| {
                    ctx.style_ui(ui);
                });
        }
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello!");
        });
        TopBottomPanel::bottom(Id::new("bottom_panel")).show(ctx, |ui| {
            self.frame_history.ui(ui);
        });
        if self.full_speed {
            ctx.request_repaint();
        }
    }
}
