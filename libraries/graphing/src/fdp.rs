// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::redundant_closure_for_method_calls)]

extern crate alloc;
mod centering;
mod collision;
mod edge;
pub mod magnetic;
mod repulsive;

pub use centering::*;
pub use collision::*;
pub use edge::*;
pub use repulsive::*;

use crate::fdp::magnetic::Magnetic;
use crate::{Graph, SharedEdge, SharedNode, SharedNodeIdentifier};
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Formatter};
use core::ops::DerefMut;
use irox_geometry::{Point, Vector, Vector2D};
use irox_units::units::angle::Angle;

const INITIAL_RADIUS: f64 = 1.0;
const INITIAL_ANGLE: Angle =
    Angle::new_radians(core::f64::consts::PI / 0.763_932_022_500_210_3_f64);

#[derive(Debug, Clone)]
pub enum Force {
    Centering(Centering),
    Edge(EdgeForce),
    Repulsive(Repulsive),
    Collision(Collision),
    Magnetic(Magnetic),
}
impl Force {
    pub fn force(&self, sim: &mut Simulation, alpha: f64) {
        match &self {
            Force::Centering(mut p) => p.force(sim, alpha),
            Force::Edge(mut e) => e.force(sim, alpha),
            Force::Repulsive(mut r) => r.force(sim, alpha),
            Force::Collision(mut c) => c.force(sim, alpha),
            Force::Magnetic(m) => m.force(sim, alpha),
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
    pub average_energy: f64,
    pub halt_on_energy: bool,
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
        if self.halt_on_energy {
            self.average_energy < 0.1
        } else {
            self.alpha < self.alpha_min
        }
    }
    pub(crate) fn tick(&mut self) -> f64 {
        let is_done_pre = self.is_done();
        self.alpha += (self.alpha_target - self.alpha) * self.alpha_decay;
        // self.alpha = self.alpha.max(self.alpha_min);
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
            average_energy: f64::INFINITY,
            halt_on_energy: false,
        }
    }
}
#[derive(Default)]
pub struct SimulationWorkingNode {
    pub current_position: Vector<f64>,
    pub current_velocity: Vector<f64>,
    pub fixed_position: Option<Point<f64>>,
}
impl Debug for SimulationWorkingNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SimulationWorkingNode")
            .field("current_position", &self.current_position)
            .field("current_velocity", &self.current_velocity)
            .field("fixed_position", &self.fixed_position)
            .finish()
    }
}
pub(crate) fn get_edge_vector(edge: &SharedEdge, sim: &mut Simulation) -> Vector<f64> {
    let mut pt1 = Vector::<f64>::default();
    let mut pt2 = Vector::<f64>::default();
    if let Some((left, right)) = edge.get_sides() {
        sim.node(&left, |n| {
            pt1 += n.current_position;
        });
        sim.node(&right, |n| {
            pt2 += n.current_position;
        });
    }

    pt2 - pt1
}
pub trait InitialNodePlacer {
    fn place_node(&mut self, node: &mut SharedNode, working: &mut SimulationWorkingNode);
}
#[derive(Debug, Clone)]
pub struct DefaultNodePlacement {
    initial_radius: f64,
    last_idx: usize,
}
impl DefaultNodePlacement {
    #[must_use]
    pub fn with_initial_radius(mut self, radius: f64) -> Self {
        self.initial_radius = radius;
        self
    }
}
impl Default for DefaultNodePlacement {
    fn default() -> Self {
        Self {
            last_idx: 0,
            initial_radius: INITIAL_RADIUS,
        }
    }
}
impl InitialNodePlacer for DefaultNodePlacement {
    fn place_node(&mut self, _node: &mut SharedNode, working: &mut SimulationWorkingNode) {
        let idx = self.last_idx;
        let init_radius = self.initial_radius * (0.5 + idx as f64).sqrt();
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
        self.params.average_energy = f64::INFINITY;
    }
    pub fn stepping<F: FnMut(&Simulation)>(&mut self, mut on_step: F) {
        while !self.is_done() {
            self.tick();
            on_step(self);
        }
    }
    pub fn iter_edges<F: FnMut(SharedEdge, &mut Simulation)>(&mut self, mut each: F) {
        let iter = self
            .graph
            .borrow()
            .edges
            .values()
            .cloned()
            .collect::<Vec<_>>();

        for edge in iter {
            each(edge, self);
        }
    }

    fn init_working_mem(
        node: &mut SharedNode,
        placement: &mut dyn InitialNodePlacer,
    ) -> Option<Shared<SimulationWorkingNode>> {
        let mut node2 = node.clone();
        let working = node.memory_mut(|mem| {
            let mem = mem?;
            let mut mem = mem.borrow_mut();
            let v = mem.get_mut_or_create::<_, Shared<SimulationWorkingNode>, _>(
                "SimulationWorkingNode",
                || {
                    let mut working = SimulationWorkingNode::default();
                    placement.place_node(&mut node2, &mut working);
                    Rc::new(RefCell::new(working))
                },
            );
            v.cloned()
        });
        working
    }
    pub fn iter_nodes<
        F: FnMut(&SharedNodeIdentifier, &mut SharedNode, &mut SimulationWorkingNode),
    >(
        &mut self,
        mut each: F,
    ) {
        for (id, node) in &mut self.graph.borrow_mut().nodes {
            let working = Self::init_working_mem(node, self.placement.as_mut());
            if let Some(working) = working {
                each(id, node, working.borrow_mut().deref_mut());
            }
        }
    }
    pub fn node<F: FnMut(&SimulationWorkingNode)>(
        &mut self,
        id: &SharedNodeIdentifier,
        mut each: F,
    ) {
        use core::ops::Deref;
        if let Some(node) = self.graph.borrow_mut().nodes.get_mut(id) {
            let working = Self::init_working_mem(node, self.placement.as_mut());
            if let Some(working) = working {
                each(working.borrow().deref());
            }
        }
    }
    pub fn node_mut<F: FnMut(&SharedNode, &mut SimulationWorkingNode)>(
        &mut self,
        id: &SharedNodeIdentifier,
        mut each: F,
    ) {
        if let Some(node) = self.graph.borrow_mut().nodes.get_mut(id) {
            let working = Self::init_working_mem(node, self.placement.as_mut());
            if let Some(working) = working {
                each(node, working.borrow_mut().deref_mut());
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
            let mut energy = 0.0f64;
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
                energy += working.current_velocity.magnitude();
            });
            let num_nodes = self.graph.borrow().nodes.len();
            energy /= num_nodes as f64;
            self.params.average_energy = energy;
        }
    }
}
