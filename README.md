Iron Oxide (IROX) Libraries
=============================
A collection of (hopefully) useful crates written in Rust.  

[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/spmadden/irox/blob/master/LICENSE)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/spmadden/irox/blob/master/LICENSE-APACHE)
![Maintenance](https://img.shields.io/maintenance/yes/2023)
![Libraries.io dependency status for GitHub repo](https://img.shields.io/librariesio/github/spmadden/irox)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/spmadden/irox/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/irox)](https://crates.io/crates/irox/)
[![docs.rs](https://img.shields.io/docsrs/irox/latest)](https://docs.rs/irox/latest/irox/)

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)
[![Static Badge](https://img.shields.io/badge/semver-2.0-blue)](https://semver.org/spec/v2.0.0.html)
[![Static Badge](https://img.shields.io/badge/conventional--commits-1.0-pink)](https://www.conventionalcommits.org/en/v1.0.0/)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Current Modules & Organization:
-----------------
 * [`data-formats`](./data-formats) - Specific data format encoders & decoders
   * [`csv`](./data-formats/csv) - Comma Separated Values encoder/decoder
   * [`nmea0183`](./data-formats/nmea0183) - GPS NMEA-0183 encoder/decoder
   * [`sirf`](./data-formats/sirf) - GPS Binary SiRF encoder/decoder
 * [`interfaces`](./interfaces) - Interfaces, Transports, and APIs for external tools
   * [`influxdb_v1`](./interfaces/influxdb_v1) - InfluxDB v1 API Client
 * [`irox`](./irox) - Aggregator module
 * [`libraries`](./libraries) - Rust 'library' crates, usually without binaries
   * [`carto`](./libraries/carto) - Cartographic & Geospatial tools
   * [`egui_irox_extras`](./libraries/egui_extras) - Extra stuff for the wonderful [`egui`](https://github.com/emilk/egui) crate
   * [`enums`](./libraries/enums) - Traits for better Enumerated Types
   * [`enums_derive`](./libraries/enums_derive) - Derivable impls of the traits in irox-enums
   * [`influxdb_v1`](./libraries/influxdb_v1) - InfluxDBv1 API Client
   * [`network`](./libraries/network) - Networking tools
   * [`stats`](./libraries/stats) - Mathematics & Statistics tools
   * [`tools`](./libraries/tools) - Dumping ground for stuff that should have been in `std`
   * [`types`](./libraries/types) - Enums and structs to describe Rust's basic type system
   * [`units`](./libraries/units) - Physical Units, Quantities, & Reference Frames
 * [`tools`](./tools) - Rust 'binary' crates, mostly just binaries that depend on crates in `libraries`
