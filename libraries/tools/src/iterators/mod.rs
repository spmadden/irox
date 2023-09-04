// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use self::looping_forever::LoopingForever;

pub mod looping_forever;

pub trait Itertools: Iterator {
    ///
    /// Returns an iterator that never ends, sequentially looping over all items in this
    /// iterator forever, unless there are no items in the iterator.
    fn looping_forever(self) -> LoopingForever<Self::Item>
    where
        Self: Sized + Iterator,
        Self::Item: Clone,
    {
        looping_forever::LoopingForever {
            index: 0,
            items: self.collect(),
        }
    }

    ///
    /// Collects a specific amount of items from the iterator.  The underlying iterator may return
    /// more, or less items than the specified count.  Any remaining uncollected items will be
    /// discarded.  If count is greater than the length of the underlying iterator, then the
    /// remainder of the returned vector will be None.
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
    /// Same behavior as ['collect_exact'], except rather than returning 'None' after the end, it
    /// returns the specified value
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
    /// Same behavior as ['collect_exact'], except rather than returning 'None' after the end, it
    /// returns the default value
    fn collect_exact_or_default(mut self, count: usize) -> Vec<Self::Item>
    where
        Self: Sized + Iterator,
        Self::Item: Clone + Default,
    {
        let mut out: Vec<Self::Item> = Vec::with_capacity(count);
        for _i in 0..count {
            out.push(self.next().unwrap_or(Default::default()))
        }
        out
    }
}

impl<T: ?Sized> Itertools for T where T: Iterator {}
