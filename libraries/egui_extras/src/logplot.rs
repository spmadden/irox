// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Log plotting widgets.

use egui::epaint::TextShape;
use egui::{
    pos2, Align, Align2, Color32, Painter, Pos2, Rect, Response, Rounding, Sense, Shape, Stroke,
    TextStyle, Ui, Vec2,
};
use egui_plot::PlotPoint;
use std::fmt::{Display, Formatter, LowerExp};
use std::sync::Arc;

#[derive(Default)]
pub struct PlotInteraction {
    pub drag_started_pos: Option<Pos2>,
    pub drag_ended_delta: Option<Vec2>,
    pub zoom_area: Option<Rect>,
}
impl PlotInteraction {
    pub fn clear(&mut self) {
        self.drag_ended_delta = None;
        self.drag_started_pos = None;
        self.zoom_area = None;
    }
    pub fn update(&mut self, response: &mut Response, painter: &mut Painter) {
        if response.drag_started() {
            self.drag_started_pos = response.interact_pointer_pos();
        } else if response.drag_stopped() {
            if let Some(start) = self.drag_started_pos {
                let first = start.x;
                if let Some(delta) = self.drag_ended_delta {
                    let second = start.x + delta.x;

                    let overlay_rect = Rect {
                        min: pos2(first.min(second), -f32::INFINITY),
                        max: pos2(first.max(second), f32::INFINITY),
                    };
                    self.clear();
                    self.zoom_area = Some(overlay_rect);
                }
            }
        } else if response.is_pointer_button_down_on() {
            let new_delt = response.drag_delta();
            let delta = self.drag_ended_delta.get_or_insert(Vec2::default());
            delta.x += new_delt.x;
            delta.y += new_delt.y;
            if let Some(start) = self.drag_started_pos {
                let first = start.x;
                let second = start.x + delta.x;

                let overlay_rect = Rect {
                    min: pos2(first.min(second), -f32::INFINITY),
                    max: pos2(first.max(second), f32::INFINITY),
                };
                let _shp = painter.rect_filled(
                    overlay_rect,
                    Rounding::ZERO,
                    Color32::from_black_alpha(64),
                );
            }
            // println!("drag delta: {:#?}", self.interaction.drag_ended_delta);
        }
    }
}

///
/// Basic plot, with ability to switch between linear and log axes.
#[derive(Default)]
pub struct BasicPlot {
    pub data: Arc<Vec<PlotPoint>>,
    pub name: Arc<String>,
    pub x_axis: Axis,
    pub y_axis: Axis,
    pub interaction: PlotInteraction,
    pub title: Option<String>,
}

impl BasicPlot {
    pub fn new(data: Arc<Vec<PlotPoint>>) -> BasicPlot {
        BasicPlot {
            data,
            ..Default::default()
        }
    }
    #[must_use]
    pub fn with_title<T: AsRef<str>>(mut self, title: T) -> Self {
        self.title = Some(title.as_ref().to_string());
        self
    }
    #[must_use]
    pub fn with_x_axis_label<T: AsRef<str>>(mut self, title: T) -> Self {
        self.x_axis.axis_label = Some(title.as_ref().to_string());
        self
    }
    #[must_use]
    pub fn with_y_axis_label<T: AsRef<str>>(mut self, title: T) -> Self {
        self.y_axis.axis_label = Some(title.as_ref().to_string());
        self
    }
    fn check_zoom(&mut self, ui: &mut Ui, response: &mut Response) {
        if let Some(area) = self.interaction.zoom_area.take() {
            let min_x = self.x_axis.unscale_value(area.min.x);
            let max_x = self.x_axis.unscale_value(area.max.x);
            self.x_axis.zoomed_range = Some((min_x, max_x));
        }
        let (zd, mousepos) = ui.input(|i| (i.zoom_delta(), i.pointer.latest_pos()));
        if (zd - 1.0).abs() != 0.0 {
            let Some(mousepos) = mousepos else {
                return;
            };
            if response.rect.contains(mousepos) {
                let zmouse = self.x_axis.screen_to_model(mousepos.x);
                let (zrmin, zrmax) = self
                    .x_axis
                    .zoomed_range
                    .get_or_insert((self.x_axis.min_val, self.x_axis.max_val));
                let zrange = *zrmax - *zrmin;
                let zmousepct = (zmouse - *zrmin) / zrange;
                let new_range = zrange / zd as f64;
                let min = zmouse - new_range * zmousepct;
                let max = zmouse + new_range * (1. - zmousepct);
                self.x_axis.zoomed_range = Some((min, max));
            }
        }
        if response.double_clicked() {
            self.x_axis.zoomed_range = None;
        }
    }
    pub fn show(&mut self, ui: &mut Ui) {
        let major_stroke = Stroke::new(2.0, ui.visuals().widgets.inactive.fg_stroke.color);
        let minor_stroke = Stroke::new(1.0, ui.visuals().widgets.open.bg_stroke.color);
        let caution_color = ui.visuals().warn_fg_color;
        let small_font = TextStyle::Small.resolve(ui.style());
        let large_font = TextStyle::Heading.resolve(ui.style());

        let size = ui.available_size();
        let mut draw_log_warning =
            self.y_axis.draw_log_clip_warning || self.x_axis.draw_log_clip_warning;

        let (mut response, mut painter) = ui.allocate_painter(size, Sense::click_and_drag());
        response.context_menu(|ui| {
            if ui
                .selectable_value(&mut self.y_axis.scale_mode, ScaleMode::Linear, "Y-Linear")
                .clicked()
            {
                ui.close_menu();
            }
            if ui
                .selectable_value(&mut self.y_axis.scale_mode, ScaleMode::Log10, "Y-Log10")
                .clicked()
            {
                ui.close_menu();
            }
            if ui
                .selectable_value(&mut self.y_axis.scale_mode, ScaleMode::DBScale, "Y-dB")
                .clicked()
            {
                ui.close_menu();
            }
            if ui.button("reset zoom").clicked() {
                self.x_axis.zoomed_range = None;
                ui.close_menu();
            }
        });
        self.interaction.update(&mut response, &mut painter);
        self.check_zoom(ui, &mut response);

        let rect = response.rect;
        let width = rect.width();
        let height = rect.height();

        // setup the y-axis label (if it exists)
        let mut y_label_additional_width = 0.0;
        if let Some(y_label) = &self.y_axis.axis_label {
            let galley = painter.layout_no_wrap(
                y_label.to_string(),
                large_font.clone(),
                ui.visuals().text_color(),
            );
            y_label_additional_width += galley.size().y + 5.;
            let mut pos = rect.left_center();
            pos.y += galley.size().x / 2.0;
            painter.add(Shape::Text(
                TextShape::new(pos, galley, ui.visuals().text_color())
                    .with_angle(-std::f32::consts::FRAC_PI_2),
            ));
        }
        let mut x_label_additional_width = 0.0;
        if let Some(x_label) = &self.x_axis.axis_label {
            let galley = painter.layout_no_wrap(
                x_label.to_string(),
                large_font.clone(),
                ui.visuals().text_color(),
            );
            let mut pos = rect.center_bottom();
            pos.y -= galley.size().y;
            x_label_additional_width += galley.size().y;
            painter.text(
                pos,
                Align2::CENTER_CENTER,
                x_label,
                large_font.clone(),
                ui.visuals().text_color(),
            );
        }

        // layout all the detents along the Y-axis to see how far we need to offset it from the
        // left side of the screen in the X-axis
        let x_offset = self
            .y_axis
            .detents
            .iter()
            .map(|(_, str)| {
                let galley =
                    painter.layout_no_wrap(str.to_string(), small_font.clone(), Color32::default());
                galley.size().x
            })
            .reduce(f32::max)
            .unwrap_or(width * 0.1)
            + y_label_additional_width;

        let y_offset = 0.0 + x_label_additional_width;

        let y_axis_x_offset = rect.min.x + x_offset + 5.;
        let y_axis_y_min = rect.min.y + height * 0.05;
        let y_axis_y_max = rect.min.y + height * 0.95;
        let x_axis_x_min = y_axis_x_offset;
        let x_axis_x_max = rect.min.x + width * 0.98;
        let x_axis_y_offset = y_axis_y_max - y_offset;

        self.x_axis.screen_origin = x_axis_x_min;
        self.x_axis.screen_limit = x_axis_x_max;
        self.x_axis.screen_range = x_axis_x_max - x_axis_x_min;
        self.x_axis.incr_sign = 1.0;
        self.y_axis.screen_origin = y_axis_y_min;
        self.y_axis.screen_limit = x_axis_y_offset;
        self.y_axis.screen_range = x_axis_y_offset - y_axis_y_min;
        self.y_axis.incr_sign = -1.0;

        let points = &self.data;
        // update and rescale the data based on this frame's painting window.
        self.x_axis.update_range(points, |p| p.x);
        self.y_axis.update_range(points, |p| p.y);

        // draw the info across the bottom of the x axis
        for detent in &self.x_axis.detents {
            let pos = Pos2 {
                x: detent.0,
                y: x_axis_y_offset + 2.,
            };
            let anchor = Align2::CENTER_TOP;
            painter.text(
                pos,
                anchor,
                &detent.1,
                small_font.clone(),
                ui.visuals().text_color(),
            );
            painter.line_segment(
                [
                    Pos2 {
                        x: detent.0,
                        y: y_axis_y_min,
                    },
                    Pos2 {
                        x: detent.0,
                        y: x_axis_y_offset,
                    },
                ],
                minor_stroke,
            );
        }
        // draw the info up the y axis - note, painted inverted!
        for detent in &self.y_axis.detents {
            let pos = Pos2 {
                x: y_axis_x_offset - 5.,
                y: detent.0,
            };
            let anchor = Align2::RIGHT_CENTER;
            painter.text(
                pos,
                anchor,
                &detent.1,
                small_font.clone(),
                ui.visuals().text_color(),
            );
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
                minor_stroke,
            );
        }
        // paint vertical 'y' axis line
        painter.line_segment(
            [
                Pos2::new(y_axis_x_offset, y_axis_y_min),
                Pos2::new(y_axis_x_offset, x_axis_y_offset),
            ],
            major_stroke,
        );
        // paint horiz 'x' axis line
        painter.line_segment(
            [
                Pos2::new(x_axis_x_min, x_axis_y_offset),
                Pos2::new(x_axis_x_max, x_axis_y_offset),
            ],
            major_stroke,
        );

        // draw the points as individual line segments
        for pnt in points.windows(2) {
            let Some(first) = pnt.first() else {
                continue;
            };
            let Some(second) = pnt.get(1) else {
                continue;
            };
            let Some(first) = self.scale_point(first) else {
                draw_log_warning = true;
                self.draw_yellow_err_line(&mut painter, first, ui);
                continue;
            };
            let Some(second) = self.scale_point(second) else {
                draw_log_warning = true;
                self.draw_yellow_err_line(&mut painter, second, ui);
                continue;
            };
            // draw the actual line
            painter.line_segment([first, second], major_stroke);
        }

        self.draw_cursor(ui, &mut response, &mut painter);

        if draw_log_warning {
            painter.text(
                rect.center_bottom(),
                Align2::CENTER_BOTTOM,
                "Warning: some points <= 0 were skipped in log10/dB mode.".to_string(),
                small_font.clone(),
                caution_color,
            );
        }
        if let Some(title) = &self.title {
            painter.text(
                rect.center_top(),
                Align2::CENTER_TOP,
                title,
                large_font.clone(),
                ui.visuals().text_color(),
            );
        }
    }

    fn scale_point(&self, pos: &PlotPoint) -> Option<Pos2> {
        Some(Pos2 {
            x: self.x_axis.scale_value(pos.x)?,
            y: self.y_axis.scale_value(pos.y)?,
        })
    }

    fn draw_yellow_err_line(&self, painter: &mut Painter, point: &PlotPoint, ui: &mut Ui) {
        let caution_color = ui.visuals().warn_fg_color;
        if point.y > 0.0 {
            // y is fine, horizontal at Y
            let Some(val) = self.y_axis.scale_value(point.y) else {
                return;
            };
            painter.line_segment(
                [
                    Pos2::new(self.x_axis.screen_origin, val),
                    Pos2::new(self.x_axis.screen_limit, val),
                ],
                Stroke::new(1.0, caution_color),
            );
        } else if point.x > 0.0 {
            // x is fine, vertical at x
            let Some(val) = self.x_axis.scale_value(point.x) else {
                return;
            };
            painter.line_segment(
                [
                    Pos2::new(val, self.y_axis.screen_origin),
                    Pos2::new(val, self.y_axis.screen_limit),
                ],
                Stroke::new(1.0, caution_color),
            );
        }
    }

    fn draw_cursor(&mut self, ui: &mut Ui, response: &mut Response, painter: &mut Painter) {
        // draw the hover cursors
        if let Some(hover) = response.hover_pos() {
            let rect = response.rect;
            let xrng = rect.min.x..=rect.max.x;
            let yrng = rect.min.y..=rect.max.y;
            let color = ui.visuals().widgets.noninteractive.fg_stroke;
            // paint the crosshair lines
            painter.hline(xrng, hover.y, color);
            painter.vline(hover.x, yrng, color);

            // paint the text
            let mod_x = self.x_axis.describe_screen_pos(hover.x);
            let mod_y = self.y_axis.describe_screen_pos(hover.y);
            let text = format!("x: {mod_x}\ny: {mod_y}");
            let color = ui.visuals().text_cursor.stroke.color;
            let font_id = TextStyle::Monospace.resolve(ui.style());
            let mut align = Align2::LEFT_BOTTOM;

            // figure out if it extends out past the rectangle
            let galley = painter.layout_no_wrap(text.to_string(), font_id.clone(), color);
            let txtrect = align.anchor_size(hover, galley.size());

            if txtrect.max.x >= rect.max.x {
                // flip the x dimension
                let [_, v] = align.0;
                align.0 = [Align::Max, v];
            }
            if txtrect.min.y <= rect.min.y {
                // flip the y dimension
                let [h, _] = align.0;
                align.0 = [h, Align::Min];
            }
            let galley = painter.layout_no_wrap(text, font_id, color);
            let rect = align.anchor_size(hover, galley.size());

            painter.rect_filled(rect, 0.0, Color32::from_white_alpha(32));
            painter.galley(rect.min, galley, color);
        }
    }
}
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ScaleMode {
    #[default]
    Linear,
    Log10,
    DBScale,
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
    pub screen_limit: f32,
    pub detents: Vec<(f32, String)>,
    pub draw_log_clip_warning: bool,

    pub zoomed_range: Option<(f64, f64)>,
    pub axis_label: Option<String>,
}

impl Axis {
    pub fn update_range<F: Fn(&PlotPoint) -> f64>(&mut self, vals: &[PlotPoint], accessor: F) {
        self.draw_log_clip_warning = false;
        self.min_val = f64::INFINITY;
        self.max_val = f64::NEG_INFINITY;
        for val in vals {
            let v = match self.scale_mode {
                ScaleMode::Linear => accessor(val),
                ScaleMode::Log10 | ScaleMode::DBScale => {
                    let v = accessor(val);
                    if v <= 0.0 {
                        self.draw_log_clip_warning = true;
                        continue;
                    }
                    v
                }
            };
            self.min_val = self.min_val.min(v);
            self.max_val = self.max_val.max(v);
        }
        if self.scale_mode == ScaleMode::DBScale {
            if self.min_val <= 0.0 {
                self.draw_log_clip_warning = true;
                self.min_val = f64::MIN_POSITIVE;
            }
            self.min_val = 10. * self.min_val.log10();
            self.max_val = 10. * self.max_val.log10();
        } else if self.scale_mode == ScaleMode::Log10 && self.min_val <= 0.0 {
            self.draw_log_clip_warning = true;
            self.min_val = f64::MIN_POSITIVE;
        }
        if let Some((min, max)) = self.zoomed_range {
            self.min_val = min;
            self.max_val = max;
        }

        self.range = self.max_val - self.min_val;
        match self.scale_mode {
            ScaleMode::Linear | ScaleMode::DBScale => {
                let model_per_point = self.range / (self.screen_range as f64);
                let base_step_size = model_per_point * 5.0f64;

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
                    let dv = min_detent + range * frac;
                    let drawpnt = self.model_to_screen(dv);
                    let label = match self.scale_mode {
                        ScaleMode::DBScale => format!("{dv:4.2} dB"),
                        _ => format!("{dv:4.2}"),
                    };
                    self.detents.push((drawpnt, label));
                }
            }
            ScaleMode::Log10 => {
                self.detents = Vec::new();
                let range = self.max_val - self.min_val;
                let mut range_exp = if range < 1.0 {
                    range.log10().floor() as i32
                } else {
                    range.log10().ceil() as i32
                };

                let mut scalef = 10f64.powi(range_exp);
                let mut mv = (self.min_val / scalef).floor() * scalef;
                if self.min_val < 1.0 {
                    range_exp = self.min_val.log10().floor() as i32;
                    scalef = 10f64.powi(range_exp);
                    mv = (self.min_val / scalef).floor() * scalef;
                }
                self.min_val = mv;
                self.max_val = (self.max_val / scalef).ceil() * scalef;
                let mut current_exp = range_exp - 1;
                let mut current = self.min_val;
                let stop = self.max_val;
                // major detents
                let drawpnt = self.model_to_screen(self.log_scale(current));
                self.detents
                    .push((drawpnt, format!("{current:.4}: 1e{current_exp:.4}")));
                while current <= stop {
                    let incr = 10f64.powi(current_exp);
                    let next = current + 10f64 * incr;
                    current = (current / incr).round() * incr;
                    let drawpnt = self.model_to_screen(self.log_scale(current));
                    self.detents
                        .push((drawpnt, format!("{current:.4}: 1e{current_exp:.4}")));

                    // minor detents
                    for _idx in 1..10 {
                        if current >= next || current >= stop {
                            break;
                        }
                        current += incr;
                        let drawpnt = self.model_to_screen(self.log_scale(current));
                        self.detents.push((drawpnt, String::default()));
                    }

                    current_exp += 1;
                }
                let drawpnt = self.model_to_screen(self.log_scale(self.max_val));
                self.detents
                    .push((drawpnt, format!("{:.4}: 1e{current_exp:.4}", self.max_val)));
            }
        }
    }

    pub fn linear_scale(&self, val: f64) -> f32 {
        ((val - self.min_val) / self.range) as f32
    }
    pub fn linear_unscale(&self, val: f32) -> f64 {
        val as f64 * self.range + self.min_val
    }

    pub fn log_scale(&self, mut val: f64) -> f64 {
        if val <= 0.0 {
            val = f64::MIN_POSITIVE;
        }
        let min_val = self.min_val.log10();
        let max_val = self.max_val.log10();
        let frac = (val.log10() - min_val) / (max_val - min_val);
        frac * self.range + self.min_val
    }

    pub fn log_unscale(&self, val: f64) -> f64 {
        let frac = (val - self.min_val) / self.range;
        let min_val = self.min_val.log10();
        let max_val = self.max_val.log10();
        let frac = frac * (max_val - min_val) + min_val;

        10f64.powf(frac)
    }

    pub fn db_scale(&self, mut val: f64) -> f64 {
        if val <= 0.0 {
            val = f64::MIN_POSITIVE;
        }
        10. * val.log10()
    }

    pub fn db_unscale(&self, val: f64) -> f64 {
        10f64.powf(val / 10.)
    }

    pub fn scale_value(&self, val: f64) -> Option<f32> {
        match self.scale_mode {
            ScaleMode::Linear => Some(self.model_to_screen(val)),
            ScaleMode::Log10 => {
                if val <= 0.0 {
                    None
                } else {
                    let v = self.log_scale(val);
                    Some(self.model_to_screen(v))
                }
            }
            ScaleMode::DBScale => {
                if val <= 0.0 {
                    None
                } else {
                    let v = self.db_scale(val);
                    Some(self.model_to_screen(v))
                }
            }
        }
    }
    pub fn unscale_value(&self, val: f32) -> f64 {
        match self.scale_mode {
            ScaleMode::Linear => self.screen_to_model(val),
            ScaleMode::Log10 => {
                let val = self.screen_to_model(val);
                self.log_unscale(val)
            }
            ScaleMode::DBScale => {
                let val = self.screen_to_model(val);
                self.db_unscale(val)
            }
        }
    }

    pub fn model_to_screen(&self, val: f64) -> f32 {
        let scaled = self.linear_scale(val);
        let scaled = scaled * self.screen_range;

        if self.incr_sign < 0.0 {
            self.screen_range + self.screen_origin - scaled
        } else {
            scaled + self.screen_origin
        }
    }

    pub fn screen_to_model(&self, val: f32) -> f64 {
        let pre = if self.incr_sign < 0.0 {
            self.screen_origin + self.screen_range - val
        } else {
            val - self.screen_origin
        };
        let pre = pre / self.screen_range;
        self.linear_unscale(pre)
    }

    pub fn describe_screen_pos(&self, val: f32) -> String {
        let v = self.screen_to_model(val);
        match self.scale_mode {
            ScaleMode::Linear => format!("{}", PrettyDec(v)),
            ScaleMode::Log10 => {
                let orig = self.log_unscale(v);
                // let scaled = orig.log10();
                format!("{orig}")
            }
            ScaleMode::DBScale => {
                let orig = self.db_unscale(v);
                let scaled = 10. * orig.log10();
                format!("{}={} dB", PrettyDec(orig), PrettyDec(scaled))
            }
        }
    }
}
struct PrettyDec<T: LowerExp>(T);
impl<T: LowerExp> Display for PrettyDec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = format!("{:.4e}", self.0);
        if v.ends_with("e0") {
            v.truncate(v.len() - 2);
        }
        f.write_str(&v)
    }
}
