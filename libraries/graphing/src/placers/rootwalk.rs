// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//
extern crate alloc;
use crate::fdp::{DefaultNodePlacement, InitialNodePlacer, SimulationWorkingNode};
use crate::walk::{BreadthWalk, GraphWalk, Walker};
use crate::{Graph, Node, Path, SharedNode, SharedNodeIdentifier};
use alloc::collections::BTreeMap;
use irox_geometry::Vector;

#[derive(Debug, Default, Clone)]
pub struct RootWalkPlacer {
    locations: BTreeMap<SharedNodeIdentifier, (i64, i64)>,
    backup: DefaultNodePlacement,
}
impl RootWalkPlacer {
    pub fn new(roots: &[SharedNodeIdentifier], graph: &Graph) -> Self {
        let mut placer = Self::default();
        let mut walker = Placer::new(&mut placer);
        for root in roots {
            let mut walk = BreadthWalk::new(graph, root.clone());
            walk.walk(&mut walker);
        }
        let xoffset = (walker.max_x - walker.initial_x) / 2;
        let yoffset = (walker.max_y - walker.initial_y) / 2;
        for (x, y) in placer.locations.values_mut() {
            *x -= xoffset;
            *y -= yoffset;
        }
        placer
    }
}

impl InitialNodePlacer for RootWalkPlacer {
    fn place_node(&mut self, node: &mut SharedNode, working: &mut SimulationWorkingNode) {
        let Some(id) = node.id() else {
            return;
        };
        let id: SharedNodeIdentifier = id.into();
        if let Some((x, y)) = self.locations.get(&id) {
            working.current_position = Vector::new(*x as f64, *y as f64);
        } else {
            self.backup.place_node(node, working);
        }
    }
}

struct Placer<'a> {
    pub current_xs: BTreeMap<i64, i64>,
    pub initial_y: i64,
    pub initial_x: i64,
    pub x_incr: i64,
    pub y_incr: i64,
    pub placer: &'a mut RootWalkPlacer,
    pub max_x: i64,
    pub max_y: i64,
}
impl<'a> Placer<'a> {
    fn new(placer: &'a mut RootWalkPlacer) -> Self {
        Self {
            current_xs: Default::default(),
            initial_y: 0,
            initial_x: 0,
            x_incr: 25,
            y_incr: 25,
            max_y: 0,
            max_x: 0,
            placer,
        }
    }
}
impl Walker for Placer<'_> {
    fn previsit_node(&mut self, _node: &Node, _current_path: &Path) {
        let id = SharedNodeIdentifier::from(_node.descriptor.id.clone());
        if !self.placer.locations.contains_key(&id) {
            let y = _current_path.steps.len() as i64 * self.y_incr + self.initial_y;
            let x = self.current_xs.entry(y).or_insert(self.initial_x);
            *x += self.x_incr;
            self.placer.locations.insert(id, (*x, y));
            self.max_x = self.max_x.max(*x);
            self.max_y = self.max_y.max(y);
        }
    }
}
