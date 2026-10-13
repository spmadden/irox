// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use irox_types::NamedVariable;

#[derive(Debug, Clone, PartialEq)]
pub struct DynamicTable {
    pub name: String,
    pub fields: NamedVariable,
}
