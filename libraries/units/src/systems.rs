// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CoordinateSystem {
    #[default]
    Model,
    Screen,
    Other,
}

pub trait CoordinateTranslator<M, S> {
    fn to_model(&self, point: &S) -> M;
    fn to_screen(&self, point: &M) -> S;
}
