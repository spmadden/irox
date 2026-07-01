// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::fdp::Simulation;
use irox_geometry::{Vector, Vector2D};

#[derive(Debug, Copy, Clone)]
pub struct Collision {
    pub radius: f64,
    pub strength: f64,
    pub iterations: usize,
}
impl Default for Collision {
    fn default() -> Self {
        Self {
            radius: 1.0,
            strength: 1.0,
            iterations: 1,
        }
    }
}
impl Collision {
    #[must_use]
    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }
    #[must_use]
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength;
        self
    }
    #[must_use]
    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }
    pub(crate) fn force(&mut self, sim: &mut Simulation, alpha: f64) {
        #[cfg(feature = "profiling")]
        profiling::scope!("Collision::force");
        let mut nodes = Vec::new();
        sim.iter_nodes(|id, _node, _working| {
            nodes.push(id.clone());
        });
        for _ in 0..self.iterations {
            let r2 = self.radius * self.radius;
            for left in &nodes {
                let mut qpos = Vector::default();
                let mut left_edges = 1.0;
                sim.node_mut(left, |n| {
                    qpos = n.current_position + n.current_velocity;
                    left_edges = n.num_edges;
                });

                for right in &nodes {
                    if left == right {
                        continue;
                    }
                    let mut npos = Vector::default();
                    let mut right_edges = 1.0;
                    sim.node_mut(right, |n| {
                        npos = n.current_position + n.current_velocity;
                        right_edges = n.num_edges;
                    });

                    let delt = qpos - npos;
                    let l = delt.magnitude();

                    if l >= r2 {
                        continue;
                    }
                    let l = l.sqrt();
                    let l = (self.radius - l) / l * self.strength;
                    let adj = delt * l * alpha;

                    // let bias = left_edges / (left_edges + right_edges);
                    let bias = 0.5;

                    sim.node_mut(left, |n| {
                        n.current_velocity += adj * (1. - bias);
                    });
                    sim.node_mut(right, |n| {
                        n.current_velocity -= adj * bias;
                    });
                }
            }
        }
    }
}
