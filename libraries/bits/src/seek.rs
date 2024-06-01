// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//
use crate::Error;

///
/// Trait to permit an atomic Seek+Read operation
pub trait SeekRead {
    /// Seek to the offset, relative to the start of the stream, and read some bytes into the
    /// provided buffer.  The number of bytes read is returned.
    fn seek_read(&mut self, out: &mut [u8], offset: u64) -> Result<usize, Error>;
}

///
/// Trait to permit an atomic Seek+Write operation
pub trait SeekWrite {
    /// Seek to the provided offset, relative to the start of the stream, and write some bytes
    /// from the provided buffer.  The number of bytes written is returned.
    fn seek_write(&mut self, input: &[u8], offset: u64) -> Result<usize, Error>;
}
