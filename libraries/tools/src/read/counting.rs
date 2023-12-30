// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};
use std::io::Read;

///
/// Wraps the reader, providing a convenient way to count the number of bytes read from the
/// underlying impl.
pub struct ReadCounting<T: Read> {
    reader: T,
    count: u64,
}

impl<T: Read> ReadCounting<T> {
    pub fn new(reader: T) -> Self {
        ReadCounting { reader, count: 0 }
    }

    /// Returns the number of bytes read to this point.
    pub fn get_read_count(&self) -> u64 {
        self.count
    }
}
impl<T: Read> Read for ReadCounting<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read = self.reader.read(buf)?;
        self.count += read as u64;
        Ok(read)
    }
}

///
/// A [`ReadCounting`] except can be shared between threads
pub struct SharedReadCounting<T: Read> {
    reader: T,
    counter: Arc<AtomicU64>,
}

impl<T: Read> SharedReadCounting<T> {
    pub fn new(reader: T) -> Self {
        SharedReadCounting {
            reader,
            counter: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Returns a shared read-only copy of the counter
    pub fn get_counter(&self) -> SharedROCounter {
        SharedROCounter {
            counter: self.counter.clone(),
        }
    }
}

impl<T: Read> Read for SharedReadCounting<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read = self.reader.read(buf)?;
        self.counter.fetch_add(read as u64, Ordering::Relaxed);
        Ok(read)
    }
}

///
/// A Read-Only counter that can be shared between threads.  The owner of the underlying counter
/// is free to update the value, but users of this object alone may not.
#[derive(Debug, Clone)]
pub struct SharedROCounter {
    counter: Arc<AtomicU64>,
}

impl SharedROCounter {
    pub fn new(counter: Arc<AtomicU64>) -> Self {
        SharedROCounter { counter }
    }

    ///
    /// Returns the current value of the counter
    pub fn get_count(&self) -> u64 {
        self.counter.load(Ordering::Relaxed)
    }
}
