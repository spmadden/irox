// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! All-Pairs Shortest Paths
//!
//! Floyd-Warshall

use crate::error::GraphError;
use crate::{Edge, Graph, SharedEdgeIdentifier, SharedNodeIdentifier};
use irox_tools::buf::Linear2DArray;
use irox_tools::sync::PoisonedError;
use std::collections::HashMap;

pub trait EdgeCoster {
    fn get_cost(&self, edge: &Edge) -> f64;
}

pub struct AllPairsShortestPath<'a> {
    graph: &'a Graph,
    node_order: HashMap<SharedNodeIdentifier, usize>,
    node_indexes: HashMap<usize, SharedNodeIdentifier>,
    costs: Linear2DArray<f64>,
    paths: Linear2DArray<Option<(SharedNodeIdentifier, SharedEdgeIdentifier)>>,
}

impl<'a> AllPairsShortestPath<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let node_count = graph.nodes.len();
        let mut node_order = HashMap::<SharedNodeIdentifier, usize>::new();
        let mut node_indexes = HashMap::<usize, SharedNodeIdentifier>::new();
        for (idx, node) in graph.nodes.keys().enumerate() {
            node_order.insert(node.clone(), idx);
            node_indexes.insert(idx, node.clone());
        }

        debug_assert_eq!(
            node_count,
            node_order.len(),
            "Node order doesn't match count"
        );
        let costs =
            Linear2DArray::new_initialized_with(node_count, node_count, |_, _| f64::INFINITY);
        let paths = Linear2DArray::new_initialized_with(node_count, node_count, |_, _| None);
        Self {
            graph,
            costs,
            paths,
            node_order,
            node_indexes,
        }
    }
    pub fn recost<T: EdgeCoster>(&mut self, coster: &T) -> Result<(), GraphError> {
        for (id, e) in &self.graph.edges {
            let edge = e.inner.read().map_err(PoisonedError::from)?;
            let cost = coster.get_cost(&edge);
            if cost < 0.0 {
                return GraphError::negative_cost(format!(
                    "Edge coster returned negative cost ({cost}) for edge {id}"
                ));
            }
            let (left, right) = edge.get_sides();
            let Some(left_idx) = self.node_order.get(left) else {
                continue;
            };
            let Some(right_idx) = self.node_order.get(right) else {
                continue;
            };
            self.costs.set(*left_idx, *right_idx, cost);
            self.paths
                .set(*left_idx, *right_idx, Some((left.clone(), id.clone())));
            if !edge.is_directed() {
                self.costs.set(*right_idx, *left_idx, cost);
                self.paths
                    .set(*right_idx, *left_idx, Some((right.clone(), id.clone())));
            }
        }
        let node_count = self.node_order.len();
        for k in 0..node_count {
            for i in 0..node_count {
                for j in 0..node_count {
                    let d_ij = self.costs.get(i, j).cloned().unwrap_or(f64::INFINITY);
                    let d_ik = self.costs.get(i, k).cloned().unwrap_or(f64::INFINITY);
                    let d_kj = self.costs.get(k, j).cloned().unwrap_or(f64::INFINITY);
                    let next = d_ik + d_kj;
                    if d_ij > next {
                        self.costs.set(i, j, next);
                        if let Some(_path) = self.paths.get_mut(k, j) {
                            let Some(_left) = self.node_indexes.get(&k) else {
                                continue;
                            };
                            // *path = Some(());
                            todo!()
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
