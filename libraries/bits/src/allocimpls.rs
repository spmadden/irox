// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

extern crate alloc;
use crate::bits::Bits;
use crate::error::Error;
use crate::mutbits::MutBits;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

macro_rules! impl_bits_pop {
    ($($ty:tt)+) => {
        impl Bits for $($ty)+ {
            fn next_u8(&mut self) -> Result<Option<u8>, Error> {
                Ok(self.pop().map(|v| v as u8))
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
impl_mutbits_vecdeque!(&mut alloc::collections::VecDeque<u8>);
impl_mutbits_vecdeque!(alloc::collections::VecDeque<u8>);

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
