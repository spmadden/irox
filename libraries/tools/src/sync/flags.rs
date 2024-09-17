// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use alloc::sync::Arc;
use std::sync::{Condvar, Mutex};

///
/// A Synchronous Signal Flag that can be asserted or cleared.
#[derive(Default, Clone)]
pub struct SyncFlag {
    mutex: Arc<Mutex<bool>>,
    condvar: Arc<Condvar>,
}

impl SyncFlag {
    ///
    /// Creates a new [`SyncFlag`] that can be shared between threads.
    #[allow(clippy::mutex_atomic)]
    pub fn new() -> SyncFlag {
        SyncFlag {
            mutex: Arc::new(Mutex::new(false)),
            condvar: Arc::new(Condvar::new()),
        }
    }

    /// Asserts the signal flag and notifies any waiters.
    pub fn set_flag(&self) {
        if let Ok(mut guard) = self.mutex.lock() {
            *guard = true;
        }
        self.condvar.notify_all();
    }

    /// Clears the signal flag and notifies any waiters.
    pub fn clear_flag(&self) {
        if let Ok(mut guard) = self.mutex.lock() {
            *guard = false;
        }
        self.condvar.notify_all();
    }

    /// Waits until the flag is notified and returns the current value of the
    /// signal.
    pub fn await_flag_notified(&self) -> bool {
        if let Ok(guard) = self.mutex.lock() {
            if let Ok(guard) = self.condvar.wait(guard) {
                return *guard;
            }
        };
        false
    }

    /// If the flag is set, immediately returns.  Otherwise waits until the flag
    /// is set and then returns.
    pub fn await_flag_set(&self) {
        loop {
            let Ok(guard) = self.mutex.lock() else {
                break;
            };
            if *guard {
                return;
            }
            let Ok(guard) = self.condvar.wait_while(guard, |v| !*v) else {
                break;
            };
            if *guard {
                return;
            }
        }
    }

    /// If the flag is cleared, immediately returns.  Otherwise waits until the
    /// flag is cleared and then returns.
    pub fn await_flag_unset(&self) {
        loop {
            let Ok(guard) = self.mutex.lock() else {
                break;
            };
            if !*guard {
                return;
            }
            let Ok(guard) = self.condvar.wait_while(guard, |v| *v) else {
                break;
            };
            if !*guard {
                return;
            }
        }
    }
}
