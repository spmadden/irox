// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use eframe::egui_wgpu::{WgpuConfiguration, WgpuSetup};
use eframe::wgpu::Backends;
use eframe::{wgpu, App, CreationContext, Frame, HardwareAcceleration, Renderer};
use egui::{CentralPanel, Context, ThemePreference, Ui, Vec2, ViewportBuilder};
use irox_egui_extras::logplot::{
    x_axis_time_millis_formatter, y_axis_units_formatter, Axis, AxisAlignmentMode, AxisUserData,
    BasicPlot, ErrorBarsType, LineWithErrorBars, PlotPoint, YAxisSide,
};
use irox_egui_extras::repainting::{RepaintManager, RepaintRequest};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame, ToolFrameOptions};
use irox_stats::windows::{
    BinStatistics, SavitszkyGolaySmoother24Builder, TimeWindow, TimedLinearSlopeFilter,
    TimedWindowFilter, WindowBinStrategy,
};
use irox_time::epoch::UnixTimestamp;
use irox_time::Duration;
use irox_tools::random::PRNG;
use irox_units::quantities::Units;
use log::error;
use std::f64::consts::TAU;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

#[allow(clippy::print_stderr)]
pub fn main() {
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));
    #[cfg(feature = "profiling")]
    {
        let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
        let _puffin_server = puffin_http::Server::new(&server_addr);
        // eprintln!("Run this to view profiling data:  puffin_viewer {server_addr}");
        puffin::set_scopes_on(true);
        let _ = std::process::Command::new("puffin_viewer")
            .arg("--url")
            .arg(server_addr)
            .spawn();
        #[allow(clippy::mem_forget)]
        std::mem::forget(_puffin_server);
    }
    let mut wgpu_options = WgpuConfiguration::default();
    match &mut wgpu_options.wgpu_setup {
        WgpuSetup::CreateNew {
            supported_backends,
            power_preference,
            ..
        } => {
            *supported_backends = Backends::PRIMARY;
            *power_preference = wgpu::PowerPreference::HighPerformance;
        }
        _ => {}
    };
    let native_options = eframe::NativeOptions {
        viewport,
        renderer: Renderer::Wgpu,
        hardware_acceleration: HardwareAcceleration::Preferred,
        // multisampling: 2,
        wgpu_options,
        vsync: true,
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "plotz performance tester",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(DEFAULT_THEME);
            Ok(Box::new(ToolFrame::new_opts(
                cc,
                Box::new(TestApp::new(cc)),
                ToolFrameOptions {
                    show_rendering_stats: true,
                    ..Default::default()
                },
            )))
        }),
    ) {
        error!("{e:?}");
    };
}
const NUM_LINES_PER_PLOT: usize = 1;
const NUM_PLOTS: usize = 1;
const DATA_RATE: Duration = Duration::from_millis(10); // 100 hz data
const MAX_DATA_TO_KEEP: Duration = Duration::from_minutes(5);
const AVERAGING_WINDOW: Duration = Duration::from_seconds_f64(0.5);
const MAX_REPAINT_RATE: Duration = Duration::from_seconds_f64(1. / 5.); // 10hz
const LINE_CTR: f64 = 5e-6;
const LINE_JITTER: f64 = 1e-6;
const LINE_INCR: f64 = 1e-8;
const LINE_EPOCH_CYCLE: Duration = Duration::from_minutes(1);
const LINE_EPOCH_BIAS: f64 = 1e-6;
const DEFAULT_CYCLE: bool = false;
const DEFAULT_THEME: ThemePreference = ThemePreference::Dark;
const DO_TROC: bool = true;
const TROC_OVERLAY: bool = false;

pub struct PlotOpts {
    running: Arc<AtomicBool>,
    handles: Vec<JoinHandle<()>>,
    plots: Vec<BasicPlot>,
}
impl Drop for PlotOpts {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

pub struct TestApp {
    plots: Vec<PlotOpts>,
    running: Arc<AtomicBool>,
    repainter: Option<RepaintManager>,
}
impl TestApp {
    pub fn new(cc: &CreationContext) -> Self {
        let mut plots = Vec::new();
        let running = Arc::new(AtomicBool::new(true));
        cc.egui_ctx.memory_mut(|_mem| {
            // let t = &mut _mem.options.tessellation_options;
            // t.feathering = false;
            // t.feathering_size_in_pixels = 0.0;
        });
        let mut num_lines = NUM_LINES_PER_PLOT;
        let repainter = RepaintManager::new(MAX_REPAINT_RATE, cc.egui_ctx.clone());

        for _pidx in 0..NUM_PLOTS {
            let repainter = repainter.requester();
            plots.push(Self::spawn_thread(
                running.clone(),
                &repainter,
                &cc.egui_ctx,
                num_lines,
            ));
            num_lines /= 2;
            num_lines *= 2;
        }

        Self {
            plots,
            running,
            repainter: Some(repainter),
        }
    }

    fn spawn_thread(
        running: Arc<AtomicBool>,
        repainter: &RepaintRequest,
        ctx: &Context,
        num_lines: usize,
    ) -> PlotOpts {
        let mut data_plot = BasicPlot::new(ctx)
            .with_x_axis_formatter(x_axis_time_millis_formatter())
            .with_y_axis_formatter(y_axis_units_formatter(Units::Volt));
        data_plot.line_highlight_focus_duration = Duration::from_seconds(1);
        data_plot.rotate_line_highlights = DEFAULT_CYCLE;

        let mut troc_plot = BasicPlot::new(ctx)
            .with_x_axis_formatter(x_axis_time_millis_formatter())
            .with_y_axis_formatter(y_axis_units_formatter(Units::Volt));
        troc_plot.line_highlight_focus_duration = Duration::from_hours(1);
        // troc_plot.rotate_line_highlights = DEFAULT_CYCLE;

        let mut handles = Vec::new();
        let mut line_off = LINE_CTR;
        for lidx in 0..num_lines {
            let running = running.clone();

            let (_, error_bars) = data_plot.add_line_with_error_bars(|line| {
                line.set_name(format!("Line {}", lidx + 1));
            });
            let (_, troc_line) = if TROC_OVERLAY {
                data_plot.y_axis_right = Some(Axis::with_configuration(AxisUserData {
                    // alignment_mode: AxisAlignmentMode::CenterOnZeroWithOppsositeRange,
                    alignment_mode: AxisAlignmentMode::CenterOnZero,
                    axis_formatter: Some(y_axis_units_formatter(Units::Volt)),
                    ..Default::default()
                }));
                data_plot.add_line_with_error_bars(|line| {
                    line.set_name(format!("Line {} TROC", lidx + 1));
                    line.yaxis_side = YAxisSide::RightAxis;
                })
            } else {
                // troc_plot.y_axis_left.alignment_mode = AxisAlignmentMode::CenterOnZero;
                troc_plot.add_line_with_error_bars(|line| {
                    line.set_name(format!("Line {} TROC", lidx + 1));
                })
            };
            let repainter = repainter.clone();
            let hndl = std::thread::spawn(move || {
                let mut last_run = UnixTimestamp::now();
                let mut average_filter = TimedWindowFilter::new(
                    AVERAGING_WINDOW,
                    WindowBinStrategy::Center,
                    Box::new(SavitszkyGolaySmoother24Builder),
                );
                let mut average_data = TimeWindow::new(MAX_DATA_TO_KEEP - AVERAGING_WINDOW);
                let mut error_data = BinStatistics::new(AVERAGING_WINDOW);

                let mut troc_filter =
                    TimedLinearSlopeFilter::new(AVERAGING_WINDOW, WindowBinStrategy::Center);
                let mut troc_data = TimeWindow::new(MAX_DATA_TO_KEEP - AVERAGING_WINDOW);
                let mut troc_stddevs = BinStatistics::new(AVERAGING_WINDOW * 20.);

                let mut rnd = irox_tools::random::PcgXshRR::default();

                let now = UnixTimestamp::now();
                {
                    let mut curiter = now - MAX_DATA_TO_KEEP;
                    while curiter <= now {
                        let val = rnd.next_in_distribution(line_off, LINE_JITTER);
                        error_data.insert(curiter, val);
                        if let Some(value) = average_filter.insert(curiter, val) {
                            average_data.add_sample(value);
                        }
                        if DO_TROC {
                            if let Some(value) = troc_filter.insert(curiter, val) {
                                troc_stddevs.insert(value.time, value.value);
                                troc_data.add_sample(value);
                            }
                        }
                        curiter += DATA_RATE;
                    }
                    #[cfg(feature = "dump_csv")]
                    if let Ok(file) = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open("troc.csv")
                    {
                        let mut csv =
                            irox_csv::CSVWriter::new(file).with_dialect(irox_csv::EXCEL_DIALECT);
                        let _ = csv.write_header();
                        troc_data.iter().for_each(|(k, v)| {
                            let _ = csv.write_line(&[
                                format!("{}", k.get_offset().as_millis()),
                                format!("{v}"),
                            ]);
                        })
                    }
                }

                let start_time = now;
                while running.load(Ordering::Relaxed) {
                    let diff = DATA_RATE - last_run.elapsed();
                    std::thread::sleep(diff.into());
                    last_run = UnixTimestamp::now();

                    let olders = last_run - MAX_DATA_TO_KEEP;
                    let epoch = ((last_run - start_time) / LINE_EPOCH_CYCLE).fract() * TAU;
                    let epoch = epoch.sin() * LINE_EPOCH_BIAS;

                    let new_val = rnd.next_in_distribution(line_off + epoch, LINE_JITTER);
                    error_data.insert(last_run, new_val);
                    if let Some(data) = average_filter.insert(last_run, new_val) {
                        average_data.add_sample(data);
                    }
                    if DO_TROC {
                        if let Some(data) = troc_filter.insert(last_run, new_val) {
                            troc_stddevs.insert(data.time, data.value);
                            troc_data.add_sample(data);
                        }
                    }

                    error_data.remove_data_before(olders);

                    let eb_data = error_data
                        .iter()
                        .map(|(_idx, bin)| (bin.start.get_offset().as_millis() as f64, bin.summary))
                        .collect::<Vec<_>>();

                    let line_data = average_data
                        .map_data(|(t, v)| PlotPoint::new(t.get_offset().as_millis() as f64, *v));

                    error_bars.replace_data(LineWithErrorBars {
                        line_data: Arc::from(line_data),
                        error_bars: Arc::from(eb_data),
                        // error_bars_type: ErrorBarsType::StdDev,
                        ..Default::default()
                    });

                    if DO_TROC {
                        troc_stddevs.remove_data_before(olders);
                        let tr_stddev_data = troc_stddevs
                            .iter()
                            .map(|(_idx, bin)| {
                                (bin.start.get_offset().as_millis() as f64, bin.summary)
                            })
                            .collect::<Vec<_>>();
                        let tr_pts = troc_data.map_data(|(t, v)| {
                            PlotPoint::new(t.get_offset().as_millis() as f64, *v)
                        });
                        troc_line.replace_data(LineWithErrorBars {
                            line_data: Arc::from(tr_pts),
                            error_bars: Arc::from(tr_stddev_data),
                            error_bars_type: ErrorBarsType::StdDev,
                        });
                    }
                    repainter.request();
                }
            });
            handles.push(hndl);
            line_off += LINE_INCR;
            // line_off *= -1.;
        }
        if DO_TROC && !TROC_OVERLAY {
            PlotOpts {
                running,
                plots: vec![data_plot, troc_plot],
                handles,
            }
        } else {
            PlotOpts {
                running,
                plots: vec![data_plot],
                handles,
            }
        }
    }
}
impl Drop for TestApp {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(rp) = self.repainter.take() {
            drop(rp);
        }
    }
}

impl App for TestApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let num_plots = self.plots.len();
            let rem = ui.available_size();
            let each = Vec2::new(rem.x / 2., rem.y / num_plots as f32);
            for plot in &mut self.plots {
                ui.horizontal(|ui| {
                    for plot in &mut plot.plots {
                        ui.allocate_ui(each, |ui| {
                            plot.show(ui);
                        });
                    }
                });
            }
        });
    }
}
impl ToolApp for TestApp {
    fn bottom_bar(&mut self, ui: &mut Ui) {
        let mut rotate_highlights = false;
        for plot in &self.plots {
            for plot in &plot.plots {
                rotate_highlights |= plot.rotate_line_highlights;
            }
        }
        if ui
            .checkbox(&mut rotate_highlights, "Cycle highlights")
            .changed()
        {
            for plot in &mut self.plots {
                for plot in &mut plot.plots {
                    plot.rotate_line_highlights = rotate_highlights;
                }
            }
        }
    }
}
