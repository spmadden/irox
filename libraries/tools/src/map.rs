// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use alloc::collections::vec_deque::Iter;
use alloc::collections::VecDeque;
use core::borrow::Borrow;
use core::hash::Hash;
use std::collections::hash_map::{Entry, ValuesMut};
use std::collections::HashMap;

///
/// Tracks the insertion order of key/value pairs.  Backed by a [`HashMap`] for storage
/// and a [`VecDeque`] for key insertion order.
pub struct OrderedHashMap<K, V> {
    map: HashMap<K, V>,
    key_order: VecDeque<K>,
}
impl<K, V> Default for OrderedHashMap<K, V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
            key_order: Default::default()
        }
    }
}

impl<K, V> OrderedHashMap<K, V> {
    pub fn new() -> OrderedHashMap<K, V> {
        Self {
            map: HashMap::new(),
            key_order: VecDeque::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> OrderedHashMap<K, V> {
        Self {
            map: HashMap::with_capacity(capacity),
            key_order: VecDeque::with_capacity(capacity),
        }
    }
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.key_order.iter()
    }
    pub fn clear(&mut self) {
        self.map.clear();
        self.key_order.clear();
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
impl<K: Eq + Hash + Clone, V> OrderedHashMap<K, V> {
    pub fn get<Q: ?Sized + Hash + Eq>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.map.get(k)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let old = self.map.remove(&k);
        if old.is_some() {
            self.key_order.retain_mut(|o| *o != k);
        }
        self.key_order.push_back(k.clone());
        self.map.insert(k, v);
        old
    }
    pub fn contains(&self, k: &K) -> bool {
        self.map.contains_key(k)
    }

    pub fn entry(&mut self, k: K) -> Entry<'_, K, V>{
        self.map.entry(k)
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V>{
        self.map.get_mut(k)
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.map.values_mut()
    }
}
impl<'a, K: Eq + Hash, V> OrderedHashMap<K, V> {
    pub fn iter(&'a self) -> OrderedMapIter<'a, K, V> {
        OrderedMapIter {
            key_iter: self.key_order.iter(),
            values: &self.map,
        }
    }
    pub fn drain(self) -> OrderedDrain<K, V> {
        OrderedDrain {
            key_iter: self.key_order,
            values: self.map,
        }
    }
}
impl<'a, K: Eq + Hash, V> IntoIterator for &'a OrderedHashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = OrderedMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct OrderedMapIter<'a, K, V> {
    key_iter: Iter<'a, K>,
    values: &'a HashMap<K, V>,
}
impl<'a, K: Eq + Hash, V> Iterator for OrderedMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.key_iter.next()?;
        let v = self.values.get(key)?;
        Some((key, v))
    }
}
pub struct OrderedMapValues<'a, K, V> {
    iter: OrderedMapIter<'a, K, V>,
}
impl<'a, K: Eq + Hash, V> Iterator for OrderedMapValues<'a, K, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_k, v)| v)
    }
}

pub struct OrderedDrain<K, V> {
    key_iter: VecDeque<K>,
    values: HashMap<K, V>,
}
impl<K: Eq + Hash, V> Iterator for OrderedDrain<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.key_iter.pop_front()?;
        let v = self.values.remove(&key)?;
        Some((key, v))
    }
}
