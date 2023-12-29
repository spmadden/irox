// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter, UpperHex, Write};

///
/// This struct purely exists to implement 'Display' for a borrowed vec, whose elements implement [`Display`]
pub struct PrettyVec<'a, T>(pub &'a Vec<T>);

pub struct PrettyVecDeque<'a, T>(pub &'a VecDeque<T>);

impl<'a, T> Display for PrettyVec<'a, T>
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

impl<'a, T> UpperHex for PrettyVec<'a, T>
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

impl<'a, T> UpperHex for PrettyVecDeque<'a, T>
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
