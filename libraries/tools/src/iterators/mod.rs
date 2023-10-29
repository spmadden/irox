// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Iterators adds the [`Itertools`] Trait, which adds a number of additional
//! helper methods to the [`Iterator`] Trait.
//!

use self::looping_forever::LoopingForever;

pub mod looping_forever;

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
}

impl<T: ?Sized> Itertools for T where T: Iterator {}
