// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//



use std::fmt::{Debug, Formatter};
use std::sync::{Arc, RwLock};

pub struct SynchronizedCell<T> {
    cell: Arc<RwLock<Option<T>>>
}

impl<T> SynchronizedCell<T> {
    pub fn empty() -> Self {
        SynchronizedCell {
            cell: Arc::new(RwLock::new(None))
        }
    }
    pub fn new(value: T) -> Self {
        SynchronizedCell {
            cell: Arc::new(RwLock::new(Some(value)))
        }
    }

    pub fn get_or_init(&self, func: impl FnOnce()->T){
        if let Ok(mut cell) = self.cell.write() {
            if cell.is_none() {
                *cell = Some(func());
                return;
            };
        }
    }
    pub fn take(&self) -> Option<T> {
        if let Ok(mut cell) = self.cell.write() {
            return cell.take();
        }
        None
    }
    pub fn maybe_mutate(&self, value: impl FnOnce(&mut T)) {
        if let Ok(mut cell) = self.cell.write() {
            if let Some(cell) = cell.as_mut() {
                value(cell);
            }
        }
    }
    pub fn get(&self, value: impl FnOnce(Option<&T>)) {
        if let Ok(cell) = self.cell.read() {
            value(cell.as_ref());
        }
    }
    pub fn maybe_get(&self, value: impl FnOnce(&T)) {
        if let Ok(cell) = self.cell.read() {
            if let Some(cell) = cell.as_ref() {
                value(cell);
            }
        }
    }
    pub fn map<R>(&self, value: impl FnOnce(Option<&T>)->R) -> Option<R> {
        if let Ok(cell) = self.cell.read() {
            return Some(value(cell.as_ref()));
        }
        None
    }
    pub fn maybe_map<R>(&self, value: impl FnOnce(&T)->R) -> Option<R> {
        if let Ok(cell) = self.cell.read() {
            if let Some(cell) = cell.as_ref() {
                return Some(value(cell));
            }
        }
        None
    }
    pub fn is_some(&self) -> bool {
        if let Ok(cell) = self.cell.read() {
            return cell.is_some();
        }
        false
    }
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

impl<T> Clone for SynchronizedCell<T> {
    fn clone(&self) -> Self {
        SynchronizedCell {
            cell: self.cell.clone()
        }
    }
}

impl<T: Debug> Debug for SynchronizedCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.map(|v| {
            v.fmt(f)
        }).unwrap_or(Ok(()))
    }
}
