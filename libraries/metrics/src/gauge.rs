// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

extern crate alloc;

use crate::{Sample, SampleError};
use irox_types::PrimitiveValue;

///
/// A Gauge is an instantaneous value of something at a specific point in time.
pub struct Gauge {
    // sinks: Vec<SI>,
    value: Option<Sample>,
}

impl Gauge {
    #[must_use]
    pub fn new() -> Self {
        Self {
            // sinks: Vec::new(),
            value: None,
        }
    }

    // /// Adds a listener for when this value changes.
    // pub fn add_sink(&mut self, sink: SI) {
    //     self.sinks.push(sink);
    // }

    /// Returns the last sample value retrieved
    #[must_use]
    pub fn last_value(&self) -> Option<&Sample> {
        self.value.as_ref()
    }

    pub fn update_infallible_value<V: Into<PrimitiveValue>, F: FnMut() -> V, T: Fn(F) -> Sample>(
        &mut self,
        timer: T,
        func: F,
    ) -> Option<&Sample> {
        let s = timer(func);
        self.value = Some(s);
        self.last_value()
    }

    pub fn update_infallible<F: FnMut() -> Sample>(&mut self, mut func: F) -> Option<&Sample> {
        let s = func();
        self.value = Some(s);
        self.last_value()
    }

    /// Updates the value from the sample source, calls the listeners
    pub fn update<F: FnMut() -> Result<Sample, SampleError>>(
        &mut self,
        mut func: F,
    ) -> Result<Option<&Sample>, SampleError> {
        let s = func()?;
        // for l in &mut self.sinks {
        //     l.new_sample(&s);
        // }
        self.value = Some(s);
        Ok(self.last_value())
    }

    pub fn set_value(&mut self, value: Sample) {
        self.value = Some(value);
    }
}
