// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

///
/// Faster exchange operation than a simple mutex that uses a flag to indicate if new data is available.
pub struct Exchanger<T> {
    new_data: Arc<AtomicBool>,
    data: Arc<Mutex<Option<T>>>,
}
impl<T> Default for Exchanger<T> {
    fn default() -> Exchanger<T> {
        Self {
            new_data: Arc::new(AtomicBool::new(false)),
            data: Arc::new(Mutex::new(None)),
        }
    }
}
impl<T> Clone for Exchanger<T> {
    fn clone(&self) -> Exchanger<T> {
        Self {
            new_data: self.new_data.clone(),
            data: self.data.clone(),
        }
    }
}
impl<T> Exchanger<T> {
    /// Returns the value of the new data available flag
    pub fn new_data_available(&self) -> bool {
        self.new_data.load(Ordering::Relaxed)
    }
    /// Assert the new data available flag
    pub fn set_data_changed(&self) {
        self.new_data.store(true, Ordering::Relaxed);
    }

    /// Replaces any data in the exchanger with the provided data.  Asserts the data has been changed.
    pub fn replace_data(&self, new_data: T) -> Option<T> {
        let mut old = None;
        if let Ok(mut lock) = self.data.lock() {
            old = lock.replace(new_data);
        }
        self.new_data.store(true, Ordering::Relaxed);
        old
    }
    /// Takes any data within the exchanger.  Deasserts the new data flag.
    pub fn take_data(&self) -> Option<T> {
        if !self.new_data_available() {
            return None;
        }
        self.new_data.store(false, Ordering::Relaxed);
        if let Ok(mut lock) = self.data.lock() {
            return lock.take();
        }
        None
    }
}
