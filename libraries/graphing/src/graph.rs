// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::redundant_closure_for_method_calls)]

use crate::{Edge, EdgeDescriptor, NodeDescriptor, SharedEdge, SharedNode};
use core::ops::Deref;
use irox_tools::options::MaybeMap;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Graph {
    pub nodes: HashMap<NodeDescriptor, SharedNode>,
    pub edges: HashMap<EdgeDescriptor, SharedEdge>,
}

impl Graph {
    pub fn add_node(&mut self, node: SharedNode) -> Result<(), String> {
        let Some(d) = node.descriptor(|v| v.cloned()) else {
            return Err(format!("Can't retrieve descriptor from {node:?}"));
        };
        self.nodes.insert(NodeDescriptor(d.id.clone().into()), node);
        Ok(())
    }
    #[allow(clippy::needless_pass_by_value)]
    pub fn add_edge(&mut self, edge: SharedEdge) -> Result<(), String> {
        let Some(d) = edge.descriptor() else {
            return Err(format!("Can't retrieve descriptor from {edge:?}"));
        };
        self.edges.insert(d.clone(), edge.clone());
        let Ok(lock) = edge.inner.read() else {
            return Err(format!("Can't get edge info for {d:?}"));
        };
        match lock.deref() {
            Edge::Directed { from, to, .. } => {
                if let Some(node) = from
                    .descriptor(|v| v.cloned())
                    .maybe_map(|v| self.nodes.get(&v))
                {
                    node.add_navigable_edge(edge.clone());
                }
                if let Some(node) = to
                    .descriptor(|v| v.cloned())
                    .maybe_map(|v| self.nodes.get(&v))
                {
                    node.add_edge(edge.clone());
                }
            }
            Edge::Undirected { left, right, .. } => {
                left.descriptor(|v| {
                    v.cloned().inspect(|v| {
                        let node = self.nodes.entry(v.clone()).or_insert(left.clone());
                        node.add_navigable_edge(edge.clone());
                    });
                });
                right.descriptor(|v| {
                    v.cloned().inspect(|v| {
                        let node = self.nodes.entry(v.clone()).or_insert(right.clone());
                        node.add_navigable_edge(edge.clone());
                    });
                });
            }
        }
        Ok(())
    }
}
