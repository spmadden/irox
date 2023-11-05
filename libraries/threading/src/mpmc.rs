// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Multi-Producer, Multi-Consumer
//!

use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::{Arc, Condvar, Mutex};

use log::trace;

use crate::TaskError;

///
/// Exchanger errors
pub enum ExchangerError<T> {
    /// Error with the underlying executor
    TaskError(TaskError),
    /// Exchanger cannot accept an element as it is full.
    ExchangerFull(T),
    /// Exchanger cannot deliver an element as it is empty.
    ExchangerEmpty,
}

impl<T> Debug for ExchangerError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExchangerError::TaskError(e) => {
                write!(f, "TaskError: {e:?}")
            }
            ExchangerError::ExchangerFull(_) => {
                write!(f, "ExchangerFull")
            }
            ExchangerError::ExchangerEmpty => {
                write!(f, "ExchangerEmpty")
            }
        }
    }
}

impl<T> PartialEq for ExchangerError<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ExchangerError::TaskError(e) => {
                if let ExchangerError::TaskError(e2) = other {
                    return e == e2;
                }
                false
            }
            ExchangerError::ExchangerFull(_) => {
                matches!(other, ExchangerError::ExchangerFull(_))
            }
            ExchangerError::ExchangerEmpty => {
                matches!(other, ExchangerError::ExchangerEmpty)
            }
        }
    }
}

struct InnerExchange<T: Send> {
    mutex: Mutex<VecDeque<T>>,
    take_condition: Condvar,
    put_condition: Condvar,
    shutdown: AtomicBool,
    max_size: usize,
    num_waiting_takers: AtomicU16,
    num_waiting_putters: AtomicU16,
}

impl<T: Send> InnerExchange<T> {
    /// Creates a new InnerExchange.
    pub fn new(max_size: usize) -> Self {
        InnerExchange {
            max_size,
            mutex: Default::default(),
            take_condition: Default::default(),
            put_condition: Default::default(),
            shutdown: AtomicBool::new(false),
            num_waiting_takers: AtomicU16::new(0),
            num_waiting_putters: AtomicU16::new(0),
        }
    }

    ///
    /// Takes and returns an element, blocking if none are available.
    pub fn take_blocking(&self) -> Result<T, ExchangerError<T>> {
        let Ok(mut elems) = self.mutex.lock() else {
            return Err(ExchangerError::TaskError(TaskError::LockingError));
        };
        if let Some(e) = elems.pop_front() {
            trace!("Take_blocking popped one");
            self.put_condition.notify_one();
            return Ok(e);
        }
        // If there's still a few elements left in the queue, at this point they can be safely
        // flushed while shutting down, since no new elements can be added if this exchanger
        // is shutdown.  Prevent the waiter from waiting if we're shutting down.
        if self.shutdown.load(Ordering::SeqCst) {
            return Err(ExchangerError::TaskError(TaskError::ExecutorStoppingError));
        }
        trace!("Take_blocking waiting for element");
        self.num_waiting_takers.fetch_add(1, Ordering::SeqCst);
        let Ok(mut elems) = self.take_condition.wait_while(elems, |e| {
            e.is_empty() && !self.shutdown.load(Ordering::SeqCst)
        }) else {
            return Err(ExchangerError::TaskError(TaskError::LockingError));
        };
        self.num_waiting_takers.fetch_sub(1, Ordering::SeqCst);

        let Some(e) = elems.pop_front() else {
            trace!("Take_blocking woken up for empty exchange");
            // empty wakeup?  probably stopping.
            return Err(ExchangerError::TaskError(TaskError::ExecutorStoppingError));
        };
        trace!("Take_blocking woken up for new element");
        self.put_condition.notify_one();
        Ok(e)
    }

    ///
    /// Attempts to take one from this exchanger.  If one is available, it is returned. This
    /// function will not block, and if it would have blocked, returns
    /// [`Err(TaskError::ExchangerEmpty)`]
    pub fn try_take(&self) -> Result<T, ExchangerError<T>> {
        let Ok(mut elems) = self.mutex.lock() else {
            return Err(ExchangerError::TaskError(TaskError::LockingError));
        };
        if let Some(e) = elems.pop_front() {
            trace!("Take_blocking popped one");
            self.put_condition.notify_one();
            return Ok(e);
        }
        // If there's still a few elements left in the queue, at this point they can be safely
        // flushed while shutting down, since no new elements can be added if this exchanger
        // is shutdown.  Prevent the waiter from waiting if we're shutting down.
        if self.shutdown.load(Ordering::SeqCst) {
            return Err(ExchangerError::TaskError(TaskError::ExecutorStoppingError));
        }
        Err(ExchangerError::ExchangerEmpty)
    }

    ///
    /// Puts an element into this exchanger to allow exchanges to occur.  Will block indefinitely
    /// until there's enough space to put one in.
    pub fn put_blocking(&self, elem: T) -> Result<(), ExchangerError<T>> {
        // prevent any new elements from being put in if we're shutting down.
        if self.shutdown.load(Ordering::SeqCst) {
            return Err(ExchangerError::TaskError(TaskError::ExecutorStoppingError));
        }

        let Ok(mut elems) = self.mutex.lock() else {
            return Err(ExchangerError::TaskError(TaskError::LockingError));
        };
        if elems.len() < self.max_size {
            trace!("Put_blocking added one");
            elems.push_back(elem);
            self.take_condition.notify_one();
            return Ok(());
        }
        trace!("Put_blocking full, waiting for empty spot");
        // full, wait until a spot opens.
        self.num_waiting_putters.fetch_add(1, Ordering::SeqCst);
        let Ok(mut elems) = self.put_condition.wait_while(elems, |e| {
            e.len() >= self.max_size && !self.shutdown.load(Ordering::SeqCst)
        }) else {
            return Err(ExchangerError::TaskError(TaskError::LockingError));
        };
        self.num_waiting_putters.fetch_sub(1, Ordering::SeqCst);

        if elems.len() == self.max_size {
            trace!("Put_blocking woken up for full, cannot add new element");
            return Err(ExchangerError::ExchangerFull(elem));
        };

        trace!("Put_blocking woken up for free space");
        elems.push_back(elem);
        self.take_condition.notify_one();
        Ok(())
    }

    ///
    /// Attempts to put a new element into the exchanger, returning [`Ok`] if it was successful.
    /// This function will not block, if the call would have blocked, returns
    /// [`Err(ExchangerError::ExchangerFull(T))`] so you can have the element back.
    pub fn try_put(&self, elem: T) -> Result<(), ExchangerError<T>> {
        // prevent any new elements from being put in if we're shutting down.
        if self.shutdown.load(Ordering::SeqCst) {
            return Err(ExchangerError::TaskError(TaskError::ExecutorStoppingError));
        }

        let Ok(mut elems) = self.mutex.lock() else {
            return Err(ExchangerError::TaskError(TaskError::LockingError));
        };
        if elems.len() < self.max_size {
            trace!("try_put added one");
            elems.push_back(elem);
            self.take_condition.notify_one();
            return Ok(());
        }
        Err(ExchangerError::ExchangerFull(elem))
    }

    ///
    /// Stops this exchanger, waking up all waiters and
    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
        while self.num_waiting_putters.load(Ordering::SeqCst) > 0 {
            self.put_condition.notify_all();
        }
        while self.num_waiting_takers.load(Ordering::SeqCst) > 0 {
            self.take_condition.notify_all();
        }
    }
}

///
/// A thread-safe, shared exchange buffer.  Allows multiple producers to push elements in, up to a
/// blocking max capacity.  Allows multiple consumers to take elements out, blocking if none
/// available.
pub struct Exchanger<T: Send> {
    exchange: Arc<InnerExchange<T>>,
}

impl<T: Send> Clone for Exchanger<T> {
    fn clone(&self) -> Self {
        Exchanger {
            exchange: self.exchange.clone(),
        }
    }
}

impl<T: Send> Exchanger<T> {
    ///
    /// Creates a new exchanger, that can store at most these elements in the queue.
    ///
    /// Note:  Putting a value of `0`/zero for max_size implies that this exchanger will do no work
    /// and exchange no items.  A recommended minimum value of `1` should be used instead.
    pub fn new(max_size: usize) -> Self {
        Exchanger {
            exchange: Arc::new(InnerExchange::new(max_size)),
        }
    }

    ///
    /// Push a new element into the exchanger, blocking until space is available.
    pub fn push(&self, elem: T) -> Result<(), ExchangerError<T>> {
        self.exchange.put_blocking(elem)
    }

    pub fn try_push(&self, elem: T) -> Result<(), ExchangerError<T>> {
        self.exchange.try_put(elem)
    }

    ///
    /// Take a new element from the exchanger, blocking until one is available.
    pub fn take(&self) -> Result<T, ExchangerError<T>> {
        self.exchange.take_blocking()
    }

    pub fn try_take(&self) -> Result<T, ExchangerError<T>> {
        self.exchange.try_take()
    }

    ///
    /// Shuts down this exchanger, preventing new pushes.  Any objects already pushed will be
    /// permitted to be taken, and once empty, takers will receive a
    /// [`TaskError::ExecutorStoppingError`]
    pub fn shutdown(&self) {
        self.exchange.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    use std::thread::JoinHandle;
    use std::time::Duration;

    use log::{error, info, Level};

    use crate::{Exchanger, ExchangerError, TaskError};

    #[test]
    pub fn test_single_sender_receiver() -> Result<(), ExchangerError<u32>> {
        // irox_log::init_console_level(Level::Trace);
        let (err_sender, err_receiver) = std::sync::mpsc::channel();
        let err_sender2 = err_sender.clone();
        let exch1 = Exchanger::<u32>::new(10);
        let exch2 = exch1.clone();
        let genthrd = std::thread::Builder::new()
            .name("Sender".to_string())
            .spawn(move || {
                let mut sent = 0;
                for i in 0..1_000 {
                    if let Err(e) = exch2.push(i) {
                        eprintln!("Error sending exchange: {e:?}");
                        if let Err(e) = err_sender.send((e, i, "send")) {
                            panic!("{e:?}");
                        }
                    }
                    sent += 1;
                }
                println!("Sent {sent}");
            })
            .unwrap();

        let recv_thrd = std::thread::Builder::new()
            .name("Receiver".to_string())
            .spawn(move || {
                let mut recvd = 0;
                for i in 0..1_000 {
                    if let Err(e) = exch1.take() {
                        eprintln!("Error receiving exchange: {e:?}");
                        if let Err(e) = err_sender2.send((e, i, "recv")) {
                            panic!("{e:?}");
                        }
                    }
                    std::thread::sleep(Duration::from_millis(1)); // simulate work.
                    recvd += 1;
                }
                println!("Received {recvd}");
            })
            .unwrap();

        genthrd.join().unwrap();
        recv_thrd.join().unwrap();

        let mut errors: bool = false;
        while let Ok(r) = err_receiver.recv() {
            let (e, i, s) = r;
            eprintln!("Error received {e:?} : {i} : {s}");
            errors = true;
        }

        assert!(!errors);

        Ok(())
    }

    #[test]
    pub fn test_multiple_receivers() {
        irox_log::init_console_level(Level::Info);
        let (err_sender, err_receiver) = std::sync::mpsc::channel();
        let err_sender2 = err_sender.clone();
        let exch1 = Exchanger::<u32>::new(10);
        let exch2 = exch1.clone();
        let exch3 = exch1.clone();
        let genthrd = std::thread::Builder::new()
            .name("Sender".to_string())
            .spawn(move || {
                let mut sent = 0;
                for i in 0..1_000_000 {
                    if let Err(e) = exch2.push(i) {
                        eprintln!("Error sending exchange: {e:?}");
                        if let Err(e) = err_sender.send((e, i, "send")) {
                            panic!("{e:?}");
                        }
                    }
                    sent += 1;
                }
                info!("Sent {sent}");
            })
            .unwrap();

        let recv_count = Arc::new(AtomicU64::new(0));
        let mut receivers: Vec<JoinHandle<()>> = Vec::new();
        for thread_idx in 0..10 {
            let counter = recv_count.clone();
            let err_sender2 = err_sender2.clone();
            let exch1 = exch1.clone();
            let recv_thrd = std::thread::Builder::new()
                .name(format!("Receiver {thread_idx}"))
                .spawn(move || {
                    let counter = counter;

                    let mut recvd = 0;
                    loop {
                        if let Err(e) = exch1.take() {
                            if e == ExchangerError::TaskError(TaskError::ExecutorStoppingError) {
                                // it's a good thing!
                                break;
                            }
                            error!("Error receiving exchange: {e:?}");
                            if let Err(e) = err_sender2.send((e, recvd, "recv")) {
                                panic!("Error sending error: {e:?}");
                            }
                            break;
                        }
                        // std::thread::sleep(Duration::from_millis(1));// simulate work.
                        recvd += 1;
                        counter.fetch_add(1, Ordering::Relaxed);
                    }
                    info!(
                        "Received {recvd} in thread {}",
                        std::thread::current().name().unwrap_or("")
                    );
                })
                .unwrap();
            receivers.push(recv_thrd);
        }
        drop(err_sender2);

        genthrd.join().unwrap();
        info!("Generator thread joined");
        exch3.shutdown();
        info!("Executor shutdown");

        for recv in receivers {
            info!("Waiting on {}", recv.thread().name().unwrap_or(""));
            recv.join().unwrap();
        }

        let mut errors: bool = false;
        while let Ok(r) = err_receiver.recv() {
            let (e, i, s) = r;
            error!("Error received {e:?} : {i} : {s}");
            errors = true;
        }

        assert!(!errors);
    }
}
