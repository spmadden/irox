// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use core::any::Any;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct AnyHashMap {
    inner: HashMap<String, Box<dyn Any>>,
}

impl AnyHashMap {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert<K: AsRef<str>, V: Any + 'static>(&mut self, key: K, value: V) {
        self.inner.insert(key.as_ref().to_string(), Box::new(value));
    }
    pub fn get<K: AsRef<str>, V: Any + 'static>(&self, key: K) -> Option<&V> {
        self.inner.get(key.as_ref()).and_then(|v| v.downcast_ref())
    }
    pub fn get_mut<K: AsRef<str>, V: Any + 'static>(&mut self, key: K) -> Option<&mut V> {
        self.inner
            .get_mut(key.as_ref())
            .and_then(|v| v.downcast_mut())
    }
    pub fn get_or_default<K: AsRef<str>, V: Any + Default + 'static>(
        &mut self,
        key: K,
    ) -> Option<&V> {
        self.inner
            .entry(key.as_ref().to_string())
            .or_insert_with(|| Box::new(V::default()))
            .downcast_ref()
    }
    pub fn get_mut_or_default<K: AsRef<str>, V: Any + Default + 'static>(
        &mut self,
        key: K,
    ) -> Option<&mut V> {
        self.inner
            .entry(key.as_ref().to_string())
            .or_insert_with(|| Box::new(V::default()))
            .downcast_mut()
    }
    pub fn take<K: AsRef<str>, V: Any + 'static>(&mut self, key: K) -> Option<V> {
        self.inner
            .remove(key.as_ref())
            .and_then(|v| v.downcast::<V>().ok())
            .map(|v| *v)
    }
}
