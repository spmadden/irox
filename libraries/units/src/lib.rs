// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! *The little Units Library that could*
//!
//! Module Structure:
//! ------------------
//!
//! * [`bounds`] - Bounding Boxes and Range Checks
//! * [`shapes`] - Ways to define and describe shapes
//!     * [`shapes::circular`] - `CircularAspect` enum and `CircularDimension` struct, describes a circle by radius or
//!         diameter with appropriate length units.
//!     * [`shapes::elliptical`] - `Ellipse` struct, describes an ellipse using two `CircularDimension` axes and an optional
//!         `CompassDirection` orientation of the first axis
//! * [`time`] - Date and Time structs and enums
//!     * [`time`] - Contains the base `Time` struct, describing a standard `Hours/minutes/seconds` framework.
//!     * [`time::datetime`] - Contains `UTCDateTime` and `LocalDateTime` structs, describing a [`time::gregorian::Date`] with
//!         a [`time::Time`], and an optional [`time::duration::Duration`] offset from UTC
//!     * [`time::duration`]- Contains `Duration`, describing a fixed amount of seconds / minutes / hours / milliseconds / etc
//!     * [`time::epoch`] - Contains `Epoch`, `UnixEpoch`, `GPSEpoch`, and others, providing the datum anchor for timestamps
//!         `UnixTimestamp`, `GPSTimestamp`, etc.
//!     * [`time::gregorian`] - Contains `Date` and `Month`, that describe a gregorian calendar date.
//!     * [`time::julian`] - Contains `JulianDate` and it's associated epochs.
//!     * [`time::format`] - Contains `DateFormat` and `DateFormatParser` to tranlate dates to and from strings.
//!       * [`time::format::iso8601`] - ISO8601 Implementations of `DateFormat` and `DateFormatParser`
//! * [`units`] - Physical Quantities
//!   * [`units::angle`] -  Angle Types, `Angle`, `AngleUnits` for `Degrees` and `Radians`
//!   * [`units::compass`] - Compass Types, `Compass`, and the absolute types: `Heading`, `Track`, `Bearing`, `Course`,
//!       `Azimuth`, `CompassOffest`, and the relative type `RelativeBearing`
//!   * [`units::datasize`] - Computer Data Sizes, `DataSize` representing `Bytes`, `Kilobytes`, etc
//!   * [`units::length`] - The SI `Length` quantity, representing `Meters`, `Feet`, etc
//!   * [`units::speed`] - The SI `Speed` quantity, representing `MetersPerSecond`, `Knots`, etc
//!   * [`units::temperature`] - The SI `Temperature` quantity, representing `Celsius`, `Kelvin`, etc

#![forbid(unsafe_code)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]

pub mod bounds;
pub mod shapes;
#[macro_use]
pub mod units;
