// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Single-Thread Executor implementation
//!

use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{Sender, TryRecvError};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread::JoinHandle;

use crate::{CompletableTask, CurrentThreadExecutor, TaskError};

///
/// An executor implementation backed by a single thread.
///
/// Unfortunately, in order to maintain a 100% "safe" codebase, it can only accept futures with a
/// lifetime of [`'static`].  The [`CurrentThreadExecutor`] does not have this limitation.
///
/// This actually uses a [`CurrentThreadExecutor`] wrapped in a single thread.
///
/// This executor will run all tasks to completion, even when dropped.
pub struct SingleThreadExecutor {
    sender: Option<Sender<TaskExchange>>,
    handle: Option<JoinHandle<()>>,
}

pub(crate) type SingleThreadFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

impl SingleThreadExecutor {
    ///
    /// Creates a new [`SingleThreadExecutor`] and spawns a new thread to back it.  The thread
    /// immediately starts attempting to execute jobs.
    ///
    /// The queue is an unlimited queue, and will happily accept as many jobs as you can pass to it.
    #[must_use]
    pub fn new() -> SingleThreadExecutor {
        let (sender, receiver) = std::sync::mpsc::channel::<TaskExchange>();

        let handle = std::thread::spawn(move || {
            let mut current = CurrentThreadExecutor::new();
            loop {
                if let Some(task) = match receiver.try_recv() {
                    Ok(e) => Some(e),
                    Err(e) => {
                        if e == TryRecvError::Disconnected {
                            break;
                        }
                        None
                    }
                } {
                    current.submit(task.inner);
                };
                current.run_some();
            }
            current.run_until_complete();
        });
        SingleThreadExecutor {
            handle: Some(handle),
            sender: Some(sender),
        }
    }

    ///
    /// Submits a new task to be run on this executor.  The task will start to be run as soon
    /// as the executor has available capacity to run it.
    ///
    /// This function returns a [`TaskHandle`] that can be used to retrieve any return
    /// result from the operation itself.
    pub fn submit<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &mut self,
        fut: F,
    ) -> Result<TaskHandle<T>, TaskError> {
        let complete = Arc::new(CompletableTask::new());
        let task = SingleThreadTask::new(Box::pin(fut), complete.clone());
        if let Some(sender) = &self.sender {
            let _res = sender.send(TaskExchange {
                inner: Box::pin(task),
            });
        }
        Ok(TaskHandle {
            completer: complete,
        })
    }

    ///
    /// Runs this executor until all tasks are complete.
    pub fn run_until_complete(self) {
        drop(self)
    }
}

///
/// Same as [`SingleThreadExecutor::new`]
impl Default for SingleThreadExecutor {
    ///
    /// Same as [`SingleThreadExecutor::new`]
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SingleThreadExecutor {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }
        if let Some(handle) = self.handle.take() {
            let _res = handle.join();
        }
    }
}

///
/// Simplifying struct to pass between the caller and the executor thread.
pub(crate) struct TaskExchange {
    pub(crate) inner: SingleThreadFuture<()>,
}

///
/// A handle to the return result of the submitted task.
pub struct TaskHandle<T> {
    pub(crate) completer: Arc<CompletableTask<T>>,
}

impl<T> TaskHandle<T> {
    ///
    /// Returns true if the task has completed and a result is available.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.completer.is_complete().unwrap_or(false)
    }

    ///
    /// Blocks until the task is complete, and returns [`Ok(T)`] if the task completed,
    /// or [`Err`] if a mutex/locking (panic) occurred during execution.
    pub fn get(&self) -> Option<T> {
        self.completer.take_blocking().ok()
    }
}

///
/// A wrapper future task that actually executes the requested task, and completes the
/// future to idnicate that the
pub(crate) struct SingleThreadTask<T> {
    future: SingleThreadFuture<T>,
    complete: Arc<CompletableTask<T>>,
}

impl<T> SingleThreadTask<T> {
    pub fn new(future: SingleThreadFuture<T>, complete: Arc<CompletableTask<T>>) -> Self {
        SingleThreadTask { future, complete }
    }
}

impl<T> Future for SingleThreadTask<T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mself = self.get_mut();
        match mself.future.as_mut().poll(cx) {
            Poll::Ready(e) => {
                let _ign = mself.complete.try_complete(e);
                Poll::Ready(())
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{SingleThreadExecutor, TaskError};

    #[test]
    pub fn test() -> Result<(), TaskError> {
        let mut exec = SingleThreadExecutor::new();
        let borrowed = String::new();
        let hnd = exec.submit(async move {
            println!("Hello from thread! {borrowed}");
        })?;

        drop(exec);
        assert!(hnd.is_complete());
        assert_eq!(Some(()), hnd.get());

        Ok(())
    }
}
