// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Log plotting widgets.

use crate::fonts;
use crate::fonts::FontSet;
use crate::fonts::BOLD;
use crate::profile_scope;
use egui::epaint::TextShape;
use egui::{
    pos2, Align, Align2, Color32, Context, FontFamily, FontId, Mesh, Painter, Pos2, Rect, Response,
    Rgba, Rounding, Sense, Shape, Stroke, TextStyle, Ui, Vec2,
};
use irox_imagery::colormaps::CLASSIC_20;
use irox_imagery::Color;
use irox_stats::rects::Rect2D;
use irox_stats::streaming::Summary;
use irox_time::datetime::UTCDateTime;
use irox_time::epoch::UnixTimestamp;
use irox_time::format::iso8601::EXTENDED_TIME_FORMAT;
use irox_time::Duration;
use irox_tools::sync::Exchanger;
use irox_units::quantities::Units;
use irox_units::units::duration::DurationUnit;
use std::fmt::{Display, Formatter, LowerExp};
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Default, Copy, Clone)]
pub struct PlotPoint {
    pub x: f64,
    pub y: f64,
}
impl PlotPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
impl From<[f64; 2]> for PlotPoint {
    fn from(p: [f64; 2]) -> Self {
        let [x, y] = p;
        Self { x, y }
    }
}

pub type FormatterFn = dyn Fn(f64) -> String + Send;

pub trait IntoColor32 {
    fn into_color32(self) -> Color32;
}
impl IntoColor32 for Color {
    fn into_color32(self) -> Color32 {
        let [a, r, g, b] = self.argb_values();
        Color32::from_rgba_unmultiplied(r, g, b, a)
    }
}
static DEFAULT_COLORMAP: &[Color] = CLASSIC_20;

/// A tracking struct for an user interaction with a plot.
///
/// This tracks whether a drag has started, the end delta of the drag, and the
/// current "painter coordinates" of the highlighted area by the mouse.
#[derive(Default)]
pub struct PlotInteraction {
    pub drag_started_pos: Option<Pos2>,
    pub drag_ended_delta: Option<Vec2>,
    pub zoom_area: Option<Rect>,
}
impl PlotInteraction {
    /// Clear and reset the interaction state back to zero.
    pub fn clear(&mut self) {
        self.drag_ended_delta = None;
        self.drag_started_pos = None;
        self.zoom_area = None;
    }
    /// Call update on every frame to track the current interaction state.
    pub fn update(&mut self, response: &mut Response, painter: &mut Painter) {
        if response.drag_started() {
            // mark that a drag has started this frame.
            self.drag_started_pos = response.interact_pointer_pos();
        } else if response.drag_stopped() {
            // mark that a drag has stopped this frame.
            if let Some(start) = self.drag_started_pos {
                // ensure that the location the drag started was tracked.
                let first = start.x;
                if let Some(delta) = self.drag_ended_delta {
                    // make sure the mouse moved between frames, and then update
                    // the zoom rectangle
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
            // update the drag delta from the start location as the mouse moves.
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
                // paint the horizontal drag rectangle.
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

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum YAxisSide {
    #[default]
    LeftAxis,
    RightAxis,
}

#[derive(Default)]
pub struct Line {
    pub name: Arc<String>,
    pub visible: AtomicBool,
    pub yaxis_side: YAxisSide,
    pub line_stroke: Stroke,
    pub text_font: FontId,
    pub sample_marker: Option<Shape>,
    pub line_exchanger: LineDataExchanger,
    pub error_bars_exchanger: ErrorBarsExchanger,

    meshes: Vec<Shape>,
    lines: Vec<Shape>,
    points: Vec<Shape>,
    bounds: Rect2D,
}
impl Line {
    pub fn set_name<T: AsRef<str>>(&mut self, name: T) {
        self.name = Arc::new(name.as_ref().to_owned());
    }

    fn bounds_as_points(&self) -> [PlotPoint; 2] {
        let a = PlotPoint {
            x: self.bounds.origin.x,
            y: self.bounds.origin.y,
        };
        let b = PlotPoint {
            x: self.bounds.far_point.x,
            y: self.bounds.far_point.y,
        };
        [a, b]
    }
}

#[derive(Default, Clone)]
pub struct LineDataExchanger {
    exchanger: Exchanger<Arc<[PlotPoint]>>,
}
impl Deref for LineDataExchanger {
    type Target = Exchanger<Arc<[PlotPoint]>>;

    fn deref(&self) -> &Self::Target {
        &self.exchanger
    }
}
pub struct LineWithErrorBars {
    pub line_data: Arc<[PlotPoint]>,
    pub error_bars: Arc<[(f64, Summary<f64>)]>,
}
#[derive(Default, Clone)]
pub struct ErrorBarsExchanger {
    exchanger: Exchanger<LineWithErrorBars>,
}
impl Deref for ErrorBarsExchanger {
    type Target = Exchanger<LineWithErrorBars>;

    fn deref(&self) -> &Self::Target {
        &self.exchanger
    }
}
pub fn y_axis_units_formatter(unit: Units) -> Box<FormatterFn> {
    Box::new(move |x| unit.format(&x))
}
pub fn x_axis_time_millis_formatter() -> Box<FormatterFn> {
    Box::new(|x: f64| {
        let val = DurationUnit::Millisecond.as_seconds(x).round() as u64;
        let ts = UnixTimestamp::from_offset(Duration::from_seconds(val));
        let dt = UTCDateTime::from(ts);
        dt.get_time().format(&EXTENDED_TIME_FORMAT)
    })
}

///
/// Basic plot, with ability to switch between linear and log axes.  This widget
/// tracks state and is meant to be saved across multiple frames.
#[derive(Default)]
pub struct BasicPlot {
    /// The data to plot each frame.
    pub lines: Vec<Line>,
    pub name: Arc<String>,
    /// The X-axis settings
    pub x_axis: Axis,
    /// The Y-axis settings
    pub y_axis_left: Axis,
    pub y_axis_right: Option<Axis>,
    /// The Interaction tracking of the plot
    pub interaction: PlotInteraction,
    /// Optional title for this plot.
    pub title: Option<String>,

    last_render_size: Rect2D,
}
macro_rules! scale_y {
    ($self:ident, $ax:ident,$pos:ident) => {{
        if let Some(y) = $ax.scale_value($pos.y) {
            scale_x!($self, $pos, y)
        } else {
            None
        }
    }};
}
macro_rules! scale_x {
    ($self:ident, $pos:ident, $y:ident) => {{
        if let Some(x) = $self.x_axis.scale_value($pos.x) {
            Some(Pos2 { x, $y })
        } else {
            None
        }
    }};
}
macro_rules! scale_point {
    ($self:ident, $pos: ident, $yaxis_side: expr) => {{
        if $yaxis_side == YAxisSide::LeftAxis {
            let ax = &$self.y_axis_left;
            scale_y!($self, ax, $pos)
        } else {
            if let Some(ax) = &$self.y_axis_right {
                scale_y!($self, ax, $pos)
            } else {
                None
            }
        }
    }};
}
impl BasicPlot {
    pub fn new(ctx: &Context) -> BasicPlot {
        fonts::load_fonts(
            FontSet {
                ubuntu_bold: true,
                ..Default::default()
            },
            ctx,
        );
        Default::default()
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
        self.y_axis_left.axis_label = Some(title.as_ref().to_string());
        self
    }
    #[must_use]
    pub fn with_x_axis_formatter(mut self, fmtr: Box<FormatterFn>) -> Self {
        self.x_axis.axis_formatter = Some(fmtr);
        self
    }
    #[must_use]
    pub fn with_y_axis_formatter(mut self, fmtr: Box<FormatterFn>) -> Self {
        self.y_axis_left.axis_formatter = Some(fmtr);
        self
    }
    // #[must_use]
    // pub fn with_line<T: AsRef<str>>(mut self, name: T, data: Arc<[PlotPoint]>) -> Self {
    //     self.add_line(move |line| {
    //         line.name = Arc::new(name.as_ref().to_string());
    //         line.data = data.clone();
    //     });
    //     self
    // }
    pub fn add_line<T: FnMut(&mut Line)>(&mut self, func: T) -> LineDataExchanger {
        self.add_line_with_error_bars(func).0
    }
    pub fn add_line_with_error_bars<T: FnMut(&mut Line)>(
        &mut self,
        mut func: T,
    ) -> (LineDataExchanger, ErrorBarsExchanger) {
        let idx = self.lines.len() % DEFAULT_COLORMAP.len();
        let color = DEFAULT_COLORMAP
            .get(idx)
            .copied()
            .unwrap_or_default()
            .into_color32();
        let stroke = Stroke::new(0.75, color);
        let mut line = Line {
            line_stroke: stroke,
            visible: AtomicBool::new(true),
            text_font: FontId::new(10., FontFamily::Name(BOLD.into())),
            ..Default::default()
        };
        func(&mut line);
        let exch = line.line_exchanger.clone();
        let errs = line.error_bars_exchanger.clone();
        line.line_exchanger.set_data_changed();
        line.error_bars_exchanger.set_data_changed();
        self.lines.push(line);
        (exch, errs)
    }
    fn check_zoom(&mut self, ui: &mut Ui, response: &mut Response) {
        profile_scope!("check_zoom", self.name.as_str());
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
    fn any_lines_changed(&self) -> bool {
        let mut changed = false;
        for line in &self.lines {
            changed |= line.line_exchanger.new_data_available();
            changed |= line.error_bars_exchanger.new_data_available();
        }
        changed
    }
    fn update_line_data(&mut self) {
        for line in &mut self.lines {
            let stroke = &line.line_stroke;
            let mut bounds = Rect2D::empty();
            let mut errors = None;
            let mut lines = None;
            if let Some(errs) = line.error_bars_exchanger.take_data() {
                errors = Some(errs.error_bars);
                lines = Some(errs.line_data);
            } else if let Some(ls) = line.line_exchanger.take_data() {
                lines = Some(ls);
            }

            if let Some(errors) = errors {
                let nval = errors.as_ref().len();
                let mut mesh = Mesh::default();
                mesh.reserve_vertices(nval * 3);
                mesh.reserve_triangles((nval - 1) * 4);

                let fill_color = Rgba::from(stroke.color)
                    // .to_opaque()
                    .multiply(0.40)
                    .into();
                let mut added_triangles = 0;
                for (x, summary) in errors.as_ref() {
                    let i = mesh.vertices.len() as u32;
                    let Some(max) = summary.max() else {
                        continue;
                    };
                    let maxpnt = PlotPoint { x: *x, y: max };
                    let Some(maxpos) = scale_point!(self, maxpnt, line.yaxis_side) else {
                        continue;
                    };
                    let Some(min) = summary.min() else {
                        continue;
                    };
                    let minpnt = PlotPoint { x: *x, y: min };
                    let range_buf = (max - min) * 0.05;

                    bounds.add_point(minpnt.x, minpnt.y - range_buf);
                    bounds.add_point(maxpnt.x, maxpnt.y + range_buf);
                    let Some(minpos) = scale_point!(self, minpnt, line.yaxis_side) else {
                        continue;
                    };

                    let upper_quad = PlotPoint {
                        x: *x,
                        y: bounds.upper_left_quadrant().center().y,
                    };
                    let Some(uqpos) = scale_point!(self, upper_quad, line.yaxis_side) else {
                        continue;
                    };
                    let lower_quad = PlotPoint {
                        x: *x,
                        y: bounds.lower_left_quadrant().center().y,
                    };
                    let Some(lqpos) = scale_point!(self, lower_quad, line.yaxis_side) else {
                        continue;
                    };
                    mesh.colored_vertex(maxpos, fill_color); // a
                    mesh.colored_vertex(uqpos, Color32::TRANSPARENT); // b
                    mesh.colored_vertex(lqpos, Color32::TRANSPARENT); // c
                    mesh.colored_vertex(minpos, fill_color); // d
                    let a = i;
                    let b = i + 1;
                    let c = i + 2;
                    let d = i + 3;

                    let e = i + 4;
                    let f = i + 5;
                    let g = i + 6;
                    let h = i + 7;

                    if added_triangles < (nval - 1) {
                        mesh.add_triangle(a, b, e);
                        mesh.add_triangle(b, e, f);

                        mesh.add_triangle(c, d, g);
                        mesh.add_triangle(d, g, h);

                        added_triangles += 1;
                    }
                }

                line.meshes = vec![Shape::mesh(mesh)];
            }
            if let Some(data) = lines {
                let points = data.as_ref();
                let mut lineout = Vec::<Pos2>::with_capacity(points.len());
                line.points.clear();
                for pnt in points {
                    bounds.add_point(pnt.x, pnt.y);
                    let Some(pnt) = scale_point!(self, pnt, line.yaxis_side) else {
                        // draw_log_warning = true;
                        // self.draw_yellow_err_line(&mut painter, pnt, self.yaxis_side, ui);
                        continue;
                    };
                    lineout.push(pnt);
                    if let Some(shp) = &line.sample_marker {
                        let mut shp = shp.clone();
                        shp.translate(Vec2::new(pnt.x, pnt.y));
                        line.points.push(shp);
                    }
                    // if let Some(pos) = response.hover_pos() {
                    //     let dx = (pos.x - pnt.x).abs();
                    //     let dy = (pos.y - pnt.y).abs();
                    //     let dist = (dx * dx + dy * dy).sqrt();
                    //     if dist <= 10.0 {
                    //         closest_hover = Some((pnt, self.yaxis_side));
                    //     }
                    // }
                }
                line.lines = vec![Shape::line(lineout, *stroke)];
            }
            if bounds != Rect2D::empty() {
                line.bounds = bounds;
            }
        }
    }
    pub fn show(&mut self, ui: &mut Ui) {
        profile_scope!("basicplot.show", self.name.as_str());
        let major_stroke = Stroke::new(2.0, ui.visuals().widgets.inactive.fg_stroke.color);
        let minor_stroke = Stroke::new(1.0, ui.visuals().widgets.open.bg_stroke.color);
        let caution_color = ui.visuals().warn_fg_color;
        let small_font = TextStyle::Small.resolve(ui.style());
        let large_font = TextStyle::Heading.resolve(ui.style());

        let size = ui.available_size();
        let draw_log_warning =
            self.y_axis_left.draw_log_clip_warning || self.x_axis.draw_log_clip_warning;

        let (mut response, mut painter) = ui.allocate_painter(size, Sense::click_and_drag());
        response.context_menu(|ui| {
            if ui
                .selectable_value(
                    &mut self.y_axis_left.scale_mode,
                    ScaleMode::Linear,
                    "Y-Linear",
                )
                .clicked()
            {
                ui.close_menu();
            }
            if ui
                .selectable_value(
                    &mut self.y_axis_left.scale_mode,
                    ScaleMode::Log10,
                    "Y-Log10",
                )
                .clicked()
            {
                ui.close_menu();
            }
            if ui
                .selectable_value(&mut self.y_axis_left.scale_mode, ScaleMode::DBScale, "Y-dB")
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
        if let Some(y_label) = &self.y_axis_left.axis_label {
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
            .y_axis_left
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

        let text_label_offset_y = self
            .lines
            .iter()
            .map(|line| {
                let galley = painter.layout_no_wrap(
                    line.name.as_str().to_string(),
                    line.text_font.clone(),
                    Color32::default(),
                );
                galley.size().y
            })
            .reduce(f32::max)
            .unwrap_or(width * 0.1);

        let y_offset = 0.0 + x_label_additional_width;

        let y_axis_x_offset = rect.min.x + x_offset + 5.;
        let y_axis_y_min = rect.min.y + height * 0.05;
        let y_axis_y_max = rect.min.y + height * 0.95 - text_label_offset_y;
        let x_axis_x_min = y_axis_x_offset;
        let x_axis_x_max = rect.min.x + width * 0.98;
        let x_axis_y_offset = y_axis_y_max - y_offset;

        self.x_axis.screen_origin = x_axis_x_min;
        self.x_axis.screen_limit = x_axis_x_max;
        self.x_axis.screen_range = x_axis_x_max - x_axis_x_min;
        self.x_axis.incr_sign = 1.0;
        self.y_axis_left.screen_origin = y_axis_y_min;
        self.y_axis_left.screen_limit = x_axis_y_offset;
        self.y_axis_left.screen_range = x_axis_y_offset - y_axis_y_min;
        self.y_axis_left.incr_sign = -1.0;
        let grid_bounds = Rect::from_min_max(
            Pos2::new(x_axis_x_min, y_axis_y_min),
            Pos2::new(x_axis_x_max, y_axis_y_max),
        );

        let lr = rect.into();
        if lr != self.last_render_size || self.any_lines_changed() {
            profile_scope!("update range", self.name.as_str());
            self.update_line_data();
            let all_points = self
                .lines
                .iter()
                .filter(|v| v.visible.load(Ordering::Relaxed))
                .flat_map(Line::bounds_as_points)
                .collect::<Vec<PlotPoint>>();
            // update and rescale the data based on this frame's painting window.
            self.x_axis.update_range(all_points.as_slice(), |p| p.x);

            let left_points = self
                .lines
                .iter()
                .filter(|v| v.visible.load(Ordering::Relaxed))
                .filter(|v| v.yaxis_side == YAxisSide::LeftAxis)
                .flat_map(Line::bounds_as_points)
                .collect::<Vec<PlotPoint>>();
            self.y_axis_left
                .update_range(left_points.as_slice(), |p| p.y);
            if let Some(y_axis_rt) = &mut self.y_axis_right {
                let rt_points = self
                    .lines
                    .iter()
                    .filter(|v| v.visible.load(Ordering::Relaxed))
                    .filter(|v| v.yaxis_side == YAxisSide::RightAxis)
                    .flat_map(Line::bounds_as_points)
                    .collect::<Vec<PlotPoint>>();
                y_axis_rt.update_range(rt_points.as_slice(), |p| p.y);
            }

            self.last_render_size = lr;
        }

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
        for detent in &self.y_axis_left.detents {
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
        if let Some(rt_y) = &self.y_axis_right {
            // draw the info up the y axis - note, painted inverted!
            for detent in &rt_y.detents {
                let pos = Pos2 {
                    x: x_axis_x_max + 5.,
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
        let closest_hover: Option<(Pos2, YAxisSide)> = None;

        // collect meshes
        for line in &self.lines {
            profile_scope!("draw mesh", line.name.as_str());
            if line.visible.load(Ordering::Relaxed) {
                for mesh in &line.meshes {
                    painter.add(mesh.clone());
                }
            }
        }
        // collect lines and points
        for line in &self.lines {
            profile_scope!("draw line", line.name.as_str());
            if line.visible.load(Ordering::Relaxed) {
                for line in &line.lines {
                    painter.add(line.clone());
                }
                for point in &line.points {
                    painter.add(point.clone());
                }
            }
        }

        // draw the points as individual line segments
        let mut start_text = rect.left_bottom();
        for line in &self.lines {
            profile_scope!("draw controls", line.name.as_str());
            let visible = line.visible.load(Ordering::Relaxed);
            let mut color = Color32::BLACK;
            if visible {
                color = line.line_stroke.color;
            }
            let galley =
                painter.layout_no_wrap(line.name.to_string(), line.text_font.clone(), color);
            let used = Align2::LEFT_BOTTOM.anchor_size(start_text, galley.size());
            if let Some(pos) = response.hover_pos() {
                if used.contains(pos) {
                    let hvr = ui.visuals().widgets.hovered;
                    let fill = hvr.bg_fill;
                    let rnd = hvr.rounding;
                    let strk = hvr.bg_stroke;
                    painter.rect(used, rnd, fill, strk);
                    if response.clicked() {
                        line.visible.swap(!visible, Ordering::Relaxed);
                    }
                }
            }
            painter.galley(used.min, galley, color);
            start_text.x += used.width() + 10.;
        }

        self.draw_cursor(ui, &mut response, &mut painter, closest_hover, &grid_bounds);

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

    fn _draw_yellow_err_line(
        &self,
        painter: &mut Painter,
        point: &PlotPoint,
        yaxis_side: YAxisSide,
        ui: &mut Ui,
    ) {
        let caution_color = ui.visuals().warn_fg_color;
        let axis = match yaxis_side {
            YAxisSide::LeftAxis => &self.y_axis_left,
            YAxisSide::RightAxis => {
                if let Some(rt) = &self.y_axis_right {
                    rt
                } else {
                    return;
                }
            }
        };
        if point.y > 0.0 {
            // y is fine, horizontal at Y
            let Some(val) = axis.scale_value(point.y) else {
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
                    Pos2::new(val, axis.screen_origin),
                    Pos2::new(val, axis.screen_limit),
                ],
                Stroke::new(1.0, caution_color),
            );
        }
    }

    fn draw_cursor(
        &mut self,
        ui: &mut Ui,
        response: &mut Response,
        painter: &mut Painter,
        closest_point: Option<(Pos2, YAxisSide)>,
        grid_bounds: &Rect,
    ) {
        // draw the hover cursors
        let (draw_pos, side) = if let Some(closest_point) = closest_point {
            closest_point
        } else if let Some(hover) = response.hover_pos() {
            (hover, YAxisSide::LeftAxis)
        } else {
            return;
        };
        if !grid_bounds.contains(draw_pos) {
            return;
        }
        let y_axis = match side {
            YAxisSide::LeftAxis => &mut self.y_axis_left,
            YAxisSide::RightAxis => {
                if let Some(ax) = &self.y_axis_right {
                    ax
                } else {
                    return;
                }
            }
        };
        let rect = response.rect;
        let xrng = rect.min.x..=rect.max.x;
        let yrng = rect.min.y..=rect.max.y;
        let color = ui.visuals().widgets.noninteractive.fg_stroke;
        // paint the crosshair lines
        painter.hline(xrng, draw_pos.y, color);
        painter.vline(draw_pos.x, yrng, color);

        // paint the text
        let mod_x = self.x_axis.describe_screen_pos(draw_pos.x);
        let mod_y = y_axis.describe_screen_pos(draw_pos.y);
        let text = format!("x: {mod_x}\ny: {mod_y}");
        let color = ui.visuals().text_cursor.stroke.color;
        let font_id = TextStyle::Monospace.resolve(ui.style());
        let mut align = Align2::LEFT_BOTTOM;

        // figure out if it extends out past the rectangle
        let galley = painter.layout_no_wrap(text.to_string(), font_id.clone(), color);
        let txtrect = align.anchor_size(draw_pos, galley.size());

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
        let rect = align.anchor_size(draw_pos, galley.size());

        painter.rect_filled(rect, 0.0, Color32::from_white_alpha(32));
        painter.galley(rect.min, galley, color);
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
    /// Minimum detected value of this axis in data-coordinates
    pub min_val: f64,
    /// Maximum detected value of this axis in data-coordinates
    pub max_val: f64,
    /// Scaling mode for this axis
    pub scale_mode: ScaleMode,
    /// -1 or +1, the direction of "positive" on this axis.
    pub incr_sign: f64,
    /// The range of this axis (max - min values) in data-coordinates
    pub range: f64,
    /// The origin (zero value) of this axis in screen coordinates
    pub screen_origin: f32,
    /// The range of this axis in screen coordinates
    pub screen_range: f32,
    /// The limit (max value) of this axis in screen coordinates
    pub screen_limit: f32,
    /// The list of detents to draw on this axis, position and label
    pub detents: Vec<(f32, String)>,
    /// Whether or not to draw a warning that the values have been clipped to
    /// positive values for `log` drawing
    pub draw_log_clip_warning: bool,

    /// If the plot has been zoomed in, this is the (min,max) values in data
    /// coordinates of the changed range.
    pub zoomed_range: Option<(f64, f64)>,
    /// Optional label to paint on this axis
    pub axis_label: Option<String>,
    /// Optional formatter for the detents on this axis, accepts the data value
    /// and returns a string as the detent.
    pub axis_formatter: Option<Box<FormatterFn>>,
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
                    let label = if let Some(fmtr) = &self.axis_formatter {
                        fmtr(dv)
                    } else {
                        match self.scale_mode {
                            ScaleMode::DBScale => format!("{dv:4.2} dB"),
                            _ => format!("{dv:4.2}"),
                        }
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
                let label = if let Some(fmtr) = &self.axis_formatter {
                    fmtr(current)
                } else {
                    format!("{current:.4}: 1e{current_exp:.4}")
                };
                self.detents.push((drawpnt, label));
                while current <= stop {
                    let incr = 10f64.powi(current_exp);
                    let next = current + 10f64 * incr;
                    current = (current / incr).round() * incr;
                    let drawpnt = self.model_to_screen(self.log_scale(current));
                    let label = if let Some(fmtr) = &self.axis_formatter {
                        fmtr(current)
                    } else {
                        format!("{current:.4}: 1e{current_exp:.4}")
                    };
                    self.detents.push((drawpnt, label));

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
                let label = if let Some(fmtr) = &self.axis_formatter {
                    fmtr(self.max_val)
                } else {
                    format!("{:.4}: 1e{current_exp:.4}", self.max_val)
                };
                self.detents.push((drawpnt, label));
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
            ScaleMode::Linear => {
                if let Some(f) = &self.axis_formatter {
                    f(v)
                } else {
                    format!("{}", PrettyDec(v))
                }
            }
            ScaleMode::Log10 => {
                let orig = self.log_unscale(v);
                if let Some(f) = &self.axis_formatter {
                    f(orig)
                } else {
                    format!("{orig}")
                }
            }
            ScaleMode::DBScale => {
                let orig = self.db_unscale(v);
                if let Some(f) = &self.axis_formatter {
                    f(orig)
                } else {
                    let scaled = 10. * orig.log10();
                    format!("{}={} dB", PrettyDec(orig), PrettyDec(scaled))
                }
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
