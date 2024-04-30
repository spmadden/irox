// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use egui_plot::{PlotPoint, PlotPoints};
use std::cmp::min;
use std::sync::Arc;

#[derive(Default)]
pub struct LogPlot {
    pub data: PlotPoints,
    pub name: Arc<String>,
    pub x_axis: Axis,
    pub y_axis: Axis,
}

impl LogPlot {
    pub fn new<T: Into<PlotPoints>>(data: T) -> LogPlot {
        LogPlot {
            data: data.into(),
            ..Default::default()
        }
    }
    pub fn show(&mut self, ui: &mut Ui) {
        let size = ui.available_size();

        let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        response.context_menu(|ui| {
            ui.selectable_value(&mut self.y_axis.scale_mode, ScaleMode::Linear, "Y-Linear");
            ui.selectable_value(&mut self.y_axis.scale_mode, ScaleMode::Log10, "Y-Log10");
        });
        let rect = response.rect;
        let width = rect.width();
        let height = rect.height();

        let y_axis_x_offset = rect.min.x + width * 0.1;
        let y_axis_y_min = rect.min.y + height * 0.1;
        let y_axis_y_max = rect.min.y + height * 0.9;
        let x_axis_x_min = y_axis_x_offset;
        let x_axis_x_max = rect.min.x + width * 0.9;
        let x_axis_y_offset = y_axis_y_max;

        self.x_axis.screen_origin = x_axis_x_min;
        self.x_axis.screen_range = x_axis_x_max - x_axis_x_min;
        self.x_axis.incr_sign = 1.0;
        self.y_axis.screen_origin = y_axis_y_min;
        self.y_axis.screen_range = y_axis_y_max - y_axis_y_min;
        self.y_axis.incr_sign = -1.0;

        let color = Color32::from_gray(128);
        let color2 = Color32::from_gray(192);
        let stroke = Stroke::new(2.0, color);
        let stroke2 = Stroke::new(1.0, color2);

        let r = rect.width() / 2.0 - 1.0;
        painter.line_segment(
            [
                Pos2::new(y_axis_x_offset, y_axis_y_min),
                Pos2::new(y_axis_x_offset, y_axis_y_max),
            ],
            stroke,
        );
        painter.line_segment(
            [
                Pos2::new(x_axis_x_min, x_axis_y_offset),
                Pos2::new(x_axis_x_max, x_axis_y_offset),
            ],
            stroke,
        );

        let points = self.data.points();
        self.x_axis.update_range(points, |p| p.x);
        self.y_axis.update_range(points, |p| p.y);

        for pnt in points.windows(2) {
            let Some(first) = pnt.get(0) else {
                continue;
            };
            let Some(second) = pnt.get(1) else {
                continue;
            };
            let first = self.scale_point(first);
            let second = self.scale_point(second);
            painter.line_segment([first, second], stroke);
        }

        for detent in &self.x_axis.detents {
            let pos = Pos2 {
                x: detent.0,
                y: x_axis_y_offset + 2.,
            };
            let anchor = Align2::CENTER_TOP;
            let font_id = FontId::proportional(10.);
            painter.text(pos, anchor, &detent.1, font_id, color);
            painter.line_segment(
                [
                    Pos2 {
                        x: detent.0,
                        y: y_axis_y_min,
                    },
                    Pos2 {
                        x: detent.0,
                        y: y_axis_y_max,
                    },
                ],
                stroke2,
            );
        }
        for detent in &self.y_axis.detents {
            let pos = Pos2 {
                x: y_axis_x_offset - 5.,
                y: detent.0,
            };
            let anchor = Align2::RIGHT_CENTER;
            let font_id = FontId::proportional(10.);
            painter.text(pos, anchor, &detent.1, font_id, color);
            painter.line_segment(
                [
                    Pos2 {
                        x: x_axis_x_min,
                        y: detent.0,
                    },
                    Pos2 {
                        x: x_axis_x_max,
                        y: detent.0,
                    },
                ],
                stroke2,
            );
        }
        // painter.circle_stroke(c, r, stroke);
        // painter.line_segment([c - vec2(0.0, r), c + vec2(0.0, r)], stroke);
        // painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
        // painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);
    }

    fn scale_point(&self, pos: &PlotPoint) -> Pos2 {
        Pos2 {
            x: self.x_axis.scale_value(pos.x),
            y: self.y_axis.scale_value(pos.y),
        }
    }
}
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ScaleMode {
    #[default]
    Linear,
    Log10,
}
#[derive(Default)]
pub struct Axis {
    pub name: Arc<String>,
    pub min_val: f64,
    pub max_val: f64,
    pub scale_mode: ScaleMode,
    pub incr_sign: f64,
    pub range: f64,
    pub screen_origin: f32,
    pub screen_range: f32,
    pub model_per_point: f64,
    pub detents: Vec<(f32, String)>,
}

impl Axis {
    pub fn update_range<F: Fn(&PlotPoint) -> f64>(&mut self, vals: &[PlotPoint], accessor: F) {
        self.min_val = f64::INFINITY;
        self.max_val = f64::NEG_INFINITY;

        for val in vals {
            let v = match self.scale_mode {
                ScaleMode::Linear => accessor(val),
                ScaleMode::Log10 => accessor(val),
            };
            self.min_val = self.min_val.min(v);
            self.max_val = self.max_val.max(v);
        }

        self.range = self.max_val - self.min_val;
        self.model_per_point = self.range / (self.screen_range as f64);
        let base_step_size = self.model_per_point * 5.0f64;

        let incr = 10f64.powi(base_step_size.abs().log10().ceil() as i32);
        let min_detent;
        let max_detent;
        if incr < 0.0 {
            min_detent = (self.min_val * incr).round() / incr;
            max_detent = (self.max_val * incr).round() / incr;
        } else {
            min_detent = (self.min_val / incr).round() * incr;
            max_detent = (self.max_val / incr).round() * incr;
        }

        let range = max_detent - min_detent;
        self.detents = Vec::with_capacity(10);
        for idx in 0..=10 {
            let frac = idx as f64 / 10f64;
            let dv = match self.scale_mode {
                ScaleMode::Linear => min_detent + range * frac,
                ScaleMode::Log10 => 10f64.powf(min_detent + range * frac),
            };
            let drawpnt = self.model_to_screen(dv);
            // let dv = match self.scale_mode {
            //     ScaleMode::Linear => dv,
            //     ScaleMode::Log10 => 10f64.powf(dv)
            // };
            self.detents.push((drawpnt, format!("{dv:4.2}")));
        }
    }

    pub fn linear_scale(&self, val: f64) -> f32 {
        ((val - self.min_val) / self.range) as f32
    }

    pub fn log_scale(&self, val: f64) -> f64 {
        let min_val = self.min_val.log10();
        let max_val = self.max_val.log10();
        let frac = (val.log10() - min_val) / (max_val - min_val);
        max_val.powf(frac) * min_val.powf(1. - frac)
    }

    pub fn scale_value(&self, val: f64) -> f32 {
        let scaled = match self.scale_mode {
            ScaleMode::Linear => self.model_to_screen(val),
            ScaleMode::Log10 => self.model_to_screen(val.log10()),
        };
        scaled
    }

    pub fn model_to_screen(&self, val: f64) -> f32 {
        let scaled = match self.scale_mode {
            ScaleMode::Linear => self.linear_scale(val),
            ScaleMode::Log10 => self.log_scale(val) as f32,
        };
        let scaled = scaled * self.screen_range;

        if self.incr_sign < 0.0 {
            self.screen_range + self.screen_origin - scaled
        } else {
            scaled + self.screen_origin
        }
    }
}
