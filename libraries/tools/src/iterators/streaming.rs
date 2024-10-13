// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

///
/// Basic iterator trait that allows the borrowing of inner items with mapped lifetimes.
pub trait LendingIterator<'a> {
    type Item<'b>
    where
        Self: 'b,
        Self: 'a;

    fn next_ref(&'a mut self) -> Option<Self::Item<'a>>;
}
