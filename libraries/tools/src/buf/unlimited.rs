// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;
use crate::buf::{Buffer, RoundBuffer};
use alloc::collections::LinkedList;
use irox_bits::Bits;
use irox_bits::{Error, MutBits};

const PAGE_SIZE: usize = 0x1000; // 4096
const PAGE_SHIFT: usize = 12;
const DATA_MASK: usize = 0x0FFF;

///
/// Zero re-allocation linked-page buffer.  Faster than a Vec for large datasets
/// because it does not require pre-allocation of memory to achieve zero-reallocation.
/// However, wastes memory for very small data sets since the minimum size quanta
/// that allocates is 4k * sizeof(T).  IE, a `u8` page will be 4k wide, but a
/// `u128` page will be 64k wide and a `Box<T>` page will either be 32k (standard
/// types) or 64k wide (`dyn` types)
#[derive(Default, Clone)]
pub struct UnlimitedBuffer<T: Clone> {
    #[allow(clippy::linkedlist)]
    data: LinkedList<RoundBuffer<PAGE_SIZE, T>>,
    len: u64,
}

impl<T: Clone> UnlimitedBuffer<T> {
    pub fn new() -> UnlimitedBuffer<T> {
        UnlimitedBuffer {
            data: LinkedList::new(),
            len: 0,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let offset = index >> PAGE_SHIFT;
        if let Some(block) = self.data.iter().nth(offset) {
            let idx = index & DATA_MASK;
            return block.get(idx);
        }
        None
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let offset = index >> PAGE_SHIFT;
        if let Some(block) = self.data.iter_mut().nth(offset) {
            let idx = index & DATA_MASK;
            return block.get_mut(idx);
        }
        None
    }

    #[allow(clippy::unused_self)]
    pub fn capacity(&self) -> usize {
        usize::MAX
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.len = 0;
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front().and_then(|v| v.front())
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut().and_then(|v| v.front_mut())
    }

    pub fn back(&self) -> Option<&T> {
        self.data.back().and_then(|v| v.back())
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.data.back_mut().and_then(|v| v.back_mut())
    }

    ///
    /// Retrieves and returns the front value
    pub fn pop_front(&mut self) -> Option<T> {
        while let Some(front) = self.data.front_mut() {
            if front.is_empty() {
                let _ = self.data.pop_front();
                continue;
            }
            return if let Some(fr) = front.pop_front() {
                self.len -= 1;
                Some(fr)
            } else {
                None
            };
        }
        None
    }

    ///
    /// Retrieves and returns the last value
    pub fn pop_back(&mut self) -> Option<T> {
        while let Some(back) = self.data.back_mut() {
            if back.is_empty() {
                let _ = self.data.pop_back();
                continue;
            }
            return if let Some(back) = back.pop_back() {
                self.len -= 1;
                Some(back)
            } else {
                None
            };
        }
        None
    }

    ///
    /// Appends all elements of the slice to the back.
    pub fn append_slice(&mut self, slice: &[T]) {
        for v in slice {
            self.push_back(v.clone());
        }
    }

    ///
    /// Push a value to the front
    pub fn push_front(&mut self, value: T) {
        let mut buf = RoundBuffer::default();
        let _ = buf.push_back(value);
        self.data.push_front(buf);
        self.len += 1;
    }

    ///
    /// Push a value to the end
    pub fn push_back(&mut self, value: T) {
        if let Some(back) = self.data.back_mut() {
            if let Err(e) = back.push_back(value) {
                let mut buf = RoundBuffer::new();
                let _ = buf.push_back(e);
                self.data.push_back(buf);
            }
        } else {
            let mut buf = RoundBuffer::new();
            let _ = buf.push_back(value);
            self.data.push_back(buf);
        }
        self.len += 1;
    }

    ///
    /// Iterate over each block of data stored
    pub fn iter_blocks<F: FnMut(&[Option<T>])>(&mut self, mut f: F) {
        self.data.iter().for_each(|v| {
            v.raw_buf(&mut f)
        })
    }
}

impl MutBits for UnlimitedBuffer<u8> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.push_back(val);
        Ok(())
    }
}

impl Bits for UnlimitedBuffer<u8> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        Ok(self.pop_front())
    }
}

impl<T: Clone> Iterator for UnlimitedBuffer<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}
