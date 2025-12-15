// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{SharedEdge, SharedNode};

#[derive(Clone)]
pub struct Path {
    pub start: SharedNode,
    pub steps: Vec<PathItem>,
    pub end: SharedNode,
}

#[derive(Clone)]
pub enum PathItem {
    Node(SharedNode),
    Edge(SharedEdge),
}
