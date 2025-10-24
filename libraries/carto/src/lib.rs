// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Cartographic & Geospatial Library

#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use irox_tools::cfg_feature_std;
pub use irox_units;

pub mod altitude;
pub mod coordinate;
pub mod error;
pub mod geo;
pub mod gps;
pub mod position_type;
pub mod proj;
pub mod range;
pub mod spcs;

cfg_feature_std! {
    pub mod ecef;
    pub mod epsg2249;
    pub mod epsg3857;
    pub mod lcc;
    pub mod local;
    pub mod tm;
}

/// ISO 3166-1 Country Codes
pub mod countrycodes {
    #![allow(clippy::non_ascii_literal)]
    include!(concat!(env!("OUT_DIR"), "/countries.rs"));
}
