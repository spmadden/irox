// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;
use crate::{Point, Rectangle};
use alloc::boxed::Box;
use alloc::vec::Vec;
use irox_tools::FloatIsh;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SplitNode<T: FloatIsh, V: Clone> {
    center: Point<T>,
    bounds: Rectangle<T>,
    element_count: usize,
    nw: Box<QuadtreeNode<T, V>>,
    ne: Box<QuadtreeNode<T, V>>,
    sw: Box<QuadtreeNode<T, V>>,
    se: Box<QuadtreeNode<T, V>>,
}
#[derive(Debug, Default, Clone, PartialEq)]
pub enum QuadtreeNode<T: FloatIsh, V: Clone> {
    #[default]
    Empty,
    Split(SplitNode<T, V>),
    Values {
        values: Vec<V>,
    },
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Quadtree<T: FloatIsh, V: Clone> {
    root: QuadtreeNode<T, V>,

    bounds: Rectangle<T>,
    element_count: usize,
    max_distance: T,
}

impl<T: FloatIsh, V: Clone> Quadtree<T, V> {}
