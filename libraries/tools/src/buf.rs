// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Stack Buffers

pub use round::*;

mod round;
pub use fixed::*;

mod fixed;
pub use fixed_u8::*;
mod fixed_u8;
pub use round_u8::*;
mod array;
mod round_u8;
pub use array::*;

use crate::cfg_feature_alloc;
cfg_feature_alloc! {
    extern crate alloc;

    pub use unlimited::*;
    mod unlimited;
}

///
/// Standard buffer functions
pub trait Buffer<T> {
    fn get(&self, index: usize) -> Option<&T>;
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    fn capacity(&self) -> usize;
    fn len(&self) -> usize;
    fn clear(&mut self);

    fn front(&self) -> Option<&T>;
    fn front_mut(&mut self) -> Option<&mut T>;
    fn back(&self) -> Option<&T>;
    fn back_mut(&mut self) -> Option<&mut T>;
    fn pop_front(&mut self) -> Option<T>;
    fn pop_back(&mut self) -> Option<T>;
    fn push_front(&mut self, value: T) -> Result<(), T>;
    fn push_back(&mut self, value: T) -> Result<(), T>;
    fn push(&mut self, value: T) -> Result<(), T> {
        self.push_back(value)
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn is_full(&self) -> bool {
        self.capacity() == self.len()
    }

    cfg_feature_alloc! {
        fn into_boxed_slice(mut self) -> alloc::boxed::Box<[T]> where Self: Sized {
            let mut vec = alloc::vec::Vec::<T>::with_capacity(self.len());
            while let Some(elem) = self.pop_front() {
                vec.push(elem);
            }
            vec.into_boxed_slice()
        }
    }
}

///
/// Trait used for things like [`Vec`] and [`VecDeq`] to pre-allocate and fill
/// with zeros.
pub trait ZeroedBuffer {
    type Output;
    fn new_zeroed(capacity: usize) -> Self::Output;
}
