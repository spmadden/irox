// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//! Tape Archive format Readers

use std::path::Path;
use std::sync::Arc;
use crate::error::Error;

///
/// Tape Archive.  Thread-safe as it's backed internally by an `Arc` and cheap to clone.
pub struct TapeArchiveReader {
    inner: Arc<TapeArchiveInner>
}
impl Clone for TapeArchiveReader {
    fn clone(&self) -> Self {
        TapeArchiveReader {
            inner: self.inner.clone()
        }
    }
}

impl TapeArchiveReader {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<TapeArchiveReader, Error> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(path)?;
        
        todo!()
    }
}

pub(crate) struct TapeArchiveInner {

}