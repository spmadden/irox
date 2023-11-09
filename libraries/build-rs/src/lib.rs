// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]

mod cargo;
mod git;

use std::collections::BTreeMap;

pub enum ErrorType {
    IOError
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VariableSource {
    Environment,
    Cargo,
    Git,
    Other(String)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VariableType {
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BuildEnvironment {
    pub(crate) variables: BTreeMap<String, BuildVariable>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BuildVariable {
    pub source: VariableSource,
    pub name: String,
    pub value: VariableType,
}