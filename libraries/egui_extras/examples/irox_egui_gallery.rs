// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::collections::BTreeMap;
use std::f64::consts::TAU;
use std::sync::Arc;

use eframe::emath::Align;
use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context, Layout, Vec2, ViewportBuilder, Widget, Window};
use egui_plot::PlotPoint;
use irox_egui_extras::about::AboutWindow;
use irox_egui_extras::logplot::BasicPlot;
use irox_egui_extras::progressbar::ProgressBar;
use irox_egui_extras::serde::EguiSerializer;
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_egui_extras::visuals::VisualsWindow;
use log::error;
use serde::Serialize;

pub fn main() {
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "irox-egui-gallery",
        native_options,
        Box::new(|cc| Ok(Box::new(ToolFrame::new(cc, Box::new(TestApp::new(cc)))))),
    ) {
        error!("{e:?}");
    };
}

pub struct TestApp {
    log_plot: BasicPlot,
    log_plot2: BasicPlot,
    log_plot3: BasicPlot,
    show_bars: bool,
    show_serde: bool,
    show_visuals: bool,
    show_plot: bool,
    show_about: bool,
}
impl TestApp {
    pub fn new(_cc: &CreationContext) -> Self {
        let mut t;
        let mut pts = Vec::with_capacity(1000);
        let mut pts2 = Vec::with_capacity(1000);
        let mut pts3 = Vec::with_capacity(1000);
        for x in 0..=1000 {
            t = (x as f64 / 1000. * 6. * TAU).sin() + 1.;
            pts.push(PlotPoint {
                x: x as f64,
                y: t, // / 1000. + 1000.,
            });
            pts2.push(PlotPoint {
                x: x as f64,
                y: t / 1000. + 1000.,
            });
            pts3.push(PlotPoint {
                x: x as f64,
                y: t * 100.,
            });
        }
        let log_plot = BasicPlot::new(Arc::new(pts))
            .with_title("log plot 1")
            .with_x_axis_label("x axis label for 1")
            .with_y_axis_label("y axis label for 1");
        let log_plot2 = BasicPlot::new(Arc::new(pts2)).with_title("log plot 2");
        let log_plot3 = BasicPlot::new(Arc::new(pts3)).with_title("log plot 3");

        TestApp {
            log_plot,
            log_plot2,
            log_plot3,
            show_bars: false,
            show_serde: false,
            show_about: false,
            show_visuals: false,
            show_plot: true,
        }
    }
}
impl App for TestApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.show_serde {
            Window::new("test serde")
                .hscroll(true)
                .vscroll(true)
                .open(&mut self.show_serde)
                .show(ctx, |ui| {
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
                .show(ctx, |ui| {
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
            self.show_visuals = VisualsWindow::show_visuals_window(ctx);
        }
        if self.show_plot {
            Window::new("test log plot")
                .constrain(true)
                .default_width(500.)
                .open(&mut self.show_plot)
                .show(ctx, |ui| {
                    self.log_plot.show(ui);
                });
            Window::new("test log plot 2")
                .constrain(true)
                .default_width(500.)
                .open(&mut self.show_plot)
                .show(ctx, |ui| {
                    self.log_plot2.show(ui);
                });
            Window::new("test log plot 3")
                .constrain(true)
                .default_width(500.)
                .open(&mut self.show_plot)
                .show(ctx, |ui| {
                    self.log_plot3.show(ui);
                });
        }
        if self.show_about {
            Window::new("About")
                .constrain(true)
                .default_width(500.)
                .open(&mut self.show_about)
                .show(ctx, |ui| {
                    AboutWindow::show_grouped(irox_egui_extras::build::get_GROUPS, ui);
                });
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Max), |ui| {
                ui.toggle_value(&mut self.show_bars, "Progressbars");
                ui.toggle_value(&mut self.show_serde, "Serde UI");
                ui.toggle_value(&mut self.show_visuals, "Theme Colors");
                ui.toggle_value(&mut self.show_plot, "Plots");
                ui.toggle_value(&mut self.show_about, "About the Build");
            });
        });
    }
}

impl ToolApp for TestApp {}

#[derive(Default, Debug, Serialize)]
pub struct TestUnitStruct;

#[derive(Default, Debug, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum TestBasicEnum {
    #[default]
    FirstValue,
    SecondValue,
}

#[derive(Default, Debug, Serialize)]
pub struct BasicNewtype(u32);

#[derive(Debug, Serialize)]
pub struct BasicTupleStruct(bool, bool, &'static str);
impl Default for BasicTupleStruct {
    fn default() -> Self {
        BasicTupleStruct(true, false, "tuple test")
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

    pub map_value: BTreeMap<TestBasicEnum, BasicStruct>,

    pub struct_variant: TupleVariant,
}

impl BasicStruct {
    pub fn new() -> Self {
        let mut map = BTreeMap::new();
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
