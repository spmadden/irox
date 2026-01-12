// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::redundant_closure_for_method_calls)]

extern crate alloc;

use crate::{EdgeDescriptor, NodeDescriptor, SharedEdge, SharedNode};
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Formatter};
use core::ops::DerefMut;
use irox_geometry::{Point, Vector, Vector2D};
use irox_units::units::angle::Angle;
use std::collections::HashMap;

const INITIAL_RADIUS: f64 = 10.0;
const INITIAL_ANGLE: Angle =
    Angle::new_radians(core::f64::consts::PI / 0.763_932_022_500_210_3_f64);

#[derive(Debug, Copy, Clone)]
pub enum Force {
    Position(PosForce),
    Edge(EdgeForce),
    Repulsive(Repulsive),
    Collision(Collision),
}
impl Force {
    pub fn force(&self, sim: &mut Simulation, alpha: f64) {
        match self {
            Force::Position(mut p) => p.force(sim, alpha),

            Force::Edge(mut e) => e.force(sim, alpha),
            Force::Repulsive(mut r) => r.force(sim, alpha),
            Force::Collision(mut c) => c.force(sim, alpha),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct SimulationParams {
    pub alpha: f64,
    pub alpha_min: f64,
    pub alpha_decay: f64,
    pub alpha_target: f64,
    pub velocity_decay: f64,
    pub tick: u64,
}
impl SimulationParams {
    #[must_use]
    pub fn with_target_iterations(&self, iterations: u32) -> Self {
        let alpha_decay = 1. - self.alpha_min.powf(1. / iterations as f64);
        Self {
            alpha_decay,
            ..*self
        }
    }
    pub(crate) fn tick(&mut self) -> f64 {
        self.alpha += (self.alpha_target - self.alpha) * self.alpha_decay;
        self.tick += 1;
        self.alpha
    }
}
impl Default for SimulationParams {
    fn default() -> Self {
        let alpha_min = 0.001f64;
        let alpha_decay = 1. - alpha_min.powf(1. / 300.);
        Self {
            alpha: 1.0,
            alpha_min,
            alpha_decay,
            alpha_target: 0.0,
            velocity_decay: 0.6,
            tick: 0,
        }
    }
}
pub struct SimulationWorkingNode {
    pub node: SharedNode,
    pub num_edges: f64,
    pub current_position: Vector<f64>,
    pub current_velocity: Vector<f64>,
    pub fixed_position: Option<Point<f64>>,
}
impl Debug for SimulationWorkingNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SimulationWorkingNode")
            .field("node", &self.node.descriptor(|v| v.cloned()))
            .field("num_edges", &self.num_edges)
            .field("current_position", &self.current_position)
            .field("current_velocity", &self.current_velocity)
            .field("fixed_position", &self.fixed_position)
            .finish()
    }
}
pub struct SimulationWorkingEdge {
    pub descriptor: EdgeDescriptor,
    pub left: Shared<SimulationWorkingNode>,
    pub right: Shared<SimulationWorkingNode>,
}
pub type Shared<T> = Rc<RefCell<T>>;
pub struct Simulation {
    pub nodes: Shared<Vec<Shared<SimulationWorkingNode>>>,
    pub edges: Shared<Vec<Shared<SimulationWorkingEdge>>>,
    pub forces: Vec<Force>,

    pub params: SimulationParams,
}

impl Simulation {
    pub fn new(
        params: SimulationParams,
        nodes: &[SharedNode],
        edges: &[SharedEdge],
        forces: Vec<Force>,
    ) -> Self {
        let mut working_nodes = Vec::new();
        let mut node_index = HashMap::<NodeDescriptor, Shared<SimulationWorkingNode>>::new();
        for (idx, n) in nodes.iter().enumerate() {
            let init_radius = INITIAL_RADIUS * (0.5 + idx as f64).sqrt();
            let init_angle = INITIAL_ANGLE * (idx as f64);
            let current_position = Vector {
                vx: init_radius * init_angle.cos(),
                vy: init_radius * init_angle.sin(),
            };
            let Some(descriptor) = n.descriptor(|f| f.cloned()) else {
                continue;
            };
            let node = Rc::new(RefCell::new(SimulationWorkingNode {
                node: n.clone(),
                num_edges: 0.0,
                current_position,
                current_velocity: Default::default(),
                fixed_position: None,
            }));
            node_index.insert(descriptor.clone(), node.clone());
            working_nodes.push(node);
        }
        let mut working_edges = Vec::new();
        for e in edges {
            let Some((a, b)) = e.get_sides() else {
                continue;
            };
            let Some(a) = a.descriptor(|f| f.cloned()) else {
                continue;
            };
            let Some(b) = b.descriptor(|f| f.cloned()) else {
                continue;
            };
            let Some(left) = node_index.get(&a).cloned() else {
                continue;
            };
            let Some(right) = node_index.get(&b).cloned() else {
                continue;
            };
            let Some(descriptor) = e.descriptor() else {
                continue;
            };
            left.borrow_mut().num_edges += 1.0;
            right.borrow_mut().num_edges += 1.0;
            let working_edge = Rc::new(RefCell::new(SimulationWorkingEdge {
                descriptor,
                left,
                right,
            }));
            working_edges.push(working_edge);
        }
        let working_edges = Rc::new(RefCell::new(working_edges));
        let working_nodes = Rc::new(RefCell::new(working_nodes));

        Self {
            nodes: working_nodes,
            edges: working_edges,
            params,
            forces,
        }
    }
    pub fn restart(&mut self) {
        self.params.alpha = 1.0;
    }
    pub fn stepping<F: FnMut(&Simulation)>(&mut self, mut on_step: F) {
        while self.params.alpha > self.params.alpha_min {
            self.tick();
            on_step(self);
        }
    }

    pub fn tick(&mut self) {
        if self.params.alpha < self.params.alpha_min {
            return;
        }
        let alpha = self.params.tick();
        for f in self.forces.clone() {
            f.clone().force(self, alpha);
        }
        for n in self.nodes.borrow().iter() {
            let mut node = n.borrow_mut();
            if let Some(fp) = node.fixed_position {
                node.current_position = fp.to_vector();
                node.current_velocity = Vector::default();
            } else {
                node.current_velocity *= self.params.velocity_decay;
                let v = node.current_velocity;
                node.current_position += v;
            }
        }
    }

    fn node_idx<F: FnMut(&mut SimulationWorkingNode)>(&self, idx: usize, mut f: F) {
        if let Some(v) = self.nodes.borrow_mut().get(idx) {
            f(v.borrow_mut().deref_mut())
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PosForce {
    pub strength: f64,
}
impl Default for PosForce {
    fn default() -> Self {
        Self { strength: 0.1 }
    }
}
impl PosForce {
    fn force(&mut self, sim: &Simulation, alpha: f64) {
        for node in sim.nodes.borrow_mut().iter_mut() {
            let adj = node.borrow().current_position * alpha * self.strength * -1.0;
            node.borrow_mut().current_velocity += adj;
        }
    }
}
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
    fn force(&mut self, sim: &Simulation, alpha: f64) {
        for edge in sim.edges.borrow().iter() {
            for _ in 0..self.iterations {
                let edge = edge.borrow();
                let mut left = edge.left.borrow_mut();
                let mut right = edge.right.borrow_mut();
                let dists = left.current_position + left.current_velocity
                    - right.current_position
                    - right.current_velocity;
                let dist = dists.magnitude().max(1.0);
                let strength = alpha / self.strength.unwrap_or(left.num_edges.min(right.num_edges));

                let adj = (dist - self.distance) / dist;
                let adj = adj * strength;
                let adj = dists * adj;
                left.current_velocity -= adj;
                right.current_velocity += adj;
            }
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Repulsive {
    pub strength: f64,
}
impl Default for Repulsive {
    fn default() -> Self {
        Self { strength: -1. }
    }
}
impl Repulsive {
    #[must_use]
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength;
        self
    }
    fn force(&mut self, sim: &Simulation, alpha: f64) {
        let len = sim.nodes.borrow().len();

        for idx1 in 0..len {
            let mut qpos = Vector::default();
            sim.node_idx(idx1, |n| {
                qpos = n.current_position;
            });

            for idx2 in 0..len {
                if idx1 == idx2 {
                    continue;
                }
                let mut npos = Vector::default();
                sim.node_idx(idx2, |n| {
                    npos = n.current_position;
                });

                let delt = qpos - npos;
                let l = delt.magnitude();
                // limit forces if really small

                let w = self.strength * alpha / l;
                let adj = delt * w;
                sim.node_idx(idx2, |n| {
                    n.current_velocity += adj;
                });
                // sim.node_idx(idx1, |n| {
                //     n.current_velocity -= adj;
                // })
            }
        }
    }
}
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
    fn force(&mut self, sim: &Simulation, _alpha: f64) {
        for _ in 0..self.iterations {
            let len = sim.nodes.borrow().len();
            let r2 = self.radius * self.radius;
            for idx1 in 0..len {
                let mut qpos = Vector::default();
                sim.node_idx(idx1, |n| {
                    qpos = n.current_position + n.current_velocity;
                });

                for idx2 in 0..len {
                    if idx1 == idx2 {
                        continue;
                    }
                    let mut npos = Vector::default();
                    sim.node_idx(idx2, |n| {
                        npos = n.current_position + n.current_velocity;
                    });

                    let delt = qpos - npos;
                    let l = delt.magnitude();

                    if l >= r2 {
                        continue;
                    }
                    let l = l.sqrt();
                    let l = (self.radius - l) / l * self.strength;
                    let adj = delt * l;

                    sim.node_idx(idx1, |n| {
                        n.current_velocity += adj;
                    });
                    sim.node_idx(idx2, |n| {
                        n.current_velocity -= adj;
                    });
                }
            }
        }
    }
}
