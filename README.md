Iron Oxide (IROX) Libraries
=============================
A collection of (hopefully) useful crates written in Rust.

[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/spmadden/irox/blob/master/LICENSE)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/spmadden/irox/blob/master/LICENSE-APACHE)
![Maintenance](https://img.shields.io/maintenance/yes/2025)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/spmadden/irox/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/irox)](https://crates.io/crates/irox/)
[![docs.rs](https://img.shields.io/docsrs/irox/latest)](https://docs.rs/irox/latest/irox/)

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](https://github.com/spmadden/irox/blob/master/CODE_OF_CONDUCT.md)
[![Semver2.0](https://img.shields.io/badge/semver-2.0-blue)](https://semver.org/spec/v2.0.0.html)
[![ConvCommits](https://img.shields.io/badge/conventional--commits-1.0-pink)](https://www.conventionalcommits.org/en/v1.0.0/)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Current Modules & Organization:
-----------------

* [`data-formats`](https://github.com/spmadden/irox/blob/master/data-formats) - Specific data format encoders & decoders
    * [`csv`] - Comma Separated Values encoder/decoder, inspired by python's `csv` module
    * [`gpx`] - GPX GPS file format reader/writer
    * [`nmea0183`] - GPS NMEA-0183 encoder/decoder
    * [`raymarine-sonar`] - Raymarine SDF Sonar Converter
    * [`sirf`] - GPS Binary SiRF encoder/decoder
* [`interfaces`](https://github.com/spmadden/irox/blob/master/interfaces) - Interfaces, Transports, and APIs for
  external tools
    * [`influxdb_v1`] - InfluxDB v1 API Client
    * [`winlocation-api`] - Interact with
      the [`Windows.Devices.Geolocation`](https://learn.microsoft.com/en-us/uwp/api/windows.devices.geolocation) API in
      idiomatic Rust
* [`irox`](https://github.com/spmadden/irox/blob/master/irox) - Aggregator module
* [`libraries`](https://github.com/spmadden/irox/blob/master/libraries) - Rust 'library' crates, usually without
  binaries
    * [`bits`] - Bits & Bobs. No-std/No-alloc bit/byte manipulation of streams
    * [`build-rs`] - Compile-time build metadata injection inspired by shadow-rs
    * [`carto`] - Cartographic & Geospatial tools
  * [`derive-helpers`] - Helper traits & functions for the proc_macro crate to aid in writing less complex derive
    macros
    * [`egui-extras`] - Extra stuff for the wonderful [`egui`](https://github.com/emilk/egui) crate
    * [`enums`] - Traits for better Enumerated Types
    * [`enums_derive`] - Derivable impls of the traits in irox-enums
    * [`fixedmath`] - Fixed Precision Math Primitives
    * [`git-tools`] - Tools and wrappers to help with GitOps
    * [`log`] - Basic console and file logging
    * [`networking`] - Networking tools
    * [`progress`] - An ecosystem for displaying progress, either in a UI or on the terminal.
    * [`stats`] - Mathematics & Statistics tools
    * [`structs`] - Traits for Struct Types - linearly serialized big endian bytes
    * [`structs_derive`] - Derivable impls of the traits in irox-structs
    * [`threading`] - Blocking and Asynchronous Threading Tools
    * [`time`] - A date & time library that aims for ease of use based on the Proleptic Gregorian Calendar.
    * [`tools`] - Dumping ground for stuff that should have been in `std`
    * [`types`] - Enums and structs to describe Rust's basic type system
    * [`units`] - Physical Units, Quantities, & Reference Frames
* [`tools`](https://github.com/spmadden/irox/blob/master/tools) - Rust 'binary' crates, mostly just binaries that depend
  on crates in `libraries`
    * [`gpsd`] - Implementation of GPSd in Rust, with support for windows targets!

Version Status
------------------

| Crate                  | no_std?        | no_alloc?        | External Deps | Status                                                                                                      |
|------------------------|----------------|------------------|---------------|-------------------------------------------------------------------------------------------------------------|
| `irox-bits`            | ![feature_std] | ![feature_alloc] | ![internal]   | [![bits-vsn-shield]][bits-crate] [![bits-doc-shield]][bits-doc]                                             |
| `irox-build-rs`        | ![std]         | ![alloc]         | ![deps_feat]  | [![build-rs-vsn-shield]][build-rs-crate] [![build-rs-doc-shield]][build-rs-doc]                             |
| `irox-carto`           | ![feature_std] | ![alloc]         | ![internal]   | [![carto-vsn-shield]][carto-crate] [![carto-doc-shield]][carto-doc]                                         |
| `irox-cryptids`        | ![no_std]      | ![no_alloc]      | ![internal]   | [![csv-vsn-shield]][csv-crate] [![csv-doc-shield]][csv-doc]                                                 |
| `irox-csv`             | ![std]         | ![alloc]         | ![internal]   | [![csv-vsn-shield]][csv-crate] [![csv-doc-shield]][csv-doc]                                                 |
| `irox-derive-helpers`  | ![std]         | ![alloc]         | ![deps_macro] | [![derive-helpers-vsn-shield]][derive-helpers-crate] [![derive-helpers-doc-shield]][derive-helpers-doc]     |
| `irox-egui-extras`     | ![std]         | ![alloc]         | ![extdeps]    | [![egui-extras-vsn-shield]][egui-extras-crate] [![egui-extras-doc-shield]][egui-extras-doc]                 |
| `irox-enums`           | ![no_std]      | ![alloc]         | ![deps_macro] | [![enums-vsn-shield]][enums-crate] [![enums-doc-shield]][enums-doc]                                         |
| `irox-enums_derive`    | ![no_std]      | ![alloc]         | ![deps_macro] | [![enums_derive-vsn-shield]][enums_derive-crate] [![enums_derive-doc-shield]][enums_derive-doc]             |
| `irox-fixedmath`       | ![no_std]      | ![no_alloc]      | ![internal]   | [![fixedmath-vsn-shield]][fixedmath-crate] [![fixedmath-doc-shield]][fixedmath-doc]                         |
| `irox-git-tools`       | ![std]         | ![alloc]         | ![extdeps]    | [![git-tools-vsn-shield]][git-tools-crate] [![git-tools-doc-shield]][git-tools-doc]                         |
| `irox-gpx`             | ![std]         | ![alloc]         | ![extdeps]    | [![gpx-vsn-shield]][gpx-crate] [![gpx-doc-shield]][gpx-doc]                                                 |
| `irox-influxdb_v1`     | ![std]         | ![alloc]         | ![extdeps]    | [![influxdb_v1-vsn-shield]][influxdb_v1-crate] [![influxdb_v1-doc-shield]][influxdb_v1-doc]                 |
| `irox-log`             | ![feature_std] | ![alloc]         | ![deps_log]   | [![log-vsn-shield]][log-crate] [![log-doc-shield]][log-doc]                                                 |
| `irox-networking`      | ![std]         | ![alloc]         | ![extdeps]    | [![networking-vsn-shield]][networking-crate] [![networking-doc-shield]][networking-doc]                     |
| `irox-nmea0183`        | ![std]         | ![alloc]         | ![extdeps]    | [![nmea0183-vsn-shield]][nmea0183-crate] [![nmea0183-doc-shield]][nmea0183-doc]                             |
| `irox-progress`        | ![std]         | ![alloc]         | ![deps_log]   | [![progress-vsn-shield]][progress-crate] [![progress-doc-shield]][progress-doc]                             |
| `irox-raymarine-sonar` | ![std]         | ![alloc]         | ![extdeps]    | [![raymarine-sonar-vsn-shield]][raymarine-sonar-crate] [![raymarine-sonar-doc-shield]][raymarine-sonar-doc] |
| `irox-sirf`            | ![std]         | ![alloc]         | ![extdeps]    | [![sirf-vsn-shield]][sirf-crate] [![sirf-doc-shield]][sirf-doc]                                             |
| `irox-stats`           | ![no_std]      | ![alloc]         | ![internal]   | [![stats-vsn-shield]][stats-crate] [![stats-doc-shield]][stats-doc]                                         |
| `irox-structs`         | ![no_std]      | ![feature_alloc] | ![deps_macro] | [![structs-vsn-shield]][structs-crate] [![structs-doc-shield]][structs-doc]                                 |
| `irox-structs_derive`  | ![std]         | ![alloc]         | ![deps_macro] | [![structs_derive-vsn-shield]][structs_derive-crate] [![structs_derive-doc-shield]][structs_derive-doc]     |
| `irox-threading`       | ![std]         | ![alloc]         | ![deps_log]   | [![threading-vsn-shield]][threading-crate] [![threading-doc-shield]][threading-doc]                         |
| `irox-time`            | ![feature_std] | ![alloc]         | ![internal]   | [![time-vsn-shield]][time-crate] [![time-doc-shield]][time-doc]                                             |
| `irox-tools`           | ![feature_std] | ![feature_alloc] | ![internal]   | [![tools-vsn-shield]][tools-crate] [![tools-doc-shield]][tools-doc]                                         |
| `irox-types`           | ![no_std]      | ![alloc]         | ![internal]   | [![types-vsn-shield]][types-crate] [![types-doc-shield]][types-doc]                                         |
| `irox-units`           | ![feature_std] | ![feature_alloc] | ![internal]   | [![units-vsn-shield]][units-crate] [![units-doc-shield]][units-doc]                                         |
| `irox-winlocation-api` | ![std]         | ![alloc]         | ![extdeps]    | [![winloc-api-vsn-shield]][winloc-api-crate] [![winloc-api-doc-shield]][winloc-api-doc]                     |

Internal Dependency Tree
-------------------------
Supply chain vulnerabilities are a real concern these days. The following graph shows what crates can be used without
external dependencies.

*Note: the macros from the [`log`](https://crates.io/crates/log) crate are the only exception, and are used only in the
tree where indicated.

![Dependency Tree](./doc/deptree.svg)

[no_std]: https://img.shields.io/badge/no__std-yes-green "Library does not require std but may have features gated by 'std'"

[no_alloc]: https://img.shields.io/badge/no__alloc-yes-green "Library does not require alloc but may have features gated by 'alloc'"

[feature_std]: https://img.shields.io/badge/std-feature-blue "std is required by default, no_std available with default-features=false"

[feature_alloc]: https://img.shields.io/badge/alloc-feature-blue "alloc is required by default, no_std available with default-features=false"

[std]: https://img.shields.io/badge/std-required-lightgrey

[alloc]: https://img.shields.io/badge/alloc-required-lightgrey

[extdeps]: https://img.shields.io/badge/external%20deps-required-lightgrey

[deps_feat]: https://img.shields.io/badge/external%20deps-feature-blue

[deps_macro]: https://img.shields.io/badge/external%20deps-proc__macro-blue

[deps_log]: https://img.shields.io/badge/external%20deps-log-blue

[internal]: https://img.shields.io/badge/external%20deps-none-green


[`bits`]: https://github.com/spmadden/irox/blob/master/libraries/bits

[bits-vsn-shield]: https://img.shields.io/crates/v/irox-bits.svg

[bits-doc-shield]: https://docs.rs/irox-bits/badge.svg

[bits-crate]: https://crates.io/crates/irox-bits

[bits-doc]: https://docs.rs/irox-bits

[`build-rs`]: https://github.com/spmadden/irox/blob/master/libraries/build-rs

[build-rs-vsn-shield]: https://img.shields.io/crates/v/irox-build-rs.svg

[build-rs-doc-shield]: https://docs.rs/irox-build-rs/badge.svg

[build-rs-crate]: https://crates.io/crates/irox-build-rs

[build-rs-doc]: https://docs.rs/irox-build-rs


[`carto`]: https://github.com/spmadden/irox/blob/master/libraries/carto

[carto-vsn-shield]: https://img.shields.io/crates/v/irox-carto.svg

[carto-doc-shield]: https://docs.rs/irox-carto/badge.svg

[carto-crate]: https://crates.io/crates/irox-carto

[carto-doc]: https://docs.rs/irox-carto

[`csv`]: https://github.com/spmadden/irox/blob/master/libraries/csv

[csv-vsn-shield]: https://img.shields.io/crates/v/irox-csv.svg

[csv-doc-shield]: https://docs.rs/irox-csv/badge.svg

[csv-crate]: https://crates.io/crates/irox-csv

[csv-doc]: https://docs.rs/irox-csv

[`derive-helpers`]: https://github.com/spmadden/irox/blob/master/libraries/derive-helpers

[derive-helpers-vsn-shield]: https://img.shields.io/crates/v/irox-derive-helpers.svg

[derive-helpers-doc-shield]: https://docs.rs/irox-derive-helpers/badge.svg

[derive-helpers-crate]: https://crates.io/crates/irox-derive-helpers

[derive-helpers-doc]: https://docs.rs/irox-derive-helpers


[`egui-extras`]: https://github.com/spmadden/irox/blob/master/libraries/egui-extras

[egui-extras-vsn-shield]: https://img.shields.io/crates/v/irox-egui-extras.svg

[egui-extras-doc-shield]: https://docs.rs/irox-egui-extras/badge.svg

[egui-extras-crate]: https://crates.io/crates/irox-egui-extras

[egui-extras-doc]: https://docs.rs/irox-egui-extras

[`enums`]: https://github.com/spmadden/irox/blob/master/libraries/enums

[enums-vsn-shield]: https://img.shields.io/crates/v/irox-enums.svg

[enums-doc-shield]: https://docs.rs/irox-enums/badge.svg

[enums-crate]: https://crates.io/crates/irox-enums

[enums-doc]: https://docs.rs/irox-enums

[`enums_derive`]: https://github.com/spmadden/irox/blob/master/libraries/enums_derive

[enums_derive-vsn-shield]: https://img.shields.io/crates/v/irox-enums_derive.svg

[enums_derive-doc-shield]: https://docs.rs/irox-enums_derive/badge.svg

[enums_derive-crate]: https://crates.io/crates/irox-enums_derive

[enums_derive-doc]: https://docs.rs/irox-enums_derive

[`fixedmath`]: https://github.com/spmadden/irox/blob/master/libraries/fixedmath

[fixedmath-vsn-shield]: https://img.shields.io/crates/v/irox-fixedmath.svg

[fixedmath-doc-shield]: https://docs.rs/irox-fixedmath/badge.svg

[fixedmath-crate]: https://crates.io/crates/irox-fixedmath

[fixedmath-doc]: https://docs.rs/irox-fixedmath

[`git-tools`]: https://github.com/spmadden/irox/blob/master/libraries/git-tools

[git-tools-vsn-shield]: https://img.shields.io/crates/v/irox-git-tools.svg

[git-tools-doc-shield]: https://docs.rs/irox-git-tools/badge.svg

[git-tools-crate]: https://crates.io/crates/irox-git-tools

[git-tools-doc]: https://docs.rs/irox-git-tools

[`gpx`]: https://github.com/spmadden/irox/blob/master/libraries/gpx

[gpx-vsn-shield]: https://img.shields.io/crates/v/irox-gpx.svg

[gpx-doc-shield]: https://docs.rs/irox-gpx/badge.svg

[gpx-crate]: https://crates.io/crates/irox-gpx

[gpx-doc]: https://docs.rs/irox-gpx

[`influxdb_v1`]: https://github.com/spmadden/irox/blob/master/libraries/influxdb_v1

[influxdb_v1-vsn-shield]: https://img.shields.io/crates/v/irox-influxdb_v1.svg

[influxdb_v1-doc-shield]: https://docs.rs/irox-influxdb_v1/badge.svg

[influxdb_v1-crate]: https://crates.io/crates/irox-influxdb_v1

[influxdb_v1-doc]: https://docs.rs/irox-influxdb_v1

[`log`]: https://github.com/spmadden/irox/blob/master/libraries/log

[log-vsn-shield]: https://img.shields.io/crates/v/irox-log.svg

[log-doc-shield]: https://docs.rs/irox-log/badge.svg

[log-crate]: https://crates.io/crates/irox-log

[log-doc]: https://docs.rs/irox-log

[`networking`]: https://github.com/spmadden/irox/blob/master/libraries/networking

[networking-vsn-shield]: https://img.shields.io/crates/v/irox-networking.svg

[networking-doc-shield]: https://docs.rs/irox-networking/badge.svg

[networking-crate]: https://crates.io/crates/irox-networking

[networking-doc]: https://docs.rs/irox-networking

[`nmea0183`]: https://github.com/spmadden/irox/blob/master/libraries/nmea0183

[nmea0183-vsn-shield]: https://img.shields.io/crates/v/irox-nmea0183.svg

[nmea0183-doc-shield]: https://docs.rs/irox-nmea0183/badge.svg

[nmea0183-crate]: https://crates.io/crates/irox-nmea0183

[nmea0183-doc]: https://docs.rs/irox-nmea0183

[`progress`]: https://github.com/spmadden/irox/blob/master/libraries/progress

[progress-vsn-shield]: https://img.shields.io/crates/v/irox-progress.svg

[progress-doc-shield]: https://docs.rs/irox-progress/badge.svg

[progress-crate]: https://crates.io/crates/irox-progress

[progress-doc]: https://docs.rs/irox-progress

[`raymarine-sonar`]: https://github.com/spmadden/irox/blob/master/libraries/raymarine-sonar

[raymarine-sonar-vsn-shield]: https://img.shields.io/crates/v/irox-raymarine-sonar.svg

[raymarine-sonar-doc-shield]: https://docs.rs/irox-raymarine-sonar/badge.svg

[raymarine-sonar-crate]: https://crates.io/crates/irox-raymarine-sonar

[raymarine-sonar-doc]: https://docs.rs/irox-raymarine-sonar

[`sirf`]: https://github.com/spmadden/irox/blob/master/libraries/sirf

[sirf-vsn-shield]: https://img.shields.io/crates/v/irox-sirf.svg

[sirf-doc-shield]: https://docs.rs/irox-sirf/badge.svg

[sirf-crate]: https://crates.io/crates/irox-sirf

[sirf-doc]: https://docs.rs/irox-sirf

[`stats`]: https://github.com/spmadden/irox/blob/master/libraries/stats

[stats-vsn-shield]: https://img.shields.io/crates/v/irox-stats.svg

[stats-doc-shield]: https://docs.rs/irox-stats/badge.svg

[stats-crate]: https://crates.io/crates/irox-stats

[stats-doc]: https://docs.rs/irox-stats

[`structs`]: https://github.com/spmadden/irox/blob/master/libraries/structs

[structs-vsn-shield]: https://img.shields.io/crates/v/irox-structs.svg

[structs-doc-shield]: https://docs.rs/irox-structs/badge.svg

[structs-crate]: https://crates.io/crates/irox-structs

[structs-doc]: https://docs.rs/irox-structs

[`structs_derive`]: https://github.com/spmadden/irox/blob/master/libraries/threading

[structs_derive-vsn-shield]: https://img.shields.io/crates/v/irox-threading.svg

[structs_derive-doc-shield]: https://docs.rs/irox-threading/badge.svg

[structs_derive-crate]: https://crates.io/crates/irox-threading

[structs_derive-doc]: https://docs.rs/irox-threading

[`threading`]: https://github.com/spmadden/irox/blob/master/libraries/threading

[threading-vsn-shield]: https://img.shields.io/crates/v/irox-threading.svg

[threading-doc-shield]: https://docs.rs/irox-threading/badge.svg

[threading-crate]: https://crates.io/crates/irox-threading

[threading-doc]: https://docs.rs/irox-threading

[`time`]: https://github.com/spmadden/irox/blob/master/libraries/time

[time-vsn-shield]: https://img.shields.io/crates/v/irox-time.svg

[time-doc-shield]: https://docs.rs/irox-time/badge.svg

[time-crate]: https://crates.io/crates/irox-time

[time-doc]: https://docs.rs/irox-time

[`tools`]: https://github.com/spmadden/irox/blob/master/libraries/tools

[tools-vsn-shield]: https://img.shields.io/crates/v/irox-tools.svg

[tools-doc-shield]: https://docs.rs/irox-tools/badge.svg

[tools-crate]: https://crates.io/crates/irox-tools

[tools-doc]: https://docs.rs/irox-tools

[`types`]: https://github.com/spmadden/irox/blob/master/libraries/types

[types-vsn-shield]: https://img.shields.io/crates/v/irox-types.svg

[types-doc-shield]: https://docs.rs/irox-types/badge.svg

[types-crate]: https://crates.io/crates/irox-types

[types-doc]: https://docs.rs/irox-types

[`units`]: https://github.com/spmadden/irox/blob/master/libraries/units

[units-vsn-shield]: https://img.shields.io/crates/v/irox-units.svg

[units-doc-shield]: https://docs.rs/irox-units/badge.svg

[units-crate]: https://crates.io/crates/irox-units

[units-doc]: https://docs.rs/irox-units

[`winlocation-api`]: https://github.com/spmadden/irox/blob/master/interfaces/win-location-api

[winloc-api-vsn-shield]: https://img.shields.io/crates/v/irox-winlocation-api.svg

[winloc-api-doc-shield]: https://docs.rs/irox-winlocation-api/badge.svg

[winloc-api-crate]: https://crates.io/crates/irox-winlocation-api

[winloc-api-doc]: https://docs.rs/irox-winlocation-api

[`gpsd`]: https://github.com/spmadden/irox/blob/master/tools/gpsd
