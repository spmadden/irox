// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

extern crate alloc;
use crate::buf::{Buffer, FixedBuf};
use alloc::collections::LinkedList;

const PAGE_SIZE: usize = 0x1000; // 4096
const PAGE_SHIFT: usize = 12;
const DATA_MASK: usize = 0x0FFF;

#[derive(Default)]
pub struct UnlimitedBuffer<T> {
    #[allow(clippy::linkedlist)]
    data: LinkedList<FixedBuf<PAGE_SIZE, T>>,
    len: u64,
}

impl<T> UnlimitedBuffer<T> {
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

    pub fn push_front(&mut self, value: T) {
        let mut buf = FixedBuf::new();
        let _ = buf.push_back(value);
        self.data.push_front(buf);
    }

    pub fn push_back(&mut self, value: T) {
        if let Some(back) = self.data.back_mut() {
            if let Err(e) = back.push_back(value) {
                let mut buf = FixedBuf::new();
                let _ = buf.push_back(e);
                self.data.push_back(buf);
            }
        }
    }
}
