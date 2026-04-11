// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use egui::emath::Align;
use egui::{Id, Key, Layout, Modifiers, TextEdit, Ui, Vec2, ViewportCommand, Widget};
use irox_tools::cfg_feature_eframe;
use log::error;
use std::sync::mpsc::Sender;

///
/// A dialog that can pop up in a new frame or be rendered within another panel
pub struct DialogWidget {
    options: Details,
    complete: bool,
    sender: Sender<UserResponse>,
    input: String,
}
impl DialogWidget {
    pub fn new(options: Details, sender: Sender<UserResponse>) -> Self {
        Self {
            options,
            sender,
            input: String::new(),
            complete: false,
        }
    }

    ///
    /// Renders a message-type dialog, that displays a label
    pub fn render_message(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(self.options.prompt.as_deref().unwrap_or_default());
            ui.end_row();
            self.check_interaction(ui);
        });
    }
    ///
    /// Renders a password dialog that displays an password-masked input and a
    /// prompt
    pub fn render_password(&mut self, ui: &mut egui::Ui) {
        {
            let ctx = ui.ctx();
            if self.options.dialog_type.is_input() && ctx.cumulative_pass_nr() == 0 {
                ctx.memory_mut(|m| {
                    m.request_focus(Id::new("pwdlg-inp"));
                });
            }
        }
        ui.vertical_centered(|ui| {
            ui.allocate_ui_with_layout(
                Vec2::new(150., 0.),
                Layout::left_to_right(Align::TOP),
                |ui| {
                    ui.label(self.options.prompt.as_deref().unwrap_or_default());
                    let _resp = TextEdit::singleline(&mut self.input)
                        .id(Id::new("pwdlg-inp"))
                        .password(true)
                        .clip_text(true)
                        .min_size(Vec2::new(150., 0.))
                        .desired_width(100f32)
                        .hint_text("hunter2")
                        .ui(ui);
                    if ui
                        .ctx()
                        .input_mut(|v| v.consume_key(Modifiers::NONE, Key::Enter))
                    {
                        self.send_result(ui, UserResponse::OkInput(self.input.clone()));
                    } else if ui
                        .ctx()
                        .input_mut(|v| v.consume_key(Modifiers::NONE, Key::Escape))
                    {
                        self.send_result(ui, UserResponse::Cancel);
                    }
                },
            );
            ui.end_row();
            self.check_interaction(ui);
        });
    }

    fn check_interaction(&mut self, ui: &mut egui::Ui) {
        if let Some(resp) = self.render_interaction(ui) {
            self.send_result(ui, resp);
        }
    }

    fn send_result(&mut self, ui: &mut Ui, resp: UserResponse) {
        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        if let Err(e) = self.sender.send(resp) {
            error!("{e:?}");
        } else {
            self.complete = true;
        }
    }

    fn render_interaction(&mut self, ui: &mut egui::Ui) -> Option<UserResponse> {
        ui.allocate_ui_with_layout(
            Vec2::new(50., 0.),
            Layout::left_to_right(Align::TOP),
            |ui| {
                if self.complete {
                    ui.disable();
                }
                match self.options.interaction_type {
                    InteractionType::Ok => {
                        if ui.button("Ok").clicked() {
                            if self.options.dialog_type == DialogType::Password {
                                return Some(UserResponse::OkInput(self.input.clone()));
                            }
                            return Some(UserResponse::Ok);
                        }
                    }
                    InteractionType::OkCancel => {
                        if ui.button("Ok").clicked() {
                            if self.options.dialog_type == DialogType::Password {
                                return Some(UserResponse::OkInput(self.input.clone()));
                            }
                            return Some(UserResponse::Ok);
                        }
                        if ui.button("Cancel").clicked() {
                            return Some(UserResponse::Cancel);
                        }
                    }
                    InteractionType::YesNo => {
                        if ui.button("Yes").clicked() {
                            return Some(UserResponse::Yes);
                        }
                        if ui.button("No").clicked() {
                            return Some(UserResponse::No);
                        }
                    }
                    InteractionType::YesNoCancel => {
                        if ui.button("Yes").clicked() {
                            return Some(UserResponse::Yes);
                        }
                        if ui.button("No").clicked() {
                            return Some(UserResponse::No);
                        }
                        if ui.button("Cancel").clicked() {
                            return Some(UserResponse::Cancel);
                        }
                    }
                }
                None
            },
        )
        .inner
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        match self.options.dialog_type {
            DialogType::Message => {
                self.render_message(ui);
            }
            DialogType::Password => {
                self.render_password(ui);
            }
            _ => {} // DialogType::Progress => {}
                    // DialogType::Error => {}
                    // DialogType::Input => {}
        }
    }
}
cfg_feature_eframe! {
    impl eframe::App for DialogWidget {
        fn ui(&mut self, outer: &mut Ui, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show_inside(outer, |ui| {
               self.show(ui);
            });
        }
    }
}
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum DialogType {
    #[default]
    Message,
    Progress,
    Error,
    Password,
    Input,
}
impl DialogType {
    pub fn is_input(&self) -> bool {
        matches!(self, DialogType::Password | DialogType::Input)
    }
}
///
/// How can the user interact with the widget?
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum InteractionType {
    #[default]
    Ok,
    OkCancel,
    YesNo,
    YesNoCancel,
}
///
/// The user's chosen interaction with the widget
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum UserResponse {
    #[default]
    Ok,
    OkInput(String),
    Cancel,
    Yes,
    No,
}
///
/// Customization details of the dialog widget
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Details {
    /// What prompt to show?
    pub prompt: Option<String>,
    /// What type of dialog to show?
    pub dialog_type: DialogType,
    /// How should the user be able to interact with the dialog?
    pub interaction_type: InteractionType,
}
///
/// Options to create a dialog frame/window
pub struct DialogOptions {
    /// Native Options for the frame
    #[cfg(all(
        feature = "eframe",
        any(feature = "glow", feature = "wgpu"),
        not(target_arch = "wasm32")
    ))]
    pub native_options: eframe::NativeOptions,
    /// Title for the frame
    pub title: String,
    /// Customization details to pass to the underlying [`DialogWidget`]
    pub details: Details,
}
impl Default for DialogOptions {
    fn default() -> Self {
        DialogOptions {
            #[cfg(all(
                feature = "eframe",
                any(feature = "glow", feature = "wgpu"),
                not(target_arch = "wasm32")
            ))]
            native_options: eframe::NativeOptions {
                multisampling: 0,
                persist_window: false,
                centered: true,
                viewport: egui::ViewportBuilder::default()
                    .with_inner_size(Vec2::new(400.0, 50.0))
                    .with_active(true)
                    .with_always_on_top()
                    .with_resizable(false)
                    .with_taskbar(false)
                    .with_minimize_button(false)
                    .with_maximize_button(false),
                ..Default::default()
            },
            title: String::from("Progress"),
            details: Details::default(),
        }
    }
}

///
/// Pop and run a dialog frame with the provided options.
#[cfg(all(
    feature = "eframe",
    any(feature = "glow", feature = "wgpu"),
    not(target_arch = "wasm32")
))]
pub fn dialog_options<F: FnOnce(&mut DialogOptions)>(opts: F) -> Option<UserResponse> {
    let mut options = DialogOptions::default();
    opts(&mut options);
    let (tx, rx) = std::sync::mpsc::channel();
    if let Err(e) = eframe::run_native(
        options.title.as_str(),
        options.native_options.clone(),
        Box::new(|_cc| Ok(Box::new(DialogWidget::new(options.details, tx)))),
    ) {
        error!("{e:?}");
    };
    rx.try_recv().ok()
}
