// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::styles::WithAlpha;
use egui::color_picker::show_color;
use egui::{vec2, RichText};

macro_rules! add_row {
    ($ui:ident, $name:expr, $col:expr) => {
        $ui.label($name);
        $ui.label(RichText::new("fg text").color($col));
        $ui.label(RichText::new("bg text").background_color($col));
        show_color($ui, $col, vec2(40., 15.));
        show_color($ui, WithAlpha::with_alpha($col, 128), vec2(40., 15.));
        $ui.label($col.to_hex());
        $ui.end_row();
    };
}

macro_rules! add_widget {
    ($ui:ident, $name:expr, $widget:expr) => {
        add_row!($ui, format!("{}.bg_fill", $name), $widget.bg_fill);
        add_row!($ui, format!("{}.weak_bg_fill", $name), $widget.weak_bg_fill);
        add_row!($ui, format!("{}.bg_stroke", $name), $widget.bg_stroke.color);
        add_row!($ui, format!("{}.fg_stroke", $name), $widget.fg_stroke.color);
    };
}

pub struct VisualsWindow;
impl VisualsWindow {
    pub fn show_visuals_window(ctx: &egui::Context) -> bool {
        let mut open = true;
        egui::Window::new("Visuals_Debug_Window")
            .constrain(true)
            .open(&mut open)
            .vscroll(true)
            .show(ctx, |ui| {
                let visuals = ui.visuals().clone();
                egui::Grid::new("Visuals_Debug_Grid")
                    .striped(true)
                    .num_columns(6)
                    .show(ui, |ui| {
                        add_row!(ui, "hyperlink_color", visuals.hyperlink_color);
                        add_row!(ui, "faint_bg_color", visuals.faint_bg_color);
                        add_row!(ui, "extreme_bg_color", visuals.extreme_bg_color);
                        add_row!(ui, "code_bg_color", visuals.code_bg_color);
                        add_row!(ui, "warn_fg_color", visuals.warn_fg_color);
                        add_row!(ui, "error_fg_color", visuals.error_fg_color);
                        add_row!(ui, "window_fill", visuals.window_fill);
                        add_row!(ui, "panel_fill", visuals.panel_fill);
                        add_row!(ui, "window_stroke", visuals.window_stroke.color);
                        add_row!(ui, "text_cursor", visuals.text_cursor.color);
                        add_widget!(ui, "noninteractive", visuals.widgets.noninteractive);
                        add_widget!(ui, "inactive", visuals.widgets.inactive);
                        add_widget!(ui, "hovered", visuals.widgets.hovered);
                        add_widget!(ui, "active", visuals.widgets.active);
                        add_widget!(ui, "open", visuals.widgets.open);
                    });
            });
        open
    }
}
