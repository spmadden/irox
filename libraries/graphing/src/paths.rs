// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{SharedEdgeIdentifier, SharedNodeIdentifier};
use core::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Path {
    pub start: SharedNodeIdentifier,
    pub steps: Vec<PathItem>,
    pub end: Option<SharedNodeIdentifier>,
}
impl Path {
    pub fn from_start(start: SharedNodeIdentifier) -> Self {
        Self {
            start,
            steps: Default::default(),
            end: None,
        }
    }
}
impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.start)?;
        for s in &self.steps {
            write!(f, " {s}")?;
        }
        if let Some(end) = &self.end {
            write!(f, " {end}")
        } else {
            Ok(())
        }
    }
}

#[derive(Clone)]
pub enum PathItem {
    Node(SharedNodeIdentifier),
    Edge(SharedEdgeIdentifier),
}

impl Display for PathItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            PathItem::Node(n) => {
                write!(f, "({n})")
            }
            PathItem::Edge(e) => {
                write!(f, "-{{{e}}}>")
            }
        }
    }
}
