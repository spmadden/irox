// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

pub type ScaleFunction = dyn FnOnce(f64) -> f64;

pub fn scale_fn_1(quantile: f64, _scale_factor: f64) -> f64 {
    (2. * quantile - 1.0).asin() * (quantile / core::f64::consts::TAU)
}
