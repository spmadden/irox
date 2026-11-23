// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use super::{clear_button, field_label, show_null_warning, Editor, EditorResponse, Field};
use egui::{text_edit::TextEditState, Id, IdSalt, TextEdit, Ui};

/// An ordered array of text inputs with controls for adding and removing rows.
///
/// Removing the last row leaves an empty array. Clear sets a nullable field to NULL.
pub struct StringListEditor;

impl Editor<'_> for StringListEditor {
    type Value = Vec<String>;

    fn show(&self, field: &mut Field<Vec<String>>, ui: &mut Ui) -> EditorResponse {
        let salt = IdSalt::new(("string_list", &field.descriptor.name));
        let result = ui.push_id(salt, |ui| {
            let label = field_label(ui, &field.descriptor);
            let rows_id = ui.make_persistent_id("rows");
            let mut changed = false;
            let mut remove_row = None;
            if let Some(values) = field.value.as_mut() {
                for (index, value) in values.iter_mut().enumerate() {
                    ui.push_id(index, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            let mut text = value.clone();
                            let response = ui
                                .add(TextEdit::singleline(&mut text).id(rows_id.with(index)))
                                .labelled_by(label.id);
                            if response.changed() && text != *value {
                                *value = text;
                                changed = true;
                            }
                            if ui.is_enabled() && ui.button("Remove").clicked() {
                                remove_row = Some(index);
                            }
                        });
                    });
                }
            } else {
                ui.label("NULL");
            }

            let (add, clear) = if ui.is_enabled() {
                ui.horizontal_wrapped(|ui| {
                    (ui.button("Add item").clicked(), clear_button(ui, field))
                })
                .inner
            } else {
                (false, false)
            };
            if clear {
                if let Some(values) = field.value.take() {
                    clear_row_state(ui, rows_id, 0..values.len());
                    changed = true;
                }
            } else if let Some((index, values)) = remove_row.zip(field.value.as_mut()) {
                // Discard caret and undo state for rows whose IDs are reused.
                clear_row_state(ui, rows_id, index..values.len());
                values.remove(index);
                changed = true;
            } else if add {
                field.value.get_or_insert_with(Vec::new).push(String::new());
                changed = true;
            }
            show_null_warning(ui, field.is_valid());
            changed
        });
        let mut response = result.response;
        if result.inner {
            response.mark_changed();
        }
        EditorResponse {
            valid: field.is_valid(),
            response,
        }
    }
}

fn clear_row_state(ui: &Ui, rows_id: Id, indices: std::ops::Range<usize>) {
    for index in indices {
        let id = rows_id.with(index);
        ui.memory_mut(|memory| memory.surrender_focus(id));
        ui.data_mut(|data| data.remove::<TextEditState>(id));
    }
}
