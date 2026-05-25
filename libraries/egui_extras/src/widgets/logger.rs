// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use egui::{Color32, RichText};
use egui_extras::Column;
use irox_imagery::colortheme::{ONE_HALF_DARK, ONE_HALF_LIGHT};
use irox_log::TimedRecord;
use irox_time::format::iso8601::BASIC_TIME_OF_DAY;
use irox_tools::static_init;
use log::{Level, Metadata, Record};
use std::sync::atomic::AtomicBool;
use std::sync::RwLock;

static_init!(get_system_appender, LoggerAppender, {
    LoggerAppender::new()
});

pub struct LoggerWidgetInfo {
    pub max_level: Level,
    pub sender: std::sync::mpsc::Sender<TimedRecord>,
}

#[derive(Default)]
pub struct LoggerAppender {
    senders: RwLock<Vec<LoggerWidgetInfo>>,
    installed: AtomicBool,
}
impl LoggerAppender {
    pub fn new() -> Self {
        Self {
            senders: Default::default(),
            installed: AtomicBool::new(false),
        }
    }
}
impl LoggerAppender {
    pub fn install(&'static self) {
        if !self
            .installed
            .swap(true, std::sync::atomic::Ordering::SeqCst)
        {
            let logger = irox_log::get_system_logger();
            logger.add_static(self);
        }
    }
    pub fn add_widget(&self, info: LoggerWidgetInfo) {
        if let Ok(mut senders) = self.senders.write() {
            senders.push(info);
        }
    }
}
impl irox_log::log::Log for LoggerAppender {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let mut enabled = false;
        if let Ok(senders) = self.senders.read() {
            enabled |= senders
                .iter()
                .any(|sender| sender.max_level >= metadata.level());
        }
        enabled
    }

    fn log(&self, record: &Record) {
        let record = TimedRecord::from(record);
        if let Ok(mut senders) = self.senders.write() {
            senders.retain(|sender| sender.sender.send(record.clone()).is_ok());
        }
    }

    fn flush(&self) {
        //no-op
    }
}

pub struct LoggerWidget {
    pub id: &'static str,
    pub max_level: Level,
    pub receiver: std::sync::mpsc::Receiver<TimedRecord>,
    pub logs: Vec<TimedRecord>,
    pub last_row_size_adjusted: usize,
}
impl LoggerWidget {
    pub fn new(id: &'static str, max_level: Level) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        let s = Self {
            id,
            max_level,
            receiver,
            logs: Vec::new(),
            last_row_size_adjusted: 0,
        };
        let info = LoggerWidgetInfo { max_level, sender };
        let appender = get_system_appender();
        appender.install();
        appender.add_widget(info);

        s
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.request_repaint_after(std::time::Duration::from_millis(100));
        self.receiver.try_iter().for_each(|record| {
            self.logs.push(record);
        });

        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);

        let resize_this_frame = self.logs.len() > self.last_row_size_adjusted;
        if resize_this_frame {
            self.last_row_size_adjusted = self.logs.len();
        }
        let theme = if ui.visuals().dark_mode {
            ONE_HALF_DARK
        } else {
            ONE_HALF_LIGHT
        };

        egui_extras::TableBuilder::new(ui)
            .striped(true)
            .stick_to_bottom(true)
            // .auto_shrink(false)
            .drag_to_scroll(false)
            .resizable(true)
            .column(Column::auto_with_initial_suggestion(200.).at_least(150.))
            .column(Column::auto_with_initial_suggestion(200.).at_least(50.))
            .column(Column::auto_with_initial_suggestion(200.).at_least(150.))
            .column(Column::remainder().clip(false))
            .header(row_height, |mut row| {
                row.col(|ui| {
                    ui.label("Time");
                });
                row.col(|ui| {
                    ui.label("Level");
                });
                row.col(|ui| {
                    ui.label("Module");
                });
                row.col(|ui| {
                    ui.label("Message");
                });
            })
            .body(|ui| {
                ui.rows(row_height, self.logs.len(), |mut row| {
                    let idx = row.index();
                    if let Some(record) = self.logs.get(idx) {
                        let color: Color32 = match record.level {
                            Level::Error => theme.red.into(),
                            Level::Warn => theme.yellow.into(),
                            Level::Info => theme.blue.into(),
                            Level::Debug => theme.magenta.into(),
                            Level::Trace => theme.cyan.into(),
                        };
                        row.col(|ui| {
                            let txt = RichText::new(format!(
                                "{}Z",
                                record.timestamp.format(&BASIC_TIME_OF_DAY)
                            ))
                            .color(color)
                            .monospace();
                            ui.label(txt);
                        });
                        row.col(|ui| {
                            let txt = RichText::new(record.level.to_string())
                                .color(color)
                                .monospace();
                            ui.label(txt);
                        });
                        row.col(|ui| {
                            let txt = RichText::new(&record.module_path).color(color).monospace();
                            ui.label(txt);
                        });
                        row.col(|ui| {
                            let txt = RichText::new(&record.message).color(color).monospace();
                            ui.label(txt);
                        });
                    }
                });
            });
    }
}
