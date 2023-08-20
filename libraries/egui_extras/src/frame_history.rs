// SPDX-License-Identifier: MIT
// Modifications Copyright 2023 IROX Contributors

//!
//! Implementations around the rendering history of individual frames
//!

use egui::util::History;

///
/// Tracks information about the render history of previous frames
///
pub struct FrameHistory {
    frame_times: History<f32>,
}

impl Default for FrameHistory {
    fn default() -> Self {
        let max_age: f32 = 1.0;
        let max_len = (max_age * 300.0).round() as usize;
        Self {
            frame_times: History::new(0..max_len, max_age),
        }
    }
}

impl FrameHistory {
    // Called first
    pub fn on_new_frame(&mut self, now: f64, previous_frame_time: Option<f32>) {
        let previous_frame_time = previous_frame_time.unwrap_or_default();
        if let Some(latest) = self.frame_times.latest_mut() {
            *latest = previous_frame_time; // rewrite history now that we know
        }
        self.frame_times.add(now, previous_frame_time); // projected
    }

    pub fn mean_frame_time(&self) -> f32 {
        self.frame_times.average().unwrap_or_default()
    }

    pub fn mean_time_interval(&self) -> f32 {
        self.frame_times.mean_time_interval().unwrap_or_default()
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.mean_time_interval()
    }

    pub fn max_fps(&self) -> f32 {
        1.0 / self.mean_frame_time()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let mean_frame_time = 1e3 * self.mean_frame_time();
        let mean_frame_interval = 1e3 * self.mean_time_interval();
        let avg_fps = self.fps();
        let utilization =
            (1.0 - (mean_frame_interval - mean_frame_time).max(0.0) / mean_frame_interval) * 100.0;

        ui.horizontal(|ui| {
            ui.label("Frame Stats:");
            ui.label(format!("Count {}", ui.ctx().frame_nr())).on_hover_text("Total number of frames rendered");
            ui.label(format!("Duration {:.2} ms / frame", mean_frame_time)).on_hover_text("Single-thread CPU time for a single frame, excluding some GPU transfer times");
            ui.label(format!("Avg FPS {:.1}", avg_fps)).on_hover_text("Average frames per second. With VSync, will max out at the monitor's refresh rate, usually about 60 FPS");
            ui.label(format!("Util: {:.2}%", utilization)).on_hover_text("Single-thread CPU Render thread utilization, lower is better");
        });
    }

    pub fn graph(&mut self, ui: &mut egui::Ui) -> egui::Response {
        use egui::*;

        ui.label("CPU usage history");

        let history = &self.frame_times;

        let height = ui.spacing().slider_width;
        let size = vec2(ui.available_size_before_wrap().x, height);
        let (rect, response) = ui.allocate_at_least(size, Sense::hover());
        let style = ui.style().noninteractive();

        let graph_top_cpu_usage = 0.010;
        let graph_rect = Rect::from_x_y_ranges(history.max_age()..=0.0, graph_top_cpu_usage..=0.0);
        let to_screen = emath::RectTransform::from_to(graph_rect, rect);

        let mut shapes = Vec::with_capacity(3 + 2 * history.len());
        shapes.push(Shape::Rect(epaint::RectShape {
            rect,
            rounding: style.rounding,
            fill: ui.visuals().extreme_bg_color,
            stroke: ui.style().noninteractive().bg_stroke,
        }));

        let rect = rect.shrink(4.0);
        let color = ui.visuals().text_color();
        let line_stroke = Stroke::new(1.0, color);

        if let Some(pointer_pos) = response.hover_pos() {
            let y = pointer_pos.y;
            shapes.push(Shape::line_segment(
                [pos2(rect.left(), y), pos2(rect.right(), y)],
                line_stroke,
            ));
            let cpu_usage = to_screen.inverse().transform_pos(pointer_pos).y;
            let text = format!("{:.1} ms", 1e3 * cpu_usage);
            shapes.push(ui.fonts(|f| {
                Shape::text(
                    f,
                    pos2(rect.left(), y),
                    Align2::LEFT_BOTTOM,
                    text,
                    TextStyle::Monospace.resolve(ui.style()),
                    color,
                )
            }));
        }

        let circle_color = color;
        let radius = 2.0;
        let right_side_time = ui.input(|i| i.time); // Time at right side of screen

        for (time, cpu_usage) in history.iter() {
            let age = (right_side_time - time) as f32;
            let pos = to_screen.transform_pos_clamped(Pos2::new(age, cpu_usage));

            shapes.push(Shape::line_segment(
                [pos2(pos.x, rect.bottom()), pos],
                line_stroke,
            ));

            if cpu_usage < graph_top_cpu_usage {
                shapes.push(Shape::circle_filled(pos, radius, circle_color));
            }
        }

        ui.painter().extend(shapes);

        response
    }
}
