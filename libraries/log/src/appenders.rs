// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use irox_tools::identifier::Identifier;
use irox_tools::static_init;
use log::{Log, Metadata, Record};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

static_init!(get_system_logger, SharedMultiLogger, {
    SharedMultiLogger::new()
});

#[derive(Default)]
pub struct MultiLogger {
    inner: BTreeMap<Identifier, Box<dyn Log>>,
    statics: Vec<&'static dyn Log>,
}

impl Log for MultiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let mut enabled = false;
        for i in self.inner.values() {
            enabled |= i.enabled(metadata);
        }
        for i in &self.statics {
            enabled |= i.enabled(metadata);
        }
        enabled
    }

    fn log(&self, record: &Record) {
        for i in self.inner.values() {
            i.log(record);
        }
        for i in &self.statics {
            i.log(record);
        }
    }

    fn flush(&self) {
        for i in self.inner.values() {
            i.flush();
        }
        for i in &self.statics {
            i.flush();
        }
    }
}

impl MultiLogger {
    pub const fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
            statics: Vec::new(),
        }
    }
    pub fn add_static(&mut self, logger: &'static dyn Log) {
        self.statics.push(logger);
    }
    pub fn add(&mut self, id: Identifier, logger: Box<dyn Log>) {
        self.inner.insert(id, logger);
    }
    pub fn remove(&mut self, id: &Identifier) {
        self.inner.remove(id);
    }
    pub fn get(&self, id: &Identifier) -> Option<&dyn Log> {
        self.inner.get(id).map(|l| &**l)
    }
    pub fn get_mut(&mut self, id: &Identifier) -> Option<&mut Box<dyn Log>> {
        self.inner.get_mut(id)
    }
}

#[derive(Default, Clone)]
pub struct SharedMultiLogger {
    inner: Arc<RwLock<MultiLogger>>,
}
impl SharedMultiLogger {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(MultiLogger::new())),
        }
    }
    pub fn add_static(&self, logger: &'static dyn Log) {
        if let Ok(mut l) = self.inner.write() {
            l.add_static(logger);
        }
    }
    pub fn add(&self, id: Identifier, logger: Box<dyn Log>) {
        if let Ok(mut l) = self.inner.write() {
            l.add(id, logger);
        }
    }
    pub fn remove(&self, id: &Identifier) {
        if let Ok(mut l) = self.inner.write() {
            l.remove(id);
        }
    }
    pub fn get(&self) -> Option<RwLockReadGuard<'_, MultiLogger>> {
        self.inner.read().ok()
    }
    pub fn get_mut(&self) -> Option<RwLockWriteGuard<'_, MultiLogger>> {
        self.inner.write().ok()
    }
}
impl Log for SharedMultiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Some(l) = self.get() {
            l.enabled(metadata)
        } else {
            false
        }
    }
    fn log(&self, record: &Record) {
        if let Some(l) = self.get() {
            l.log(record);
        }
    }
    fn flush(&self) {
        if let Some(l) = self.get() {
            l.flush();
        }
    }
}
