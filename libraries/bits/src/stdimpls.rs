// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::mutbits::MutBits;
use crate::{Bits, Error, SeekRead, SeekWrite};

impl Bits for std::fs::File {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        use std::io::Read;
        let mut buf: [u8; 1] = [0];
        let read = self.read(&mut buf)?;
        if read == 1 {
            return Ok(Some(buf[0]));
        }
        Ok(None)
    }
}

impl MutBits for std::fs::File {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        use std::io::Write;
        Ok(self.write_all(&[val])?)
    }
}

#[cfg(windows)]
impl SeekRead for std::fs::File {
    fn seek_read(&mut self, out: &mut [u8], offset: u64) -> Result<usize, Error> {
        Ok(std::os::windows::fs::FileExt::seek_read(self, out, offset)?)
    }
}
#[cfg(windows)]
impl SeekWrite for std::fs::File {
    fn seek_write(&mut self, input: &[u8], offset: u64) -> Result<usize, Error> {
        Ok(std::os::windows::fs::FileExt::seek_write(
            self, input, offset,
        )?)
    }
}

#[cfg(unix)]
impl SeekRead for std::fs::File {
    fn seek_read(&mut self, out: &mut [u8], offset: u64) -> Result<usize, Error> {
        Ok(std::os::unix::fs::FileExt::read_at(self, out, offset)?)
    }
}
#[cfg(unix)]
impl SeekWrite for std::fs::File {
    fn seek_write(&mut self, input: &[u8], offset: u64) -> Result<usize, Error> {
        Ok(std::os::unix::fs::FileExt::write_at(self, input, offset)?)
    }
}
