// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Cartographic & Geospatial Library

#![forbid(unsafe_code)]

pub use irox_units;

pub mod altitude;
pub mod coordinate;
pub mod epsg3857;
pub mod error;
pub mod geo;
pub mod gps;
pub mod position_type;
pub mod proj;
pub mod tm;

/// ISO 3166-1 Country Codes
pub mod countrycodes {
    #![allow(clippy::non_ascii_literal)]
    include!(concat!(env!("OUT_DIR"), "/countries.rs"));
}
