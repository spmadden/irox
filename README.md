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

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](https://github.com/spmadden/irox/blob/master/CODE_OF_CONDUCT.md)
[![Static Badge](https://img.shields.io/badge/semver-2.0-blue)](https://semver.org/spec/v2.0.0.html)
[![Static Badge](https://img.shields.io/badge/conventional--commits-1.0-pink)](https://www.conventionalcommits.org/en/v1.0.0/)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Current Modules & Organization:
-----------------
 * [`data-formats`](https://github.com/spmadden/irox/blob/master/data-formats) - Specific data format encoders & decoders
   * [`csv`](https://github.com/spmadden/irox/blob/master/data-formats/csv) - Comma Separated Values encoder/decoder
   * [`gpx`](https://github.com/spmadden/irox/blob/master/data-formats/gpx) - GPX GPS file format reader/writer
   * [`nmea0183`](https://github.com/spmadden/irox/blob/master/data-formats/nmea0183) - GPS NMEA-0183 encoder/decoder
   * [`raymarine_sonar`](https://github.com/spmadden/irox/blob/master/data-formats/raymarine_sonar) - Raymarine SDF Sonar Converter 
   * [`sirf`](https://github.com/spmadden/irox/blob/master/data-formats/sirf) - GPS Binary SiRF encoder/decoder
 * [`interfaces`](https://github.com/spmadden/irox/blob/master/interfaces) - Interfaces, Transports, and APIs for external tools
   * [`influxdb_v1`](https://github.com/spmadden/irox/blob/master/interfaces/influxdb_v1) - InfluxDB v1 API Client
   * [`win-location-api`](https://github.com/spmadden/irox/blob/master/interfaces/win-location-api) - Interact with the [`Windows.Devices.Geolocation`](https://learn.microsoft.com/en-us/uwp/api/windows.devices.geolocation) API in idiomatic Rust
 * [`irox`](https://github.com/spmadden/irox/blob/master/irox) - Aggregator module
 * [`libraries`](https://github.com/spmadden/irox/blob/master/libraries) - Rust 'library' crates, usually without binaries
   * [`carto`](https://github.com/spmadden/irox/blob/master/libraries/carto) - Cartographic & Geospatial tools
   * [`egui_irox_extras`](https://github.com/spmadden/irox/blob/master/libraries/egui_extras) - Extra stuff for the wonderful [`egui`](https://github.com/emilk/egui) crate
   * [`enums`](https://github.com/spmadden/irox/blob/master/libraries/enums) - Traits for better Enumerated Types
   * [`enums_derive`](https://github.com/spmadden/irox/blob/master/libraries/enums_derive) - Derivable impls of the traits in irox-enums
   * [`jaxb`](https://github.com/spmadden/irox/blob/master/libraries/enums_derive) - XML Toolkit fashioned after, and inspired by Java's JAXB
   * [`network`](https://github.com/spmadden/irox/blob/master/libraries/network) - Networking tools
   * [`stats`](https://github.com/spmadden/irox/blob/master/libraries/stats) - Mathematics & Statistics tools
   * [`structs`](https://github.com/spmadden/irox/blob/master/libraries/structs) - Traits for Struct Types - linearly serialized big endian bytes
   * [`structs_derive`](https://github.com/spmadden/irox/blob/master/libraries/structs_derive) - Derivable impls of the traits in irox-structs
   * [`tools`](https://github.com/spmadden/irox/blob/master/libraries/tools) - Dumping ground for stuff that should have been in `std`
   * [`types`](https://github.com/spmadden/irox/blob/master/libraries/types) - Enums and structs to describe Rust's basic type system
   * [`units`](https://github.com/spmadden/irox/blob/master/libraries/units) - Physical Units, Quantities, & Reference Frames
 * [`tools`](https://github.com/spmadden/irox/blob/master/tools) - Rust 'binary' crates, mostly just binaries that depend on crates in `libraries`
   * [`gpsd`](https://github.com/spmadden/irox/blob/master/tools/gpsd) - Implementation of GPSd in Rust, with support for windows targets!