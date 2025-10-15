// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Adds a helper App Wrapper called 'ToolFrame' that provides a boilerplate tool app for quick bootstrapping of new apps.

use crate::frame_history::FrameHistory;
use eframe::emath::Align;
use eframe::{App, CreationContext, Frame, Storage};
use egui::{
    menu, Context, Id, Layout, RawInput, ThemePreference, TopBottomPanel, Ui, ViewportCommand,
    Visuals, Window,
};
use std::time::Duration;

///
/// A 'ToolFrame' is a egui App that provides a basic Menu bar, Bottom Status Bar, and pre-fills it with some utilities
/// like the Style UI, and rendering statistics.
///
/// Purpose is to reduce the amount of duplicated code across multiple tools.
pub struct ToolFrame {
    style_ui: bool,
    full_speed: bool,
    show_rendering_stats: bool,
    show_file_menu: bool,
    frame_history: FrameHistory,
    child: Box<dyn ToolApp>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ToolFrameOptions {
    pub full_speed: bool,
    pub show_rendering_stats: bool,
    pub show_file_menu: bool,
}
impl Default for ToolFrameOptions {
    fn default() -> Self {
        Self {
            full_speed: false,
            show_rendering_stats: false,
            show_file_menu: true,
        }
    }
}

impl ToolFrame {
    #[must_use]
    pub fn new(_cc: &CreationContext, child: Box<dyn ToolApp>) -> Self {
        Self::new_opts(_cc, child, ToolFrameOptions::default())
    }
    #[must_use]
    pub fn new_opts(
        _cc: &CreationContext,
        child: Box<dyn ToolApp>,
        opts: ToolFrameOptions,
    ) -> Self {
        let ToolFrameOptions {
            full_speed,
            show_rendering_stats,
            show_file_menu,
        } = opts;
        Self {
            style_ui: false,
            full_speed,
            show_rendering_stats,
            show_file_menu,
            frame_history: FrameHistory::default(),
            child,
        }
    }

    pub fn show_rendering_stats(&mut self, show: bool) {
        self.show_rendering_stats = show;
    }
    pub fn full_speed(&mut self, full_speed: bool) {
        self.full_speed = full_speed;
    }
}

impl App for ToolFrame {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);

        TopBottomPanel::top(Id::new("top_panel")).show(ctx, |ui| {
            menu::bar(ui, |ui| {
                if self.show_file_menu {
                    ui.menu_button("File", |ui| {
                        self.child.file_menu(ui);

                        if ui.button("Exit").clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }
                    });
                }

                ui.menu_button("Settings", |ui| {
                    self.child.settings_menu(ui);
                    ui.checkbox(&mut self.show_rendering_stats, "Show Rendering Metrics");
                    ui.checkbox(&mut self.full_speed, "Continuous Render");

                    if ui.button("Style").clicked() {
                        self.style_ui = true;
                        ui.close_menu();
                    }
                });

                self.child.menu(ui);
            });
        });

        if self.style_ui {
            Window::new("style")
                .open(&mut self.style_ui)
                .show(ctx, |ui| {
                    let mut theme = ctx.options(|o| o.theme_preference);
                    theme.radio_buttons(ui);
                    if theme != ctx.options(|o| o.theme_preference) {
                        ctx.set_theme(theme);
                    }
                    ctx.style_ui(ui, ctx.theme());
                });
        }

        TopBottomPanel::bottom(Id::new("bottom_panel")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.show_rendering_stats {
                    self.frame_history.ui(ui);
                }

                self.child.bottom_bar(ui);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ctx.style().visuals.dark_mode {
                        if ui.button("\u{2600}").clicked() {
                            ctx.set_theme(ThemePreference::Light);
                        }
                    } else {
                        if ui.button("\u{1F318}").clicked() {
                            ctx.set_theme(ThemePreference::Dark);
                        }
                    }
                });
            });
        });

        self.child.update(ctx, frame);

        if self.full_speed {
            ctx.request_repaint();
        }
    }
    #[cfg(target_arch = "wasm32")]
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        self.child.as_any_mut()
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        self.child.save(storage)
    }
    #[cfg(feature = "glow")]
    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        self.child.on_exit(gl)
    }

    #[cfg(not(feature = "glow"))]
    fn on_exit(&mut self) {
        self.child.on_exit()
    }

    fn auto_save_interval(&self) -> Duration {
        self.child.auto_save_interval()
    }

    fn clear_color(&self, visuals: &Visuals) -> [f32; 4] {
        self.child.clear_color(visuals)
    }

    fn persist_egui_memory(&self) -> bool {
        self.child.persist_egui_memory()
    }

    fn raw_input_hook(&mut self, ctx: &Context, raw_input: &mut RawInput) {
        self.child.raw_input_hook(ctx, raw_input)
    }
}

///
/// Downstream users should implement this trait.
///
/// * Implement `menu` to append new menu items to the menu bar
/// * Implement `file_menu` to append new items to the file menu in the menu bar
/// * Implement `settings_menu` to append new items to the settings menu in the menu bar
/// * Implement `bottom_bar` to append new items to the bottom status bar
///
/// [`egui::App`] is a required trait.  `update` will be called LAST, and is appropriate for adding a [`egui::CentralPanel`]
pub trait ToolApp: App {
    /// Appends stuff to the menu
    fn menu(&mut self, _ui: &mut Ui) {}

    /// Appends bits to the file menu, 'Exit' is always last.
    fn file_menu(&mut self, _ui: &mut Ui) {}

    /// Appends bits to the settings menu.
    fn settings_menu(&mut self, _ui: &mut Ui) {}

    /// Appends bits to the bottom bar/panel
    fn bottom_bar(&mut self, _ui: &mut Ui) {}
}
