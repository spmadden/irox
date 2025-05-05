// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//! More complex synchronization primitives than in the STD.

pub use eventual::*;
pub use flags::*;
pub use optional::*;
use std::sync::MutexGuard;
mod eventual;
mod once;
pub use exchange::*;
mod exchange;
mod flags;
mod optional;

pub enum MaybeLocked<'a, T> {
    Borrowed(&'a T),
    MutBorrowed(&'a mut T),
    Locked(MutexGuard<'a, T>),
}

#[allow(clippy::should_implement_trait)]
impl<'a, T> MaybeLocked<'a, T> {
    pub fn deref(&'a self) -> &'a T {
        match self {
            MaybeLocked::Borrowed(t) => t,
            MaybeLocked::MutBorrowed(t) => t,
            MaybeLocked::Locked(t) => t,
        }
    }

    pub fn deref_mut(&'a mut self) -> Option<&'a mut T> {
        match self {
            MaybeLocked::MutBorrowed(t) => Some(t),
            MaybeLocked::Locked(t) => Some(t),
            _ => None,
        }
    }
    pub fn map<F: FnMut(&'a T) -> R, R>(&'a self, mut func: F) -> R {
        match self {
            MaybeLocked::Borrowed(t) => func(t),
            MaybeLocked::MutBorrowed(t) => func(t),
            MaybeLocked::Locked(t) => func(t),
        }
    }
}
