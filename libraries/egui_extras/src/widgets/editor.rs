// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

mod restrictedvalues;
mod stringlist;
mod uniquevalues;

pub use restrictedvalues::*;
pub use stringlist::*;
pub use uniquevalues::*;

use egui::{Checkbox, IdSalt, Response, Ui};
pub use irox_types::fields::*;

/// The outcome of rendering a field editor.
pub struct EditorResponse {
    /// Whether the resulting value satisfies the field and editor constraints.
    pub valid: bool,
    /// The editor response, marked changed when the stored value changes.
    pub response: Response,
}

/// An egui control bound to a typed field.
///
/// A disabled UI displays the field and its validation warnings in read-only mode.
///
/// When showing several records in the same UI, give each record its own
/// `Ui::push_id` scope keyed by the record so repeated field names have distinct IDs.
pub trait Editor<'a> {
    type Value: Clone + Default;
    fn show(&self, field: &'a mut Field<Self::Value>, ui: &'a mut Ui) -> EditorResponse;
}

/// A checkbox that displays NULL as indeterminate.
pub struct BooleanEditor;

impl Editor<'_> for BooleanEditor {
    type Value = bool;

    fn show(&self, field: &mut Field<bool>, ui: &mut Ui) -> EditorResponse {
        let salt = IdSalt::new(("boolean", &field.descriptor.name));
        ui.push_id(salt, |ui| {
            ui.horizontal_wrapped(|ui| {
                let mut checked = field.value.unwrap_or(field.descriptor.default_value);
                let mut response = hover_help(
                    ui.add(
                        Checkbox::new(&mut checked, &field.descriptor.display_name)
                            .indeterminate(field.value.is_none()),
                    ),
                    field.descriptor.help_description.as_deref(),
                );
                let mut clear = clear_button(ui, field);
                if ui.is_enabled() && field.descriptor.nullable && field.value.is_some() {
                    response.context_menu(|ui| {
                        if ui.button("Clear").clicked() {
                            clear = true;
                            ui.close();
                        }
                    });
                }
                if clear {
                    field.value = None;
                    response.mark_changed();
                } else if response.changed() {
                    field.value = Some(checked);
                }

                let valid = field.is_valid();
                show_null_warning(ui, valid);
                EditorResponse { valid, response }
            })
            .inner
        })
        .inner
    }
}

fn hover_help(response: Response, description: Option<&str>) -> Response {
    match description {
        Some(description) if !response.enabled() => response.on_disabled_hover_text(description),
        Some(description) => response.on_hover_text(description),
        None => response,
    }
}

fn field_label<T: Clone + Default>(ui: &mut Ui, descriptor: &FieldDescriptor<T>) -> Response {
    hover_help(
        ui.label(&descriptor.display_name),
        descriptor.help_description.as_deref(),
    )
}

fn clear_button<T: Clone + Default>(ui: &mut Ui, field: &Field<T>) -> bool {
    ui.is_enabled()
        && field.descriptor.nullable
        && ui
            .add_enabled(field.value.is_some(), egui::Button::new("Clear"))
            .clicked()
}

fn show_null_warning(ui: &mut Ui, valid: bool) {
    if !valid {
        ui.colored_label(ui.visuals().warn_fg_color, "A value is required");
    }
}

#[cfg(test)]
mod tests;
