// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::redundant_closure_for_method_calls)]

use crate::{Edge, SharedEdge, SharedEdgeIdentifier, SharedNode, SharedNodeIdentifier};
use core::ops::Deref;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Graph {
    pub nodes: HashMap<SharedNodeIdentifier, SharedNode>,
    pub edges: HashMap<SharedEdgeIdentifier, SharedEdge>,
}

impl Graph {
    pub fn add_node(&mut self, node: SharedNode) -> Result<(), String> {
        let Some(d) = node.descriptor(|v| v.cloned()) else {
            return Err(format!("Can't retrieve descriptor from {node:?}"));
        };
        self.nodes.insert(d.id.clone().into(), node);
        Ok(())
    }
    #[allow(clippy::needless_pass_by_value)]
    pub fn add_edge(&mut self, edge: SharedEdge) -> Result<(), String> {
        let Some(d) = edge.descriptor() else {
            return Err(format!("Can't retrieve descriptor from {edge:?}"));
        };
        self.edges.insert(d.id.clone().into(), edge.clone());
        let Ok(lock) = edge.inner.read() else {
            return Err(format!("Can't get edge info for {d:?}"));
        };
        match lock.deref() {
            Edge::Directed { from, to, .. } => {
                self.nodes
                    .get(from)
                    .and_then(|v| v.add_navigable_edge(&edge).err());
                self.nodes.get(to).and_then(|v| v.add_edge(&edge).err());
            }
            Edge::Undirected { left, right, .. } => {
                self.nodes
                    .get(left)
                    .and_then(|v| v.add_navigable_edge(&edge).err());
                self.nodes
                    .get(right)
                    .and_then(|v| v.add_navigable_edge(&edge).err());
            }
        }
        Ok(())
    }
}
