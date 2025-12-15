// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::SharedNode;
use irox_units::quantities::Quantity;

pub struct FlowDescription {
    pub flow_type: FlowType,
    pub quantity: Quantity<f64>,
}

pub enum FlowType {
    FloodSource { start: SharedNode },
    FloodSink { end: SharedNode },
    Directed { start: SharedNode, end: SharedNode },
}

pub struct FloodSource {
    pub source: SharedNode,
    pub capacity: Quantity<f64>,
}
