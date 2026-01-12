// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;
use crate::{EdgeDescriptor, SharedNode};
use alloc::sync::Arc;
use core::hash::{Hash, Hasher};
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub enum Edge {
    Directed {
        descriptor: EdgeDescriptor,
        from: SharedNode,
        to: SharedNode,
    },
    Undirected {
        descriptor: EdgeDescriptor,
        left: SharedNode,
        right: SharedNode,
    },
}

impl Edge {
    pub fn descriptor(&self) -> &EdgeDescriptor {
        match self {
            Edge::Directed { descriptor, .. } | Edge::Undirected { descriptor, .. } => descriptor,
        }
    }
    pub fn get_sides(&self) -> (&SharedNode, &SharedNode) {
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
    pub fn get_sides(&self) -> Option<(SharedNode, SharedNode)> {
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
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(self.descriptor(), state)
    }
}
