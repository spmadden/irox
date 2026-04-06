// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::redundant_closure_for_method_calls)]

extern crate alloc;
mod centering;
mod collision;
mod edge;
mod repulsive;
pub use centering::*;
pub use collision::*;
pub use edge::*;
pub use repulsive::*;

use crate::{Graph, SharedNode};
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Formatter};
use irox_geometry::{Point, Vector};
use irox_tools::identifier::{Identifier, SharedIdentifier};
use irox_units::units::angle::Angle;

const INITIAL_RADIUS: f64 = 1.0;
const INITIAL_ANGLE: Angle =
    Angle::new_radians(core::f64::consts::PI / 0.763_932_022_500_210_3_f64);

#[derive(Debug, Copy, Clone)]
pub enum Force {
    Centering(Centering),
    Edge(EdgeForce),
    Repulsive(Repulsive),
    Collision(Collision),
}
impl Force {
    pub fn force(&self, sim: &mut Simulation, alpha: f64) {
        match self {
            Force::Centering(mut p) => p.force(sim, alpha),
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
    pub target_iterations: u32,
    pub tick: u64,
    pub reset_velocity_on_next_tick: bool,
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
    pub fn update_target_iterations(&mut self) {
        self.alpha_decay = 1. - self.alpha_min.powf(1. / self.target_iterations as f64);
    }
    pub fn is_done(&self) -> bool {
        self.alpha < self.alpha_min
    }
    pub(crate) fn tick(&mut self) -> f64 {
        let is_done_pre = self.is_done();
        self.alpha += (self.alpha_target - self.alpha) * self.alpha_decay;
        self.tick += 1;
        let is_done_post = self.is_done();
        self.reset_velocity_on_next_tick = is_done_post && !is_done_pre;
        self.alpha
    }
}
impl Default for SimulationParams {
    fn default() -> Self {
        let alpha_min = 0.001f64;
        let target_iterations = 300u32;
        let alpha_decay = 1. - alpha_min.powf(1. / target_iterations as f64);
        Self {
            alpha: 1.0,
            alpha_min,
            alpha_decay,
            alpha_target: 0.0,
            velocity_decay: 0.6,
            tick: 0,
            target_iterations,
            reset_velocity_on_next_tick: false,
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
        self.params.is_done()
    }
    pub fn restart(&mut self) {
        self.params.alpha = 1.0;
        self.params.reset_velocity_on_next_tick = true;
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
        #[cfg(feature = "profiling")]
        profiling::scope!("FDP::tick");
        if self.params.reset_velocity_on_next_tick {
            self.params.reset_velocity_on_next_tick = false;
            self.iter_nodes(|_id, _node, working| {
                working.current_velocity = Vector::default();
            });
        }
        if self.is_done() {
            return;
        }
        let alpha = self.params.tick();

        // apply forces
        {
            #[cfg(feature = "profiling")]
            profiling::scope!("FDP::tick::apply_forces");
            for f in self.forces.clone() {
                f.clone().force(self, alpha);
            }
        }
        {
            #[cfg(feature = "profiling")]
            profiling::scope!("FDP::tick::update-positions");
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
}
