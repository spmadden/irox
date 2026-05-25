// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

extern crate alloc;
use alloc::string::{String, ToString};
use core::ops::Deref;
use irox_time::datetime::UTCDateTime;
use log::{Level, Record};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedRecord {
    pub file: Option<String>,
    pub line: Option<u32>,
    pub level: Level,
    pub message: String,
    pub module_path: String,
    pub target: String,
}
impl From<&Record<'_>> for OwnedRecord {
    fn from(record: &Record) -> Self {
        let module = record
            .module_path()
            .unwrap_or("")
            .split("::")
            .last()
            .unwrap_or_default()
            .to_string();
        Self {
            message: record.args().to_string(),
            module_path: module,
            file: record.file().map(ToString::to_string),
            line: record.line(),
            level: record.level(),
            target: record.target().to_string(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TimedRecord {
    pub timestamp: UTCDateTime,
    pub record: OwnedRecord,
}

impl Deref for TimedRecord {
    type Target = OwnedRecord;
    fn deref(&self) -> &Self::Target {
        &self.record
    }
}
impl From<OwnedRecord> for TimedRecord {
    fn from(record: OwnedRecord) -> Self {
        Self {
            timestamp: UTCDateTime::now(),
            record,
        }
    }
}
impl From<&Record<'_>> for TimedRecord {
    fn from(record: &Record) -> Self {
        TimedRecord::from(OwnedRecord::from(record))
    }
}
