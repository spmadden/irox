// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use crate::geometry::GeometryType;
use irox_tools::FloatIsh;
use alloc::boxed::Box;

#[derive(Debug, Clone, PartialEq)]
pub struct KDTree<PlaneType: FloatIsh, ValueType> {
    root: KDNode<PlaneType, ValueType>,
    size: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub enum KDNode<PlaneType: FloatIsh, ValueType> {
    Empty,
    Inner {
        plane_value: PlaneType,
        plane: KDAxis,
        lesser_or_eq: Box<KDNode<PlaneType, ValueType>>,
        greater: Box<KDNode<PlaneType, ValueType>>,
        size: usize,
        depth: usize,
    },
    Value {
        value: ValueType,
        geometry: GeometryType<PlaneType>,
    },
}
impl<PlaneType: FloatIsh, ValueType> KDNode<PlaneType, ValueType> {
    pub fn insert(&mut self) {}
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KDAxis {
    XAxis,
    YAxis,
}

impl KDAxis {
    #[must_use]
    pub fn next(&self) -> Self {
        match self {
            KDAxis::XAxis => KDAxis::YAxis,
            KDAxis::YAxis => KDAxis::XAxis,
        }
    }
}
