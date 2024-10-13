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

    /// Reads all data from the offset into the provided buffer.  May require multiple file ops &
    /// seeks if the calls do not read all the data.
    fn seek_read_all(&mut self, mut out: &mut [u8], mut offset: u64) -> Result<(), Error> {
        while !out.is_empty() {
            let split = self.seek_read(out, offset)?;
            let (_, newo) = out.split_at_mut(split);
            out = newo;
            offset += split as u64;
        }
        Ok(())
    }
}

///
/// Trait to permit an atomic Seek+Write operation
pub trait SeekWrite {
    /// Seek to the provided offset, relative to the start of the stream, and write some bytes
    /// from the provided buffer.  The number of bytes written is returned.
    fn seek_write(&mut self, input: &[u8], offset: u64) -> Result<usize, Error>;

    /// Writes all the data to the provided offset.  May require multiple file ops & seeks
    /// if the calls do not write all the data.
    fn seek_write_all(&mut self, mut input: &[u8], mut offset: u64) -> Result<(), Error> {
        while !input.is_empty() {
            let split = self.seek_write(input, offset)?;
            let (_, nxt) = input.split_at(split);
            offset += split as u64;
            input = nxt;
        }
        Ok(())
    }
}
