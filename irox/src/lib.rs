// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![forbid(unsafe_code)]

#[cfg(feature = "bits")]
pub extern crate irox_bits;
#[cfg(feature = "build")]
pub extern crate irox_build_rs;
#[cfg(feature = "carto")]
pub extern crate irox_carto;
#[cfg(feature = "cryptids")]
pub extern crate irox_cryptids;
#[cfg(feature = "csv")]
pub extern crate irox_csv;
#[cfg(feature = "derive-helpers")]
pub extern crate irox_derive_helpers;
#[cfg(feature = "dot")]
pub extern crate irox_dot;
#[cfg(feature = "egui-extras")]
pub extern crate irox_egui_extras;
#[cfg(feature = "enums")]
pub extern crate irox_enums;
#[cfg(feature = "fixedmath")]
pub extern crate irox_fixedmath;
#[cfg(feature = "geometry")]
pub extern crate irox_geometry;
#[cfg(feature = "git-tools")]
pub extern crate irox_git_tools;
#[cfg(feature = "gpx")]
pub extern crate irox_gpx;
#[cfg(feature = "graphing")]
pub extern crate irox_graphing;
#[cfg(feature = "influxdb_v1")]
pub extern crate irox_influxdb_v1;
#[cfg(feature = "log")]
pub extern crate irox_log;
#[cfg(feature = "metrics")]
pub extern crate irox_metrics;
#[cfg(feature = "networking")]
pub extern crate irox_networking;
#[cfg(feature = "nmea0183")]
pub extern crate irox_nmea0183;
#[cfg(feature = "progress")]
pub extern crate irox_progress;
#[cfg(feature = "protobuf")]
pub extern crate irox_protobuf;
#[cfg(feature = "raymarine-sonar")]
pub extern crate irox_raymarine_sonar;
#[cfg(feature = "sirf")]
pub extern crate irox_sirf;
#[cfg(feature = "stats")]
pub extern crate irox_stats;
#[cfg(feature = "structs")]
pub extern crate irox_structs;
#[cfg(feature = "threading")]
pub extern crate irox_threading;
#[cfg(feature = "time")]
pub extern crate irox_time;
#[cfg(feature = "tools")]
pub extern crate irox_tools;
#[cfg(feature = "types")]
pub extern crate irox_types;
#[cfg(feature = "units")]
pub extern crate irox_units;
#[cfg(feature = "win-loc-api")]
pub extern crate irox_winlocation_api;
#[cfg(feature = "bits")]
pub use irox_bits as bits;

#[cfg(feature = "build")]
pub use irox_build_rs as build;

#[cfg(feature = "carto")]
pub use irox_carto as carto;

#[cfg(feature = "cryptids")]
pub use irox_cryptids as cryptids;

#[cfg(feature = "csv")]
pub use irox_csv as csv;

#[cfg(feature = "dot")]
pub use irox_dot as dot;

#[cfg(feature = "derive-helpers")]
pub use irox_derive_helpers as derive_helpers;

#[cfg(feature = "egui-extras")]
pub use irox_egui_extras as egui_extras;

#[cfg(feature = "enums")]
pub use irox_enums as enums;

#[cfg(feature = "fixedmath")]
pub use irox_fixedmath as fixedmath;

#[cfg(feature = "enums_derive")]
pub use irox_enums_derive;

#[cfg(feature = "geometry")]
pub use irox_geometry as geometry;

#[cfg(feature = "git-tools")]
pub use irox_git_tools as git_tools;

#[cfg(feature = "graphing")]
pub use irox_graphing as graphing;

#[cfg(feature = "gpx")]
pub use irox_gpx as gpx;

#[cfg(feature = "influxdb_v1")]
pub use irox_influxdb_v1 as influxdb_v1;

#[cfg(feature = "log")]
pub use irox_log as log;

#[cfg(feature = "metrics")]
pub use irox_metrics as metrics;

#[cfg(feature = "networking")]
pub use irox_networking as networking;

#[cfg(feature = "nmea0183")]
pub use irox_nmea0183 as nmea0183;

#[cfg(feature = "progress")]
pub use irox_progress as progress;

#[cfg(feature = "protobuf")]
pub use irox_protobuf as protobuf;

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

#[cfg(feature = "ubx")]
pub use irox_ubx as ubx;

#[cfg(feature = "win-loc-api")]
pub use irox_winlocation_api as winlocation_api;
