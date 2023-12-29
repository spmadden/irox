// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

extern crate alloc;
use alloc::collections::VecDeque;
use std::io::Read;

/// Always returns zero
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReadEmpty;
impl Read for ReadEmpty {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}

#[derive(Default, Clone)]
pub struct ReadAny {
    deque: VecDeque<u8>,
}

impl Read for ReadAny {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.deque.read(buf)
    }
}

impl From<String> for ReadAny {
    fn from(value: String) -> Self {
        ReadAny {
            deque: VecDeque::from(Vec::from(value.as_str())),
        }
    }
}
impl From<&String> for ReadAny {
    fn from(value: &String) -> Self {
        ReadAny {
            deque: VecDeque::from(Vec::from(value.as_str())),
        }
    }
}
impl From<Vec<u8>> for ReadAny {
    fn from(value: Vec<u8>) -> Self {
        ReadAny {
            deque: VecDeque::from(value),
        }
    }
}
impl From<&Vec<u8>> for ReadAny {
    fn from(value: &Vec<u8>) -> Self {
        ReadAny {
            deque: VecDeque::from(value.clone()),
        }
    }
}
