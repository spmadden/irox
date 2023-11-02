// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use eframe::emath::lerp;
use egui::{
    Color32, NumExt, Rect, Response, Rgba, Sense, Stroke, TextStyle, Ui, Vec2, Widget, WidgetText,
};

#[derive(Debug, Default, Clone)]
pub struct ProgressBar {
    progress: f32,
    text: String,
    animate: bool,
    is_indeterminate: bool,
    desired_width: Option<f32>,
    desired_height: Option<f32>,
}

impl ProgressBar {
    #[must_use]
    pub fn new(progress: f32) -> ProgressBar {
        ProgressBar {
            progress,
            ..Default::default()
        }
    }
    #[must_use]
    pub fn indeterminate() -> ProgressBar {
        ProgressBar {
            is_indeterminate: true,
            ..Default::default()
        }
    }
    #[must_use]
    pub fn text(self, text: String) -> ProgressBar {
        ProgressBar { text, ..self }
    }
    #[must_use]
    pub fn desired_width(self, width: f32) -> ProgressBar {
        ProgressBar {
            desired_width: Some(width),
            ..self
        }
    }
    #[must_use]
    pub fn desired_height(self, height: f32) -> ProgressBar {
        ProgressBar {
            desired_height: Some(height),
            ..self
        }
    }
}

impl Widget for ProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let ProgressBar {
            progress,
            desired_width,
            desired_height,
            text,
            animate,
            is_indeterminate,
        } = self;

        let animate = animate && progress < 1.0;

        let desired_width =
            desired_width.unwrap_or_else(|| ui.available_size_before_wrap().x.at_least(96.0));
        let height = desired_height.unwrap_or(ui.spacing().interact_size.y);
        let (outer_rect, response) =
            ui.allocate_exact_size(egui::vec2(desired_width, height), Sense::hover());

        if ui.is_rect_visible(response.rect) {
            if animate {
                ui.ctx().request_repaint();
            }

            let visuals = ui.style().visuals.clone();
            let rounding = outer_rect.height() / 2.0;
            let time = ui.input(|i| i.time).cos().abs() as f32;
            ui.painter()
                .rect(outer_rect, rounding, visuals.extreme_bg_color, Stroke::NONE);
            let inner_rect = if is_indeterminate {
                let max_x = outer_rect.width() * 0.8;
                let offset = lerp(0.0f32..=max_x, time);
                Rect::from_min_size(
                    egui::pos2(outer_rect.min.x + offset, outer_rect.min.y),
                    egui::vec2(outer_rect.width() * 0.2, outer_rect.height()),
                )
            } else {
                Rect::from_min_size(
                    outer_rect.min,
                    egui::vec2(
                        (outer_rect.width() * progress).at_least(outer_rect.height()),
                        outer_rect.height(),
                    ),
                )
            };

            let (dark, bright) = (0.7, 1.0);
            let color_factor = lerp(dark..=bright, time);

            ui.painter().rect(
                inner_rect,
                rounding,
                Color32::from(Rgba::from(visuals.selection.bg_fill) * color_factor as f32),
                Stroke::NONE,
            );

            let text: WidgetText = text.into();

            let galley = text.into_galley(ui, Some(false), f32::INFINITY, TextStyle::Button);
            let text_pos = outer_rect.left_center() - Vec2::new(0.0, galley.size().y / 2.0)
                + egui::vec2(ui.spacing().item_spacing.x, 0.0);
            let text_color = visuals
                .override_text_color
                .unwrap_or(visuals.selection.stroke.color);
            galley.paint_with_fallback_color(
                &ui.painter().with_clip_rect(outer_rect),
                text_pos,
                text_color,
            );
        }

        response
    }
}
