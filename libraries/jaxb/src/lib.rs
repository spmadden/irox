// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

pub mod error;
pub mod schema;
pub mod types;

pub struct DataModel {
    pub structs: Vec<DataStruct>,
}

pub struct DataStruct {}
