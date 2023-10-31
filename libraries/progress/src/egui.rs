// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;
use std::time::Duration;

use egui::collapsing_header::CollapsingState;
use egui::{Context, ProgressBar, Ui, Widget};

use irox_time::format::iso8601::ISO8601Duration;
use irox_time::format::Format;

use crate::{get_human, ProgressPrinter, Task};

#[derive(Clone)]
pub struct EguiProgressWindow {
    completed: Arc<AtomicU64>,
    tasks: Arc<RwLock<Vec<Task>>>,
    running: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl EguiProgressWindow {
    pub fn new(context: Context) -> EguiProgressWindow {
        let r2 = Arc::new(AtomicBool::new(true));
        let running = r2.clone();
        let handle = std::thread::spawn(move || {
            while r2.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(100));
                context.request_repaint();
            }
        });
        EguiProgressWindow {
            completed: Arc::new(AtomicU64::new(0)),
            tasks: Arc::new(RwLock::new(Vec::new())),
            handle: Arc::new(Mutex::new(Some(handle))),
            running,
        }
    }
}
impl Drop for EguiProgressWindow {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Ok(mut handle) = self.handle.lock() {
            if let Some(handle) = handle.take() {
                let _ok = handle.join();
            }
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

    fn paint_infinite(&self, ui: &mut Ui, task: &Task) -> bool {
        let current = task.current_progress_count();
        let name = task.get_name();

        let current = current as f64;
        let (current, unit) = get_human!(current);
        let mut speed = String::new();

        if let Some(started) = task.get_started() {
            let elapsed = started.elapsed().as_seconds_f64();
            let avg_per_sec = current / elapsed;
            let (avg_per_sec, avg_unit) = get_human!(avg_per_sec);
            speed = format!("{avg_per_sec:.02}{avg_unit}/s");
        }

        let rem_str = ISO8601Duration.format(&task.get_remaining_time());
        let text = format!("{name} {current:.02}{unit} {rem_str} {speed}");
        let id = ui.make_persistent_id(task.get_id());
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ui.label(text);
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

    fn paint_task(&self, ui: &mut Ui, task: &Task) -> bool {
        let frac = task.current_progress_frac() as f32;
        let current = task.current_progress_count();
        let max = task.max_elements();
        let name = task.get_name();

        if max == u64::MAX {
            return self.paint_infinite(ui, task);
        }

        let current = current as f64;
        let (current, unit) = get_human!(current);
        let mut speed = String::new();

        if let Some(started) = task.get_started() {
            let elapsed = started.elapsed().as_seconds_f64();
            let avg_per_sec = current / elapsed;
            let (avg_per_sec, avg_unit) = get_human!(avg_per_sec);
            speed = format!("{avg_per_sec:.02}{avg_unit}/s");
        }
        let max = max as f64;
        let (max, maxunit) = get_human!(max);

        let rem_str = ISO8601Duration.format(&task.get_remaining_time());
        let text = format!(
            "{:<3.0}% {name} ({current:.02}{unit}/{max:.02}{maxunit}) {rem_str} {speed}",
            frac * 100.
        );
        let id = ui.make_persistent_id(task.get_id());
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ProgressBar::new(frac).text(text).ui(ui);
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
