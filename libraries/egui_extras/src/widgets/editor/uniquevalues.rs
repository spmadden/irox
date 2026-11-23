// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use super::{
    clear_button, field_label, show_null_warning, Editor, EditorResponse, Field, RestrictedValue,
    RestrictedValueDropdown,
};
use egui::text::{CCursor, CCursorRange};
use egui::{EventFilter, Id, IdSalt, Key, Modifiers, Popup, Response, TextEdit, Ui};

/// A string editor backed by values supplied by the caller.
pub struct UniqueValuesEditor<'a> {
    mode: UniqueValuesMode<'a>,
}

enum UniqueValuesMode<'a> {
    Editable(&'a [String]),
    Restricted(RestrictedValueDropdown<'a, String>),
}

impl<'a> UniqueValuesEditor<'a> {
    /// Allow arbitrary text with autocomplete from the supplied values.
    ///
    /// Suggestions preserve caller order and use case-insensitive prefix matching.
    /// Up and Down choose a suggestion, Enter accepts it, and Escape dismisses it.
    #[must_use]
    pub fn editable(values: &'a [String]) -> Self {
        Self {
            mode: UniqueValuesMode::Editable(values),
        }
    }

    /// Borrow the supplied distinct values for a restricted dropdown.
    #[must_use]
    pub fn restricted(values: &'a [String]) -> Self {
        Self {
            mode: UniqueValuesMode::Restricted(RestrictedValueDropdown::new(values)),
        }
    }

    /// Offer the supplied distinct values with optional help text for each choice.
    #[must_use]
    pub fn restricted_with_options(options: &'a [RestrictedValue<String>]) -> Self {
        Self {
            mode: UniqueValuesMode::Restricted(RestrictedValueDropdown::with_options(options)),
        }
    }

    fn show_editable(values: &[String], field: &mut Field<String>, ui: &mut Ui) -> EditorResponse {
        let salt = IdSalt::new(("unique_values", &field.descriptor.name));
        let result = ui.push_id(salt, |ui| {
            let label = field_label(ui, &field.descriptor);
            let changed = ui
                .horizontal_wrapped(|ui| Self::show_input(ui, field, label.id, values))
                .inner;
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

    fn show_input(ui: &mut Ui, field: &mut Field<String>, label: Id, values: &[String]) -> bool {
        let id = ui.make_persistent_id("text");
        let mut text = field.value.clone().unwrap_or_default();
        let input = TextEdit::singleline(&mut text)
            .id(id)
            .hint_text(if field.value.is_none() { "NULL" } else { "" })
            .return_key(None);
        if !ui.is_enabled() {
            ui.memory_mut(|memory| memory.surrender_focus(id));
            ui.data_mut(|data| data.remove::<CompletionState>(id));
            ui.add(input).labelled_by(label);
            return false;
        }
        let mut state = ui
            .data(|data| data.get_temp::<CompletionState>(id))
            .unwrap_or_default();
        let navigation = state.navigation(ui, id);

        let edit = ui
            .add(input.event_filter(state.event_filter()))
            .labelled_by(label);
        if edit.changed() || edit.gained_focus() {
            state.dismissed = false;
            state.highlight = 0;
        }
        let clear = clear_button(ui, field);

        // A click on a suggestion can take focus away from the text input.
        let matches = if !clear
            && !state.dismissed
            && (edit.has_focus() || (state.visible && selecting_with_pointer(&edit)))
        {
            suggestions(values, &text)
        } else {
            Vec::new()
        };
        let active = !matches.is_empty();
        let escape = edit.has_focus()
            && active
            && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape));
        let enter = edit.has_focus()
            && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter));
        let selected = if active && !escape {
            state.update_highlight(navigation, matches.len());
            if enter {
                matches.get(state.highlight).copied()
            } else {
                show_suggestions(&edit, &matches, state.highlight)
            }
        } else {
            if enter {
                edit.surrender_focus();
            }
            None
        };

        if let Some(value) = selected {
            edit.request_focus();
            if let Some(mut text_state) = TextEdit::load_state(ui.ctx(), id) {
                text_state
                    .cursor
                    .set_char_range(Some(CCursorRange::one(CCursor::new(value.chars().count()))));
                TextEdit::store_state(ui.ctx(), id, text_state);
            }
        }
        state.dismissed |= clear || escape || selected.is_some();
        state.visible = active && !state.dismissed;
        if edit.has_focus() {
            ui.memory_mut(|memory| memory.set_focus_lock_filter(id, state.event_filter()));
            ui.data_mut(|data| data.insert_temp(id, state));
        } else {
            ui.data_mut(|data| data.remove::<CompletionState>(id));
        }

        if clear {
            return field.value.take().is_some();
        }
        let new_value = selected
            .map(str::to_owned)
            .or_else(|| edit.changed().then_some(text));
        if let Some(value) = new_value.filter(|value| field.value.as_ref() != Some(value)) {
            field.value = Some(value);
            true
        } else {
            false
        }
    }
}

impl Editor<'_> for UniqueValuesEditor<'_> {
    type Value = String;

    fn show(&self, field: &mut Field<String>, ui: &mut Ui) -> EditorResponse {
        match &self.mode {
            UniqueValuesMode::Editable(values) => Self::show_editable(values, field, ui),
            UniqueValuesMode::Restricted(dropdown) => dropdown.show(field, ui),
        }
    }
}

fn suggestions<'a>(values: &'a [String], text: &str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    if !text.is_empty() {
        let prefix = text.to_lowercase();
        for value in values {
            let value = value.as_str();
            if value != text
                && !matches.contains(&value)
                && value.to_lowercase().starts_with(&prefix)
            {
                matches.push(value);
                if matches.len() == 8 {
                    break;
                }
            }
        }
    }
    matches
}

#[derive(Clone, Default)]
struct CompletionState {
    highlight: usize,
    visible: bool,
    dismissed: bool,
}

impl CompletionState {
    fn event_filter(&self) -> EventFilter {
        EventFilter {
            horizontal_arrows: true,
            vertical_arrows: true,
            escape: self.visible,
            ..Default::default()
        }
    }

    fn navigation(&self, ui: &Ui, id: Id) -> Option<Key> {
        if !self.visible || !ui.memory(|memory| memory.has_focus(id)) {
            return None;
        }
        ui.input_mut(|input| {
            [Key::ArrowDown, Key::ArrowUp]
                .into_iter()
                .find(|&key| input.consume_key(Modifiers::NONE, key))
        })
    }

    fn update_highlight(&mut self, navigation: Option<Key>, count: usize) {
        let next = match navigation {
            Some(Key::ArrowDown) => self.highlight.saturating_add(1),
            Some(Key::ArrowUp) => self.highlight.saturating_sub(1),
            _ => self.highlight,
        };
        self.highlight = next.min(count.saturating_sub(1));
    }
}

fn selecting_with_pointer(edit: &Response) -> bool {
    edit.lost_focus()
        && edit
            .ctx
            .read_response(Popup::default_response_id(edit))
            .is_some_and(|popup| popup.contains_pointer())
        && edit
            .ctx
            .input(|input| input.pointer.any_pressed() || input.pointer.any_released())
}

fn show_suggestions<'a>(edit: &Response, matches: &[&'a str], highlight: usize) -> Option<&'a str> {
    let popup = Popup::from_response(edit)
        .width(edit.rect.width())
        .show(|ui| {
            matches.iter().enumerate().find_map(|(index, &value)| {
                ui.selectable_label(index == highlight, value)
                    .clicked()
                    .then_some(value)
            })
        })?;
    // Keep the popup open through release if egui surrenders focus on press.
    if popup.response.contains_pointer() && edit.ctx.input(|input| input.pointer.any_down()) {
        edit.request_focus();
    }
    popup.inner
}
