// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::widgets::{Editor, EditorResponse};
use egui::Ui;
use irox_types::fields::Field;
use std::marker::PhantomData;

pub struct RestrictedValueDropdown<'a, T: Clone + Default> {
    pub valid_values: &'a [T],
    _phan: PhantomData<T>,
}

impl<T: Clone + Default> Editor<'_> for RestrictedValueDropdown<'_, T> {
    type Value = T;

    fn show(&self, _field: &mut Field<Self::Value>, _ui: &mut Ui) -> EditorResponse {
        todo!()
    }
}
