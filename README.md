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
    * [`raymarine_sonar`](https://github.com/spmadden/irox/blob/master/data-formats/raymarine_sonar) - Raymarine SDF
      Sonar Converter
    * [`sirf`](https://github.com/spmadden/irox/blob/master/data-formats/sirf) - GPS Binary SiRF encoder/decoder
* [`interfaces`](https://github.com/spmadden/irox/blob/master/interfaces) - Interfaces, Transports, and APIs for
  external tools
    * [`influxdb_v1`](https://github.com/spmadden/irox/blob/master/interfaces/influxdb_v1) - InfluxDB v1 API Client
    * [`win-location-api`](https://github.com/spmadden/irox/blob/master/interfaces/win-location-api) - Interact with
      the [`Windows.Devices.Geolocation`](https://learn.microsoft.com/en-us/uwp/api/windows.devices.geolocation) API in
      idiomatic Rust
* [`irox`](https://github.com/spmadden/irox/blob/master/irox) - Aggregator module
* [`libraries`](https://github.com/spmadden/irox/blob/master/libraries) - Rust 'library' crates, usually without
  binaries
    * [`carto`](https://github.com/spmadden/irox/blob/master/libraries/carto) - Cartographic & Geospatial tools
    * [`egui_irox_extras`](https://github.com/spmadden/irox/blob/master/libraries/egui_extras) - Extra stuff for the
      wonderful [`egui`](https://github.com/emilk/egui) crate
    * [`enums`](https://github.com/spmadden/irox/blob/master/libraries/enums) - Traits for better Enumerated Types
    * [`enums_derive`](https://github.com/spmadden/irox/blob/master/libraries/enums_derive) - Derivable impls of the
      traits in irox-enums
    * [`network`](https://github.com/spmadden/irox/blob/master/libraries/network) - Networking tools
    * [`progress`](https://github.com/spmadden/irox/blob/master/libraries/progress) - An ecosystem for displaying
      progress, either in a UI or on the terminal.
    * [`stats`](https://github.com/spmadden/irox/blob/master/libraries/stats) - Mathematics & Statistics tools
    * [`structs`](https://github.com/spmadden/irox/blob/master/libraries/structs) - Traits for Struct Types - linearly
      serialized big endian bytes
    * [`structs_derive`](https://github.com/spmadden/irox/blob/master/libraries/structs_derive) - Derivable impls of the
      traits in irox-structs
    * [`threading`](https://github.com/spmadden/irox/blob/master/libraries/threading) - Blocking and Asynchronous
      Threading Tools
    * [`time`](https://github.com/spmadden/irox/blob/master/libraries/time) - A date & time library that aims for ease
      of use based on the Proleptic Gregorian Calendar.
    * [`tools`](https://github.com/spmadden/irox/blob/master/libraries/tools) - Dumping ground for stuff that should
      have been in `std`
    * [`types`](https://github.com/spmadden/irox/blob/master/libraries/types) - Enums and structs to describe Rust's
      basic type system
    * [`units`](https://github.com/spmadden/irox/blob/master/libraries/units) - Physical Units, Quantities, & Reference
      Frames
* [`tools`](https://github.com/spmadden/irox/blob/master/tools) - Rust 'binary' crates, mostly just binaries that depend
  on crates in `libraries`
    * [`gpsd`](https://github.com/spmadden/irox/blob/master/tools/gpsd) - Implementation of GPSd in Rust, with support
      for windows targets!

Version Status
------------------

| Crate                  | Status                                                                                                                                                                                                                           |
|------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `irox-carto`           | [![Crates.io](https://img.shields.io/crates/v/irox-carto.svg)](https://crates.io/crates/irox-carto) [![Documentation](https://docs.rs/irox-carto/badge.svg)](https://docs.rs/irox-carto)                                         |
| `irox-csv`             | [![Crates.io](https://img.shields.io/crates/v/irox-csv.svg)](https://crates.io/crates/irox-csv) [![Documentation](https://docs.rs/irox-csv/badge.svg)](https://docs.rs/irox-csv)                                                 |
| `irox-enums`           | [![Crates.io](https://img.shields.io/crates/v/irox-enums.svg)](https://crates.io/crates/irox-enums) [![Documentation](https://docs.rs/irox-enums/badge.svg)](https://docs.rs/irox-enums)                                         |
| `irox-enums_derive`    | [![Crates.io](https://img.shields.io/crates/v/irox-enums_derive.svg)](https://crates.io/crates/irox-enums_derive) [![Documentation](https://docs.rs/irox-enums_derive/badge.svg)](https://docs.rs/irox-enums_derive)             |
| `irox-gpx`             | [![Crates.io](https://img.shields.io/crates/v/irox-gpx.svg)](https://crates.io/crates/irox-gpx) [![Documentation](https://docs.rs/irox-gpx/badge.svg)](https://docs.rs/irox-gpx)                                                 |
| `irox-influxdb_v1`     | [![Crates.io](https://img.shields.io/crates/v/irox-influxdb_v1.svg)](https://crates.io/crates/irox-influxdb_v1) [![Documentation](https://docs.rs/irox-influxdb_v1/badge.svg)](https://docs.rs/irox-influxdb_v1)                 |
| `irox-networking`      | [![Crates.io](https://img.shields.io/crates/v/irox-networking.svg)](https://crates.io/crates/irox-networking) [![Documentation](https://docs.rs/irox-networking/badge.svg)](https://docs.rs/irox-networking)                     |
| `irox-progress`        | [![Crates.io](https://img.shields.io/crates/v/irox-progress.svg)](https://crates.io/crates/irox-progress) [![Documentation](https://docs.rs/irox-progress/badge.svg)](https://docs.rs/irox-progress)                             |
| `irox-nmea0183`        | [![Crates.io](https://img.shields.io/crates/v/irox-nmea0183.svg)](https://crates.io/crates/irox-nmea0183) [![Documentation](https://docs.rs/irox-nmea0183/badge.svg)](https://docs.rs/irox-nmea0183)                             |
| `irox-raymarine-sonar` | [![Crates.io](https://img.shields.io/crates/v/irox-raymarine-sonar.svg)](https://crates.io/crates/irox-raymarine-sonar) [![Documentation](https://docs.rs/irox-raymarine-sonar/badge.svg)](https://docs.rs/irox-raymarine-sonar) |
| `irox-sirf`            | [![Crates.io](https://img.shields.io/crates/v/irox-sirf.svg)](https://crates.io/crates/irox-sirf) [![Documentation](https://docs.rs/irox-sirf/badge.svg)](https://docs.rs/irox-sirf)                                             |
| `irox-stats`           | [![Crates.io](https://img.shields.io/crates/v/irox-stats.svg)](https://crates.io/crates/irox-stats) [![Documentation](https://docs.rs/irox-stats/badge.svg)](https://docs.rs/irox-stats)                                         |
| `irox-structs`         | [![Crates.io](https://img.shields.io/crates/v/irox-structs.svg)](https://crates.io/crates/irox-structs) [![Documentation](https://docs.rs/irox-structs/badge.svg)](https://docs.rs/irox-structs)                                 |
| `irox-structs_derive`  | [![Crates.io](https://img.shields.io/crates/v/irox-structs_derive.svg)](https://crates.io/crates/irox-structs_derive) [![Documentation](https://docs.rs/irox-structs_derive/badge.svg)](https://docs.rs/irox-structs_derive)     |
| `irox-threading`       | [![Crates.io](https://img.shields.io/crates/v/irox-threading.svg)](https://crates.io/crates/irox-threading) [![Documentation](https://docs.rs/irox-threading/badge.svg)](https://docs.rs/irox-threading)                         |
| `irox-time`            | [![Crates.io](https://img.shields.io/crates/v/irox-time.svg)](https://crates.io/crates/irox-time) [![Documentation](https://docs.rs/irox-time/badge.svg)](https://docs.rs/irox-time)                                             |
| `irox-tools`           | [![Crates.io](https://img.shields.io/crates/v/irox-tools.svg)](https://crates.io/crates/irox-tools) [![Documentation](https://docs.rs/irox-tools/badge.svg)](https://docs.rs/irox-tools)                                         |
| `irox-units`           | [![Crates.io](https://img.shields.io/crates/v/irox-units.svg)](https://crates.io/crates/irox-units) [![Documentation](https://docs.rs/irox-units/badge.svg)](https://docs.rs/irox-units)                                         |
| `irox-winlocation-api` | [![Crates.io](https://img.shields.io/crates/v/irox-winlocation-api.svg)](https://crates.io/crates/irox-winlocation-api) [![Documentation](https://docs.rs/irox-winlocation-api/badge.svg)](https://docs.rs/irox-winlocation-api) |

