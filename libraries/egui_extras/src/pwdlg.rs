// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use eframe::emath::Align;
use eframe::{Frame, NativeOptions};
use egui::{
    Context, Id, Key, Layout, Modifiers, TextEdit, Vec2, ViewportBuilder, ViewportCommand, Widget,
};
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
    pub fn render_message(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(self.options.prompt.as_deref().unwrap_or_default());
            ui.end_row();
            self.check_interaction(ui, ctx);
        });
    }
    ///
    /// Renders a password dialog that displays an password-masked input and a
    /// prompt
    pub fn render_password(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        if self.options.dialog_type.is_input() && ctx.cumulative_pass_nr() == 0 {
            ctx.memory_mut(|m| {
                m.request_focus(Id::new("pwdlg-inp"));
            });
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
                    if ctx.input_mut(|v| v.consume_key(Modifiers::NONE, Key::Enter)) {
                        self.send_result(ctx, UserResponse::OkInput(self.input.clone()));
                    } else if ctx.input_mut(|v| v.consume_key(Modifiers::NONE, Key::Escape)) {
                        self.send_result(ctx, UserResponse::Cancel);
                    }
                },
            );
            ui.end_row();
            self.check_interaction(ui, ctx);
        });
    }

    fn check_interaction(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if let Some(resp) = self.render_interaction(ui) {
            self.send_result(ctx, resp);
        }
    }

    fn send_result(&mut self, ctx: &Context, resp: UserResponse) {
        ctx.send_viewport_cmd(ViewportCommand::Close);
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

    pub fn ui(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        match self.options.dialog_type {
            DialogType::Message => {
                self.render_message(ctx, ui);
            }
            DialogType::Password => {
                self.render_password(ctx, ui);
            }
            _ => {} // DialogType::Progress => {}
                    // DialogType::Error => {}
                    // DialogType::Input => {}
        }
    }
}
impl eframe::App for DialogWidget {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ctx, ui);
        });
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
    pub native_options: NativeOptions,
    /// Title for the frame
    pub title: String,
    /// Customization details to pass to the underlying [`DialogWidget`]
    pub details: Details,
}
impl Default for DialogOptions {
    fn default() -> Self {
        DialogOptions {
            native_options: NativeOptions {
                multisampling: 0,
                persist_window: false,
                centered: true,
                viewport: ViewportBuilder::default()
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
#[cfg(any(feature = "glow", feature = "wgpu"))]
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
