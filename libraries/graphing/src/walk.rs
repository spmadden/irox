// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use crate::{Edge, Graph, Node, SharedNode};
use alloc::collections::VecDeque;
use irox_tools::identifier::SharedIdentifier;

pub trait Walker {
    fn previsit_node(&mut self, node: &Node);
    fn walk_edge(&mut self, edge: &Edge);
    fn postvisit_node(&mut self, node: &Node);
}
pub trait WalkerMut {
    fn previsit_node(&mut self, node: &mut Node);
    fn walk_edge(&mut self, edge: &mut Edge);
    fn postvisit_node(&mut self, node: &mut Node);
}
pub trait GraphWalk {
    fn walk<T: Walker>(&mut self, walker: &mut T);
}

pub struct BredthWalk<'a> {
    pub graph: &'a Graph,
    pub next_nodes: VecDeque<SharedNode>,
    pub visited_nodes: Vec<SharedIdentifier>,
    pub visited_edges: Vec<SharedIdentifier>,
}
impl<'a> BredthWalk<'a> {
    pub fn new(graph: &'a Graph, start_node: SharedNode) -> Self {
        Self {
            graph,
            next_nodes: VecDeque::from([start_node]),
            visited_nodes: Vec::new(),
            visited_edges: Vec::new(),
        }
    }
}
impl GraphWalk for BredthWalk<'_> {
    fn walk<T: Walker>(&mut self, walker: &mut T) {
        while !self.next_nodes.is_empty() {
            let Some(next_node) = self.next_nodes.pop_front() else {
                continue;
            };
            let Some(node_id) = next_node.id() else {
                continue;
            };
            if self.visited_nodes.contains(&node_id) {
                continue;
            }
            next_node.get(|n| {
                walker.previsit_node(n);
                for e in &n.navigable_edges {
                    e.get(|e| {
                        walker.walk_edge(e);
                        self.visited_edges.push(e.id());
                    });
                    let Some(e2) = e.opposite_side_of(&node_id) else {
                        continue;
                    };
                    self.next_nodes.push_back(e2);
                }
                walker.postvisit_node(n);
            });
            self.visited_nodes.push(node_id);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_walk() {}
}
