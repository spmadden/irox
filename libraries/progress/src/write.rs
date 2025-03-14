// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::Task;
use irox_bits::{Error, MutBits};
use std::io::Write;

///
/// A Writer Task accepts a Writer and a task and updates the task with the count of bytes written.
pub struct WriterTask<T: Write> {
    writer: T,
    task: Task,
}

impl<T: Write> WriterTask<T> {
    /// Creates a new Writer Task
    #[must_use]
    pub fn new(writer: T, task: Task) -> Self {
        WriterTask { writer, task }
    }
}

impl<T: Write> Drop for WriterTask<T> {
    fn drop(&mut self) {
        self.task.mark_all_completed();
    }
}

impl<T: Write> Write for WriterTask<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let wrote = self.writer.write(buf)?;
        self.task.mark_some_completed(wrote as u64);
        Ok(wrote)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<T: Write> MutBits for WriterTask<T> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.task.mark_some_completed(1);
        Ok(self.writer.write_all(&[val])?)
    }

    fn write_all_bytes(&mut self, val: &[u8]) -> Result<(), Error> {
        self.task.mark_some_completed(val.len() as u64);
        Ok(self.writer.write_all(val)?)
    }
}
