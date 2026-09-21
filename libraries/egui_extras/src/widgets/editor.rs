// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use egui::{Checkbox, Color32, Frame, Ui, Widget};
pub use irox_types::fields::*;

pub struct EditorResponse {
    pub valid: bool,
    pub response: egui::Response,
}

pub trait Editor {
    type Value: Clone + Default;
    fn show(&self, field: &mut Field<Self::Value>, ui: &mut Ui) -> EditorResponse;
}

pub struct BooleanEditor;

impl Editor for BooleanEditor {
    type Value = bool;

    fn show(&self, field: &mut Field<bool>, ui: &mut Ui) -> EditorResponse {
        let fill = if !field.is_valid() {
            Color32::ORANGE
        } else {
            Color32::TRANSPARENT
        };

        let resp = Frame::NONE
            .fill(fill)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let mut value = field.value.take();
                    let resp = if let Some(initial) = &mut value {
                        ui.label(&field.descriptor.display_name);
                        let resp = Checkbox::without_text(initial).ui(ui);
                        resp.context_menu(|ui| {
                            if ui.button("Clear").clicked() {
                                value = None;
                            }
                        });
                        resp
                    } else {
                        let mut out = field.descriptor.default_value;
                        ui.label(&field.descriptor.display_name);
                        let resp = Checkbox::without_text(&mut out).indeterminate(true).ui(ui);
                        if resp.changed() {
                            value = Some(out);
                        }
                        resp
                    };

                    field.value = value;

                    if field.is_valid() {
                        ui.label("\u{2714}");
                    } else {
                        ui.label("\u{26A0}");
                    }

                    resp
                })
                .inner
            })
            .inner;

        EditorResponse {
            valid: field.is_valid(),
            response: resp,
        }
    }
}
