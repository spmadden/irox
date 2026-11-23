// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use eframe::emath::Vec2;
use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Ui, ViewportBuilder};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_egui_extras::widgets::{
    BooleanEditor, Editor, Field, FieldDescriptorBuilder, RestrictedValue, StringListEditor,
    UniqueValuesEditor, ValueRelationEditor,
};
use irox_log::log::{error, Level};

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

const DISTRICTS: [(i32, &str); 3] = [(101, "Central"), (205, "North"), (320, "South")];

pub struct TestApp {
    read_only: bool,
    inspected: Field<bool>,
    active: Field<bool>,
    tags: Field<Vec<String>>,
    street: Field<String>,
    previous_street: Field<String>,
    district: Field<i32>,
    street_names: Vec<String>,
    previous_street_options: [RestrictedValue<String>; 2],
}
impl TestApp {
    pub fn new(_cc: &CreationContext) -> Self {
        TestApp {
            read_only: false,
            inspected: field("Inspected", "Whether the site has been inspected", None),
            active: Field {
                descriptor: FieldDescriptorBuilder::new(&"Active")
                    .with_help_description(&"Whether the site is currently in use")
                    .with_nullable(false)
                    .with_enforce_not_null(true)
                    .build(),
                value: None,
            },
            tags: field(
                "Tags",
                "Keywords describing the site",
                Some(vec!["cycleway".to_owned(), "paved".to_owned()]),
            ),
            street: field(
                "Street name",
                "Type a new name or pick a matching street",
                Some("Map".to_owned()),
            ),
            previous_street: field(
                "Previous street name",
                "Choose a former name from the address register",
                None,
            ),
            district: field(
                "District",
                "Administrative district for this site",
                Some(205),
            ),
            street_names: ["Maple Avenue", "Maple Court", "Oak Street"]
                .map(str::to_owned)
                .to_vec(),
            previous_street_options: [
                RestrictedValue {
                    value: "Maple Road".to_owned(),
                    help_description: Some("Name used before the 2018 renumbering".to_owned()),
                },
                RestrictedValue::new("Old Mill Lane".to_owned()),
            ],
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.read_only, "Read only");
        ui.add_enabled_ui(!self.read_only, |ui| self.show_fields(ui));
    }

    fn show_fields(&mut self, ui: &mut Ui) {
        BooleanEditor.show(&mut self.inspected, ui);
        BooleanEditor.show(&mut self.active, ui);
        ui.separator();
        StringListEditor.show(&mut self.tags, ui);
        ui.separator();
        UniqueValuesEditor::editable(&self.street_names).show(&mut self.street, ui);
        UniqueValuesEditor::restricted_with_options(&self.previous_street_options)
            .show(&mut self.previous_street, ui);
        ui.separator();
        ValueRelationEditor::new(&DISTRICTS).show(&mut self.district, ui);
        ui.label(format!("Stored district key: {:?}", self.district.value));
    }
}

fn field<T: Clone + Default>(label: &str, help_description: &str, value: Option<T>) -> Field<T> {
    Field {
        descriptor: FieldDescriptorBuilder::new(&label)
            .with_help_description(&help_description)
            .build(),
        value,
    }
}

impl App for TestApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        CentralPanel::default().show(ui, |ui| {
            self.show(ui);
        });
    }
}
impl ToolApp for TestApp {}
