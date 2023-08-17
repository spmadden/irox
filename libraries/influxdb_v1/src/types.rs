// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::collections::BTreeMap;

use crate::error::{self, Error, ErrorType};

fn get_or_error(key: &'static str, map: &mut BTreeMap<String, String>) -> Result<String, Error> {
    let Some(value) = map.remove(key) else {
        return Error::err(ErrorType::MissingKeyError(key.to_string()), "Missing Key");
    };
    Ok(value)
}

#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    pub name: String,

    // TODO: replace with actual duration class
    pub duration: String,
    pub shard_group_duration: String,

    pub replica_n: String,
    pub default: String,

    pub other_values: BTreeMap<String, String>,
}

impl TryFrom<BTreeMap<String, String>> for RetentionPolicy {
    type Error = error::Error;

    fn try_from(map: BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let mut map = map.clone();
        let name = get_or_error("name", &mut map)?;
        let duration = get_or_error("duration", &mut map)?;
        let shard_group_duration = get_or_error("shardGroupDuration", &mut map)?;
        let replica_n = get_or_error("replicaN", &mut map)?;
        let default = get_or_error("default", &mut map)?;
        Ok(RetentionPolicy {
            name,
            duration,
            shard_group_duration,
            replica_n,
            default,
            other_values: map,
        })
    }
}
