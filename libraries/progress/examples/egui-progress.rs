// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use std::fs::OpenOptions;
use std::io::{BufWriter, Error};
use std::thread::JoinHandle;

use eframe::{App, CreationContext, Frame, NativeOptions};
use egui::{CentralPanel, Context, Window};
use log::error;

use irox_progress::egui::EguiProgressWindow;
use irox_progress::write::WriterTask;
use irox_progress::{ProgressPrinter, Task};
use irox_tools::bits::MutBits;
use irox_tools::random::Random;

struct ProgressApp {
    prog: EguiProgressWindow,
    writes_task: Option<Task>,
}

impl ProgressApp {
    pub fn new(cc: &CreationContext) -> ProgressApp {
        let context = cc.egui_ctx.clone();
        let prog = EguiProgressWindow::new(context);

        ProgressApp {
            prog,
            writes_task: None,
        }
    }

    pub fn new_task(&self) {
        let elements = 1000;
        let mut random = Random::default();
        let task = Task::new(random.next_u64(), "test task".to_string(), elements);
        self.prog.track_task_progress(&task);

        let child = task.new_child_task(random.next_u64(), "child 1".to_string(), elements / 2);
        let grandchild = child.new_child_task(random.next_u64(), "gc 1".to_string(), elements / 4);
        Self::start_task_thread(task, elements);
        Self::start_task_thread(child, elements / 2);
        Self::start_task_thread(grandchild, elements / 4);
    }

    pub fn start_writes_task(&mut self) {
        if let Some(task) = &self.writes_task {
            if !task.is_complete() && !task.is_cancelled() {
                return;
            }
        }

        let task = Task::new_infinite_named("Writing...".to_string());
        self.prog.track_task_progress(&task);
        task.mark_started();
        self.writes_task = Some(task.clone());

        let _res: JoinHandle<Result<(), Error>> = std::thread::spawn(move || {
            let out = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open("test")?;
            let out = BufWriter::new(out);
            let mut out = WriterTask::new(out, task.clone());
            let mut rand = Random::default();
            for _i in 0..62500000 {
                // 500MB
                out.write_be_u64(rand.next_u64())?;
            }
            task.mark_all_completed();
            Ok(())
        });
    }

    pub fn start_task_thread(task: Task, elements: u64) {
        std::thread::spawn(move || {
            task.mark_started();
            for _i in 0..elements {
                if task.is_cancelled() {
                    break;
                }
                task.mark_one_completed();

                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            task.mark_all_completed();
        });
    }
}

impl App for ProgressApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |_ui| {
            Window::new("progress").show(ctx, |ui| {
                if ui.button("Start one").clicked() {
                    self.new_task();
                }
                let enabled = self
                    .writes_task
                    .as_ref()
                    .map(|e| e.is_complete() || e.is_cancelled())
                    .unwrap_or(true);
                ui.add_enabled_ui(enabled, |ui| {
                    if ui.button("random writes").clicked() {
                        self.start_writes_task();
                    }
                });
                self.prog.ui(ui);
            });
        });
    }
}

pub fn main() -> Result<(), std::io::Error> {
    let native_options = NativeOptions {
        multisampling: 0,
        persist_window: false,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "irox-progress-test",
        native_options,
        Box::new(|cc| Box::new(ProgressApp::new(cc))),
    ) {
        error!("{e:?}");
    };

    Ok(())
}
