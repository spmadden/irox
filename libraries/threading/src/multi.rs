// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Multi-Threaded executors
//!

use std::collections::VecDeque;
use std::future::Future;
use std::sync::Arc;
use std::thread::JoinHandle;

use log::{debug, error};

use crate::single::{SingleThreadTask, TaskExchange};
use crate::{
    CompletableTask, CurrentThreadExecutor, Exchanger, ExchangerError, TaskError, TaskHandle,
};

pub(crate) struct Worker {
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        exchanger: Exchanger<WorkerCommand>,
        name: String,
    ) -> Result<Worker, std::io::Error> {
        let handle = std::thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                let mut current = CurrentThreadExecutor::new();
                loop {
                    let task = match exchanger.take() {
                        Ok(e) => match e {
                            WorkerCommand::Run(t) => t,
                            WorkerCommand::Close => {
                                debug!("Close command received, closing worker {:?}", name);
                                break;
                            }
                        },
                        Err(e) => {
                            if let ExchangerError::TaskError(e) = e {
                                if e != TaskError::ExecutorStoppingError {
                                    error!("Error receiving new task: {e:?}");
                                }
                            } else {
                                error!("Error receiving new task: {e:?}");
                            }
                            break;
                        }
                    };
                    current.submit(task.inner);
                    current.run_until_complete();
                }
            })?;
        Ok(Worker {
            handle: Some(handle),
        })
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            let _res = handle.join();
        }
    }
}

#[derive(Debug)]
pub struct Builder {
    name: String,
    max_workers: usize,
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}

impl Builder {
    #[must_use]
    pub fn new() -> Builder {
        Builder {
            name: String::new(),
            max_workers: 1,
        }
    }

    #[must_use]
    pub fn with_name(self, name: &str) -> Self {
        Builder {
            name: name.to_string(),
            ..self
        }
    }

    #[must_use]
    pub fn with_max_workers(self, max_workers: usize) -> Self {
        Builder {
            max_workers,
            ..self
        }
    }

    ///
    /// Creates a new [`MultiThreadedExecutor`] that has just a single worker, and can grow workers
    /// up to the number of CPU cores automatically.
    #[must_use]
    #[cfg(feature = "num_cpus")]
    pub fn with_num_cpu_workers(self) -> Self {
        self.with_max_workers(num_cpus::get())
    }

    #[must_use]
    pub fn build(self) -> MultiThreadedExecutor {
        MultiThreadedExecutor {
            exchanger: Exchanger::new(1),
            workers: Default::default(),
            max_workers: self.max_workers,
            worker_ctr: 0,
            name: self.name,
        }
    }
}

pub struct MultiThreadedExecutor {
    exchanger: Exchanger<WorkerCommand>,
    workers: VecDeque<Worker>,
    max_workers: usize,
    worker_ctr: usize,
    name: String,
}

impl Default for MultiThreadedExecutor {
    fn default() -> Self {
        Self::new_single()
    }
}

impl MultiThreadedExecutor {
    ///
    /// Creates a new [`MultiThreadedExecutor`] that has just a single worker.
    pub fn new_single() -> MultiThreadedExecutor {
        MultiThreadedExecutor::new_fixed(1)
    }

    ///
    /// Creates a new [`MultiThreadedExecutor`] with a fixed number of workers pre-allocated to it,
    /// and cannot automatically grow to add new workers.
    ///
    /// Note: Workers can be added or removed after-the-fact with the appropriate functions.
    pub fn new_fixed(worker_count: usize) -> MultiThreadedExecutor {
        let mut mte = Builder::new()
            .with_name(&format!("MTExec Fixed {worker_count}"))
            .with_max_workers(worker_count)
            .build();

        for _i in 0..worker_count {
            if let Err(e) = mte.add_worker() {
                error!("Error adding worker: {e:?}");
            }
        }

        mte
    }

    ///
    /// Submits a new task to be run on this executor.  The task will start to be run as soon
    /// as the executor has available capacity to run it.
    ///
    /// If there is no available worker, and additional workers are permitted to be added, a new
    /// worker will be allocated and started to service the task, and will be cached for future use.
    ///
    /// If no available worker and no additional workers are permitted, this will block until a
    /// worker accepts the task.  It will NOT be queued.  Once accepted, it will be immediately
    /// executed.
    ///
    /// This function returns a [`TaskHandle`] that can be used to retrieve any return
    /// result from the operation itself.
    pub fn submit<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &mut self,
        fut: F,
    ) -> Result<TaskHandle<T>, TaskError> {
        let complete = Arc::new(CompletableTask::new());
        let task = TaskExchange {
            inner: Box::pin(SingleThreadTask::<T>::new(Box::pin(fut), complete.clone())),
        };
        let task = WorkerCommand::Run(task);

        let task = match self.exchanger.try_push(task) {
            Ok(()) => {
                return Ok(TaskHandle {
                    completer: complete,
                })
            }
            Err(e) => match e {
                ExchangerError::TaskError(e) => {
                    return Err(e);
                }
                ExchangerError::ExchangerFull(task) => {
                    if self.worker_ctr < self.max_workers {
                        if let Err(e) = self.add_worker() {
                            error!("Tried to add worker, but could not: {e:?}");
                        }
                    }
                    task
                }
                ExchangerError::ExchangerEmpty => {
                    return Err(TaskError::ExchangerError);
                }
            },
        };

        if let Err(e) = self.exchanger.push(task) {
            error!("Error exchanging task: {e:?}");
            return match e {
                ExchangerError::TaskError(e) => Err(e),
                _ => {
                    error!("error exchanging task with worker: {e:?}");
                    Err(TaskError::ExchangerError)
                }
            };
        }

        Ok(TaskHandle {
            completer: complete,
        })
    }

    ///
    /// Manually adds an additional worker to this Executor.  This WILL spawn and start a new
    /// thread and will immediately start accepting tasks.
    ///
    /// This function will return an error type if the operating system does not permit a new thread
    /// to be spawned for the worker.
    pub fn add_worker(&mut self) -> Result<(), std::io::Error> {
        let name = format!("{}: Worker {}", self.name, self.worker_ctr);
        self.worker_ctr += 1;
        let worker = Worker::new(self.exchanger.clone(), name)?;
        self.workers.push_back(worker);
        Ok(())
    }

    ///
    /// Manually removes a worker from the Executor.  Tries to find a worker that isn't doing
    /// anything first and remove that.  If everyone's busy, just removes and blocks the front,
    /// the oldest worker.
    pub fn remove_worker(&mut self) {
        // scan for a worker not doing anything first.

        fn try_remove_idle(workers: &mut VecDeque<Worker>) -> bool {
            for idx in 0..workers.len() {
                let Some(worker) = workers.get(idx) else {
                    break;
                };
                let finished = if let Some(handle) = &worker.handle {
                    handle.is_finished()
                } else {
                    false
                };
                if finished {
                    let worker = workers.swap_remove_back(idx);
                    if let Some(worker) = worker {
                        // just force drop it, which will join the thread.
                        drop(worker);
                    }
                    return true;
                }
            }
            false
        }

        // see if any have already stopped.
        if try_remove_idle(&mut self.workers) {
            debug!("Successfully removed idle worker.");
            return;
        }

        // send the command to stop one
        if let Err(e) = self.exchanger.push(WorkerCommand::Close) {
            match e {
                ExchangerError::TaskError(e) => {
                    if e != TaskError::ExecutorStoppingError {
                        error!("Error commanding worker to close: {e:?}");
                    }
                }
                e => {
                    error!("Error commanding worker to close: {e:?}");
                }
            }
        }
    }

    pub fn shutdown(&self) {
        self.exchanger.shutdown()
    }
}

impl Drop for MultiThreadedExecutor {
    fn drop(&mut self) {
        self.exchanger.shutdown();
        while !self.workers.is_empty() {
            self.remove_worker();
        }
    }
}

pub(crate) enum WorkerCommand {
    Close,
    Run(TaskExchange),
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use log::{debug, trace};

    use crate::MultiThreadedExecutor;

    #[test]
    pub fn test_one() {
        // irox_log::init_console_level(Level::Info);

        let mut exec = MultiThreadedExecutor::new_fixed(1);

        let mut answers = Vec::new();
        for i in 0..100 {
            answers.push(
                exec.submit(async move {
                    std::thread::sleep(Duration::from_millis(1));
                    i
                })
                .unwrap(),
            );
            trace!("Submitted {i}");
        }

        exec.shutdown();

        let mut i = 0;
        for answer in answers {
            let ans = answer.get().unwrap();
            assert_eq!(ans, i);
            i += 1;
        }
    }

    #[test]
    pub fn test_ten() {
        // irox_log::init_console_level(Level::Trace);

        let mut exec = MultiThreadedExecutor::new_fixed(10);

        let mut answers = Vec::new();
        for i in 0..1000 {
            answers.push(
                exec.submit(async move {
                    std::thread::sleep(Duration::from_millis(1));
                    i
                })
                .unwrap(),
            );
            trace!("Submitted {i}");
        }
        debug!("Submitted all");

        let mut i = 0;
        for answer in answers {
            let ans = answer.get().unwrap();
            assert_eq!(ans, i);
            i += 1;
        }
    }
}
