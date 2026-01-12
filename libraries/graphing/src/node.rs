// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;
use crate::{Descriptor, NodeDescriptor, SharedEdge};
use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use irox_geometry::Point;
use irox_structs_derive::Shared;

#[derive(Clone, Shared)]
#[shared()]
pub struct Node {
    pub descriptor: NodeDescriptor,
    pub navigable_edges: Vec<SharedEdge>,
    pub all_edges: Vec<SharedEdge>,
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Node")
            .field("descriptor", &self.descriptor.id)
            .finish()
    }
}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.descriptor, state)
    }
}

pub struct PositionedNode {
    pub descriptor: Descriptor,
    pub center_position: Point<f64>,
}

impl SharedNode {
    pub fn add_navigable_edge(&self, edge: SharedEdge) {
        if let Ok(mut lock) = self.inner.write() {
            lock.all_edges.push(edge.clone());
            lock.navigable_edges.push(edge);
        }
    }
    pub fn add_edge(&self, edge: SharedEdge) {
        if let Ok(mut lock) = self.inner.write() {
            lock.all_edges.push(edge);
        }
    }
}
