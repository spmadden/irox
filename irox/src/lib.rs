// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

#[cfg(feature = "carto")]
pub use irox_carto;
#[cfg(feature = "csv")]
pub use irox_csv;
#[cfg(feature = "egui-extras")]
pub use irox_egui_extras;
#[cfg(feature = "enums")]
pub use irox_enums;
#[cfg(feature = "enums_derive")]
pub use irox_enums_derive;
#[cfg(feature = "gpx")]
pub use irox_gpx;
#[cfg(feature = "influxdb_v1")]
pub use irox_influxdb_v1;
#[cfg(feature = "networking")]
pub use irox_networking;
#[cfg(feature = "nmea0183")]
pub use irox_nmea0183;
#[cfg(feature = "raymarine-sonar")]
pub use irox_raymarine_sonar;
#[cfg(feature = "sirf")]
pub use irox_sirf;
#[cfg(feature = "stats")]
pub use irox_stats;
#[cfg(feature = "structs")]
pub use irox_structs;
#[cfg(feature = "structs_derive")]
pub use irox_structs_derive;
#[cfg(feature = "threading")]
pub use irox_threading;
#[cfg(feature = "time")]
pub use irox_time;
#[cfg(feature = "tools")]
pub use irox_tools;
#[cfg(feature = "types")]
pub use irox_types;
#[cfg(feature = "units")]
pub use irox_units;
#[cfg(feature = "win-loc-api")]
pub use irox_winlocation_api;
