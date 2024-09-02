// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Stack Buffers

pub use round::*;

mod round;
pub use fixed::*;
mod fixed;
mod str;
cfg_feature_alloc! {
    pub use pyramids::*;
    mod pyramids;
}

pub use str::*;
use crate::cfg_feature_alloc;

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
}
