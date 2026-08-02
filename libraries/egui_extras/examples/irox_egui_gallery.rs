// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use eframe::emath::Align;
use eframe::{App, CreationContext, Frame};
use egui::{Align2, CentralPanel, Id, Layout, Ui, Vec2, ViewportBuilder, Window};
use irox_build_rs::BuildEnvironment;
use irox_egui_extras::about::AboutWindow;
use irox_egui_extras::composite::CompositeApp;
#[cfg(feature = "plots")]
use irox_egui_extras::logplot::{BasicPlot, IntoColor32, PlotPoint};
use irox_egui_extras::progressbar::ProgressBar;
#[cfg(feature = "serde")]
use irox_egui_extras::serde::EguiSerializer;
#[cfg(feature = "serde")]
use irox_egui_extras::styles::StylePersistingApp;
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_egui_extras::visuals::VisualsWindow;
use irox_egui_extras::widgets::logger::LoggerWidget;
use irox_egui_extras::widgets::toast::{Toast, ToastState};
use irox_tools::cfg_feature_serde;
use irox_tools::random::PRNG;
use irox_tools::static_init;
use log::{error, Level};
#[cfg(feature = "serde")]
use serde::Serialize;
use std::time::Duration;

static_init!(get_env, BuildEnvironment, {
    irox_build_rs::generate_build_environment().unwrap_or_default()
});

pub fn main() {
    irox_log::init_multi_logger();
    irox_log::init_console_level(Level::Info);
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "irox-egui-gallery",
        native_options,
        Box::new(|cc| {
            let mut comp = CompositeApp::default();
            #[cfg(feature = "serde")]
            comp.add(Box::new(StylePersistingApp::new(cc)));
            comp.add(Box::new(ToolFrame::new(cc, Box::new(TestApp::new(cc)))));
            Ok(Box::new(comp))
        }),
    ) {
        error!("{e:?}");
    };
}
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum AboutTabs {
    #[default]
    Important,
    Grouped,
    All,
}
#[cfg(feature = "plots")]
pub struct Plots {
    log_plot: BasicPlot,
    log_plot2: BasicPlot,
    log_plot3: BasicPlot,
}
#[cfg(feature = "plots")]
impl Plots {
    pub fn new(ctx: &egui::Context) -> Self {
        let mut t;
        let mut pts = Vec::with_capacity(1000);
        let mut pts2 = Vec::with_capacity(1000);
        let mut pts3 = Vec::with_capacity(1000);
        let mut phases: [Vec<PlotPoint>; 5] = [
            Vec::with_capacity(1000),
            Vec::with_capacity(1000),
            Vec::with_capacity(1000),
            Vec::with_capacity(1000),
            Vec::with_capacity(1000),
        ];
        for x in 0..=1000 {
            t = (x as f64 / 1000. * 6. * std::f64::consts::TAU).sin() + 1.;
            pts.push(PlotPoint {
                x: x as f64,
                y: t, // / 1000. + 1000.,
            });
            let phase_offset = 14.;
            let mut p = phase_offset;
            for phase in phases.as_mut() {
                let a = x as f64 + p;
                let y = t;
                phase.push(PlotPoint { x: a, y });
                p += phase_offset;
            }
            pts2.push(PlotPoint {
                x: x as f64,
                y: t / 1000. + 1000.,
            });
            pts3.push(PlotPoint {
                x: x as f64,
                y: t * 100.,
            });
        }
        let mut log_plot = BasicPlot::new(ctx)
            // .with_line("line", Arc::from(pts))
            .with_title("log plot 1")
            .with_x_axis_label("x axis label for 1")
            .with_y_axis_label("y axis label for 1")
            .with_y_axis_formatter(Box::new(|val| format!("{val:.3}")))
            .with_x_axis_formatter(Box::new(|val| format!("{val:.1}")));
        for phase in phases {
            let phase = std::sync::Arc::<[PlotPoint]>::from(phase);
            let data = log_plot.add_line(move |line| {
                line.name = std::sync::Arc::from("phase".to_string());
                line.sample_marker = Some(egui::Shape::circle_filled(
                    egui::Pos2::default(),
                    1.,
                    irox_imagery::Color::rgb_hex(0xFFFFFF).into_color32(),
                ));
            });
            data.replace_data(phase);
        }
        let mut log_plot2 = BasicPlot::new(ctx).with_title("log plot 2");
        log_plot2
            .add_line(|line| {
                line.name = std::sync::Arc::new("line".to_string());
            })
            .replace_data(std::sync::Arc::from(pts2));
        let mut log_plot3 = BasicPlot::new(ctx).with_title("log plot 3");
        log_plot3
            .add_line(|line| {
                line.name = std::sync::Arc::new("line".to_string());
            })
            .replace_data(std::sync::Arc::from(pts3));
        Self {
            log_plot,
            log_plot2,
            log_plot3,
        }
    }
    pub fn show(&mut self, show: &mut bool, ui: &mut Ui) {
        Window::new("test log plot")
            .constrain(true)
            .default_width(500.)
            .open(show)
            .show(ui.ctx(), |ui| {
                self.log_plot.show(ui);
            });
        Window::new("test log plot 2")
            .constrain(true)
            .default_width(500.)
            .open(show)
            .show(ui.ctx(), |ui| {
                self.log_plot2.show(ui);
            });
        Window::new("test log plot 3")
            .constrain(true)
            .default_width(500.)
            .open(show)
            .show(ui.ctx(), |ui| {
                self.log_plot3.show(ui);
            });
    }
}
pub struct TestApp {
    show_bars: bool,
    #[cfg(feature = "serde")]
    show_serde: bool,
    show_visuals: bool,
    #[cfg(feature = "plots")]
    show_plot: bool,
    #[cfg(feature = "plots")]
    plots: Plots,
    show_about: bool,
    about_tabs: AboutTabs,
    logger_widget: LoggerWidget,
    toast_idx: usize,
    toasts: Vec<Toast>,
}
impl TestApp {
    pub fn new(_cc: &CreationContext) -> Self {
        let logger_widget = LoggerWidget::new("irox_egui_gallery_logger", Level::Info);
        std::thread::spawn(|| -> ! {
            let mut rand = irox_tools::random::PcgMcgXslRr::new_seed(0);

            loop {
                let sleep_ms = rand.next_in_range(100., 2500.) as u64;
                std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
                let level = match rand.next_in_range(0., 100.) {
                    x if x < 20. => Level::Error,
                    x if x < 40. => Level::Warn,
                    x if x < 60. => Level::Info,
                    x if x < 80. => Level::Debug,
                    _ => Level::Trace,
                };
                log::log!(
                    level,
                    "{}",
                    irox_app_helpers::commit_messages::get_message()
                );
            }
        });
        TestApp {
            show_bars: false,
            #[cfg(feature = "serde")]
            show_serde: false,
            show_about: false,
            show_visuals: false,
            #[cfg(feature = "plots")]
            show_plot: true,
            #[cfg(feature = "plots")]
            plots: Plots::new(&_cc.egui_ctx),
            about_tabs: Default::default(),
            logger_widget,
            toasts: Vec::new(),
            toast_idx: 0,
        }
    }
}
impl App for TestApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        #[cfg(feature = "serde")]
        if self.show_serde {
            Window::new("test serde")
                .hscroll(true)
                .vscroll(true)
                .open(&mut self.show_serde)
                .show(ui.ctx(), |ui| {
                    let def = BasicStruct::new();

                    let mut ser = EguiSerializer::new();
                    if let Ok(()) = def.serialize(&mut ser) {
                        ser.show(ui);
                    }
                });
        }

        if self.show_bars {
            Window::new("progress bars")
                .constrain(true)
                .default_width(500.)
                .open(&mut self.show_bars)
                .show(ui.ctx(), |ui| {
                    ProgressBar::indeterminate()
                        .text_center("I'm indeterminate!".to_string())
                        .ui(ui);

                    ProgressBar::new(0.5)
                        .text_left("Left text".to_string())
                        .text_center("Center text for a 50% bar".to_string())
                        .text_right("Right text".to_string())
                        .ui(ui);
                });
        }
        if self.show_visuals {
            self.show_visuals = VisualsWindow::show_visuals_window(ui.ctx());
        }
        #[cfg(feature = "plots")]
        if self.show_plot {
            self.plots.show(&mut self.show_plot, ui);
        }
        if self.show_about {
            Window::new("About")
                .constrain(true)
                .default_width(500.)
                .open(&mut self.show_about)
                .show(ui.ctx(), |ui| {
                    // AboutWindow::show_grouped(irox_egui_extras::build::get_GROUPS, ui);
                    ui.horizontal_top(|ui| {
                        ui.radio_value(&mut self.about_tabs, AboutTabs::Important, "Important");
                        ui.radio_value(&mut self.about_tabs, AboutTabs::Grouped, "Grouped");
                        ui.radio_value(&mut self.about_tabs, AboutTabs::All, "All");
                    });
                    let pe = get_env().as_parsed_environment();
                    match self.about_tabs {
                        AboutTabs::Important => {
                            AboutWindow::show_important(|| &pe.all_items, ui);
                        }
                        AboutTabs::Grouped => {
                            AboutWindow::show_grouped(|| &pe.grouped, ui);
                        }
                        AboutTabs::All => {
                            AboutWindow::show(|| &pe.all_items, ui);
                        }
                    }
                });
        }

        Window::new("logger")
            .resizable([true, false])
            .max_height(150.)
            .min_width(800.)
            .resizable(true)
            .anchor(
                Align2::CENTER_BOTTOM,
                [0., -ui.text_style_height(&egui::TextStyle::Body) * 1.25],
            )
            .show(ui.ctx(), |ui| {
                self.logger_widget.show(ui);
            });

        self.toasts.retain(|toast| {
            let state = toast.show(&ui.ctx().clone(), ui);
            state != ToastState::Done
        });

        CentralPanel::default().show_inside(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Max), |ui| {
                ui.toggle_value(&mut self.show_bars, "Progressbars");
                #[cfg(feature = "serde")]
                ui.toggle_value(&mut self.show_serde, "Serde UI");
                ui.toggle_value(&mut self.show_visuals, "Theme Colors");
                #[cfg(feature = "plots")]
                ui.toggle_value(&mut self.show_plot, "Plots");
                ui.toggle_value(&mut self.show_about, "About the Build");

                if ui.button("Toast!").clicked() {
                    let toast = Toast::new(
                        Id::new(format!("Toast_{}", self.toast_idx)),
                        "Test Toast".to_string(),
                        "This is a test toast\nIt may be multi-lined".to_string(),
                        irox_time::Duration::from_seconds(10),
                        ui,
                    );
                    self.toast_idx += 1;
                    self.toasts.insert(0, toast);
                }
            });
        });
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(1)
    }
    fn persist_egui_memory(&self) -> bool {
        true
    }
}

impl ToolApp for TestApp {}

cfg_feature_serde! {
    #[derive(Default, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct TestUnitStruct;

    #[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub enum TestBasicEnum {
        #[default]
        FirstValue,
        SecondValue,
    }

    #[derive(Default, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct BasicNewtype(u32);

    #[derive(Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct BasicTupleStruct(bool, bool, &'static str);
    impl Default for BasicTupleStruct {
        fn default() -> Self {
            BasicTupleStruct(true, false, "tuple test")
        }
    }

    #[derive(Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub enum TupleVariant {
        First(u32),
        Second(bool, u8),
        Third { third_bool: bool, third_u32: u32 },
    }
    impl Default for TupleVariant {
        fn default() -> Self {
            TupleVariant::Second(true, 10)
        }
    }

    #[derive(Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct BasicStruct {
        pub bool_value: bool,
        pub i8_value: i8,
        pub i16_value: i16,
        pub i32_value: i32,
        pub i64_value: i64,
        pub i128_value: i128,
        pub u8_value: u8,
        pub u16_value: u16,
        pub u32_value: u32,
        pub u64_value: u64,
        pub u128_value: u128,

        pub string_value: &'static str,
        pub bytes_value: &'static [u8],

        pub optional_some: Option<&'static str>,
        pub optional_none: Option<&'static str>,

        pub unit_value: (),
        pub unit_struct: TestUnitStruct,

        pub enum_value: TestBasicEnum,
        pub newtype_value: BasicNewtype,

        pub sec_array_value: &'static [TestBasicEnum; 3],
        pub sec_vec_empty_value: Vec<TestBasicEnum>,
        pub sec_vec_some_value: Vec<TestBasicEnum>,

        pub tuple_value: (bool, &'static str, u32),

        pub tuple_struct_value: BasicTupleStruct,

        pub tuple_variant_value: TupleVariant,

        pub map_value: std::collections::BTreeMap<TestBasicEnum, BasicStruct>,

        pub struct_variant: TupleVariant,
    }

    impl BasicStruct {
        pub fn new() -> Self {
            let mut map = std::collections::BTreeMap::new();
            map.insert(TestBasicEnum::FirstValue, BasicStruct::default());

            BasicStruct {
                bool_value: false,
                i8_value: 0,
                i16_value: 0,
                i32_value: 0,
                i64_value: 0,
                i128_value: 0,
                u8_value: 0,
                u16_value: 0,
                u32_value: 0,
                u64_value: 0,
                u128_value: 0,
                string_value: "",
                bytes_value: &[],
                optional_some: None,
                optional_none: None,
                unit_value: (),
                unit_struct: Default::default(),
                enum_value: Default::default(),
                newtype_value: Default::default(),
                sec_array_value: &[
                    TestBasicEnum::FirstValue,
                    TestBasicEnum::SecondValue,
                    TestBasicEnum::FirstValue,
                ],
                sec_vec_empty_value: vec![],
                sec_vec_some_value: vec![TestBasicEnum::SecondValue, TestBasicEnum::FirstValue],
                tuple_value: (true, "tuple string", 10),

                tuple_struct_value: Default::default(),
                tuple_variant_value: Default::default(),

                map_value: map,

                struct_variant: TupleVariant::Third {
                    third_bool: false,
                    third_u32: 128,
                },
            }
        }
    }

    impl Default for BasicStruct {
        fn default() -> Self {
            BasicStruct {
                bool_value: false,
                i8_value: 0,
                i16_value: 0,
                i32_value: 0,
                i64_value: 0,
                i128_value: 0,
                u8_value: 0,
                u16_value: 0,
                u32_value: 0,
                u64_value: 0,
                u128_value: 0,
                string_value: "",
                bytes_value: &[],
                optional_some: None,
                optional_none: None,
                unit_value: (),
                unit_struct: Default::default(),
                enum_value: Default::default(),
                newtype_value: Default::default(),
                sec_array_value: &[
                    TestBasicEnum::SecondValue,
                    TestBasicEnum::FirstValue,
                    TestBasicEnum::FirstValue,
                ],
                sec_vec_empty_value: vec![],
                sec_vec_some_value: vec![],
                tuple_value: (true, "default_tup", 0),
                tuple_struct_value: Default::default(),
                tuple_variant_value: Default::default(),
                map_value: Default::default(),
                struct_variant: Default::default(),
            }
        }
    }
}
