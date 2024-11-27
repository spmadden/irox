// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::geo::ellipse::Ellipse;
use irox_units::units::length::Length;

pub const CLARKE_1866_SEMI_MAJOR_LENGTH: Length = Length::new_meters(6378206.4);
pub const CLARKE_1866_SEMI_MINOR_LENGTH: Length = Length::new_meters(6356583.8);
pub const CLARKE_1866_INV_FLATTENING: f64 = 294.978689214;
pub const CLARKE_1866_PARAMS: Ellipse = Ellipse::named(
    "CLARKE_1866",
    CLARKE_1866_SEMI_MAJOR_LENGTH,
    CLARKE_1866_INV_FLATTENING,
);
