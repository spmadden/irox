// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! A Multi-Stream File is a file that allows multiple interlaced data streams within it.  Like
//! databases, pagefiles, and the ilk.  Each stream is essentially a linked list of pages, where
//! the last 4 bytes of the last page point to the next page index.

use crate::buf::{Buffer, FixedBuf, RoundU8Buffer};
use crate::codec::{encode_integer, DecodeVByte};
use crate::IntegerValue;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use irox_bits::{Bits, BitsErrorKind, Error, MutBits, SeekRead, SeekWrite};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

pub const DEFAULT_BLOCK_SIZE: usize = 4 * 1024; // 16K
pub const DATA_SIZE: usize = DEFAULT_BLOCK_SIZE - 4;
pub const HEADER: &[u8] = b"IRXMSB";

macro_rules! broken_pipe {
    () => {
        Err(Error::new(
            BitsErrorKind::BrokenPipe,
            "Error: Lock poisoned",
        ))
    };
}

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
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Arc<MultiStreamWriter>, Error> {
        let inner = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.as_ref())?;

        Ok(Arc::new(MultiStreamWriter {
            inner: Arc::new(Mutex::new(inner)),
            num_streams: Arc::new(AtomicU8::new(1)),
            current_block: Arc::new(AtomicU32::new(1)),
            stream_first_blocks: Arc::new(Mutex::new(Default::default())),
            stream_latest_blocks: Arc::new(Mutex::new(Default::default())),
        }))
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
                return broken_pipe!();
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
                    return broken_pipe!();
                };
                lock.seek_write_all(&header, 0)?;
            }
        }
        let offset = block_idx as u64 * DEFAULT_BLOCK_SIZE as u64;
        let Ok(mut lock) = self.inner.lock() else {
            return broken_pipe!();
        };
        lock.seek_write_all(block, offset)?;
        let Ok(mut l2) = self.stream_latest_blocks.lock() else {
            return broken_pipe!();
        };
        lock.write_all(&[0, 0, 0, 0])?;
        let last_block_idx = l2.entry(stream_idx).or_insert(block_idx);
        if *last_block_idx != block_idx {
            let offset = *last_block_idx as u64 * DEFAULT_BLOCK_SIZE as u64 + DATA_SIZE as u64;
            // println!("Updating {last_block_idx} at offset {offset:0X} to be new block {block_idx}");
            let byts = block_idx.to_be_bytes();
            lock.seek_write_all(&byts, offset)?;
            *last_block_idx = block_idx;
        }

        Ok(())
    }

    pub fn len(&self) -> Result<u64, Error> {
        if let Ok(lock) = self.inner.lock() {
            return Ok(lock.metadata()?.len());
        }
        broken_pipe!()
    }
    pub fn is_empty(&self) -> Result<bool, Error> {
        Ok(self.len()? == 0)
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
        let mut buf = FixedBuf::default();
        let _ = buf.write_be_u16(0);
        StreamWriter {
            parent,
            buf,
            stream_idx,
        }
    }
}

impl MutBits for StreamWriter {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        if self.buf.is_full() {
            let v = &self.buf.into_buf_default();
            self.parent.write_block(self.stream_idx, v)?;
            self.buf.write_be_u16(0x0)?;
        }
        self.buf.write_u8(val)
    }
}
impl Drop for StreamWriter {
    fn drop(&mut self) {
        if !self.buf.is_empty() {
            let len = self.buf.len() as u16 - 2;
            let v = &mut self.buf.into_buf_default();
            let _ = v.as_mut_slice().write_be_u16(len);
            let _ = self.parent.write_block(self.stream_idx, v);
        }
    }
}

///
/// Reader for a multi-stream file.
#[derive(Clone)]
pub struct MultiStreamReader {
    inner: Arc<Mutex<File>>,
    stream_next_block: Arc<Mutex<BTreeMap<u8, u32>>>,
}
impl MultiStreamReader {
    /// Opens a multi-stream file and returns readers for every stream contained therein.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Vec<StreamReader>, Error> {
        let mut inner = OpenOptions::new().read(true).open(path.as_ref())?;
        let mut header_buf = [0u8; DEFAULT_BLOCK_SIZE];
        inner.seek_read_all(&mut header_buf, 0)?;
        let (magic, mut data) = header_buf.split_at(HEADER.len());
        if magic != HEADER {
            return Err(BitsErrorKind::InvalidData.into());
        }

        let mut stream_next_block = BTreeMap::<u8, u32>::new();
        let mut expected_stream_idx = 1;
        while let Some(read_idx) = data.next_u8()? {
            if read_idx == 0 {
                // 0'th idx indicates EOL
                break;
            }
            if read_idx != expected_stream_idx {
                return Err(BitsErrorKind::InvalidData.into());
            }
            let start_block = data.decode_vbyte()? as u32;
            stream_next_block.insert(read_idx, start_block);
            expected_stream_idx += 1;
        }
        let stream_ids = stream_next_block.keys().copied().collect::<Vec<_>>();
        let parent = Arc::new(MultiStreamReader {
            inner: Arc::new(Mutex::new(inner)),
            stream_next_block: Arc::new(Mutex::new(stream_next_block)),
        });
        let mut out = Vec::<StreamReader>::new();
        for k in stream_ids {
            out.push(StreamReader::new(parent.clone(), k));
        }

        Ok(out)
    }
    pub(crate) fn read_next_block(
        &self,
        stream_idx: u8,
        buf: &mut RoundU8Buffer<DATA_SIZE>,
    ) -> Result<(), Error> {
        let block_idx = {
            let Ok(lock) = self.stream_next_block.lock() else {
                return broken_pipe!();
            };
            let Some(v) = lock.get(&stream_idx) else {
                return Ok(());
            };
            *v
        };
        let next_idx = {
            let Ok(mut lock) = self.inner.lock() else {
                return broken_pipe!();
            };
            let offset = block_idx as u64 * DEFAULT_BLOCK_SIZE as u64;
            buf.as_ref_mut(|_, buf| {
                lock.seek_read_all(buf, offset)?;
                Ok(buf.len())
            })?;
            lock.read_be_u32()?
        };
        let Ok(mut lock) = self.stream_next_block.lock() else {
            return broken_pipe!();
        };
        lock.insert(stream_idx, next_idx);
        Ok(())
    }
}
pub struct StreamReader {
    parent: Arc<MultiStreamReader>,
    stream_idx: u8,
    buf: RoundU8Buffer<DATA_SIZE>,
    stream_counter: u64,
}
impl StreamReader {
    pub fn new(parent: Arc<MultiStreamReader>, stream_idx: u8) -> StreamReader {
        StreamReader {
            stream_idx,
            parent,
            buf: RoundU8Buffer::default(),
            stream_counter: 0,
        }
    }
    pub fn stream_position(&self) -> u64 {
        self.stream_counter
    }
}
impl Bits for StreamReader {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        if self.buf.is_empty() {
            self.parent
                .read_next_block(self.stream_idx, &mut self.buf)?;
            let lim = self.buf.read_be_u16()?;
            if lim > 0 {
                self.buf.limit(lim as usize)?;
            }
            if self.buf.is_empty() {
                return Ok(None);
            }
        }
        self.stream_counter += 1;
        Ok(self.buf.pop_front())
    }
}

#[cfg(all(test, feature = "std"))]
mod test {
    use crate::read::{MultiStreamReader, MultiStreamWriter, StreamReader, DATA_SIZE};
    use irox_bits::{Bits, Error, MutBits};
    use std::sync::Arc;
    use std::thread::JoinHandle;
    use std::time::Instant;

    const NUM_BLOCKS: usize = 100_000;

    fn spawn_writer_stream(ms: &Arc<MultiStreamWriter>, value: u8) -> JoinHandle<()> {
        let mut stream = ms.new_stream();
        std::thread::spawn(move || {
            let num_blocks = NUM_BLOCKS;
            let count = num_blocks * DATA_SIZE - 100;
            for _ in 0..count {
                stream.write_u8(value).unwrap();
            }
        })
    }

    #[test]
    #[ignore]
    pub fn test_write() -> Result<(), Error> {
        let ms = MultiStreamWriter::new("./test_multistream.ms")?;
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
        let len = NUM_BLOCKS as u64 * DATA_SIZE as u64 * 6 - 600;
        let bs = len as f64 / end.as_secs_f64();
        let mbs = bs / 1e6;
        let lmb = len as f64 / 1e6;
        println!("Wrote {lmb} MB in {end:?} = {mbs:02.02} MB/s");
        Ok(())
    }

    fn spawn_reader_stream(mut stream: StreamReader, value: u8) -> JoinHandle<()> {
        std::thread::spawn(move || {
            let num_blocks = NUM_BLOCKS;
            let count = num_blocks * DATA_SIZE - 100;
            for _ in 0..count {
                let len = stream.stream_position();
                assert_eq!(value, stream.read_u8().unwrap(), "at position {len}");
            }
        })
    }

    #[test]
    #[ignore]
    pub fn test_read() -> Result<(), Error> {
        let mut streams = MultiStreamReader::open("./test_multistream.ms")?;
        assert_eq!(streams.len(), 6);

        let start = Instant::now();

        let mut drain = streams.drain(..);
        let mut handles = vec![
            spawn_reader_stream(drain.next().unwrap(), 0xA),
            spawn_reader_stream(drain.next().unwrap(), 0x9),
            spawn_reader_stream(drain.next().unwrap(), 0xF),
            spawn_reader_stream(drain.next().unwrap(), 0x5),
            spawn_reader_stream(drain.next().unwrap(), 0x3),
            spawn_reader_stream(drain.next().unwrap(), 0x2),
        ];

        handles.drain(..).for_each(|h| h.join().unwrap());

        let end = start.elapsed();
        let len = NUM_BLOCKS as u64 * DATA_SIZE as u64 * 6 - 600;
        let bs = len as f64 / end.as_secs_f64();
        let mbs = bs / 1e6;
        let lmb = len as f64 / 1e6;
        println!("Read {lmb} MB in {end:?} = {mbs:02.02} MB/s");
        Ok(())
    }
}
