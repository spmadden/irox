// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::streaming::{Max, Mean, Min};
use alloc::collections::BTreeMap;
use core::marker::PhantomData;

pub struct ResolutionExponent(pub i8);

pub struct Cell<K, V> {
    pub base2_exponent: i8,
    pub start_inclusive: K,
    pub end_exclusive: K,
    pub min: Min<V>,
    pub max: Max<V>,
    pub mean: Mean<V>,
}

pub struct TimePyramidLevel<K, V> {
    pub exponent: i8,
    pub level_data: BTreeMap<K, V>,
}

pub struct TimePyramidMap<K, V> {
    pub data: BTreeMap<i8, TimePyramidLevel<K, V>>,
    _phan: PhantomData<V>,
}
