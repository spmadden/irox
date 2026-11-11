// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Identifier {
    internal: irox_tools::identifier::Identifier,
    external: egui::Id,
}
impl<T: Into<irox_tools::identifier::Identifier>> From<T> for Identifier {
    fn from(value: T) -> Self {
        let internal = value.into();
        Self {
            external: egui::Id::unique(&internal),
            internal,
        }
    }
}
impl Identifier {
    pub fn internal(&self) -> &irox_tools::identifier::Identifier {
        &self.internal
    }
    pub fn external(&self) -> egui::Id {
        self.external
    }
    #[must_use]
    pub fn with<T: Into<irox_tools::identifier::Identifier>>(&self, o: T) -> Identifier {
        self.internal.with(o).into()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.internal(), f)
    }
}
