// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use eframe::{self, Frame, NativeOptions};
use egui::plot::{AxisBools, Line, Plot, PlotPoints};
use egui::{menu, CentralPanel, Context, Id, TopBottomPanel, Window};

use irox_egui_extras::composite::CompositeApp;
use irox_egui_extras::frame_history::FrameHistory;
use irox_egui_extras::styles::StylePersistingApp;
use irox_stats::Distribution;

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

struct HalflifesApp {
    style_ui: bool,
    full_speed: bool,
    line: Vec<[f64; 2]>,

    frame_history: FrameHistory,
}

impl HalflifesApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let std = irox_stats::standard::StandardDistribution::default();
        let line = (-1000..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x, std.pdf(x)]
            })
            .collect();
        HalflifesApp {
            style_ui: false,
            full_speed: false,
            frame_history: FrameHistory::default(),
            line,
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

            let pts: PlotPoints = self.line.clone().into();
            let line = Line::new(pts);

            Plot::new("my_plot")
                // .view_aspect(2.0)
                // .allow_drag(false)
                // .allow_scroll(true)
                .allow_boxed_zoom(false)
                .allow_zoom(AxisBools { x: true, y: false })
                .center_y_axis(true)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        });
        TopBottomPanel::bottom(Id::new("bottom_panel")).show(ctx, |ui| {
            self.frame_history.ui(ui);
        });
        if self.full_speed {
            ctx.request_repaint();
        }
    }
}
