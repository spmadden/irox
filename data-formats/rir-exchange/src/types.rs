// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use crate::RegionalRegistry;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Version {
    version: String,
    registry: RegionalRegistry,
    serial: String,
    record_count: u64,
    // start_date:
}
