// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use eframe::emath::Vec2;
use eframe::{App, CreationContext, Frame};
use egui::{Align2, CentralPanel, ComboBox, FontId, ScrollArea, Sense, Ui, ViewportBuilder};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_imagery::colormaps::flat::{
    ALIZARIN, AMETHYST, ASBESTOS, ASPHALT, BLUEHOLE, CARROT, CLOUDS, CONCRETE, MIDNIGHT, NEPHI,
    ORANGE, POMEGRANITE, PUMPKIN, RIVER, SEA, SILVER, SUNFLOWER, TURQUOISE, WISTERIA,
};
use irox_imagery::colormaps::material::{
    AMBER, BLUE, BROWN, CYAN, FIRE, GREEN, GREY, INDIGO, LEAF, LIME, PINK, PURPLE, RED, ROYAL, SKY,
    SLATE, TEAL, YELLOW,
};
use irox_imagery::colormaps::DIVERGENT_19;
use irox_imagery::Color;
use log::{error, Level};

#[derive(Copy, Clone)]
pub struct ColorSet {
    pub name: &'static str,
    pub set: &'static [&'static [Color]],
}

pub const COLOR_SETS: &[ColorSet] = &[
    ColorSet {
        name: "Tableau 10",
        set: &[irox_imagery::colormaps::TABLEAU_10],
    },
    ColorSet {
        name: "Classic 10",
        set: &[irox_imagery::colormaps::CLASSIC_10],
    },
    ColorSet {
        name: "Classic 20",
        set: &[irox_imagery::colormaps::CLASSIC_20],
    },
    ColorSet {
        name: "Flat",
        set: &[
            POMEGRANITE,
            ALIZARIN,
            AMETHYST,
            WISTERIA,
            BLUEHOLE,
            RIVER,
            TURQUOISE,
            SEA,
            NEPHI,
            SUNFLOWER,
            ORANGE,
            CARROT,
            PUMPKIN,
            CLOUDS,
            SILVER,
            CONCRETE,
            ASBESTOS,
            ASPHALT,
            MIDNIGHT,
        ],
    },
    ColorSet {
        name: "Material",
        set: &[
            RED,
            PINK,
            PURPLE,
            ROYAL,
            INDIGO,
            BLUE,
            SKY,
            CYAN,
            TEAL,
            GREEN,
            LEAF,
            LIME,
            YELLOW,
            AMBER,
            irox_imagery::colormaps::material::ORANGE,
            FIRE,
            BROWN,
            GREY,
            SLATE,
        ],
    },
];

pub fn main() {
    irox_log::init_console_level(Level::Info);
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "irox-egui-gallery-colors",
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
    selected_colorset: usize,
    divergent: bool,
}
impl TestApp {
    pub fn new(_cc: &CreationContext) -> Self {
        TestApp {
            selected_colorset: 0,
            divergent: false,
        }
    }
}

impl App for TestApp {
    #[allow(clippy::indexing_slicing)]
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            ComboBox::new("chooser", "Color Map").show_index(
                ui,
                &mut self.selected_colorset,
                COLOR_SETS.len(),
                |i| COLOR_SETS[i].name,
            );
            ui.checkbox(&mut self.divergent, "Divergent");
            if let Some(set) = COLOR_SETS.get(self.selected_colorset) {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        let set_len = set.set.len();
                        let iter = if self.divergent && set_len == 19 {
                            DIVERGENT_19
                                .iter()
                                .map(|i| &set.set[*i])
                                .collect::<Vec<_>>()
                        } else {
                            set.set.iter().collect::<Vec<_>>()
                        };
                        for set in iter {
                            for (idx, color) in set.iter().enumerate() {
                                if idx > 0 && idx % 10 == 0 {
                                    ui.end_row();
                                }
                                let (rect, _response) =
                                    ui.allocate_at_least(Vec2::new(64., 32.), Sense::hover());
                                let painter = ui.painter();
                                painter.rect_filled(rect, 0., *color);
                                painter.text(
                                    rect.center(),
                                    Align2::CENTER_CENTER,
                                    format!("{color}"),
                                    FontId::monospace(10.),
                                    color.invert_black_white().into(),
                                );
                            }
                            ui.end_row();
                        }
                    });
                });
            }
        });
    }
}
impl ToolApp for TestApp {}
