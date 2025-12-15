// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Augmentations and tweaks for a better progress bar

use egui::emath::lerp;
use egui::epaint::text::TextWrapMode;
use egui::{
    Color32, NumExt, Rect, Response, Rgba, Sense, Stroke, StrokeKind, TextStyle, Ui, Vec2, Widget,
    WidgetText,
};

///
/// A progress bar that supports both determinate (`[0->100]`) and indeterminate modes.
#[derive(Debug, Default, Clone)]
pub struct ProgressBar {
    /// Progress across the bar, in the range `[0->1]`
    pub progress: f32,
    /// Text to left align inside of the bar
    pub left_text: Option<String>,
    /// Text to center align inside of the bar
    pub center_text: Option<String>,
    /// Text to right align inside of the bar
    pub right_text: Option<String>,
    /// Request an immediate repaint of the bar
    pub animate: bool,
    /// The indicated progress is ignored, and the bar bounces back and forth
    pub is_indeterminate: bool,
    /// The desired draw width of the bar
    pub desired_width: Option<f32>,
    /// The desired draw height of the bar
    pub desired_height: Option<f32>,
}

impl ProgressBar {
    ///
    /// Creates a new progress bar with the specified determinate progress in the range `[0,1]`
    #[must_use]
    pub fn new(progress: f32) -> ProgressBar {
        ProgressBar {
            progress,
            ..Default::default()
        }
    }

    ///
    /// Creates a new progress bar that is indeterminate.  The little bar will bounce back and
    /// forth to indicate stuff is happening.
    #[must_use]
    pub fn indeterminate() -> ProgressBar {
        ProgressBar {
            is_indeterminate: true,
            ..Default::default()
        }
    }

    ///
    /// The same as [`Self::text_left`]
    #[must_use]
    pub fn text(self, text: String) -> ProgressBar {
        ProgressBar {
            left_text: Some(text),
            ..self
        }
    }

    ///
    /// Draws the provided text left aligned
    #[must_use]
    pub fn text_left(self, text: String) -> ProgressBar {
        self.text(text)
    }

    ///
    /// Draws the provided text center aligned
    #[must_use]
    pub fn text_center(self, text: String) -> ProgressBar {
        ProgressBar {
            center_text: Some(text),
            ..self
        }
    }

    ///
    /// Draws the provided text right aligned
    #[must_use]
    pub fn text_right(self, text: String) -> ProgressBar {
        ProgressBar {
            right_text: Some(text),
            ..self
        }
    }

    ///
    /// Sets the desired draw width of the bar
    #[must_use]
    pub fn desired_width(self, width: f32) -> ProgressBar {
        ProgressBar {
            desired_width: Some(width),
            ..self
        }
    }

    ///
    /// Sets the desired draw height of the bar
    #[must_use]
    pub fn desired_height(self, height: f32) -> ProgressBar {
        ProgressBar {
            desired_height: Some(height),
            ..self
        }
    }

    pub fn ui(self, ui: &mut Ui) -> Response {
        let ProgressBar {
            progress,
            desired_width,
            desired_height,
            left_text,
            center_text,
            right_text,
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
            ui.painter().rect(
                outer_rect,
                rounding,
                visuals.extreme_bg_color,
                Stroke::NONE,
                StrokeKind::Inside,
            );
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
                Color32::from(Rgba::from(visuals.selection.bg_fill) * color_factor),
                Stroke::NONE,
                StrokeKind::Inside,
            );

            if let Some(text) = left_text {
                let text: WidgetText = text.into();

                let galley = text.into_galley(
                    ui,
                    Some(TextWrapMode::Truncate),
                    f32::INFINITY,
                    TextStyle::Button,
                );
                let text_pos = outer_rect.left_center() - Vec2::new(0.0, galley.size().y / 2.0)
                    + egui::vec2(ui.spacing().item_spacing.x, 0.0);
                let text_color = visuals
                    .override_text_color
                    .unwrap_or(visuals.selection.stroke.color);
                ui.painter().galley(text_pos, galley, text_color);
            }
            if let Some(text) = center_text {
                let text: WidgetText = text.into();

                let galley = text.into_galley(
                    ui,
                    Some(TextWrapMode::Truncate),
                    f32::INFINITY,
                    TextStyle::Button,
                );
                let size = galley.size();
                let text_pos = outer_rect.center() - Vec2::new(size.x / 2.0, size.y / 2.0);
                let text_color = visuals
                    .override_text_color
                    .unwrap_or(visuals.selection.stroke.color);
                ui.painter().galley(text_pos, galley, text_color);
            }

            if let Some(text) = right_text {
                let text: WidgetText = text.into();

                let galley = text.into_galley(
                    ui,
                    Some(TextWrapMode::Truncate),
                    f32::INFINITY,
                    TextStyle::Button,
                );
                let size = galley.size();
                let text_pos = outer_rect.right_center()
                    - Vec2::new(size.x, size.y / 2.0)
                    - egui::vec2(ui.spacing().item_spacing.x, 0.0);
                let text_color = visuals
                    .override_text_color
                    .unwrap_or(visuals.selection.stroke.color);
                ui.painter().galley(text_pos, galley, text_color);
            }
        }

        response
    }
}

impl Widget for ProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        ProgressBar::ui(self, ui)
    }
}
