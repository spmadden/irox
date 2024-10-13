// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! A Multi-Stream File is a file that allows multiple interlaced data streams within it.  Like
//! databases, pagefiles, and the ilk.  Each stream is essentially a linked list of pages, where
//! the last 4 bytes of the last page point to the next page index.

use crate::buf::{Buffer, FixedBuf};
use crate::codec::vbyte::encode_integer;
use crate::IntegerValue;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use irox_bits::{BitsErrorKind, Error, MutBits, SeekWrite};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

pub const DEFAULT_BLOCK_SIZE: usize = 16 * 1024; // 16K
pub const DATA_SIZE: usize = DEFAULT_BLOCK_SIZE - 4;
pub const HEADER: &[u8] = b"IRXMSB";

///
/// Writer for a Multi-Stream File.
#[derive(Clone)]
pub struct MultiStreamWriter {
    inner: Arc<Mutex<File>>,
    num_streams: Arc<AtomicU8>,
    current_block: Arc<AtomicU32>,
    stream_first_blocks: Arc<Mutex<BTreeMap<u8, u32>>>,
    stream_latest_blocks: Arc<Mutex<BTreeMap<u8, u32>>>,
}
impl MultiStreamWriter {
    ///
    /// Creates a new writer against the provided path.  If the file exists, will be truncated and
    /// any data removed.  If it doesn't exist, it will be created.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<MultiStreamWriter, Error> {
        let inner = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.as_ref())?;
        Ok(MultiStreamWriter {
            inner: Arc::new(Mutex::new(inner)),
            num_streams: Arc::new(AtomicU8::new(0)),
            current_block: Arc::new(AtomicU32::new(1)),
            stream_first_blocks: Arc::new(Mutex::new(Default::default())),
            stream_latest_blocks: Arc::new(Mutex::new(Default::default())),
        })
    }
    ///
    /// Creates a new buffered data stream within this writer.
    pub fn new_stream(self: &Arc<Self>) -> StreamWriter {
        let idx = self.num_streams.fetch_add(1, Ordering::AcqRel);
        StreamWriter::new(self.clone(), idx)
    }

    pub(crate) fn write_block(&self, stream_idx: u8, block: &[u8; DATA_SIZE]) -> Result<(), Error> {
        let block_idx = self.current_block.fetch_add(1, Ordering::AcqRel);
        {
            let Ok(mut lock) = self.stream_first_blocks.lock() else {
                return Err(Error::new(
                    BitsErrorKind::BrokenPipe,
                    "Error: Lock poisoned",
                ));
            };
            let stream_first_entry = lock.entry(stream_idx).or_insert(block_idx);
            if *stream_first_entry == block_idx {
                let mut header = [0u8; DEFAULT_BLOCK_SIZE];
                let mut hdr = header.as_mut_slice();
                hdr.write_all(HEADER)?;
                for (k, v) in lock.iter() {
                    hdr.write_u8(*k)?;
                    hdr.write_all(&encode_integer(IntegerValue::U32(*v)))?;
                }
                drop(lock);
                let Ok(mut lock) = self.inner.lock() else {
                    return Err(Error::new(
                        BitsErrorKind::BrokenPipe,
                        "Error: Lock poisoned",
                    ));
                };
                lock.seek_write(&header, 0)?;
            }
        }
        let offset = block_idx as u64 * DEFAULT_BLOCK_SIZE as u64;
        let Ok(mut lock) = self.inner.lock() else {
            return Err(Error::new(
                BitsErrorKind::BrokenPipe,
                "Error: Lock poisoned",
            ));
        };
        lock.seek_write(block, offset)?;
        let Ok(mut l2) = self.stream_latest_blocks.lock() else {
            return Err(Error::new(
                BitsErrorKind::BrokenPipe,
                "Error: Lock poisoned",
            ));
        };
        let last_block_idx = l2.entry(stream_idx).or_insert(block_idx);
        if *last_block_idx != block_idx {
            let offset = *last_block_idx as u64 * DEFAULT_BLOCK_SIZE as u64 + DATA_SIZE as u64;
            // println!("Updating {last_block_idx} at offset {offset:0X} to be new block {block_idx}");
            let byts = block_idx.to_be_bytes();
            lock.seek_write(&byts, offset)?;
            *last_block_idx = block_idx;
        }

        Ok(())
    }
}

///
/// A buffered writer for a single data stream.
pub struct StreamWriter {
    parent: Arc<MultiStreamWriter>,
    buf: FixedBuf<DATA_SIZE, u8>,
    stream_idx: u8,
}
impl StreamWriter {
    pub(crate) fn new(parent: Arc<MultiStreamWriter>, stream_idx: u8) -> StreamWriter {
        StreamWriter {
            parent,
            buf: FixedBuf::new(),
            stream_idx,
        }
    }
}

impl MutBits for StreamWriter {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        if self.buf.is_full() {
            let v = &self.buf.into_buf_default();
            self.parent.write_block(self.stream_idx, v)?;
        }
        self.buf.write_u8(val)
    }
}
impl Drop for StreamWriter {
    fn drop(&mut self) {
        if !self.buf.is_empty() {
            let v = &self.buf.into_buf_default();
            let _ = self.parent.write_block(self.stream_idx, v);
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod test {
    use crate::read::{MultiStreamWriter, DATA_SIZE};
    use irox_bits::{Error, MutBits};
    use std::sync::Arc;
    use std::thread::JoinHandle;
    use std::time::Instant;

    const NUM_BLOCKS: usize = 100_000;

    fn spawn_writer_stream(ms: &Arc<MultiStreamWriter>, value: u8) -> JoinHandle<()> {
        let mut stream = ms.new_stream();
        std::thread::spawn(move || {
            let num_blocks = NUM_BLOCKS;
            let count = num_blocks * DATA_SIZE;
            for _ in 0..count {
                stream.write_u8(value).unwrap();
            }
        })
    }

    #[test]
    #[ignore]
    pub fn test() -> Result<(), Error> {
        let ms = MultiStreamWriter::new("e:/test_multistream.ms")?;
        let ms = Arc::new(ms);
        let start = Instant::now();
        let mut handles = vec![
            spawn_writer_stream(&ms, 0xA),
            spawn_writer_stream(&ms, 0x9),
            spawn_writer_stream(&ms, 0xF),
            spawn_writer_stream(&ms, 0x5),
            spawn_writer_stream(&ms, 0x3),
            spawn_writer_stream(&ms, 0x2),
        ];

        handles.drain(..).for_each(|h| h.join().unwrap());

        let end = start.elapsed();
        let len = NUM_BLOCKS as u64 * DATA_SIZE as u64 * 6;
        let bs = len as f64 / end.as_secs_f64();
        let mbs = bs / 1e6;
        let lmb = len as f64 / 1e6;
        println!("Wrote {lmb} MB in {end:?} = {mbs:02.02} MB/s");
        Ok(())
    }
}
