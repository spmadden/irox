// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use eframe::emath::Align;
use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context, Layout, Vec2, ViewportBuilder};
use irox_egui_extras::logplot::{BasicPlot, PlotPoint};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_log::log::error;
use irox_stats::windows::{
    KernelGenerator, SavitskyGolay1DerivOrder2, SavitszkyGolaySmoother23, SavitszkyGolaySmoother45,
};
use std::sync::Arc;

pub fn main() {
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "irox-stats-gallery",
        native_options,
        Box::new(|cc| Ok(Box::new(ToolFrame::new(cc, Box::new(TestApp::new(cc)))))),
    ) {
        error!("{e:?}");
    };
}
pub struct TestApp {
    log_plot: BasicPlot,
}
impl TestApp {
    pub fn new(_cc: &CreationContext) -> Self {
        let mut log_plot = BasicPlot::new(&_cc.egui_ctx);
        let m = 9;
        let windows: &[(&str, Box<dyn KernelGenerator>)] = &[
            ("SG23", Box::new(SavitszkyGolaySmoother23::new(m))),
            ("SG45", Box::new(SavitszkyGolaySmoother45::new(m))),
            ("SG1D2", Box::new(SavitskyGolay1DerivOrder2::new(m))),
        ];
        for (name, wind) in windows {
            let data = log_plot.add_line(|line| {
                line.name = Arc::new((*name).to_string());
                line.line_stroke.width = 2.;
            });
            data.replace_data(Arc::from(
                (-40..=40)
                    .map(|v| PlotPoint::new(v as f64 / 10., wind.get_kernel_value(v as f64 / 10.)))
                    .collect::<Vec<_>>(),
            ));
        }
        Self { log_plot }
    }
}
impl App for TestApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Max), |ui| {
                self.log_plot.show(ui);
            });
        });
    }
}
impl ToolApp for TestApp {}
