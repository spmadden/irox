// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use egui::{Color32, TextEdit, Ui};
use irox_time::datetime::UTCDateTime;
use irox_time::format::iso8601::{ExtendedDateTimeFormat, ISO8601DateTime};
use irox_time::format::{Format, FormatParser};

pub struct DateRangeSelector {
    pub original_date: UTCDateTime,
    pub original_date_str: String,
    specify_time: bool,
    pub manual_time: String,
    pub manual_time_parsed: Option<UTCDateTime>,
    live_time: bool,
    pub earliest_time: UTCDateTime,
    pub latest_time: UTCDateTime,
    pub validation_errors: bool,
}

impl DateRangeSelector {
    pub fn new(
        original_date: UTCDateTime,
        earliest_time: UTCDateTime,
        latest_time: UTCDateTime,
    ) -> Self {
        let original_date_str = ExtendedDateTimeFormat.format(&original_date);
        Self {
            original_date,
            original_date_str,
            specify_time: false,
            validation_errors: false,
            manual_time: String::new(),
            earliest_time,
            latest_time,
            manual_time_parsed: None,
            live_time: false,
        }
    }
    pub fn specify_time(&mut self, time: UTCDateTime) {
        self.specify_time = true;
        self.manual_time = time.to_string();
        self.manual_time_parsed = Some(time);
    }

    pub fn update_earliest_time(&mut self, earliest_time: UTCDateTime) {
        self.earliest_time = earliest_time;
    }

    pub fn update_latest_time(&mut self, latest_time: UTCDateTime) {
        self.latest_time = latest_time
    }

    pub fn is_time_overridden(&self) -> bool {
        self.live_time || self.specify_time
    }

    pub fn get_specified_time(&self) -> UTCDateTime {
        if !self.is_time_overridden() {
            return self.original_date;
        }
        let Some(parsed) = self.manual_time_parsed else {
            return self.original_date;
        };
        parsed
    }

    pub fn get_override_time(&self) -> Option<UTCDateTime> {
        if self.is_time_overridden() {
            self.manual_time_parsed
        } else {
            None
        }
    }

    pub fn show(&mut self, label: &str, ui: &mut Ui) {
        self.validation_errors = false;
        ui.label(format!("{label}: "));
        ui.label(&self.original_date_str);
        if ui
            .toggle_value(&mut self.specify_time, "specify time")
            .changed()
            && self.specify_time
        {
            self.live_time = false;
            self.manual_time = self.original_date_str.clone();
            self.manual_time_parsed = None;
        }
        if ui
            .toggle_value(&mut self.live_time, "specify now")
            .changed()
            && self.live_time
        {
            self.specify_time = false;
            let time = UTCDateTime::now();
            self.manual_time = ISO8601DateTime.format(&time);
            self.manual_time_parsed = Some(time);
        }
        if self.is_time_overridden() {
            let mut hint = None;
            let mut err_color = None;
            if self.manual_time_parsed.is_none() || self.specify_time {
                err_color = match ISO8601DateTime.try_from(&self.manual_time) {
                    Ok(time) => {
                        self.manual_time_parsed = Some(time);
                        if time < self.earliest_time {
                            hint = Some(format!("time {time} < {}", self.earliest_time));
                            self.validation_errors = true;
                            Some(Color32::RED)
                        } else if time > self.latest_time {
                            hint = Some(format!("time {time} > {}", self.latest_time));
                            self.validation_errors = true;
                            Some(Color32::RED)
                        } else {
                            self.validation_errors = false;
                            None
                        }
                    }
                    Err(_) => {
                        hint = Some("invalid time format".to_string());
                        self.validation_errors = true;
                        self.manual_time_parsed = None;
                        Some(Color32::RED)
                    }
                }
            };
            let resp = TextEdit::singleline(&mut self.manual_time)
                .min_size(egui::vec2(400., 0.))
                .text_color_opt(err_color)
                .show(ui);
            if let Some(hint) = hint {
                resp.response.response.on_hover_text(hint);
            }
        }
    }
}
