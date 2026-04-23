// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;

use crate::walk::{BreadthWalk, GraphWalk, PrevisitNodeWalker};
use crate::{Graph, SharedNodeIdentifier};
use alloc::collections::BTreeSet;

pub struct Roots;

impl Roots {
    pub fn run(graph: &Graph) -> Vec<SharedNodeIdentifier> {
        let mut roots = Vec::new();
        let inverted = graph.inverted_edges();
        let mut remaining_nodes: BTreeSet<SharedNodeIdentifier> =
            inverted.nodes.keys().cloned().collect();
        while let Some(node) = remaining_nodes.pop_first() {
            let mut walk = BreadthWalk::new(&inverted, node);
            walk.walk(&mut PrevisitNodeWalker::new(|node, _path| {
                let id: SharedNodeIdentifier = node.descriptor.id.clone().into();
                remaining_nodes.remove(&id);
                if node.navigable_edges.is_empty() && !roots.contains(&id) {
                    // terminal exit node
                    roots.push(id);
                }
            }));
        }
        roots
    }
}
