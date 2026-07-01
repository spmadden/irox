// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;
use crate::fdp::Simulation;
use alloc::collections::VecDeque;
use irox_geometry::{Vector, Vector2D};

#[derive(Debug, Copy, Clone)]
pub struct Repulsive {
    pub strength: f64,
    pub edge_distance: f64,
}
impl Default for Repulsive {
    fn default() -> Self {
        Self {
            strength: -1.,
            edge_distance: 30.,
        }
    }
}
impl Repulsive {
    #[must_use]
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength;
        self
    }
    #[must_use]
    pub fn with_edge_distance(mut self, edge_distance: f64) -> Self {
        self.edge_distance = edge_distance;
        self
    }
    pub(crate) fn force(&mut self, sim: &mut Simulation, alpha: f64) {
        #[cfg(feature = "profiling")]
        profiling::scope!("Repulsive::force");
        let mut nodes = VecDeque::with_capacity(sim.graph.borrow().nodes.len());
        sim.iter_nodes(|id, _node, _working| {
            nodes.push_back(id.clone());
        });
        while let Some(left) = nodes.pop_front() {
            let left = &left;
            let mut qpos = Vector::default();
            let mut left_edges = 1.0;
            sim.node_mut(left, |node, working| {
                qpos = working.current_position;
                left_edges = node.num_edges() as f64;
            });
            for right in &nodes {
                if left == right {
                    continue;
                }

                let mut right_edges = 1.0;
                let mut npos = Vector::default();
                sim.node_mut(right, |node, working| {
                    npos = working.current_position;
                    right_edges = node.num_edges() as f64;
                });

                let delt = qpos - npos;
                let l = delt.magnitude();
                if l > 10. * self.edge_distance {
                    continue;
                }
                let l = l.powf(2.).max(0.5);

                let w = self.strength * alpha / l;
                let adj = delt * w;
                let mut bias = left_edges / (left_edges + right_edges);
                if !bias.is_normal() {
                    bias = 0.5;
                }
                sim.node_mut(right, |_node, working| {
                    working.current_velocity += adj * bias;
                });
                sim.node_mut(left, |_node, working| {
                    working.current_velocity -= adj * (1.0 - bias);
                })
            }
        }
    }
}
