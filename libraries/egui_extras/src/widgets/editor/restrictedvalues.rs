// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use std::fmt::Display;

use super::{field_label, hover_help, show_null_warning, Editor, EditorResponse, Field};
use egui::{ComboBox, Ui};

/// A value that can be selected in a restricted dropdown.
pub struct RestrictedValue<T> {
    pub value: T,
    /// Text shown when the pointer hovers over this option.
    pub help_description: Option<String>,
}

impl<T> RestrictedValue<T> {
    /// Create an option with no help text.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self {
            value,
            help_description: None,
        }
    }
}

/// A combo box whose options are both the displayed labels and the stored values.
pub struct RestrictedValueDropdown<'a, T: Clone + Default> {
    options: RestrictedOptions<'a, T>,
}

enum RestrictedOptions<'a, T> {
    Values(&'a [T]),
    Described(&'a [RestrictedValue<T>]),
}

impl<'a, T: Clone + Default> RestrictedValueDropdown<'a, T> {
    /// Borrow values directly from the caller.
    #[must_use]
    pub fn new(values: &'a [T]) -> Self {
        Self {
            options: RestrictedOptions::Values(values),
        }
    }

    /// Borrow options with optional help text.
    #[must_use]
    pub fn with_options(options: &'a [RestrictedValue<T>]) -> Self {
        Self {
            options: RestrictedOptions::Described(options),
        }
    }
}

impl<T: Clone + Default + PartialEq + Display> Editor<'_> for RestrictedValueDropdown<'_, T> {
    type Value = T;

    fn show(&self, field: &mut Field<T>, ui: &mut Ui) -> EditorResponse {
        match self.options {
            RestrictedOptions::Values(values) => show_dropdown(
                ui,
                field,
                "restricted_value",
                values.iter().map(|value| (value, value, None)),
            ),
            RestrictedOptions::Described(options) => show_dropdown(
                ui,
                field,
                "restricted_value",
                options.iter().map(|option| {
                    (
                        &option.value,
                        &option.value,
                        option.help_description.as_deref(),
                    )
                }),
            ),
        }
    }
}

/// A single-select relation editor that displays labels and stores typed keys.
///
/// The caller supplies rows in display order with one row per key. Refresh the
/// options by passing the current rows each frame. A stored key missing from the
/// current rows remains visible and can be replaced.
pub struct ValueRelationEditor<'a, K, L> {
    rows: &'a [(K, L)],
}

impl<'a, K, L> ValueRelationEditor<'a, K, L> {
    #[must_use]
    pub fn new(rows: &'a [(K, L)]) -> Self {
        Self { rows }
    }
}

impl<K, L> Editor<'_> for ValueRelationEditor<'_, K, L>
where
    K: Clone + Default + PartialEq + Display,
    L: AsRef<str>,
{
    type Value = K;

    fn show(&self, field: &mut Field<K>, ui: &mut Ui) -> EditorResponse {
        show_dropdown(
            ui,
            field,
            "value_relation",
            self.rows
                .iter()
                .map(|(key, label)| (key, label.as_ref(), None)),
        )
    }
}

fn show_dropdown<'row, K, L, I>(
    ui: &mut Ui,
    field: &mut Field<K>,
    editor_id: &str,
    options: I,
) -> EditorResponse
where
    K: Clone + Default + PartialEq + Display + 'row,
    L: Display,
    I: Iterator<Item = (&'row K, L, Option<&'row str>)> + Clone,
{
    let selected = options
        .clone()
        .find(|(key, _, _)| field.value.as_ref() == Some(*key));
    let (text, missing_key) = match (selected, &field.value) {
        (Some((_, label, _)), _) => (label.to_string(), false),
        (None, Some(key)) => (format!("Unavailable: {key}"), true),
        (None, None) => (String::new(), false),
    };
    if !ui.is_enabled() {
        let response = ui
            .horizontal_wrapped(|ui| {
                let label = field_label(ui, &field.descriptor);
                ui.label(text).labelled_by(label.id)
            })
            .inner;
        let valid = show_dropdown_warning(ui, field, missing_key);
        return EditorResponse { valid, response };
    }
    let result = ComboBox::new(
        (editor_id, &field.descriptor.name),
        &field.descriptor.display_name,
    )
    .selected_text(text)
    .show_ui(ui, |ui| {
        if field.descriptor.nullable && ui.selectable_label(field.value.is_none(), "").clicked() {
            ui.close();
            return field.value.take().is_some();
        }
        for (key, label, description) in options {
            let selected = field.value.as_ref() == Some(key);
            let option = hover_help(
                ui.selectable_label(selected, label.to_string()),
                description,
            );
            if option.clicked() {
                ui.close();
                if !selected {
                    field.value = Some(key.clone());
                }
                return !selected;
            }
        }
        false
    });
    let changed = result.inner.unwrap_or(false);
    let mut response = hover_help(
        result.response,
        field.descriptor.help_description.as_deref(),
    );
    if changed {
        response.mark_changed();
    }

    let valid = show_dropdown_warning(ui, field, missing_key && !changed);
    EditorResponse { valid, response }
}

fn show_dropdown_warning<T: Clone + Default>(
    ui: &mut Ui,
    field: &Field<T>,
    missing_key: bool,
) -> bool {
    if missing_key {
        ui.colored_label(
            ui.visuals().warn_fg_color,
            "Value is absent from the options",
        );
        false
    } else {
        let valid = field.is_valid();
        show_null_warning(ui, valid);
        valid
    }
}
