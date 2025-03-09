// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Stuff that should have been in [`egui`], but isn't.
//!

#[macro_export]
macro_rules! profile_scope {
    ($name:expr) => {
        #[cfg(feature = "profiling")]
        profiling::scope!($name);
    };
    ($name:expr, $data:expr) => {
        #[cfg(feature = "profiling")]
        profiling::scope!($name, $data);
    };
}

/// Historical frame rendering statistics
pub mod frame_history;

/// Utilities around [`egui::style`]
#[cfg(feature = "serde")]
pub mod styles;

/// [`eframe::App`] composition tools
pub mod composite;

pub mod about;
/// A customization of [`egui::widgets::ProgressBar`]
pub mod progressbar;
/// A popup progress widget
pub mod progresswindow;

pub mod fonts;
#[cfg(feature = "plots")]
pub mod logplot;
pub mod repainting;
#[cfg(feature = "serde")]
pub mod serde;
pub mod toolframe;
pub mod visuals;

pub trait WithAlpha {
    #[must_use]
    fn with_alpha(self, alpha: u8) -> Self;
}
impl WithAlpha for egui::Color32 {
    #[must_use]
    fn with_alpha(self, alpha: u8) -> Self {
        let [r, g, b, _] = self.to_srgba_unmultiplied();
        egui::Color32::from_rgba_unmultiplied(r, g, b, alpha)
    }
}
