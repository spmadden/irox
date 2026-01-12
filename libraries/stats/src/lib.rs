// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! A collection of digital signals processing and statistics functions

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;
extern crate core;

use alloc::vec::Vec;

pub use gaussian as standard;

pub mod abg;
pub mod decay;
pub mod filter;
pub mod fitting;
pub mod gaussian;
pub mod lttb;
mod macros;
pub mod points;
pub mod pyramid;
pub mod rects;
pub mod sampling;
pub mod streaming;
pub mod streams;
#[cfg(any(
    all(doc, docsrs),
    all(feature = "std", any(unix, windows), feature = "miniz")
))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "std", any(unix, windows), feature = "miniz")))
)]
pub mod tsdf;
#[cfg(any(all(doc, docsrs), feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod windows;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DistributionParams {
    Mean(f64),
    StandardDeviation(f64),
    Variance(f64),
    Other(&'static str, f64),
}

/// This trait represents a statistical distribution
pub trait Distribution {
    /// computes the probability distribution function for a particular value
    fn pdf(&self, value: f64) -> f64;

    /// Returns a set of the known parameters of this distribution.
    fn get_params(&self) -> Vec<DistributionParams>;
}
