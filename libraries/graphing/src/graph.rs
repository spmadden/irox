// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::redundant_closure_for_method_calls)]

use crate::{Edge, Node, SharedEdge, SharedEdgeIdentifier, SharedNode, SharedNodeIdentifier};
use core::ops::Deref;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Graph {
    pub nodes: HashMap<SharedNodeIdentifier, SharedNode>,
    pub edges: HashMap<SharedEdgeIdentifier, SharedEdge>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }
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
        let id: SharedEdgeIdentifier = d.id.clone().into();
        self.edges.insert(id.clone(), edge.clone());
        let Ok(lock) = edge.inner.read() else {
            return Err(format!("Can't get edge info for {d:?}"));
        };
        match lock.deref() {
            Edge::Directed { from, to, .. } => {
                self.nodes
                    .get(from)
                    .and_then(|v| v.add_navigable_edge(&id).err());
                self.nodes.get(to).and_then(|v| v.add_edge(&id).err());
            }
            Edge::Undirected { left, right, .. } => {
                self.nodes
                    .get(left)
                    .and_then(|v| v.add_navigable_edge(&id).err());
                self.nodes
                    .get(right)
                    .and_then(|v| v.add_navigable_edge(&id).err());
            }
        }
        Ok(())
    }
    pub fn remove_edge(&mut self, id: &SharedEdgeIdentifier) {
        self.edges.remove(id);
    }

    pub fn remove_node(&mut self, id: &SharedNodeIdentifier) {
        if let Some(node) = self.nodes.remove(id) {
            if let Ok(lock) = node.read_lock() {
                for edge in &lock.all_edges {
                    self.remove_edge(edge);
                }
            }
        }
    }

    ///
    /// Creates a graph with all navigable edges inverted
    #[must_use]
    pub fn inverted_edges(&self) -> Self {
        let mut outgraph = Graph::new();
        for node in self.nodes.values() {
            node.get(|node| {
                let _ = outgraph.add_node(
                    Node {
                        descriptor: node.descriptor.clone(),
                        navigable_edges: vec![],
                        all_edges: vec![],
                    }
                    .into(),
                );
            });
        }
        for edge in self.edges.values() {
            if edge.is_directed() {
                edge.get(|f| {
                    if let Edge::Directed {
                        descriptor,
                        from,
                        to,
                    } = f
                    {
                        let newedge: SharedEdge = Edge::Directed {
                            descriptor: descriptor.clone(),
                            from: to.clone(),
                            to: from.clone(),
                        }
                        .into();
                        let _ = outgraph.add_edge(newedge);
                    }
                });
            } else {
                let _ = outgraph.add_edge(edge.clone());
            }
        }
        outgraph
    }
}
