// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use std::sync::Arc;
use std::time::Duration;

use eframe::{self, Frame};
use egui::{CentralPanel, Context};
use log::error;

use irox_egui_extras::composite::CompositeApp;
use irox_egui_extras::logplot::{BasicPlot, PlotPoint};
use irox_egui_extras::styles::StylePersistingApp;
use irox_egui_extras::toolframe::ToolApp;
use irox_stats::Distribution;

use crate::run::Run;

mod run;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions {
        multisampling: 0,
        ..Default::default()
    };

    #[cfg(feature = "profiling")]
    #[allow(clippy::print_stderr)]
    {
        let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
        #[allow(clippy::unwrap_used)]
        let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();
        eprintln!("Run this to view profiling data:  puffin_viewer {server_addr}");
        puffin::set_scopes_on(true);
        let _ = std::process::Command::new("puffin_viewer")
            .arg("--url")
            .arg(server_addr)
            .spawn()
            .ok();
        #[allow(clippy::mem_forget)]
        std::mem::forget(_puffin_server);
    }

    if let Err(e) = eframe::run_native(
        "irox-halflifes",
        native_options,
        Box::new(|cc| {
            let mut comp = CompositeApp::default();
            comp.add(Box::new(StylePersistingApp::new(cc)));
            comp.add(Box::new(irox_egui_extras::toolframe::ToolFrame::new(
                cc,
                Box::new(HalflifesApp::new(cc)),
            )));

            Ok(Box::new(comp))
            // Ok(Box::new(HalflifesApp::new(cc)))
        }),
    ) {
        error!("{e:?}");
    };
}
#[cfg(target_arch = "wasm32")]
fn get_canvas_element_by_id(canvas_id: &str) -> Option<eframe::web_sys::HtmlCanvasElement> {
    use eframe::wasm_bindgen::JsCast;
    let document = eframe::web_sys::window()?.document()?;
    let canvas = document.get_element_by_id(canvas_id)?;
    canvas.dyn_into::<eframe::web_sys::HtmlCanvasElement>().ok()
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    let canvas = get_canvas_element_by_id("the_canvas_id").expect("canvas");
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    // let mut comp = CompositeApp::default();
                    // comp.add(Box::new(StylePersistingApp::new(cc)));
                    Ok(Box::new(HalflifesApp::new(cc)))
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}

struct HalflifesApp {
    plot: BasicPlot,
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

        let mut plot = BasicPlot::new(&_cc.egui_ctx);
        for run in runs.iter().map(|f| {
            f.iter()
                .enumerate()
                .map(|(idx, v)| [idx as f64, *v].into())
                .collect::<Vec<_>>()
        }) {
            let run = Arc::<[PlotPoint]>::from(run);
            let data = plot.add_line(move |line| line.set_name("line"));
            data.replace_data(run);
        }

        HalflifesApp { plot }
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
            self.plot.show(ui);
        });
    }
}

impl ToolApp for HalflifesApp {}
