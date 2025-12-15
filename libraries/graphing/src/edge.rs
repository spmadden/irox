// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;
use crate::{EdgeDescriptor, SharedNode};
use alloc::sync::Arc;
use core::hash::{Hash, Hasher};
use std::sync::Mutex;

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
}
pub type SharedEdge = Arc<Mutex<Edge>>;

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(self.descriptor(), state)
    }
}
