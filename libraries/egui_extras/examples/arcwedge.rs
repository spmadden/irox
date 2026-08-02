// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use eframe::emath::Vec2;
use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Grid, Id, Slider, Ui, ViewportBuilder, Widget};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_egui_extras::widgets::arcwedge::{ArcWedge, ArcWedgeSet};
use irox_imagery::colormaps::DIVERGENT_19;
use irox_tools::iterators::Itertools;
use irox_units::units::angle::Angle;
use log::{error, Level};

pub fn main() {
    irox_log::init_console_level(Level::Info);
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "irox-egui-gallery-arcwedge",
        native_options,
        Box::new(|cc| {
            let comp = Box::new(ToolFrame::new(cc, Box::new(TestApp::new(cc))));
            Ok(comp)
        }),
    ) {
        error!("{e:?}");
    };
}

pub struct TestApp {
    pub start_angle: f32,
    pub end_angle: f32,
    pub pad_angle: f32,
    pub inner_length: f32,
    pub outer_length: f32,
    pub pad_length: f32,
    pub size: f32,
}
impl TestApp {
    pub fn new(_cc: &CreationContext) -> Self {
        TestApp {
            start_angle: 0.0,
            end_angle: 45.0,
            pad_angle: 1.0,
            inner_length: 150.0,
            outer_length: 200.0,
            pad_length: 1.0,
            size: 450.0,
        }
    }
}

impl App for TestApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            Grid::new("arcwedge_grid").show(ui, |ui| {
                Slider::new(&mut self.start_angle, 0.0..=360.0)
                    .text("Start Angle")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.end_angle, self.start_angle..=360.0)
                    .text("End Angle")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.pad_angle, 0.0..=5.0)
                    .text("Pad Angle")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.inner_length, 0.0..=self.size)
                    .text("Inner Length")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.outer_length, self.inner_length..=self.size)
                    .text("Outer Length")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.pad_length, 0.0..=5.0)
                    .text("Pad Length")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.size, 1.0..=1024.0)
                    .text("Size")
                    .ui(ui);
                ui.end_row();
            });
            let mut wedgeset = ArcWedgeSet {
                identifier: Id::new("wedge1"),
                size: self.size,
                wedges: vec![],
            };
            let width = self.end_angle - self.start_angle;
            let count = (360. / width).floor() as usize;
            let set = irox_imagery::colormaps::flat::FLAT;
            let bold = 5;
            let light = 1;
            for (i, idx) in DIVERGENT_19
                .iter()
                .looping_forever()
                .take(count)
                .enumerate()
            {
                let start = self.start_angle + width * (i as f32);
                let end = self.start_angle + width * (i as f32 + 1.);
                let light = set
                    .get(*idx)
                    .and_then(|v| v.get(light))
                    .copied()
                    .unwrap_or_default();
                let bold = set
                    .get(*idx)
                    .and_then(|v| v.get(bold))
                    .copied()
                    .unwrap_or_default();
                let wedge = ArcWedge {
                    identifier: Id::new(format!("wedge1_{i}")),
                    start_angle: Angle::new_degrees(start as f64),
                    end_angle: Angle::new_degrees(end as f64),
                    pad_angle: Angle::new_degrees(self.pad_angle as f64),
                    inner_length: self.inner_length,
                    outer_length: self.outer_length,
                    pad_length: self.pad_length,
                    fill_color: light.into(),
                    hovered_fill_color: bold.into(),
                    stroke_color: bold.into(),
                };
                wedgeset.wedges.push(wedge)
            }
            wedgeset.show(ui);
        });
    }
}
impl ToolApp for TestApp {}
