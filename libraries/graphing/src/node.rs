// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{NodeDescriptor, SharedEdge};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

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
