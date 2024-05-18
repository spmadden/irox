// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

#[cfg(feature = "bits")]
pub use irox_bits as bits;
#[cfg(feature = "build")]
pub use irox_build_rs as build;
#[cfg(feature = "carto")]
pub use irox_carto as carto;
#[cfg(feature = "csv")]
pub use irox_csv as csv;
#[cfg(feature = "egui-extras")]
pub use irox_egui_extras as egui_extras;
#[cfg(feature = "enums")]
pub use irox_enums as enums;
#[cfg(feature = "enums_derive")]
pub use irox_enums_derive;
#[cfg(feature = "git-tools")]
pub use irox_git_tools as git_tools;
#[cfg(feature = "gpx")]
pub use irox_gpx as gpx;
#[cfg(feature = "influxdb_v1")]
pub use irox_influxdb_v1 as influxdb_v1;
#[cfg(feature = "log")]
pub use irox_log as log;
#[cfg(feature = "networking")]
pub use irox_networking as networking;
#[cfg(feature = "nmea0183")]
pub use irox_nmea0183 as nmea0183;
#[cfg(feature = "progress")]
pub use irox_progress as progress;
#[cfg(feature = "raymarine-sonar")]
pub use irox_raymarine_sonar as raymarine_sonar;
#[cfg(feature = "sirf")]
pub use irox_sirf as sirf;
#[cfg(feature = "stats")]
pub use irox_stats as stats;
#[cfg(feature = "structs")]
pub use irox_structs as structs;
#[cfg(feature = "structs_derive")]
pub use irox_structs_derive;
#[cfg(feature = "threading")]
pub use irox_threading as threading;
#[cfg(feature = "time")]
pub use irox_time as time;
#[cfg(feature = "tools")]
pub use irox_tools as tools;
#[cfg(feature = "types")]
pub use irox_types as types;
#[cfg(feature = "units")]
pub use irox_units as units;
#[cfg(feature = "win-loc-api")]
pub use irox_winlocation_api as winlocation_api;
