// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use crate::Task;
use std::io::Read;

///
/// A Reader Task takes a Read, and updates the task with the bytes as they're read.
pub struct ReaderTask<T: Read> {
    reader: T,
    task: Task,
}

impl<T: Read> ReaderTask<T> {
    /// Creates a new reader task from the specified reader, and task
    pub fn new(reader: T, task: Task) -> Self {
        ReaderTask { reader, task }
    }
}

impl<T: Read> Read for ReaderTask<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read = self.reader.read(buf)?;
        self.task.mark_some_completed(read as u64);
        Ok(read)
    }
}
