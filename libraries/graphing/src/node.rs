// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;
use crate::{NodeDescriptor, SharedEdge};
use alloc::sync::Arc;
use core::hash::{Hash, Hasher};
use std::sync::Mutex;

pub struct Node {
    pub descriptor: NodeDescriptor,
    pub navigable_edges: Vec<SharedEdge>,
    pub all_edges: Vec<SharedEdge>,
}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.descriptor, state)
    }
}

pub type SharedNode = Arc<Mutex<Node>>;
