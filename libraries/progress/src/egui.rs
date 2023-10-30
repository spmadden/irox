// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use egui::collapsing_header::CollapsingState;
use egui::{ProgressBar, Ui, Widget};

use crate::{ProgressPrinter, Task};

#[derive(Clone)]
pub struct EguiProgressWindow {
    completed: Arc<AtomicU64>,
    tasks: Arc<RwLock<Vec<Task>>>,
}

impl EguiProgressWindow {
    pub fn new() -> EguiProgressWindow {
        EguiProgressWindow {
            completed: Arc::new(AtomicU64::new(0)),
            tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl EguiProgressWindow {
    pub fn ui(&self, ui: &mut Ui) {
        let tasks = self.tasks.clone();
        let Ok(mut tasks) = tasks.write() else {
            return;
        };
        ui.vertical(|ui| {
            ui.label(format!(
                "{} tasks completed, {} tasks pending",
                self.completed.load(Ordering::Relaxed),
                tasks.len()
            ));
            tasks.retain(|task| self.paint_task(ui, task));
        });
    }

    fn paint_task(&self, ui: &mut Ui, task: &Task) -> bool {
        let frac = task.current_progress_frac() as f32;
        let name = task.get_name();
        let text = format!("{:<3.0}% {name}", frac * 100.);
        let id = ui.make_persistent_id(task.get_id());
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ProgressBar::new(frac).animate(true).text(text).ui(ui);
            })
            .body(|ui| {
                task.each_child(|t| {
                    if !t.is_complete() {
                        self.paint_task(ui, t);
                    }
                });
            });

        if task.is_complete() {
            self.completed.fetch_add(1, Ordering::Relaxed);
        }
        !task.is_complete()
    }
}

impl ProgressPrinter for EguiProgressWindow {
    fn track_task_progress(&self, task: &Task) {
        if let Ok(mut tasks) = self.tasks.clone().write() {
            tasks.push(task.clone())
        }
    }
}
