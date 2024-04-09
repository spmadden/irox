// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::Buffer;

pub struct StrBuf<const N: usize> {
    chars: [char; N],
    len: usize,
}
impl<const N: usize> Default for StrBuf<N> {
    fn default() -> Self {
        Self {
            chars: [0 as char; N],
            len: 0,
        }
    }
}

impl<const N: usize> core::fmt::Write for StrBuf<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            self.write_char(ch)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        if self.len == N {
            return Err(core::fmt::Error);
        }
        self.chars[self.len] = c;
        self.len += 1;
        Ok(())
    }
}

impl<const N: usize> Buffer<char> for StrBuf<N> {
    fn get(&self, index: usize) -> Option<&char> {
        if index >= N || index >= self.len {
            return None;
        }
        self.chars.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut char> {
        if index >= N || index >= self.len {
            return None;
        }
        self.chars.get_mut(index)
    }

    fn capacity(&self) -> usize {
        N
    }

    fn len(&self) -> usize {
        self.len
    }

    fn clear(&mut self) {
        self.len = 0
    }

    fn front(&self) -> Option<&char> {
        self.get(0)
    }

    fn front_mut(&mut self) -> Option<&mut char> {
        self.get_mut(0)
    }

    fn back(&self) -> Option<&char> {
        if N == 0 || self.len == 0 {
            return None;
        }
        self.get(self.len - 1)
    }

    fn back_mut(&mut self) -> Option<&mut char> {
        if N == 0 || self.len == 0 {
            return None;
        }
        self.get_mut(self.len - 1)
    }

    fn pop_front(&mut self) -> Option<char> {
        if N == 0 || self.len == 0 {
            return None;
        }
        let out = self.chars[0];
        for idx in 1..self.len {
            self.chars[idx - 1] = self.chars[idx];
        }
        self.len -= 1;
        Some(out)
    }

    fn pop_back(&mut self) -> Option<char> {
        if N == 0 || self.len == 0 {
            return None;
        }
        let idx = self.len - 1;
        self.len -= 1;
        Some(self.chars[idx])
    }

    fn push_front(&mut self, value: char) -> Result<(), char> {
        if N == 0 || self.len == N {
            return Err(value);
        }
        for idx in 0..self.len {
            self.chars[idx + 1] = self.chars[idx];
        }
        self.chars[0] = value;
        self.len += 1;
        Ok(())
    }

    fn push_back(&mut self, value: char) -> Result<(), char> {
        if N == 0 || self.len == N {
            return Err(value);
        }
        self.chars[self.len] = value;
        self.len += 1;
        Ok(())
    }
}
