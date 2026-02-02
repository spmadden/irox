// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::Point;
use irox_tools::FloatIsh;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum QuadtreeNode<T: FloatIsh, V: Clone> {
    #[default]
    Empty,
    Split {
        center: Point<T>,
        element_count: usize,
        nw: Box<QuadtreeNode<T, V>>,
        ne: Box<QuadtreeNode<T, V>>,
        sw: Box<QuadtreeNode<T, V>>,
        se: Box<QuadtreeNode<T, V>>,
    },
    Values {
        values: Vec<V>,
    },
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Quadtree<T: FloatIsh, V: Clone> {
    root: QuadtreeNode<T, V>,
}
