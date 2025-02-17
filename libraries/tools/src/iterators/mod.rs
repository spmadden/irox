// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Iterators adds the [`Itertools`] Trait, which adds a number of additional
//! helper methods to the [`Iterator`] Trait.
//!

extern crate alloc;
use self::join::Joining;
use self::looping_forever::LoopingForever;

use crate::iterators::join::MultiJoining;
use alloc::vec::Vec;

mod join;
pub mod looping_forever;
mod streaming;
mod zip;

pub use streaming::*;
pub use zip::*;

///
/// Itertools adds helpful additional methods to [`Iterator`]
pub trait Itertools: Iterator {
    ///
    /// Returns an iterator that never ends, sequentially looping over all items in this
    /// iterator forever, unless there are no items in the iterator.
    fn looping_forever(self) -> LoopingForever<Self::Item>
    where
        Self: Sized + Iterator,
        Self::Item: Clone,
    {
        LoopingForever {
            index: 0,
            items: self.collect(),
        }
    }

    ///
    /// Collects a specific amount of items from the iterator.  The underlying iterator may return
    /// more, or less items than the specified count.  Any remaining uncollected items will be
    /// discarded.  If count is greater than the length of the underlying iterator, then the
    /// remainder of the returned vector will be None.
    #[must_use]
    fn collect_exact(mut self, count: usize) -> Vec<Option<Self::Item>>
    where
        Self: Sized + Iterator,
        Self::Item: Clone,
    {
        let mut out: Vec<Option<Self::Item>> = Vec::with_capacity(count);
        for _i in 0..count {
            out.push(self.next())
        }
        out
    }

    ///
    /// Same behavior as [`Itertools::collect_exact`], except rather than returning 'None' after the end, it
    /// returns the specified value
    #[must_use]
    fn collect_exact_or(mut self, count: usize, def: Self::Item) -> Vec<Self::Item>
    where
        Self: Sized + Iterator,
        Self::Item: Clone,
    {
        let mut out: Vec<Self::Item> = Vec::with_capacity(count);
        for _i in 0..count {
            out.push(self.next().unwrap_or(def.clone()))
        }
        out
    }

    ///
    /// Same behavior as [`Itertools::collect_exact`], except rather than returning 'None' after the end, it
    /// returns the default value
    #[must_use]
    fn collect_exact_or_default(mut self, count: usize) -> Vec<Self::Item>
    where
        Self: Sized + Iterator,
        Self::Item: Clone + Default,
    {
        let mut out: Vec<Self::Item> = Vec::with_capacity(count);
        for _i in 0..count {
            out.push(self.next().unwrap_or_default());
        }
        out
    }

    ///
    /// Collects and returns upto 'count' items from this iterator.  If this
    /// iterator is exhausted, the returned vector is empty.  Note, the returned
    /// vector may have *up to* the requested number of items if the iterator
    /// exhausts before filling the chunk.
    #[must_use]
    fn collect_next_chunk(&mut self, count: usize) -> Vec<Self::Item> {
        let mut out: Vec<Self::Item> = Vec::with_capacity(count);
        for _i in 0..count {
            match self.next() {
                Some(e) => out.push(e),
                None => break,
            }
        }
        out
    }

    ///
    /// Returns the elements in this iterator interspersed with the joining delimiter.
    /// For example, if this iterator contains `[A, B, C, D]` and the delimiter is `z`, then the
    /// final iteration sequence will be `[A, z, B, z, C, z, D]`
    #[must_use]
    fn joining(self, delim: Self::Item) -> Joining<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Joining::new(self, delim)
    }

    ///
    /// Returns the elements in this iterator interspersed with the elements in the joining delimeter.
    /// For example, if this iterator contains `[A, B, C, D]` and the delimiters are `[1, 2]`, then
    /// the final iteration sequence will be `[A, 1, 2, B, 1, 2, C, 1, 2, D]`
    #[must_use]
    fn joining_multi(self, delim: &[Self::Item]) -> MultiJoining<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        MultiJoining::new(self, delim)
    }
}

impl<T: ?Sized> Itertools for T where T: Iterator {}

pub struct Moderator {
    start: usize,
    modulus: usize,
    max_count: usize,
    idx: usize,
}
impl Moderator {
    pub fn new(start: usize, modulus: usize) -> Self {
        Self {
            start,
            modulus,
            max_count: modulus,
            idx: 0,
        }
    }
    pub fn new_limited(start: usize, modulus: usize, max_count: usize) -> Self {
        Self {
            start,
            modulus,
            max_count,
            idx: 0,
        }
    }
}
impl Iterator for Moderator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.max_count {
            return None;
        }
        let out = (self.start + self.idx) % self.modulus;
        self.idx += 1;
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::iterators::Moderator;

    #[test]
    pub fn test_moderator() {
        let exp = &[0, 1, 2, 3, 4, 5];
        let mut iter = Moderator::new(0, 6);
        for e in exp {
            assert_eq!(iter.next(), Some(*e));
        }
        assert_eq!(iter.next(), None);
    }
    #[test]
    pub fn test_moderator2() {
        let exp = &[3, 4, 5, 0, 1, 2];
        let mut iter = Moderator::new(3, 6);
        for e in exp {
            assert_eq!(iter.next(), Some(*e));
        }
        assert_eq!(iter.next(), None);
    }
}
