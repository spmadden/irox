// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Stuff that should have been in [`egui`], but isn't.
//!

/// Historical frame rendering statistics
pub mod frame_history;

/// Utilities around [`egui::style`]
#[cfg(feature = "serde")]
pub mod styles;

/// [`eframe::App`] composition tools
pub mod composite;

/// A customization of [`egui::widgets::ProgressBar`]
pub mod progressbar;

#[cfg(feature = "plots")]
pub mod logplot;
#[cfg(feature = "serde")]
pub mod serde;
pub mod toolframe;
