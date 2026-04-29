// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::walk::{BreadthWalk, GraphWalk, WalkerAdding, WalkerCreating};
use crate::{Graph, SharedNodeIdentifier};

pub struct Updown;
impl Updown {
    pub fn run(graph: &Graph, interested_nodes: &[SharedNodeIdentifier]) -> Graph {
        let mut out = Graph::new();
        {
            let mut walker = WalkerAdding::new(&mut out);
            for node in interested_nodes {
                let mut walk = BreadthWalk::new(graph, node.clone());
                walk.walk(&mut walker);
            }
        }
        {
            let mut walker = WalkerCreating::default();
            let inverted = graph.inverted_edges();
            for node in interested_nodes {
                let mut walk = BreadthWalk::new(&inverted, node.clone());
                walk.walk(&mut walker);
            }
            let inverted = walker.finish().inverted_edges();
            out.merge_from(&inverted);
        }
        out
    }
}
