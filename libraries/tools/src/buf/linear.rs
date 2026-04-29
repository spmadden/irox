// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Linear2DArray<T> {
    width: usize,
    height: usize,
    size: usize,
    data: Box<[T]>,
}
impl<T> Linear2DArray<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut data = Vec::<T>::with_capacity(size);
        data.resize(size, Default::default());
        Self {
            width,
            height,
            size,
            data: data.into_boxed_slice(),
        }
    }
    pub fn new_initialized_with<F: Fn(usize, usize) -> T>(
        width: usize,
        height: usize,
        initializer: F,
    ) -> Self {
        let size = width * height;
        let mut data = Vec::<T>::with_capacity(size);
        for y in 0..height {
            for x in 0..width {
                let item = initializer(x, y);
                data.push(item);
            }
        }
        Self {
            width,
            height,
            size,
            data: data.into_boxed_slice(),
        }
    }

    const fn index_of(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, n: usize, m: usize) -> Option<&T> {
        let idx = self.index_of(n, m);
        if idx >= self.size {
            return None;
        }
        self.data.get(idx)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let idx = self.index_of(x, y);
        if idx >= self.size {
            return None;
        }
        self.data.get_mut(idx)
    }
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<T> {
        let idx = self.index_of(x, y);
        if let Some(v) = self.data.get_mut(idx) {
            Some(core::mem::replace(v, value))
        } else {
            None
        }
    }
}
