// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use egui::{TextEdit, Ui};

pub struct DownloadPathWidget {
    pub cwd: String,
    pub manual_path: String,
    pub specify_path: bool,
}

impl DownloadPathWidget {
    pub fn new(directory_name: &str) -> DownloadPathWidget {
        let cwd = std::env::current_dir()
            .unwrap_or_default()
            .join(directory_name);
        let cwd = cwd.to_string_lossy().to_string().replace("\\\\?\\", "");
        DownloadPathWidget {
            manual_path: String::new(),
            cwd,
            specify_path: false,
        }
    }

    pub fn set_manual_path(&mut self, path: String) {
        self.manual_path = path;
        self.specify_path = true;
    }

    pub fn get_download_path(&self) -> String {
        if self.specify_path {
            self.manual_path.clone()
        } else {
            self.cwd.clone()
        }
    }

    pub fn show(&mut self, label: &str, ui: &mut Ui) {
        ui.label(format!("{label}: "));
        ui.label(&self.cwd);
        ui.label("or manual: ");
        if ui.checkbox(&mut self.specify_path, "").changed() && self.specify_path {
            self.manual_path = self.cwd.clone();
        }
        if self.specify_path {
            let _resp = TextEdit::singleline(&mut self.manual_path)
                .min_size(egui::vec2(400., 0.))
                .show(ui)
                .response;
        }
    }
}
