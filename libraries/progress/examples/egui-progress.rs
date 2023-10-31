// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use eframe::{App, CreationContext, Frame, NativeOptions};
use egui::{CentralPanel, Color32, Context, Window};
use log::error;

use irox_progress::egui::EguiProgressWindow;
use irox_progress::{ProgressPrinter, Task};
use irox_tools::random::Random;

struct ProgressApp {
    prog: EguiProgressWindow,
}
impl ProgressApp {
    pub fn new(cc: &CreationContext) -> ProgressApp {
        let context = cc.egui_ctx.clone();
        let prog = EguiProgressWindow::new(context);

        ProgressApp { prog }
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

    pub fn start_task_thread(task: Task, elements: u64) {
        std::thread::spawn(move || {
            task.mark_started();
            for _i in 0..elements {
                task.mark_one_completed();

                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        });
    }
}
impl App for ProgressApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().visuals.widgets.open.bg_fill =
                Color32::from_rgba_unmultiplied(0, 0, 0, 128);
            Window::new("progress").show(ctx, |ui| {
                if ui.button("Start one").clicked() {
                    self.new_task();
                }
                self.prog.ui(ui);
            });
        });
    }
}

pub fn main() -> Result<(), std::io::Error> {
    let native_options = NativeOptions {
        multisampling: 0,
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
