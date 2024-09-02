// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use core::marker::PhantomData;
use alloc::collections::BTreeMap;
use crate::streaming::{Min, Max, Mean};

pub struct ResolutionExponent(pub i8);

pub struct Cell<K, V>{
    pub base2_exponent: i8,
    pub start_inclusive: K,
    pub end_exclusive: K,
    pub min: Min<V>,
    pub max: Max<V>,
    pub mean: Mean<V>,
}



pub struct TimePyramidLevel<K, V> {
    pub exponent: i8,
    pub level_data: BTreeMap<K, V>
}

pub struct TimePyramidMap<K, V>{
    data: BTreeMap<i8, TimePyramidLevel<K, V>>,
    _phan: PhantomData<V>,
}