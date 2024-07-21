// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::time::Duration;

use eframe::{self, Frame};
use egui::{CentralPanel, Context};
use egui_plot::{Line, Plot, PlotPoints};
use log::error;

use irox_egui_extras::composite::CompositeApp;
use irox_egui_extras::styles::StylePersistingApp;
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_stats::Distribution;

use crate::run::Run;

mod run;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions {
        multisampling: 0,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "irox-halflifes",
        native_options,
        Box::new(|cc| {
            let mut comp = CompositeApp::default();
            comp.add(Box::new(StylePersistingApp::new(cc)));
            comp.add(Box::new(ToolFrame::new(
                cc,
                Box::new(HalflifesApp::new(cc)),
            )));

            Ok(Box::new(comp))
        }),
    ) {
        error!("{e:?}");
    };
}
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| {
                    let mut comp = CompositeApp::default();
                    comp.add(Box::new(StylePersistingApp::new(cc)));
                    comp.add(Box::new(HalflifesApp::new(cc)));

                    Box::new(comp)
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}

struct HalflifesApp {
    data: Vec<Vec<[f64; 2]>>,
}

impl HalflifesApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let run = Run::new(
            Duration::from_secs(10000),
            Duration::from_secs(1000).into(),
            Duration::from_secs(300),
        );
        let mut runs: Vec<Vec<f64>> = vec![
            run.run_data(20.0, Duration::from_secs(0)),
            run.run_data(20.0, Duration::from_secs(400)),
            run.run_data(20.0, Duration::from_secs(800)),
        ];
        let mut combined: Vec<f64> = Vec::from([0_f64; 10000]);
        combined.iter_mut().enumerate().for_each(|(idx, v)| {
            for x in &runs {
                if let Some(val) = x.get(idx) {
                    *v += *val;
                } else {
                    error!("BAD IDX: {idx}");
                }
            }
        });
        runs.push(combined);

        let data = runs
            .iter()
            .map(|f| {
                f.iter()
                    .enumerate()
                    .map(|(idx, v)| [idx as f64, *v])
                    .collect()
            })
            .collect();

        HalflifesApp { data }
    }
}

fn _generate_profile_for_start_time(start_time: i32) -> [Vec<[f64; 2]>; 2] {
    let std = irox_stats::standard::StandardDistribution::new(1.5, 1.5 / 3.0);
    let mut absorbed: Vec<[f64; 2]> = Vec::new();
    let mut present: Vec<[f64; 2]> = Vec::new();
    let mut sum1 = 0.0;
    let tau: f64 = 2.0_f64.ln() / 6.0;
    for i in 0..1000 {
        let x = i as f64 * 0.1;
        sum1 += std.pdf(x - 0.05) * 0.1 * 20.0;
        let hl = std::f64::consts::E.powf(-x * tau);
        let x = x + start_time as f64 / 10.;
        absorbed.push([x, sum1]);
        present.push([x, hl * sum1]);
    }
    [absorbed, present]
}

impl eframe::App for HalflifesApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // ui.add(egui::Slider::new(&mut self.first, 0.0..=10.0).text("first"));
            // ui.add(egui::Slider::new(&mut self.second, 0.0..=10.0).text("second"));
            Plot::new("my_plot")
                // .view_aspect(2.0)
                // .allow_drag(true)
                // .allow_scroll(true)
                .allow_boxed_zoom(true)
                // .allow_double_click_reset(true)
                // .allow_zoom(AxisBools { x: true, y: false })
                // .data_aspect(1.0)
                // .center_y_axis(true)
                // .height(1.0)
                .show(ui, |plot_ui| {
                    for data in &self.data {
                        plot_ui.line(Line::new(PlotPoints::new(data.clone())));
                    }
                });
        });
    }
}

impl ToolApp for HalflifesApp {}
