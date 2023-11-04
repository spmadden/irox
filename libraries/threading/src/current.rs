// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

use crate::{LocalCompletableTask, LocalFuture};

trait LocalFutureType<'a>: Future<Output = ()> + 'a + HasLocalWaker {}

///
/// An Executor that doesn't spawn new threads, just runs on the current thread.
#[derive(Default)]
pub struct CurrentThreadExecutor<'a> {
    processing_queue: VecDeque<Pin<Box<dyn LocalFutureType<'a, Output = ()>>>>,
}

impl<'a> CurrentThreadExecutor<'a> {
    /// Create a new [`CurrentThreadExecutor`]
    pub fn new() -> Self {
        CurrentThreadExecutor::default()
    }

    ///
    /// Submit a new task to this executor.  Note:  This does not immediately run the task, you
    /// still need to call either [`CurrentThreadExecutor::run_some`] or
    /// [`CurrentThreadExecutor::run_until_complete`]
    pub fn submit<T: 'a>(&mut self, fut: impl Future<Output = T> + 'a) -> LocalTaskHandle<T> {
        let task = LocalTask {
            future: Box::pin(fut),
            waker: Arc::new(LocalWaker::default()),
            complete: LocalCompletableTask::new(),
        };
        let handle = task.join_handle();
        self.processing_queue.push_back(Box::pin(task));
        handle
    }

    ///
    /// Runs a single loop through the processing queue, in order, letting each task attempt to do
    /// work.
    pub fn run_some(&mut self) {
        let mut pinned = Pin::new(self);
        let mut pending = VecDeque::new();
        while let Some(mut task) = pinned.processing_queue.pop_front() {
            if !task.needs_wake() {
                pending.push_back(task);
                continue;
            }
            let waker = Waker::from(task.get_waker());
            let mut context = Context::from_waker(&waker);

            match task.as_mut().poll(&mut context) {
                Poll::Ready(()) => {}
                Poll::Pending => {
                    // reschedule task again.
                    task.get_waker()
                        .needs_running
                        .store(false, Ordering::Relaxed);
                    pending.push_back(task);
                }
            }
        }
        pinned.processing_queue.append(&mut pending);
    }

    ///
    /// Runs this executor until all submitted tasks are complete.
    pub fn run_until_complete(mut self) {
        while !self.processing_queue.is_empty() {
            self.run_some();
        }
    }
}

///
/// Local thread Waker struct
pub struct LocalWaker {
    needs_running: AtomicBool,
}

impl Default for LocalWaker {
    fn default() -> Self {
        LocalWaker {
            needs_running: AtomicBool::new(true),
        }
    }
}

impl Wake for LocalWaker {
    fn wake(self: Arc<Self>) {
        self.needs_running.store(true, Ordering::Relaxed);
    }
}

trait HasLocalWaker {
    fn needs_wake(&self) -> bool;
    fn clear_wake(&self);
    fn get_waker(&self) -> Arc<LocalWaker>;
}
///
/// A task that can be run on the current thread.
pub struct LocalTask<'a, T> {
    future: LocalFuture<'a, T>,
    waker: Arc<LocalWaker>,
    complete: LocalCompletableTask<T>,
}
impl<'a, T> HasLocalWaker for LocalTask<'a, T>
where
    T: 'a,
{
    fn needs_wake(&self) -> bool {
        self.waker.needs_running.load(Ordering::Relaxed)
    }

    fn clear_wake(&self) {
        self.waker.needs_running.store(false, Ordering::Relaxed);
    }

    fn get_waker(&self) -> Arc<LocalWaker> {
        self.waker.clone()
    }
}
impl<'a, T> LocalTask<'a, T>
where
    T: 'a,
{
    pub fn join_handle(&self) -> LocalTaskHandle<T> {
        LocalTaskHandle {
            result: self.complete.clone(),
        }
    }
}

impl<'a, T> Future for LocalTask<'a, T> {
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

impl<'a, T: 'a> LocalFutureType<'a> for LocalTask<'a, T> {}

///
/// A handle to the submitted task, to retrieve the result of the operation
pub struct LocalTaskHandle<T> {
    result: LocalCompletableTask<T>,
}

impl<T> LocalTaskHandle<T> {
    ///
    /// Attempts to retrive the result of the operation.  If the operation isn't complete yet,
    /// returns [`None`]
    pub fn get(&mut self) -> Option<T> {
        match self.result.get() {
            Poll::Ready(e) => Some(e),
            Poll::Pending => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CurrentThreadExecutor;

    #[test]
    pub fn test() {
        let mut executor = CurrentThreadExecutor::new();

        let mut handle = executor.submit(async { println!("Hello async") });
        let mut handle2 = executor.submit(async { println!("Hello async2") });

        assert_eq!(None, handle.get());
        assert_eq!(None, handle2.get());

        executor.run_until_complete();

        assert_ne!(None, handle.get());
        assert_ne!(None, handle2.get());
    }
}
