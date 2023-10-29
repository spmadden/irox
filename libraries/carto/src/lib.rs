// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

pub use irox_units::*;

pub mod altitude;
pub mod coordinate;
pub mod epsg3857;
pub mod error;
pub mod geo;
pub mod gps;
pub mod proj;
pub mod tm;

pub mod countrycodes {
    #![allow(clippy::non_ascii_literal)]
    include!(concat!(env!("OUT_DIR"), "/countries.rs"));
}
