// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! An ecosystem for displaying progress, either in a UI or on the terminal.
//!

#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock};

pub use irox_time;
use irox_time::epoch::UnixTimestamp;
use irox_time::Duration;
use irox_tools::random::{Random, PRNG};
use irox_tools::sync::SynchronizedOptional;
use irox_tools::vec::RetainTake;

pub mod console;
#[cfg(feature = "egui")]
pub mod egui;
pub mod read;
pub mod write;

static RAND: OnceLock<Mutex<Random>> = OnceLock::new();

/// Shared random numbers.
fn get_random_id() -> u64 {
    if let Ok(mut rand) = RAND.get_or_init(|| Mutex::new(Random::default())).lock() {
        return rand.next_u64();
    };
    Random::default().next_u64()
}

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
    max_elements: AtomicU64,
    current_status: SynchronizedOptional<String>,
    _element_units: TaskElementUnits,
    created: UnixTimestamp,
    started: OnceLock<UnixTimestamp>,
    ended: OnceLock<UnixTimestamp>,
    remaining: RwLock<Duration>,
    children: RwLock<Vec<Task>>,
}

///
/// A task is a specific tracked operation.  It has:
/// - A Name
/// - A unique ID
/// - A creation time
/// - The current progress of the task (in "elements"), the 'counter'.
/// - An (optional) maximum number of elements.
/// - A time the task was "started"
/// - The estimated amount of time the task has remaining
/// - A time the task has "ended"
/// - A list of any "Child Tasks" that this task can spawn.
#[derive(Debug, Clone)]
pub struct Task {
    inner: Arc<TaskInner>,
    cancelled: Arc<AtomicBool>,
}

impl Task {
    /// Creates a new finite, named task with the specified ID.
    #[must_use]
    pub fn new(id: u64, name: String, max_elements: u64) -> Task {
        let inner = TaskInner {
            id: AtomicU64::new(id),
            name,
            max_elements: AtomicU64::new(max_elements),
            _element_units: TaskElementUnits::None,
            counter: AtomicU64::new(0),
            current_status: SynchronizedOptional::empty(),
            children: RwLock::new(Vec::new()),
            created: UnixTimestamp::now(),
            started: OnceLock::new(),
            ended: OnceLock::new(),
            remaining: RwLock::new(Duration::default()),
        };
        Task {
            inner: Arc::new(inner),
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Creates a new infinite, named task with a specific ID.
    #[must_use]
    pub fn new_infinite(id: u64, name: String) -> Task {
        Self::new(id, name, u64::MAX)
    }

    /// Creates a new infinite, named task with a random ID
    #[must_use]
    pub fn new_infinite_named(name: String) -> Task {
        let id = get_random_id();
        Task::new_infinite(id, name)
    }

    /// Creates a new finite, named task with a random ID.
    #[must_use]
    pub fn new_named(name: String, max_elements: u64) -> Task {
        let id = get_random_id();
        Task::new(id, name, max_elements)
    }

    /// Returns the number of elements completed in the range `0..=max_elements`
    #[must_use]
    pub fn current_progress_count(&self) -> u64 {
        self.inner.counter.load(Ordering::SeqCst)
    }

    /// Updates the current progress counter to be the specified value
    pub fn set_current_progress_count(&self, current_progress: u64) {
        self.inner.counter.store(current_progress, Ordering::SeqCst);
    }

    /// Returns the maximum number of elements of this task
    #[must_use]
    pub fn max_elements(&self) -> u64 {
        self.inner.max_elements.load(Ordering::SeqCst)
    }

    pub fn set_max_elements(&self, max_elements: u64) {
        self.inner
            .max_elements
            .store(max_elements, Ordering::SeqCst)
    }

    /// Returns the current progress as a fraction in the range `0..=1`
    #[must_use]
    pub fn current_progress_frac(&self) -> f64 {
        let cur = self.current_progress_count() as f64;
        let max = self.max_elements() as f64;
        cur / max
    }

    /// Returns the ID of this task.
    #[must_use]
    pub fn get_id(&self) -> u64 {
        self.inner.id.load(Ordering::SeqCst)
    }

    /// Returns the name of this task
    #[must_use]
    pub fn get_name(&self) -> &str {
        self.inner.name.as_str()
    }

    /// Returns the time this task was created
    #[must_use]
    pub fn get_created(&self) -> UnixTimestamp {
        self.inner.created
    }

    /// Returns the time at which this task started, or [`None`] if the task hasn't started yet.
    #[must_use]
    pub fn get_started(&self) -> Option<&UnixTimestamp> {
        self.inner.started.get()
    }

    /// Increments the 'completed' counter.
    pub fn mark_one_completed(&self) {
        let completed = self.inner.counter.fetch_add(1, Ordering::SeqCst);
        self.update_remaining();
        if completed == self.max_elements() {
            self.mark_ended();
        }
    }

    fn update_remaining(&self) {
        let completed = self.inner.counter.load(Ordering::SeqCst);
        if completed > 0 {
            if let Some(started) = self.get_started() {
                let mult = 1. / self.current_progress_frac();
                let elapsed = started.elapsed();
                let est_end = elapsed * mult;
                if let Ok(mut remaining) = self.inner.remaining.write() {
                    *remaining = est_end - elapsed;
                }
            }
        }
    }

    /// Mark this task complete.  Does not affect sub-tasks.
    pub fn mark_all_completed(&self) {
        self.inner
            .counter
            .store(self.max_elements(), Ordering::SeqCst);
        if let Ok(mut remaining) = self.inner.remaining.write() {
            *remaining = Duration::default();
        }
        self.mark_ended();
    }

    /// Mark some some portion of this task as completed.
    pub fn mark_some_completed(&self, completed: u64) {
        self.inner.counter.fetch_add(completed, Ordering::SeqCst);
        self.update_remaining()
    }

    pub fn get_remaining_time(&self) -> Duration {
        if let Ok(remaining) = self.inner.remaining.read() {
            return *remaining;
        }
        Duration::default()
    }

    /// Marks this task as started.  If the task has already started, does nothing.
    pub fn mark_started(&self) {
        let _res = self.inner.started.set(UnixTimestamp::now());
    }

    /// Returns the time at which this task ended, or None if the task hasn't ended yet.
    #[must_use]
    pub fn get_ended(&self) -> Option<&UnixTimestamp> {
        self.inner.ended.get()
    }

    /// Marks this task as ended.  If this task has already ended, does nothing.
    pub fn mark_ended(&self) {
        let _res = self.inner.ended.set(UnixTimestamp::now());
    }

    /// Returns the number of child tasks this task has
    #[must_use]
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

    pub fn clean_completed_children(&self) -> Vec<Task> {
        if let Ok(mut write) = self.inner.children.write() {
            return write.retain_take(Task::is_complete);
        }
        vec![]
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

    ///
    /// Appends this task as a tracked child task.
    pub fn push_new_child_task(&self, task: Task) {
        let write = self.inner.children.write();
        if let Ok(mut write) = write {
            write.push(task)
        }
    }

    /// Returns true if this task is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.inner.ended.get().is_some() || self.current_progress_frac() >= 1.
    }

    /// Marks this task as "Cancelled".  Users of this task may opt to ignore this flag, it's
    /// really more like a suggestion.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
        self.each_child(|ch| {
            ch.cancel();
        })
    }

    /// Returns true if this task has been marked 'cancelled'.  Cancelling a task is a one-way
    /// operation.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    /// Gets a copy of the current status
    #[must_use]
    pub fn current_status(&self) -> Option<Arc<String>> {
        self.inner.current_status.get()
    }

    /// Sets the optional current status of this task
    pub fn set_current_status<T: AsRef<str>>(&self, status: Option<T>) {
        let _res = self
            .inner
            .current_status
            .set(status.map(|v| v.as_ref().to_string()));
    }
}

#[macro_export]
macro_rules! get_human {
    ($inp:ident) => {{
        let temp = ((1. + $inp).log10() / 3.) as u32;
        let chr = match temp {
            0 => "",
            1 => "K",
            2 => "M",
            3 => "G",
            4 => "T",
            5 => "P",
            6 => "E",
            _ => "?",
        };
        let inp = $inp / 10f64.powf(3. * temp as f64);
        (inp, chr)
    }};
}
