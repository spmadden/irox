// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;
use crate::bits::Bits;
use crate::error::Error;
use crate::mutbits::MutBits;
use crate::BitsWrapper;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

macro_rules! impl_bits_pop {
    ($($ty:tt)+) => {
        impl Bits for $($ty)+ {
            fn next_u8(&mut self) -> Result<Option<u8>, Error> {
                if self.is_empty() {
                    return Ok(None)
                }
                Ok(Some(self.remove(0) as u8))
            }

            fn read_some_into<T: MutBits>(&mut self, into: &mut T) -> Result<usize, Error> {
                Ok(into.write_some_bytes(self.as_ref()))
            }
        }
    };
}

impl_bits_pop!(String);
impl_bits_pop!(&mut String);
impl_bits_pop!(Vec<u8>);
impl_bits_pop!(&mut Vec<u8>);
macro_rules! impl_bits_vecdeque {
    ($($ty:tt)+) => {
        impl Bits for $($ty)+ {
            fn next_u8(&mut self) -> Result<Option<u8>, Error> {
                Ok(self.pop_front())
            }

            fn read_some_into<T: MutBits>(&mut self, into: &mut T) -> Result<usize, Error> {
                let mut wrote = 0;
                while let Some(val) = self.pop_front() {
                    let Ok(()) = into.write_u8(val) else {
                        return Ok(wrote);
                    };
                    wrote += 1;
                }
                Ok(wrote)
            }
        }
    };
}
impl_bits_vecdeque!(VecDeque<u8>);
impl_bits_vecdeque!(&mut VecDeque<u8>);

macro_rules! impl_mutbits_vecdeque {
    ($($ty:tt)+) => {
        impl MutBits for $($ty)+ {
            fn write_u8(&mut self, val: u8) -> Result<(), Error> {
                self.push_back(val);
                Ok(())
            }
        }
    };
}
impl_mutbits_vecdeque!(&mut VecDeque<u8>);
impl_mutbits_vecdeque!(VecDeque<u8>);

macro_rules! impl_push {
    ($cast:ty, $($ty:tt)+) => {
        impl MutBits for $($ty)+ {
            fn write_u8(&mut self, val: u8) -> Result<(), Error> {
                self.push(val as $cast);
                Ok(())
            }
        }
    };
}
impl_push!(char, &mut String);
impl_push!(char, String);
impl_push!(u8, Vec<u8>);
impl_push!(u8, &mut Vec<u8>);

impl<T: Bits> Bits for Box<T> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        T::next_u8(self)
    }
}
impl<T: MutBits> MutBits for Box<T> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        T::write_u8(self, val)
    }
}

///
/// A struct to count the number of bytes moving through it.
pub struct SharedCountingBits<'a, B> {
    inner: BitsWrapper<'a, B>,
    count: Arc<AtomicU64>,
}

impl<'a, B> SharedCountingBits<'a, B> {
    pub fn new(inner: BitsWrapper<'a, B>) -> Self {
        Self {
            inner,
            count: Arc::new(AtomicU64::new(0)),
        }
    }
    /// Get a reference to the counter
    pub fn get_count(&self) -> SharedROCounter {
        SharedROCounter::new(self.count.clone())
    }
}
impl<'a, B: Bits> Bits for SharedCountingBits<'a, B> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        let res = self.inner.next_u8();
        if let Ok(Some(_)) = &res {
            self.count.fetch_add(1, Ordering::Relaxed);
        }
        res
    }
}
impl<'a, B: MutBits> MutBits for SharedCountingBits<'a, B> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.inner.write_u8(val)?;
        self.count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

///
/// A Read-Only counter that can be shared between threads.  The owner of the underlying counter
/// is free to update the value, but users of this object alone may not.
#[derive(Debug, Clone)]
pub struct SharedROCounter {
    counter: Arc<AtomicU64>,
}

impl SharedROCounter {
    pub fn new(counter: Arc<AtomicU64>) -> Self {
        SharedROCounter { counter }
    }

    ///
    /// Returns the current value of the counter
    pub fn get_count(&self) -> u64 {
        self.counter.load(Ordering::Relaxed)
    }
}
