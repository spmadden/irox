// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use alloc::collections::VecDeque;
use irox_bits::{Bits, Error};

pub struct IDATStream {
    current_chunk: Vec<u8>,
    current_chunk_idx: usize,
    remaining_chunks: VecDeque<Vec<u8>>,
}

impl IDATStream {
    pub fn new(chunks: VecDeque<Vec<u8>>) -> Self {
        Self {
            current_chunk: Vec::new(),
            current_chunk_idx: 0,
            remaining_chunks: chunks,
        }
    }
}

impl Bits for IDATStream {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        if self.current_chunk.is_empty() || self.current_chunk_idx >= self.current_chunk.len() {
            if let Some(next) = self.remaining_chunks.pop_front() {
                self.current_chunk = next;
                self.current_chunk_idx = 0;
            } else {
                return Ok(None);
            }
        }
        let out = self.current_chunk.get(self.current_chunk_idx).copied();
        self.current_chunk_idx += 1;
        Ok(out)
    }
}
