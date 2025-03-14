// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use egui::Ui;

pub const IMPORTANT_NAMES: &[(&str, &str)] = &[
    ("CARGO_PKG_NAME", "Package Name"),
    ("CARGO_PKG_DESCRIPTION", "Package Description"),
    ("CARGO_PKG_VERSION", "Package Version"),
    ("CARGO_PKG_REPOSITORY", "Repository"),
    ("GIT_COMMIT_FULLHASH", "Git Commit Hash"),
    ("GIT_COMMIT_DATETIME", "Git Commit Date"),
    ("GIT_COMMIT_AUTHOR", "Git Commit Author"),
    ("GIT_DESCRIBE", "Git Commit Description"),
    ("GIT_IS_CLEAN", "Is clean (not dirty) build"),
    ("PROFILE", "Build Profile"),
    ("BUILD_TIME", "Build Time"),
    ("HOST", "Build Host Platform"),
    ("RUSTUP_TOOLCHAIN", "Build Toolchain"),
    ("RUSTC_VERSION", "Rust Version"),
    ("CARGO_VERSION", "Cargo Version"),
];

pub struct AboutWindow;

impl AboutWindow {
    pub fn show<'a, F: Fn() -> &'a std::collections::BTreeMap<&'a str, &'a str>>(
        providerfn: F,
        ui: &mut Ui,
    ) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("about_grid_display")
                .num_columns(2)
                .striped(true)
                .spacing([40.0, 4.0])
                .show(ui, |ui| {
                    for (k, v) in providerfn() {
                        ui.label(*k);
                        ui.label(*v);
                        ui.end_row();
                    }
                });
        });
    }

    ///
    /// Show only those items listed as 'important' above
    pub fn show_important<'a, F: Fn() -> &'a std::collections::BTreeMap<&'a str, &'a str>>(
        providerfn: F,
        ui: &mut Ui,
    ) {
        let data = providerfn();
        egui::Grid::new("about_grid_display")
            .num_columns(2)
            .striped(true)
            .spacing([40.0, 4.0])
            .show(ui, |ui| {
                let mut repo = Option::<&str>::None;
                for (key, disp) in IMPORTANT_NAMES {
                    if let Some(v) = data.get(key) {
                        ui.label(*disp);
                        match *key {
                            "CARGO_PKG_REPOSITORY" => {
                                ui.hyperlink(*v);
                                if v.contains("github") {
                                    repo = Some(*v);
                                }
                            }
                            "GIT_COMMIT_FULLHASH" => {
                                if let Some(repo) = repo {
                                    ui.hyperlink_to(*v, format!("{repo}/commit/{}", *v));
                                } else {
                                    ui.label(*v);
                                }
                            }
                            _ => {
                                ui.label(*v);
                            }
                        };
                        ui.end_row();
                    }
                }
            });
    }

    pub fn show_grouped<
        'a,
        F: Fn()
            -> &'a std::collections::BTreeMap<&'a str, std::collections::BTreeMap<&'a str, &'a str>>,
    >(
        providerfn: F,
        ui: &mut Ui,
    ) {
        let data = providerfn();
        egui::Grid::new("about_grid_display")
            .num_columns(2)
            .striped(true)
            .spacing([40.0, 4.0])
            .show(ui, |ui| {
                let mut repo = Option::<&str>::None;
                for group in data.values() {
                    for (key, disp) in IMPORTANT_NAMES {
                        if let Some(v) = group.get(key) {
                            ui.label(*disp);
                            match *key {
                                "CARGO_PKG_REPOSITORY" => {
                                    ui.hyperlink(*v);
                                    if v.contains("github") {
                                        repo = Some(*v);
                                    }
                                }
                                "GIT_COMMIT_FULLHASH" => {
                                    if let Some(repo) = repo {
                                        ui.hyperlink_to(*v, format!("{repo}/commit/{}", *v));
                                    } else {
                                        ui.label(*v);
                                    }
                                }
                                _ => {
                                    ui.label(*v);
                                }
                            };
                            ui.end_row();
                        }
                    }
                }
            });
        for (gname, group) in data {
            ui.collapsing(*gname, |ui| {
                AboutWindow::show(|| group, ui);
            });
        }
    }
}
