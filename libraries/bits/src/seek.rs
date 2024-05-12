// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//
use crate::Error;

pub trait SeekRead {
    fn seek_read(&mut self, out: &mut [u8], offset: u64) -> Result<usize, Error>;
}

pub trait SeekWrite {
    fn seek_write(&mut self, input: &[u8], offset: u64) -> Result<usize, Error>;
}
