// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;
use crate::{Descriptor, NodeDescriptor, SharedEdge, SharedEdgeIdentifier};
use core::fmt::Display;
use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use irox_geometry::Point;
use irox_structs_derive::Shared;
use irox_tools::identifier::{Identifier, SharedIdentifier};
use irox_tools::sync::PoisonedError;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SharedNodeIdentifier(SharedIdentifier);
impl Deref for SharedNodeIdentifier {
    type Target = SharedIdentifier;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<SharedIdentifier> for SharedNodeIdentifier {
    fn from(value: SharedIdentifier) -> Self {
        Self(value)
    }
}
impl From<Identifier> for SharedNodeIdentifier {
    fn from(value: Identifier) -> Self {
        SharedIdentifier::from(value).into()
    }
}
impl Display for SharedNodeIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self.0.deref(), f)
    }
}

#[derive(Clone, Shared)]
#[shared()]
pub struct Node {
    pub descriptor: NodeDescriptor,
    pub navigable_edges: Vec<SharedEdgeIdentifier>,
    pub all_edges: Vec<SharedEdgeIdentifier>,
}
impl Node {
    pub fn from_id(id: Identifier) -> Self {
        Node {
            descriptor: NodeDescriptor(Descriptor::from(id)),
            navigable_edges: vec![],
            all_edges: vec![],
        }
    }
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Node")
            .field("descriptor", &self.descriptor.id)
            .finish()
    }
}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.descriptor, state)
    }
}

pub struct PositionedNode {
    pub descriptor: Descriptor,
    pub center_position: Point<f64>,
}

impl SharedNode {
    pub fn id(&self) -> Option<SharedIdentifier> {
        if let Ok(lock) = self.inner.read() {
            return Some(lock.descriptor.id.clone());
        }
        None
    }
    pub fn add_navigable_edge(&self, edge: &SharedEdge) -> Result<(), PoisonedError> {
        let mut lock = self.inner.write()?;
        let id = edge.inner.read()?.id();
        lock.all_edges.push(id.clone());
        lock.navigable_edges.push(id);
        Ok(())
    }
    pub fn add_edge(&self, edge: &SharedEdge) -> Result<(), PoisonedError> {
        let mut lock = self.inner.write()?;
        let id = edge.inner.read()?.id();
        lock.all_edges.push(id);
        Ok(())
    }
}
