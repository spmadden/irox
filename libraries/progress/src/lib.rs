// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! An ecosystem for displaying progress, either in a UI or on the terminal.
//!

#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock, RwLock};

use irox_time::epoch::UnixTimestamp;

pub mod console;
#[cfg(feature = "egui")]
pub mod egui;

///
/// A way to display progress.
pub trait ProgressPrinter {
    /// Track and display the progress for this specific task.
    fn track_task_progress(&self, task: &Task);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TaskElementUnits {
    None,
    Bytes,
    Bits,
}

#[derive(Debug)]
struct TaskInner {
    id: AtomicU64,
    name: String,
    counter: AtomicU64,
    max_elements: u64,
    _element_units: TaskElementUnits,
    created: UnixTimestamp,
    started: OnceLock<UnixTimestamp>,
    ended: OnceLock<UnixTimestamp>,
    children: RwLock<Vec<Task>>,
}

#[derive(Debug, Clone)]
pub struct Task {
    inner: Arc<TaskInner>,
}

impl Task {
    pub fn new(id: u64, name: String, max_elements: u64) -> Task {
        let inner = TaskInner {
            id: AtomicU64::new(id),
            name,
            max_elements,
            _element_units: TaskElementUnits::None,
            counter: AtomicU64::new(0),
            children: RwLock::new(Vec::new()),
            created: UnixTimestamp::now(),
            started: OnceLock::new(),
            ended: OnceLock::new(),
        };
        Task {
            inner: Arc::new(inner),
        }
    }

    /// Returns the number of elements completed in the range `0..=max_elements`
    pub fn current_progress_count(&self) -> u64 {
        self.inner.counter.load(Ordering::Relaxed)
    }

    /// Returns the maximum number of elements of this task
    pub fn max_elements(&self) -> u64 {
        self.inner.max_elements
    }

    /// Returns the current progress as a fraction in the range `0..=1`
    pub fn current_progress_frac(&self) -> f64 {
        let cur = self.current_progress_count() as f64;
        let max = self.max_elements() as f64;
        cur / max
    }

    /// Returns the ID of this task.
    pub fn get_id(&self) -> u64 {
        self.inner.id.load(Ordering::Relaxed)
    }

    /// Returns the name of this task
    pub fn get_name(&self) -> &str {
        self.inner.name.as_str()
    }

    /// Returns the time this task was created
    pub fn get_created(&self) -> UnixTimestamp {
        self.inner.created
    }

    /// Returns the time at which this task started, or [`None`] if the task hasn't started yet.
    pub fn get_started(&self) -> Option<&UnixTimestamp> {
        self.inner.started.get()
    }

    /// Increments the 'completed' counter.
    pub fn mark_one_completed(&self) {
        let completed = self.inner.counter.fetch_add(1, Ordering::Relaxed);
        if completed == self.inner.max_elements {
            self.mark_ended();
        }
    }

    /// Mark this task complete.  Does not affect sub-tasks.
    pub fn mark_all_completed(&self) {
        self.inner
            .counter
            .store(self.inner.max_elements, Ordering::Relaxed);
        self.mark_ended();
    }

    /// Marks this task as started.  If the task has already started, does nothing.
    pub fn mark_started(&self) {
        let _res = self.inner.started.set(UnixTimestamp::now());
    }

    /// Returns the time at which this task ended, or None if the task hasn't ended yet.
    pub fn get_ended(&self) -> Option<&UnixTimestamp> {
        self.inner.ended.get()
    }

    /// Marks this task as ended.  If this task has already ended, does nothing.
    pub fn mark_ended(&self) {
        let _res = self.inner.ended.set(UnixTimestamp::now());
    }

    /// Returns the number of child tasks this task has
    pub fn num_children(&self) -> usize {
        let read = self.inner.children.read();
        let Ok(read) = read else {
            return 0;
        };
        read.len()
    }

    /// Iterates over each child task, providing a reference of the child task to the input function
    pub fn each_child<F: FnMut(&Task)>(&self, func: F) {
        let read = self.inner.children.read();
        let Ok(read) = read else {
            return;
        };
        read.iter().for_each(func)
    }

    ///
    /// Creates a new child task of this task
    #[must_use]
    pub fn new_child_task(&self, id: u64, name: String, max_elements: u64) -> Task {
        loop {
            let write = self.inner.children.write();
            if let Ok(mut write) = write {
                let task = Task::new(id, name, max_elements);
                let t2 = task.clone();
                write.push(task);
                return t2;
            };
        }
    }

    /// Returns true if this task is complete.
    pub fn is_complete(&self) -> bool {
        self.inner.ended.get().is_some() || self.current_progress_frac() >= 1.
    }
}
