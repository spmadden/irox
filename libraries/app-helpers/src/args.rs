// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use irox_types::PrimitiveField;

pub enum ArgValueType {
    Primitive(PrimitiveField),
}

pub struct Arg {
    pub name: &'static str,
}

pub struct ArgParser;
