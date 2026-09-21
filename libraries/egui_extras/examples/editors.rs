// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use eframe::emath::Vec2;
use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Ui, ViewportBuilder};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_egui_extras::widgets::Editor;
use irox_egui_extras::widgets::{BooleanEditor, Field};
use irox_log::log::{error, Level};
use irox_types::fields::FieldDescriptorBuilder;

pub fn main() {
    irox_log::init_console_level(Level::Info);
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "irox-egui-editors",
        native_options,
        Box::new(|cc| {
            let comp = Box::new(ToolFrame::new(cc, Box::new(TestApp::new(cc))));
            Ok(comp)
        }),
    ) {
        error!("{e:?}");
    };
}

pub struct TestApp {
    field1: Field<bool>,
    field2: Field<bool>,
}
impl TestApp {
    pub fn new(_cc: &CreationContext) -> Self {
        TestApp {
            field1: Field {
                descriptor: FieldDescriptorBuilder::new(&"nullable_field")
                    .with_display_name(&"Field One - Nullable")
                    .with_help_description(&"This is a nullable boolean field")
                    .with_nullable(true)
                    .build(),
                value: None,
            },
            field2: Field {
                descriptor: FieldDescriptorBuilder::new(&"not_nullable_field")
                    .with_display_name(&"Field One - Not Nullable")
                    .with_help_description(&"This is not a nullable boolean field")
                    .with_nullable(false)
                    .with_enforce_not_null(true)
                    .build(),
                value: None,
            },
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        BooleanEditor.show(&mut self.field1, ui);
        ui.end_row();
        BooleanEditor.show(&mut self.field2, ui);
    }
}

impl App for TestApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            self.show(ui);
        });
    }
}
impl ToolApp for TestApp {}
