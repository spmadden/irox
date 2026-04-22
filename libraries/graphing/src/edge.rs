// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;
use crate::{EdgeDescriptor, SharedNodeIdentifier};
use alloc::sync::Arc;
use core::fmt::{Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use irox_tools::identifier::SharedIdentifier;
use std::sync::RwLock;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SharedEdgeIdentifier(SharedIdentifier);
impl Deref for SharedEdgeIdentifier {
    type Target = SharedIdentifier;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<SharedIdentifier> for SharedEdgeIdentifier {
    fn from(value: SharedIdentifier) -> Self {
        Self(value)
    }
}
impl Display for SharedEdgeIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self.0.deref(), f)
    }
}

#[derive(Debug, Clone)]
pub enum Edge {
    Directed {
        descriptor: EdgeDescriptor,
        from: SharedNodeIdentifier,
        to: SharedNodeIdentifier,
    },
    Undirected {
        descriptor: EdgeDescriptor,
        left: SharedNodeIdentifier,
        right: SharedNodeIdentifier,
    },
}

impl Edge {
    pub fn id(&self) -> SharedEdgeIdentifier {
        self.descriptor().id.clone().into()
    }
    pub fn descriptor(&self) -> &EdgeDescriptor {
        match self {
            Edge::Directed { descriptor, .. } | Edge::Undirected { descriptor, .. } => descriptor,
        }
    }
    pub fn get_sides(&self) -> (&SharedNodeIdentifier, &SharedNodeIdentifier) {
        match self {
            Edge::Directed { from, to, .. } => (from, to),
            Edge::Undirected { left, right, .. } => (left, right),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SharedEdge {
    pub inner: Arc<RwLock<Edge>>,
}
impl From<Edge> for SharedEdge {
    fn from(value: Edge) -> Self {
        Self {
            inner: Arc::new(RwLock::new(value)),
        }
    }
}
impl SharedEdge {
    pub fn id<F: FnMut(SharedEdgeIdentifier)>(&self, mut f: F) {
        if let Ok(lock) = self.inner.read() {
            f(lock.id())
        }
    }
    pub fn get_mut<F: FnMut(&mut Edge)>(&self, mut f: F) {
        if let Ok(mut lock) = self.inner.write() {
            f(&mut lock);
        }
    }
    pub fn get<F: FnMut(&Edge)>(&self, mut f: F) {
        if let Ok(lock) = self.inner.read() {
            f(&lock);
        }
    }
    pub fn get_sides(&self) -> Option<(SharedNodeIdentifier, SharedNodeIdentifier)> {
        if let Ok(lock) = self.inner.read() {
            let (a, b) = lock.get_sides();
            Some((a.clone(), b.clone()))
        } else {
            None
        }
    }
    pub fn descriptor(&self) -> Option<EdgeDescriptor> {
        if let Ok(lock) = self.inner.read() {
            Some(lock.descriptor().clone())
        } else {
            None
        }
    }
    pub fn opposite_side_of(&self, node: &SharedNodeIdentifier) -> Option<SharedNodeIdentifier> {
        let (a, b) = self.get_sides()?;
        if &a == node {
            Some(b.clone())
        } else if &b == node {
            Some(a.clone())
        } else {
            None
        }
    }
    pub fn read<R, T: FnMut(&Edge) -> R>(&self, mut f: T) -> Option<R> {
        if let Ok(lock) = self.inner.read() {
            Some(f(lock.deref()))
        } else {
            None
        }
    }
    pub fn write<R, T: FnMut(&mut Edge) -> R>(&self, mut f: T) -> Option<R> {
        if let Ok(mut lock) = self.inner.write() {
            Some(f(lock.deref_mut()))
        } else {
            None
        }
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(self.descriptor(), state)
    }
}
