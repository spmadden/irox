// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::redundant_closure_for_method_calls)]

extern crate alloc;

use crate::{Graph, SharedNode};
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Formatter};
use irox_geometry::{Point, Vector, Vector2D};
use irox_tools::identifier::{Identifier, SharedIdentifier};
use irox_units::units::angle::Angle;

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
    pub node: SharedIdentifier,
    pub num_edges: f64,
    pub current_position: Vector<f64>,
    pub current_velocity: Vector<f64>,
    pub fixed_position: Option<Point<f64>>,
}
impl Debug for SimulationWorkingNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SimulationWorkingNode")
            .field("node", &self.node)
            .field("num_edges", &self.num_edges)
            .field("current_position", &self.current_position)
            .field("current_velocity", &self.current_velocity)
            .field("fixed_position", &self.fixed_position)
            .finish()
    }
}
#[derive(Debug, Clone)]
pub struct SimulationWorkingEdge {
    pub id: SharedIdentifier,
    pub left: SharedIdentifier,
    pub right: SharedIdentifier,
}
pub trait InitialNodePlacer {
    fn place_node(&mut self, node: &mut SharedNode, working: &mut SimulationWorkingNode);
}
#[derive(Debug, Default, Clone)]
pub struct DefaultNodePlacement {
    last_idx: usize,
}
impl InitialNodePlacer for DefaultNodePlacement {
    fn place_node(&mut self, _node: &mut SharedNode, working: &mut SimulationWorkingNode) {
        let idx = self.last_idx;
        let init_radius = INITIAL_RADIUS * (0.5 + idx as f64).sqrt();
        let init_angle = INITIAL_ANGLE * (idx as f64);
        let current_position = Vector {
            vx: init_radius * init_angle.cos(),
            vy: init_radius * init_angle.sin(),
        };
        working.current_position = current_position;
        self.last_idx += 1;
    }
}
pub type Shared<T> = Rc<RefCell<T>>;
pub struct Simulation {
    pub graph: Shared<Graph>,
    pub forces: Vec<Force>,

    pub working_nodes: BTreeMap<SharedIdentifier, SimulationWorkingNode>,
    pub working_edges: BTreeMap<SharedIdentifier, SimulationWorkingEdge>,

    pub params: SimulationParams,
    pub placement: Box<dyn InitialNodePlacer>,
}

impl Simulation {
    pub fn new(
        params: SimulationParams,
        forces: Vec<Force>,
        graph: Shared<Graph>,
        placement: Box<dyn InitialNodePlacer>,
    ) -> Self {
        Self {
            working_nodes: Default::default(),
            working_edges: Default::default(),
            graph,
            params,
            forces,
            placement,
        }
    }
    pub fn is_done(&self) -> bool {
        self.params.alpha < self.params.alpha_min
    }
    pub fn restart(&mut self) {
        self.params.alpha = 1.0;
    }
    pub fn stepping<F: FnMut(&Simulation)>(&mut self, mut on_step: F) {
        while !self.is_done() {
            self.tick();
            on_step(self);
        }
    }
    pub fn iter_edges<F: FnMut(SimulationWorkingEdge, &mut Simulation)>(&mut self, mut each: F) {
        let iter = self
            .graph
            .borrow()
            .edges
            .iter()
            .map(|(left, right)| (left.clone(), right.clone()))
            .collect::<Vec<_>>();

        for (id, edge) in iter {
            let working = self
                .working_edges
                .entry(id.clone())
                .or_insert_with_key(|id| {
                    let (left, right) = edge
                        .get_sides()
                        .map(|(left, right)| {
                            (
                                left.id()
                                    .unwrap_or_else(|| Identifier::random_string().into()),
                                right
                                    .id()
                                    .unwrap_or_else(|| Identifier::random_string().into()),
                            )
                        })
                        .unwrap_or_else(|| {
                            (
                                Identifier::random_string().into(),
                                Identifier::random_string().into(),
                            )
                        });
                    SimulationWorkingEdge {
                        id: id.clone(),
                        left,
                        right,
                    }
                });
            each(working.clone(), self);
        }
    }

    pub fn iter_nodes<F: FnMut(&SharedIdentifier, &mut SharedNode, &mut SimulationWorkingNode)>(
        &mut self,
        mut each: F,
    ) {
        for (id, node) in &mut self.graph.borrow_mut().nodes {
            let num_edges = node.all_edges(|v| v.map(|v| v.len())).unwrap_or_default();
            let working = self
                .working_nodes
                .entry(id.clone())
                .or_insert_with_key(|id| {
                    let mut new = SimulationWorkingNode {
                        node: id.clone(),
                        num_edges: num_edges as f64,
                        current_position: Default::default(),
                        current_velocity: Default::default(),
                        fixed_position: None,
                    };
                    self.placement.place_node(node, &mut new);
                    new
                });
            each(id, node, working);
        }
    }
    pub fn node<F: FnMut(&SimulationWorkingNode)>(&self, id: &SharedIdentifier, mut each: F) {
        if let Some(working) = self.working_nodes.get(id) {
            each(working);
        }
    }
    pub fn node_mut<F: FnMut(&mut SimulationWorkingNode)>(
        &mut self,
        id: &SharedIdentifier,
        mut each: F,
    ) {
        if let Some(working) = self.working_nodes.get_mut(id) {
            each(working);
        } else {
            let mut graph = self.graph.borrow_mut();
            if let Some(node) = graph.nodes.get_mut(id) {
                let num_edges = node.all_edges(|v| v.map(|v| v.len())).unwrap_or_default();

                let mut new = SimulationWorkingNode {
                    node: id.clone(),
                    num_edges: num_edges as f64,
                    current_position: Default::default(),
                    current_velocity: Default::default(),
                    fixed_position: None,
                };
                self.placement.place_node(node, &mut new);
                each(&mut new);
            }
        }
    }

    pub fn tick(&mut self) {
        if self.is_done() {
            return;
        }
        let alpha = self.params.tick();

        // apply forces
        for f in self.forces.clone() {
            f.clone().force(self, alpha);
        }
        let decay = self.params.velocity_decay;
        self.iter_nodes(|_id, _node, working| {
            // finalize node position & velocity
            if let Some(fp) = working.fixed_position {
                working.current_position = fp.to_vector();
                working.current_velocity = Vector::default();
            } else {
                working.current_velocity *= decay;
                let v = working.current_velocity;
                working.current_position += v;
            }
        });
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
    pub fn new(strength: f64) -> Self {
        Self { strength }
    }
    fn force(&mut self, sim: &mut Simulation, alpha: f64) {
        for node in sim.working_nodes.values_mut() {
            let adj = node.current_position * alpha * self.strength * -1.0;
            node.current_position += adj;
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
    fn force(&mut self, sim: &mut Simulation, alpha: f64) {
        sim.iter_edges(|data, sim| {
            let SimulationWorkingEdge {
                id: _id,
                left,
                right,
            } = data;
            for _ in 0..self.iterations {
                let mut dists = Vector::<f64>::default();
                let mut left_edges = 0.0;
                sim.node_mut(&left, |n| {
                    dists += n.current_position;
                    dists += n.current_velocity;
                    left_edges = n.num_edges;
                });
                let mut right_edges = 0.0;
                sim.node_mut(&right, |n| {
                    dists -= n.current_position;
                    dists -= n.current_velocity;
                    right_edges = n.num_edges;
                });
                let dist = dists.magnitude().max(1.0);
                let strength = alpha / self.strength.unwrap_or(left_edges.min(right_edges));

                let adj = (dist - self.distance) / dist;
                let adj = adj * strength;
                let adj = dists * adj;
                let bias = left_edges / (left_edges + right_edges);
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
    fn force(&mut self, sim: &mut Simulation, alpha: f64) {
        let mut nodes = Vec::new();
        sim.iter_nodes(|id, _node, _working| {
            nodes.push(id.clone());
        });
        for left in &nodes {
            let mut qpos = Vector::default();
            let mut left_edges = 1.0;
            sim.node_mut(left, |n| {
                qpos = n.current_position;
                left_edges = n.num_edges;
            });
            for right in &nodes {
                if left == right {
                    continue;
                }
                let mut right_edges = 1.0;
                let mut npos = Vector::default();
                sim.node_mut(right, |n| {
                    npos = n.current_position;
                    right_edges = n.num_edges;
                });

                let delt = qpos - npos;
                let l = delt.magnitude().powi(2);
                // limit forces if really small

                let w = self.strength * alpha / l;
                let adj = delt * w;
                let bias = left_edges / (left_edges + right_edges);
                sim.node_mut(right, |n| {
                    n.current_velocity += adj * (1.0 - bias);
                });
                sim.node_mut(left, |n| {
                    n.current_velocity -= adj * bias;
                })
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
    fn force(&mut self, sim: &mut Simulation, alpha: f64) {
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

                    let bias = left_edges / (left_edges + right_edges);
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
