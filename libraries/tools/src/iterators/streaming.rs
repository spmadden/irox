// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

///
/// Basic iterator trait that allows the borrowing of inner items with mapped lifetimes.
pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next_ref<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
