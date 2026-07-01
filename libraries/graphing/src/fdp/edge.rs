// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::fdp::{Simulation, SimulationWorkingEdge};
use irox_geometry::{Vector, Vector2D};

#[derive(Debug, Copy, Clone)]
pub struct EdgeForce {
    pub distance: f64,
    pub iterations: usize,
    pub strength: Option<f64>,
}
impl Default for EdgeForce {
    fn default() -> Self {
        Self {
            distance: 30.0,
            iterations: 1,
            strength: None,
        }
    }
}
impl EdgeForce {
    #[must_use]
    pub fn with_distance(mut self, distance: f64) -> Self {
        self.distance = distance;
        self
    }
    #[must_use]
    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }
    #[must_use]
    pub fn with_fixed_strength(mut self, strength: Option<f64>) -> Self {
        self.strength = strength;
        self
    }
    pub(crate) fn force(&mut self, sim: &mut Simulation, alpha: f64) {
        #[cfg(feature = "profiling")]
        profiling::scope!("Edge::force");
        sim.iter_edges(|data, sim| {
            let SimulationWorkingEdge {
                id: _id,
                left,
                right,
                directed: _directed,
            } = data;
            for _ in 0..self.iterations {
                let mut dists = Vector::<f64>::default();
                let mut left_edges = 1.0;
                sim.node_mut(&left, |n| {
                    dists += n.current_position;
                    dists += n.current_velocity;
                    left_edges = n.num_edges;
                });
                let mut right_edges = 1.0;
                sim.node_mut(&right, |n| {
                    dists -= n.current_position;
                    dists -= n.current_velocity;
                    right_edges = n.num_edges;
                });
                let dist = dists.magnitude().max(1.0);

                let mut strength = self.strength.unwrap_or(left_edges.min(right_edges));
                if !strength.is_normal() {
                    strength = 1.0;
                }
                let adj = (dist - self.distance) / dist;
                let adj = adj * alpha / strength;
                let adj = dists * adj;
                let mut bias = left_edges / (left_edges + right_edges);
                if !bias.is_normal() {
                    bias = 0.5;
                }
                sim.node_mut(&left, |n| {
                    n.current_velocity -= adj * (1.0 - bias);
                });
                sim.node_mut(&right, |n| {
                    n.current_velocity += adj * (bias);
                });
            }
        });
    }
}
