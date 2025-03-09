// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;
use std::time::Duration;

use crate::progressbar::ProgressBar;
use egui::collapsing_header::CollapsingState;
use egui::{Align, Context, CursorIcon, Layout, Ui};
use irox_progress::{get_human, ProgressPrinter, Task};
use irox_time::format::iso8601::ISO8601Duration;
use irox_time::format::Format;

#[derive(Clone)]
pub struct EguiProgressWindow {
    completed: Arc<AtomicU64>,
    tasks: Arc<RwLock<Vec<Task>>>,
    running: Arc<AtomicBool>,
    any_tasks_active: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl EguiProgressWindow {
    pub fn new(context: Context) -> EguiProgressWindow {
        let r2 = Arc::new(AtomicBool::new(true));
        let running = r2.clone();
        let active = Arc::new(AtomicBool::new(false));
        let a2 = active.clone();
        let handle = std::thread::spawn(move || {
            while r2.load(Ordering::Relaxed) {
                let millis = if a2.load(Ordering::Relaxed) { 50 } else { 1000 };
                std::thread::sleep(Duration::from_millis(millis));
                context.request_repaint();
            }
        });
        EguiProgressWindow {
            completed: Arc::new(AtomicU64::new(0)),
            tasks: Arc::new(RwLock::new(Vec::new())),
            handle: Arc::new(Mutex::new(Some(handle))),
            any_tasks_active: active,
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
        ui.allocate_ui_with_layout(
            ui.available_size(),
            Layout::top_down_justified(Align::Min),
            |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{} tasks completed, {} tasks pending",
                        self.completed.load(Ordering::Relaxed),
                        tasks.len()
                    ));
                    ui.allocate_ui_with_layout(
                        ui.available_size_before_wrap(),
                        Layout::right_to_left(Align::Center),
                        |ui| {
                            if ui
                                .button("\u{1F5D9}*")
                                .on_hover_text("Cancel all tasks")
                                .clicked()
                            {
                                tasks.iter().for_each(Task::cancel);
                            }
                        },
                    );
                });

                let mut any_tasks_active = false;
                tasks.retain(|task| {
                    let active = self.paint_task(ui, task);
                    any_tasks_active |= active;
                    active
                });
                self.any_tasks_active
                    .store(any_tasks_active, Ordering::Relaxed);
            },
        );
    }

    fn get_speed_text(task: &Task) -> String {
        if let Some(started) = task.get_started() {
            let elapsed = started.elapsed().as_seconds_f64();
            let avg_per_sec = task.current_progress_count() as f64 / elapsed;
            let (avg_per_sec, avg_unit) = get_human!(avg_per_sec);
            return format!("{avg_per_sec:.02}{avg_unit}/s");
        }
        String::new()
    }

    fn paint_finite_header(ui: &mut Ui, task: &Task) {
        let frac = task.current_progress_frac() as f32;
        let current = task.current_progress_count();
        let max = task.max_elements();
        let name = task.get_name();
        let current = current as f64;
        let (current, unit) = get_human!(current);

        let speed = Self::get_speed_text(task);

        let max = max as f64;
        let (max, maxunit) = get_human!(max);
        let status = task
            .current_status()
            .map(|v| format!(" {v}"))
            .unwrap_or_default();

        let rem_str = ISO8601Duration.format(&task.get_remaining_time());
        let left_text = format!("{:<3.0}% {name}{status}", frac * 100.);
        let right_text = format!("({current:.02}{unit}/{max:.02}{maxunit}) {rem_str} {speed} ");
        ProgressBar::new(frac)
            .text_left(left_text)
            .text_right(right_text)
            .ui(ui);
    }

    fn paint_infinite_header(ui: &mut Ui, task: &Task) {
        let current = task.current_progress_count();
        let name = task.get_name();

        let current = current as f64;
        let (current, unit) = get_human!(current);
        let speed = Self::get_speed_text(task);
        let status = task
            .current_status()
            .map(|v| format!(": {v}"))
            .unwrap_or_default();
        let left_text = format!("{name}{status}");
        let right_text = format!("{current:.02}{unit} {speed}");

        ProgressBar::indeterminate()
            // .desired_width(desired_width)
            .text_left(left_text)
            .text_right(right_text)
            .ui(ui);
    }

    fn paint_task(&self, ui: &mut Ui, task: &Task) -> bool {
        let is_infinite = task.max_elements() == u64::MAX;

        let id = ui.make_persistent_id(task.get_id());
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                ui.allocate_ui_with_layout(
                    ui.available_size_before_wrap(),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        if task.is_cancelled() {
                            ui.label("\u{1F6AB}")
                                .on_hover_cursor(CursorIcon::Wait)
                                .on_hover_text("Task cancelled");
                        } else if ui
                            .button("\u{1F5D9}")
                            .on_hover_text("Request Task Cancel")
                            .clicked()
                        {
                            task.cancel();
                        };
                        if is_infinite {
                            Self::paint_infinite_header(ui, task);
                        } else {
                            Self::paint_finite_header(ui, task);
                        }
                    },
                );
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
