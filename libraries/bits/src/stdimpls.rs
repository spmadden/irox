// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::mutbits::MutBits;
use crate::{Bits, BitsWrapper, Error, Seek, SeekFrom, SeekRead, SeekWrite};

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
impl Seek for std::fs::File {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        std::io::Seek::seek(self, pos.into())?;
        Ok(std::io::Seek::stream_position(self)?)
    }
}

impl Bits for std::net::TcpStream {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        BitsWrapper::Borrowed(self).next_u8()
    }
}
impl Bits for &mut std::net::TcpStream {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        BitsWrapper::Borrowed(self).next_u8()
    }
}
impl MutBits for std::net::TcpStream {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        BitsWrapper::Borrowed(self).write_u8(val)
    }
}
impl MutBits for &mut std::net::TcpStream {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        BitsWrapper::Borrowed(self).write_u8(val)
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
