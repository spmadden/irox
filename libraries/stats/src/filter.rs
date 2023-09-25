// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! This module provides Discrete Convolution Filters for Digital Signals Processing.
//!
//! Many algorithms sourced from the wonderful [DSPGuide](https://www.dspguide.com)
//!

use std::collections::vec_deque::Drain;
use std::collections::VecDeque;

///
/// Represents a convolving operation with streaming input data against the provided kernel.
#[derive(Clone)]
pub struct StreamingFilter {
    kernel: Vec<f64>,
    buffer: VecDeque<f64>,
}

impl StreamingFilter {
    ///
    /// Creates a new streaming filter with the provided input kernel
    pub fn new(kernel: Vec<f64>) -> StreamingFilter {
        let len = kernel.len() - 2;
        let buffer = VecDeque::from(vec![0.0; len]);
        StreamingFilter { kernel, buffer }
    }

    ///
    /// Adds the sample to the internal data buffer.  If there are enough samples in the buffer,
    /// the convolve operation is run, and the output returned.  Otherwise returns None.
    pub fn add_and_convolve(&mut self, sample: f64) -> Option<f64> {
        self.buffer.push_back(0.0_f64);

        self.kernel.iter().enumerate().for_each(|(idx, k)| {
            if let Some(val) = self.buffer.get_mut(idx) {
                *val += sample * *k;
            }
        });

        self.buffer.pop_front()
    }

    ///
    /// Drains the residual data in the buffer to the caller.
    pub fn drain(&mut self) -> Drain<f64> {
        self.buffer.drain(..)
    }
}

///
/// Performs the [discrete convolution](https://en.wikipedia.org/wiki/Convolution#Discrete_convolution)
/// function.  Convolves the provided kernel against the provided data.
pub fn convolve<K: AsRef<[f64]>, D: AsRef<[f64]>>(kernel: K, data: D) -> Vec<f64> {
    let kernel = kernel.as_ref();
    let data = data.as_ref();
    let length = kernel.len() + data.len();
    let mut out: Vec<f64> = Vec::with_capacity(length);
    out.resize(length, 0.0_f64);

    data.iter().enumerate().for_each(|(d_idx, d)| {
        kernel.iter().enumerate().for_each(|(k_idx, k)| {
            let idx = d_idx + k_idx;
            if let Some(val) = out.get_mut(idx) {
                *val += *d * *k;
            }
        });
    });

    out
}
