// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Tools to represent a task that's "eventually" complete.  Very similar to a future, but without
//! the rigidity and infrastructure.
//!
//! # Example:
//! ```no_run
//! use irox_tools::sync::{Eventual, EventualStatus};
//! let eventual: Eventual<()> = Eventual::default();
//! loop {
//!     match eventual.get() {
//!         EventualStatus::NotReady => {
//!             // start the task, put into running state.
//!             eventual.start();
//!             // spawn a thread, enqueue a task, make the thing, etc.
//!             continue;
//!         }
//!         EventualStatus::Running => {
//!             // spin, maybe sleep or busy wait.
//!             core::hint::spin_loop();
//!             std::thread::sleep(std::time::Duration::from_millis(100));
//!             continue;
//!         }
//!         EventualStatus::CompleteEmpty => {
//!             // task is complete, but didn't return a response.
//!             break;
//!         }
//!         EventualStatus::Complete(value) => {
//!             // task is complete - use the value provided.
//!             // note - value is behind an Arc, so multiple calls to `get` if complete
//!             // will return the same value in memory
//!             break;
//!         }
//!     }
//! }
//! ```
extern crate alloc;

use alloc::sync::Arc;
use core::fmt::{Debug, Formatter};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::sync::{Condvar, Mutex, RwLock};

///
/// The current status of the Eventual Task
#[derive(Default)]
pub enum EventualStatus<T> {
    /// Task has not started yet, or has failed and should be retried
    #[default]
    NotReady,
    /// Task is actively running
    Running,
    /// Task is complete, and did not return a response (success/fail is not represented here, use a inner `Result`)
    CompleteEmpty,
    /// Task is complete, and has returned this response.
    Complete(T),
}
impl<T> Debug for EventualStatus<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            EventualStatus::NotReady => write!(f, "NotReady"),
            EventualStatus::Running => write!(f, "Running"),
            EventualStatus::CompleteEmpty => write!(f, "CompleteEmpty"),
            EventualStatus::Complete(_) => write!(f, "Complete"),
        }
    }
}
impl<T> From<Option<T>> for EventualStatus<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            None => EventualStatus::CompleteEmpty,
            Some(v) => EventualStatus::Complete(v),
        }
    }
}
impl<T> From<EventualStatus<T>> for Option<T> {
    fn from(value: EventualStatus<T>) -> Self {
        if let EventualStatus::Complete(v) = value {
            return Some(v);
        }
        None
    }
}
impl<T: Clone> Clone for EventualStatus<T> {
    fn clone(&self) -> Self {
        match self {
            EventualStatus::NotReady => EventualStatus::NotReady,
            EventualStatus::Running => EventualStatus::Running,
            EventualStatus::CompleteEmpty => EventualStatus::CompleteEmpty,
            EventualStatus::Complete(c) => EventualStatus::Complete(c.clone()),
        }
    }
}
impl<T> EventualStatus<T> {
    /// If the task is complete, return the completed value.  If it's not complete (or completed empty)
    /// returns [`None`]
    pub fn take(&mut self) -> Option<T> {
        if !self.is_complete() {
            return None;
        }
        core::mem::replace(self, EventualStatus::CompleteEmpty).into()
    }
    /// True if the task is completed.
    pub fn is_complete(&self) -> bool {
        if let EventualStatus::NotReady = self {
            return false;
        } else if let EventualStatus::Running = self {
            return false;
        }
        true
    }
    /// True if the task is pending
    pub fn is_pending(&self) -> bool {
        !self.is_complete()
    }
}

struct EventualInner<T> {
    condvar: Condvar,
    guard: Mutex<()>,
    val: RwLock<EventualStatus<Arc<T>>>,
}
impl<T> Default for EventualInner<T> {
    fn default() -> Self {
        EventualInner {
            condvar: Default::default(),
            guard: Default::default(),
            val: RwLock::new(EventualStatus::NotReady),
        }
    }
}
impl<T> EventualInner<T> {
    fn new(status: EventualStatus<Arc<T>>) -> Self {
        EventualInner {
            condvar: Default::default(),
            guard: Default::default(),
            val: RwLock::new(status),
        }
    }
}
///
/// Represents a computation result that may eventually appear.  Semantically equivalent to a Future,
/// but doesn't necessarily require an Async ecosystem around it.
#[derive(Clone)]
pub struct Eventual<T> {
    inner: Arc<EventualInner<T>>,
}
impl<T> Default for Eventual<T> {
    fn default() -> Self {
        Eventual {
            inner: Arc::new(EventualInner::default()),
        }
    }
}

impl<T> Eventual<T> {
    /// Creates a new already completed task with the specified value
    pub fn new_loaded(val: T) -> Self {
        Eventual {
            inner: Arc::new(EventualInner::new(EventualStatus::Complete(Arc::new(val)))),
        }
    }
    /// Sets the completion status of this task.  [`Some`] becomes [`EventualStatus::Complete`],
    /// [`None`] becomes [`EventualStatus::CompleteEmpty`]
    pub fn set(&self, val: Option<T>) {
        if let Ok(mut write) = self.inner.val.write() {
            let val = val.map(Arc::new);
            *write = val.into();
        }
        if self.is_ready() {
            self.inner.condvar.notify_all();
        }
    }
    ///
    /// Sets the completed status to [`EventualStatus::Complete`] using the specified value.
    pub fn set_shared(&self, val: Arc<T>) {
        if let Ok(mut write) = self.inner.val.write() {
            *write = EventualStatus::Complete(val);
            self.inner.condvar.notify_all()
        }
    }
    ///
    /// Gets the current status of this task.  This is the primary method to poll over.
    /// See the module docs for an example.
    pub fn get(&self) -> EventualStatus<Arc<T>> {
        if let Ok(read) = self.inner.val.read() {
            return read.clone();
        }
        EventualStatus::NotReady
    }
    ///
    /// If the task is complete, take the Arc out of storage.  Further calls to `get` or `take` will
    /// return `None`
    pub fn take(&self) -> Option<Arc<T>> {
        if let Ok(mut write) = self.inner.val.write() {
            return write.take();
        }
        None
    }
    ///
    /// Move the task state from `NotReady` or `CompleteEmpty` to `Running`.  Does nothing if already
    /// complete.
    pub fn start(&self) {
        if let Ok(mut write) = self.inner.val.write() {
            match *write {
                EventualStatus::NotReady | EventualStatus::CompleteEmpty => {
                    *write = EventualStatus::Running;
                }
                _ => {
                    //noop
                }
            }
        }
    }
    ///
    /// True if this task is complete with [`EventualStatus::CompleteEmpty`] or [`EventualStatus::Complete`]
    pub fn is_ready(&self) -> bool {
        if let Ok(read) = self.inner.val.read() {
            return read.is_complete();
        }
        false
    }
    ///
    /// True if this task has status [`EventualStatus::NotRunning`] or [`EventualStatus::Running`]
    pub fn is_pending(&self) -> bool {
        if let Ok(read) = self.inner.val.read() {
            return read.is_pending();
        }
        true
    }
    ///
    /// Blocks until this task transitions into [`EventualStatus::CompleteEmpty`] or
    /// [`EventualStatus::Complete`].  If already in this state, quick returns.
    pub fn block_until_ready(&self) -> EventualStatus<Arc<T>> {
        match self.get() {
            EventualStatus::CompleteEmpty => return EventualStatus::CompleteEmpty,
            EventualStatus::Complete(v) => return EventualStatus::Complete(v),
            _ => {}
        }
        if let Ok(guard) = self.inner.guard.lock() {
            let _unused = self.inner.condvar.wait_while(guard, |()| self.is_pending());
        }
        self.get()
    }
}

impl<T> Future for Eventual<T> {
    type Output = EventualStatus<Arc<T>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let status = self.get();
        if status.is_complete() {
            return Poll::Ready(status);
        }
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
