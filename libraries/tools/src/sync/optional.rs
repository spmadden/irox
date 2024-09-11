// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//! Contains the [`SynchronizedOptional`], [`SharedCell`] and other associated primitives

use alloc::sync::Arc;
use core::fmt::{Debug, Formatter};
use core::ops::Deref;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

///
/// Basically a [`RwLock<Option<Arc<T>>>`] - the benefits here being:
/// 1. This structure is [`Sync`] in that it can be shared between threads
/// 2. This structure can be lazily initialized, and then reset back to [`None`], and back again
/// 3. This structure provides multiple access to a single shared instance, like a [`String`] so we're
///    not cloning it a bunch of times unnecessarily.  ([`Arc<T>`])
///
/// The [`Arc<T>`] bit is because you can't return a `&T` out of a [`RwLock<T>`] the way you can
/// from a [`std::sync::OnceLock<T>`].  The only reason this isn't a OnceLock is because I wanted
/// a `swap` method that was atomic, rather than relying on a [`std::sync::OnceLock<T>::take`]
/// followed by a [`std::sync::OnceLock<T>::set`], which allows a very slight race condition.
pub struct SynchronizedOptional<T> {
    inner: RwLock<Option<Arc<T>>>,
}

impl<T> Default for SynchronizedOptional<T> {
    fn default() -> Self {
        SynchronizedOptional::empty()
    }
}

impl<T> SynchronizedOptional<T> {
    /// Returns a new uninitialized/empty struct
    #[must_use]
    pub fn empty() -> Self {
        Self {
            inner: RwLock::new(None),
        }
    }

    /// Returns a new struct initialized with the provided value
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            inner: RwLock::new(Some(Arc::new(value))),
        }
    }

    /// Returns a new struct initialized with the provided already shared value
    #[must_use]
    pub fn new_shared(value: Arc<T>) -> Self {
        Self {
            inner: RwLock::new(Some(value)),
        }
    }

    /// If this struct has been initialized, returns a copy of the data, otherwise None
    #[must_use]
    pub fn get(&self) -> Option<Arc<T>> {
        if let Ok(read) = self.inner.read() {
            return read.clone();
        }
        None
    }

    /// Sets the value to be the specified value, throwing away any value that was stored previously
    /// Returns the value provided as a parameter if it was unable to replace the value.
    pub fn set(&self, value: Option<T>) -> Result<(), Option<T>> {
        if let Ok(mut write) = self.inner.write() {
            *write = value.map(Arc::new);
            return Ok(());
        }
        Err(value)
    }

    /// Sets the value to be the specified value, throwing away any value that was stored previously
    /// Returns the value provided as a parameter if it was unable to replace the value.
    pub fn set_shared(&self, value: Arc<T>) -> Result<(), Arc<T>> {
        if let Ok(mut write) = self.inner.write() {
            *write = Some(value);
            return Ok(());
        }

        Err(value)
    }

    /// Takes the value out of this structure, leaving `None` in it's place.
    pub fn take(&self) -> Option<Arc<T>> {
        if let Ok(mut write) = self.inner.write() {
            let out = write.clone();
            *write = None;
            return out;
        }
        None
    }

    /// Swaps the value contained within this structure (if any) with the value provided.  Upon
    /// success, returns the old value (which is possibly [`None`]).
    /// Will only fail if the lock has been poisoned, at which point it returns the provided
    /// value back to you.
    pub fn swap(&self, value: T) -> Result<Option<Arc<T>>, T> {
        if let Ok(mut write) = self.inner.write() {
            let inner = write.clone();
            *write = Some(Arc::new(value));
            return Ok(inner);
        }
        Err(value)
    }

    /// Swaps the value contained within this structure (if any) with the already shared value
    /// provided.  Upon success, returns the old value (which is possibly [`None`]).
    /// Will only fail if the lock has been poisoned, at which point it returns the provided
    /// value back to you.
    pub fn swap_shared(&self, value: Arc<T>) -> Result<Option<Arc<T>>, Arc<T>> {
        if let Ok(mut write) = self.inner.write() {
            let inner = write.clone();
            *write = Some(value);
            return Ok(inner);
        }
        Err(value)
    }
}

impl<T> Debug for SynchronizedOptional<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get())
    }
}

///
/// Atomic shared version of `OnceLock` - backed by an `Arc<RwLock<Option<T>>>`.
///
/// ```
/// # use irox_tools::sync::SharedCell;
/// let value = SharedCell::<bool>::empty(); // or default()
/// assert_eq!(None, value.take());
///
/// value.set(true);
/// assert_eq!(Some(true), value.take());
/// assert_eq!(None, value.take());
/// ```
#[derive(Debug)]
pub struct SharedCell<T> {
    inner: Arc<RwLock<Option<T>>>,
}
impl<T> Clone for SharedCell<T> {
    fn clone(&self) -> Self {
        SharedCell {
            inner: self.inner.clone(),
        }
    }
}
impl<T> Default for SharedCell<T> {
    fn default() -> Self {
        SharedCell {
            inner: Arc::new(RwLock::new(None)),
        }
    }
}
impl<T> From<Option<T>> for SharedCell<T> {
    fn from(value: Option<T>) -> Self {
        SharedCell {
            inner: Arc::new(RwLock::new(value)),
        }
    }
}
impl<T> SharedCell<T> {
    /// Creates a new, unset cell.
    pub fn empty() -> Self {
        None.into()
    }
    /// Create a new, set cell with the provided value.
    pub fn new(val: T) -> Self {
        Some(val).into()
    }
    /// Sets the value, throwing away any old value
    pub fn set(&self, val: T) {
        if let Ok(mut lock) = self.inner.write() {
            *lock = Some(val)
        }
    }
    /// Takes the value, if previously set.
    pub fn take(&self) -> Option<T> {
        if let Ok(mut lock) = self.inner.write() {
            return lock.take();
        }
        None
    }
    /// Peeks at the value, does not consume it.
    pub fn peek<V: Fn(Option<&T>)>(&self, func: V) {
        if let Ok(lock) = self.inner.read() {
            func(lock.as_ref())
        }
    }

    ///
    /// Returns a shared (locked) reference to the inner object
    pub fn as_ref(&self) -> ReadGuard<T> {
        if let Ok(lock) = self.inner.read() {
            return Some(lock).into();
        }
        None.into()
    }

    /// Mutably peeks at the value, allows for edits.
    pub fn peek_mut<V: FnMut(Option<&mut T>)>(&self, mut func: V) {
        if let Ok(mut lock) = self.inner.write() {
            func(lock.as_mut())
        }
    }
    ///
    /// Returns a mutable (locked) reference to the inner object.
    pub fn as_mut(&self) -> WriteGuard<T> {
        if let Ok(lock) = self.inner.write() {
            return Some(lock).into();
        }
        None.into()
    }
}
/// Type-safe wrapper around a [`RwLockReadGuard`]
#[derive(Default)]
pub struct ReadGuard<'a, T> {
    lock: Option<RwLockReadGuard<'a, Option<T>>>,
}
impl<'a, T> From<Option<RwLockReadGuard<'a, Option<T>>>> for ReadGuard<'a, T> {
    fn from(value: Option<RwLockReadGuard<'a, Option<T>>>) -> Self {
        ReadGuard { lock: value }
    }
}
impl<'a, T> Deref for ReadGuard<'a, T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        if let Some(lock) = &self.lock {
            return lock.deref();
        }
        &None
    }
}

/// Type-safe wrapper around a [`RwLockWriteGuard`]
#[derive(Default)]
pub struct WriteGuard<'a, T> {
    lock: Option<RwLockWriteGuard<'a, Option<T>>>,
}
impl<'a, T> Deref for WriteGuard<'a, T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        if let Some(lock) = &self.lock {
            return lock.deref();
        }
        &None
    }
}
impl<'a, T> From<Option<RwLockWriteGuard<'a, Option<T>>>> for WriteGuard<'a, T> {
    fn from(value: Option<RwLockWriteGuard<'a, Option<T>>>) -> Self {
        WriteGuard { lock: value }
    }
}
