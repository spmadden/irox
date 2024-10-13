// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Write};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use irox_bits::{BitsErrorKind, Error, MutBits, SeekWrite};
use crate::buf::FixedBuf;
use crate::codec::vbyte::encode_integer;
use crate::IntegerValue;

pub const DEFAULT_BLOCK_SIZE: usize = 4096;
pub const HEADER: &[u8] = b"IRXMSB";

pub struct MultiStreamWriter<const BlockSize: usize> {
    inner: Arc<Mutex<File>>,
    num_streams: Arc<AtomicU8>,
    current_block: Arc<AtomicU64>,
    stream_first_blocks: Arc<Mutex<BTreeMap<u8, u64>>>,
}
impl<const BlockSize: usize> MultiStreamWriter<BlockSize> {
    pub fn new(inner: File) -> MultiStreamWriter<BlockSize> {
        MultiStreamWriter {
            inner: Arc::new(Mutex::new(inner)),
            num_streams: Arc::new(AtomicU8::new(0)),
            current_block: Arc::new(AtomicU64::new(1)),
            stream_first_blocks: Arc::new(Mutex::new(Default::default())),
        }
    }
    pub fn new_stream<'a>(&'a self) -> StreamWriter<'a, BlockSize> {
        let idx= self.num_streams.fetch_add(1, Ordering::AcqRel);
        StreamWriter::new(self, idx)
    }
    pub(crate) fn write_block<'a>(&'a self, stream_idx: u8, block: &[u8;BlockSize]) -> Result<(), Error>{
        let block_idx = self.current_block.fetch_add(1, Ordering::AcqRel);
        {
            let Ok(mut lock) = self.stream_first_blocks.lock() else {
                return Err(Error::new(BitsErrorKind::BrokenPipe, "Error: Lock poisoned"));
            };
            if *lock.entry(stream_idx).or_insert(block_idx) == block_idx {
                let mut header = [0u8; BlockSize];
                let mut hdr = header.as_mut_slice();
                hdr.write_all(HEADER)?;
                for (k, v) in lock.iter() {
                    hdr.write_u8(*k)?;
                    hdr.write_all(&encode_integer(IntegerValue::U64(*v)))?;
                }
                drop(lock);
                let Ok(mut lock) = self.inner.lock() else {
                    return Err(Error::new(BitsErrorKind::BrokenPipe, "Error: Lock poisoned"));
                };
                lock.seek_write(&header, 0)?;
            }
        }
        let offset = block_idx * BlockSize as u64;
        let Ok(mut lock) = self.inner.lock() else {
            return Err(Error::new(BitsErrorKind::BrokenPipe, "Error: Lock poisoned"));
        };
        lock.seek_write(block, offset)?;

        Ok(())
    }
}

pub struct StreamWriter<'a, const BlockSize: usize> {
    parent: &'a MultiStreamWriter<BlockSize>,
    buf: FixedBuf<BlockSize, u8>,
    used: usize,
    stream_idx: u8,
}
impl<'a, const BlockSize: usize> StreamWriter<'a, BlockSize> {
    pub(crate) fn new(parent: &'a MultiStreamWriter<BlockSize>, stream_idx: u8) -> StreamWriter<'a, BlockSize> {
        StreamWriter {
            parent,
            buf: FixedBuf::new(),
            used: 0,
            stream_idx,
        }
    }
}

impl<'a, const BlockSize: usize> MutBits for StreamWriter<'a, BlockSize> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.parent.write_block(self.stream_idx, &self.buf.into_buf_default());
        todo!()
    }
}