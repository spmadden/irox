// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use egui::{Checkbox, Ui, Widget};

pub struct NullableBooleanEditor<'a> {
    name: &'a str,
    initial_value: Option<bool>,
    default_value: bool,
}

impl<'a> NullableBooleanEditor<'a> {
    pub fn new(name: &'a str, initial_value: Option<bool>, default_value: bool) -> Self {
        Self {
            name,
            initial_value,
            default_value,
        }
    }
    pub fn show(&self, ui: &mut Ui) -> Option<bool> {
        if let Some(mut initial) = self.initial_value {
            Checkbox::new(&mut initial, self.name).ui(ui);
            Some(initial)
        } else {
            let mut out = self.default_value;
            Checkbox::new(&mut out, self.name)
                .indeterminate(true)
                .ui(ui)
                .changed()
                .then_some(out)
        }
    }
}
