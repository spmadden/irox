// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! A collection of utilities for the u64 built-in
//!

///
/// Finds the minimum and maximum value in the provided iterator.
/// Example:
/// ```
/// let values : Vec<u64> = vec![0, 5, 30, 20, 2];
/// let (min, max) = irox_tools::u64::min_max(&values);
///
/// assert_eq!(min, 0);
/// assert_eq!(max, 30);
/// ```
#[must_use]
pub fn min_max(iter: &[u64]) -> (u64, u64) {
    let mut min = u64::MAX;
    let mut max = u64::MIN;

    for val in iter {
        min = min.min(*val);
        max = max.max(*val);
    }

    (min, max)
}
