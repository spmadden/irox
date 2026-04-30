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
#[cfg(all(feature = "eframe", any(feature = "glow", feature = "wgpu")))]
pub use eframe;

pub use egui;

/// Historical frame rendering statistics
pub mod frame_history;

/// Utilities around [`egui::style`]
#[cfg(all(
    feature = "serde",
    feature = "eframe",
    any(feature = "glow", feature = "wgpu")
))]
pub mod styles;

/// [`eframe::App`] composition tools
#[cfg(all(feature = "eframe", any(feature = "glow", feature = "wgpu")))]
pub mod composite;

pub mod about;
/// A customization of [`egui::widgets::ProgressBar`]
pub mod progressbar;
/// A popup progress widget
pub mod progresswindow;

pub mod arrows;
pub mod drawpanel;
pub mod fontmesh;
pub mod fonts;
#[cfg(feature = "plots")]
pub mod logplot;
pub mod pwdlg;
pub mod repainting;
#[cfg(feature = "serde")]
pub mod serde;
pub mod testimage;
#[cfg(all(feature = "eframe", any(feature = "glow", feature = "wgpu")))]
pub mod toolframe;
mod utils;
pub mod visuals;
#[cfg(all(
    target_arch = "wasm32",
    feature = "eframe",
    any(feature = "glow", feature = "wgpu")
))]
pub mod wasm;

pub use utils::*;

pub trait WithAlpha {
    #[must_use]
    fn with_alpha(self, alpha: u8) -> Self;
}
impl WithAlpha for egui::Color32 {
    fn with_alpha(self, alpha: u8) -> Self {
        let [r, g, b, _] = self.to_srgba_unmultiplied();
        egui::Color32::from_rgba_unmultiplied(r, g, b, alpha)
    }
}

pub fn start_profiling() {
    #[cfg(all(feature = "profiling", not(target_arch = "wasm32")))]
    {
        let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
        let _puffin_server = puffin_http::Server::new(&server_addr);
        // eprintln!("Run this to view profiling data:  puffin_viewer {server_addr}");
        puffin::set_scopes_on(true);
        let _ = std::process::Command::new("puffin_viewer")
            .arg("--url")
            .arg(server_addr)
            .spawn();
        #[allow(clippy::mem_forget)]
        std::mem::forget(_puffin_server);
    }
}
