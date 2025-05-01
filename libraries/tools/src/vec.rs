// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Bolt-ons for [`Vec`] and [`VecDeque`] - better displays and iteration tools

extern crate alloc;
use crate::buf::ZeroedBuffer;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::{vec, vec::Vec};
use core::fmt::{Display, Formatter, UpperHex, Write};

///
/// This struct purely exists to implement [`Display`] and [`UpperHex`] for a borrowed Vec, whose elements implement [`Display`] or [`UpperHex`]
pub struct PrettyVec<'a, T>(pub &'a Vec<T>);

///
/// This struct purely exists to implement [`Display`] and [`UpperHex`] for a borrowed VecDeque, whose elements implement [`Display`] or [`UpperHex`]
pub struct PrettyVecDeque<'a, T>(pub &'a VecDeque<T>);

impl<T> Display for PrettyVec<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let vals: Vec<String> = self.0.iter().map(|f| format!("{f}")).collect();
        if f.alternate() {
            f.write_fmt(format_args!("{vals:#?}"))
        } else {
            f.write_fmt(format_args!("{vals:?}"))
        }
    }
}

impl<T> UpperHex for PrettyVec<'_, T>
where
    T: UpperHex,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut buf = String::new();
        for val in self.0 {
            buf.write_fmt(format_args!("{val:0X}"))?;
        }
        f.write_fmt(format_args!("{buf}"))
    }
}

impl<T> UpperHex for PrettyVecDeque<'_, T>
where
    T: UpperHex,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut buf = String::new();
        for val in self.0 {
            buf.write_fmt(format_args!("{val:0X}"))?;
        }
        f.write_fmt(format_args!("{buf}"))
    }
}

///
/// The `retain+take` operation is exactly what it sounds like.  Retain only those elements that
/// match the predicate, but return whole owned copies of the items.  For efficiency's sake, some
/// implementations *may* iterate backwards through the container.
pub trait RetainTake<T> {
    fn retain_take<F>(&mut self, predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool;
}

impl<T> RetainTake<T> for Vec<T> {
    fn retain_take<F>(&mut self, mut predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        let mut idx = self.len();
        let mut out: Vec<T> = Vec::new();
        loop {
            idx -= 1;
            if let Some(elem) = self.get(idx) {
                if !predicate(elem) {
                    out.push(self.swap_remove(idx));
                }
            }
            if idx == 0 {
                break;
            }
        }
        out
    }
}

impl<T> RetainTake<T> for VecDeque<T> {
    fn retain_take<F>(&mut self, mut predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        let mut idx = self.len();
        let mut out: Vec<T> = Vec::new();
        loop {
            idx -= 1;
            if let Some(elem) = self.get(idx) {
                if !predicate(elem) {
                    if let Some(elem) = self.swap_remove_back(idx) {
                        out.push(elem);
                    }
                }
            }
            if idx == 0 {
                break;
            }
        }
        out
    }
}

impl<T: Default + Sized + Copy> ZeroedBuffer for Vec<T> {
    type Output = Self;

    fn new_zeroed(capacity: usize) -> Self::Output {
        vec![T::default(); capacity]
    }
}

impl<T: Default + Sized + Copy> ZeroedBuffer for VecDeque<T> {
    type Output = Self;

    fn new_zeroed(capacity: usize) -> Self::Output {
        VecDeque::from(vec![T::default(); capacity])
    }
}
impl<T: Default + Sized + Copy> ZeroedBuffer for Box<[T]> {
    type Output = Self;

    fn new_zeroed(capacity: usize) -> Self::Output {
        let mut vec = Vec::new();
        vec.resize_with(capacity, Default::default);
        vec.into_boxed_slice()
    }
}
