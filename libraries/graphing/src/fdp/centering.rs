// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::fdp::Simulation;

#[derive(Debug, Copy, Clone)]
pub struct Centering {
    pub strength: f64,
}
impl Default for Centering {
    fn default() -> Self {
        Self { strength: 0.1 }
    }
}
impl Centering {
    pub fn new(strength: f64) -> Self {
        Self { strength }
    }
    pub(crate) fn force(&mut self, sim: &mut Simulation, alpha: f64) {
        for node in sim.working_nodes.values_mut() {
            let adj = node.current_position * alpha * self.strength * -1.0;
            node.current_position += adj;
        }
    }
}
