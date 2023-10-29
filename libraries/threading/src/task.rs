// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use std::cell::{OnceCell, RefCell};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};
use std::task::Poll;

pub type SendFuture<'a, T> = dyn Future<Output = T> + Send + 'a;
pub type SharedSendFuture<'a, T> = Arc<SendFuture<'a, T>>;
pub type SharedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type LocalFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
pub type LocalVoidFuture<'a> = LocalFuture<'a, ()>;

#[derive(Debug, Copy, Clone)]
pub enum TaskError {
    LockingError,
    NotCompletedError,
}

struct CompletableTaskInner<T> {
    result: OnceCell<T>,
    is_complete: bool,
}
impl<T> CompletableTaskInner<T> {
    pub fn new() -> Self {
        CompletableTaskInner {
            result: OnceCell::new(),
            is_complete: false,
        }
    }

    pub fn try_set(&mut self, value: T) -> Result<(), T> {
        if self.is_complete {
            return Err(value);
        };
        let oldval = self.result.set(value);
        self.is_complete = true;
        oldval?;

        Ok(())
    }

    pub fn take(&mut self) -> Option<T> {
        self.result.take()
    }
}

///
/// A `CompletableTask` is a one-time-use shuttle struct to enable tasks/threads
/// to provide the result of an compute operation.  Once the task is completed,
/// any additional attempts to complete the task results in an error.
///
/// This is thread-safe equivalent to [`OnceCell<T>`], but combines the ability
/// to block the current thread until the task completes.
pub struct CompletableTask<T> {
    inner: Arc<(Mutex<CompletableTaskInner<T>>, Condvar)>,
}

impl<T> CompletableTask<T> {
    ///
    /// Creates a new [`CompletableTask`]
    pub fn new() -> CompletableTask<T> {
        let inner = CompletableTaskInner::new();
        CompletableTask {
            inner: Arc::new((Mutex::new(inner), Condvar::new())),
        }
    }

    ///
    /// Attempt to complete this task with the specified value.
    ///
    /// Returns `Ok(())` if the task was successfully completed.
    /// Returns `Err(value)` with the provided value if:
    /// * The task has already completed
    /// * Any errors in locking or mutex poisoning prevented the completion
    pub fn try_complete(&self, value: T) -> Result<(), T> {
        let arc = self.inner.clone();
        let Ok(mut inner) = arc.0.lock() else {
            return Err(value);
        };
        if inner.is_complete {
            return Err(value);
        }
        inner.try_set(value)?;

        arc.1.notify_all();
        Ok(())
    }

    ///
    /// Checks if the task has been completed.
    ///
    /// * Returns `Ok(true)` if the task has been completed
    /// * Returns `Ok(false)` if the task has NOT been completed
    /// * Returns `Err(())` if any errors in locking prevented the checks
    pub fn is_complete(&self) -> Result<bool, TaskError> {
        let arc = self.inner.clone();
        let Ok(inner) = arc.0.lock() else {
            return Err(TaskError::LockingError);
        };
        Ok(inner.is_complete)
    }

    ///
    /// Gets the result of the operation if it has been set.  Does NOT block until
    /// the task is complete.  Use [`CompletableTask::take_blocking`] for blocking requests.
    ///
    /// Returns `Ok(Poll::Ready(T))` if the task has been completed
    /// Returns `Ok(Poll::Pending))` if the task has NOT been completed
    /// Returns `Err(())` if the underlying mutex has been poisoned and is corrupt.
    pub fn try_take(&self) -> Result<Poll<T>, TaskError> {
        let arc = self.inner.clone();
        let Ok(mut inner) = arc.0.lock() else {
            return Err(TaskError::LockingError);
        };
        if let Some(val) = inner.take() {
            return Ok(Poll::Ready(val));
        }
        Ok(Poll::Pending)
    }

    ///
    /// Gets the result of the operation, blocking until the operation is complete.
    ///
    /// Returns `Ok(T)` if the operation completed,
    /// Returns `Err(())` if any error happens.
    pub fn take_blocking(&self) -> Result<T, TaskError> {
        let arc = self.inner.clone();
        let Ok(inner) = arc.0.lock() else {
            return Err(TaskError::LockingError);
        };
        let Ok(mut res) = arc.1.wait_while(inner, |v| !v.is_complete) else {
            return Err(TaskError::LockingError);
        };
        if let Some(val) = res.result.take() {
            return Ok(val);
        }
        Err(TaskError::NotCompletedError)
    }
}

impl<T> Default for CompletableTask<T> {
    fn default() -> Self {
        CompletableTask::new()
    }
}

pub struct LocalCompletableTask<T> {
    result: Rc<RefCell<Option<T>>>,
}

impl<T> Clone for LocalCompletableTask<T> {
    fn clone(&self) -> Self {
        LocalCompletableTask {
            result: self.result.clone(),
        }
    }
}

impl<T> LocalCompletableTask<T> {
    pub fn new() -> Self {
        LocalCompletableTask {
            result: Rc::new(RefCell::new(None)),
        }
    }

    pub fn try_complete(&self, value: T) -> Result<(), T> {
        let res = self.result.clone();
        if res.borrow().is_some() {
            return Err(value);
        }
        if let Some(t) = res.replace(Some(value)) {
            return Err(t);
        }
        Ok(())
    }

    pub fn get(&self) -> Poll<T> {
        if let Some(v) = self.result.take() {
            return Poll::Ready(v);
        }
        Poll::Pending
    }
}

impl<T> Default for LocalCompletableTask<T> {
    fn default() -> Self {
        LocalCompletableTask::new()
    }
}
