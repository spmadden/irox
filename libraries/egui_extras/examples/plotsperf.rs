// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context, Vec2, ViewportBuilder};
use irox_egui_extras::logplot::{
    x_axis_time_millis_formatter, y_axis_units_formatter, BasicPlot, PlotPoint,
};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_stats::windows::{
    SavitszkyGolaySmoother24Builder, TimeWindow, TimedWindowFilter, WindowBinStrategy,
};
use irox_time::epoch::UnixTimestamp;
use irox_time::Duration;
use irox_tools::random::PRNG;
use irox_units::quantities::Units;
use log::error;
use std::collections::BTreeMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::JoinHandle;

pub fn main() {
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "plotz performance tester",
        native_options,
        Box::new(|cc| Ok(Box::new(ToolFrame::new(cc, Box::new(TestApp::new(cc)))))),
    ) {
        error!("{e:?}");
    };
}
const NUM_LINES_PER_PLOT: usize = 16;
const NUM_PLOTS: usize = 3;
const DATA_RATE: Duration = Duration::from_millis(10); // 100 hz data
const MAX_DATA_TO_KEEP: Duration = Duration::from_minutes(1);
const AVERAGING_WINDOW: Duration = Duration::from_seconds(1);
const LINE_CTR: f64 = 5e-6;
const LINE_JITTER: f64 = 1e-6;
const LINE_INCR: f64 = 1e-7;

pub struct PlotOpts {
    running: Arc<AtomicBool>,
    handles: Vec<JoinHandle<()>>,
    plot: BasicPlot,
}
impl Drop for PlotOpts {
    fn drop(&mut self) {
        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

pub struct TestApp {
    plots: Vec<PlotOpts>,
    running: Arc<AtomicBool>,
}
impl TestApp {
    pub fn new(cc: &CreationContext) -> Self {
        let mut plots = Vec::new();
        let running = Arc::new(AtomicBool::new(true));
        cc.egui_ctx.memory_mut(|mem| {
            let _t = &mut mem.options.tessellation_options;
            // t.feathering = false;
            // t.feathering_size_in_pixels = 0.0;
        });
        for _pidx in 0..NUM_PLOTS {
            plots.push(Self::spawn_thread(running.clone(), &cc.egui_ctx));
        }

        Self { plots, running }
    }

    fn spawn_thread(running: Arc<AtomicBool>, ctx: &Context) -> PlotOpts {
        let mut plot = BasicPlot::new(ctx)
            .with_x_axis_formatter(x_axis_time_millis_formatter())
            .with_y_axis_formatter(y_axis_units_formatter(Units::Volt));
        let mut handles = Vec::new();
        let mut line_off = LINE_CTR;
        for lidx in 0..NUM_LINES_PER_PLOT {
            let running = running.clone();

            let linedata = plot.add_line(|line| {
                line.set_name(format!("Line {}", lidx + 1));
            });
            let average_line = plot.add_line(|line| {
                line.set_name(format!("Average {}", lidx + 1));
            });
            let ctx = ctx.clone();
            let hndl = std::thread::spawn(move || {
                let mut last_run = UnixTimestamp::now();
                let mut data = BTreeMap::<UnixTimestamp, PlotPoint>::new();
                let mut average_filter = TimedWindowFilter::new(
                    AVERAGING_WINDOW,
                    WindowBinStrategy::Center,
                    Box::new(SavitszkyGolaySmoother24Builder),
                );
                let mut average_data = TimeWindow::new(MAX_DATA_TO_KEEP - AVERAGING_WINDOW / 2.);
                let mut rnd = irox_tools::random::PcgXshRR::default();

                let now = UnixTimestamp::now();
                let mut curiter = now - MAX_DATA_TO_KEEP;
                while curiter <= now {
                    let val = rnd.next_in_distribution(line_off, LINE_JITTER);
                    data.insert(
                        curiter,
                        PlotPoint::new(curiter.get_offset().as_millis() as f64, val),
                    );
                    if let Some(value) = average_filter.insert(curiter, val) {
                        average_data.add_sample(value);
                    }
                    curiter += DATA_RATE;
                }

                while running.load(std::sync::atomic::Ordering::Relaxed) {
                    let diff = DATA_RATE - last_run.elapsed();
                    std::thread::sleep(diff.into());
                    last_run = UnixTimestamp::now();

                    let new_val = rnd.next_in_distribution(line_off, LINE_JITTER);
                    data.insert(
                        last_run,
                        PlotPoint::new(last_run.get_offset().as_millis() as f64, new_val),
                    );
                    if let Some(data) = average_filter.insert(last_run, new_val) {
                        average_data.add_sample(data);
                    }
                    let olders = last_run - MAX_DATA_TO_KEEP;
                    data = data.split_off(&olders);
                    let v = data.values().copied().collect::<Vec<_>>();
                    linedata.replace_data(Arc::from(v));

                    average_line.replace_data(Arc::from(
                        average_data.map_data(|(t, v)| {
                            PlotPoint::new(t.get_offset().as_millis() as f64, *v)
                        }),
                    ));
                    ctx.request_repaint();
                }
            });
            handles.push(hndl);
            line_off += LINE_INCR;
            // line_off *= -1.;
        }
        PlotOpts {
            running,
            plot,
            handles,
        }
    }
}
impl Drop for TestApp {
    fn drop(&mut self) {
        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
    }
}

impl App for TestApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let num_plots = self.plots.len();
            let rem = ui.available_size();
            let each = Vec2::new(rem.x, rem.y / num_plots as f32);
            for plot in &mut self.plots {
                ui.allocate_ui(each, |ui| {
                    plot.plot.show(ui);
                });
            }
        });
    }
}
impl ToolApp for TestApp {}
