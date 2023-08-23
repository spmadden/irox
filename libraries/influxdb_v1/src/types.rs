// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::collections::BTreeMap;

use irox_tools::types::{NamedPrimitive, Primitives};

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

#[derive(Debug, Clone, Default)]
pub struct MeasurementDescriptor {
    pub(crate) name: String,
    pub(crate) fields: Vec<NamedPrimitive>,
    pub(crate) tags: Vec<String>,
}

impl MeasurementDescriptor {
    pub(crate) fn new(name: String) -> MeasurementDescriptor {
        MeasurementDescriptor {
            name,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub fn fields(&self) -> &Vec<NamedPrimitive> {
        &self.fields
    }

    #[must_use]
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub(crate) fn merge_field_key_map(
        &mut self,
        map: &BTreeMap<String, String>,
    ) -> Result<(), Error> {
        let Some(name) = map.get("name") else {
            return Error::err_str(ErrorType::MissingKeyError("name".to_string()), "Missing key name".to_string());
        };
        if !name.eq(&self.name) {
            return Error::err_str(
                ErrorType::NameKeyMismatch,
                format!("Name mismatch, found {name} expected {}", self.name),
            );
        }
        let Some(field_key) = map.get("fieldKey") else {
            return Error::err_str(ErrorType::MissingKeyError("fieldKey".to_string()), "Missing key fieldKey".to_string());
        };
        let Some(field_type) = map.get("fieldType") else {
            return Error::err_str(ErrorType::MissingKeyError("fieldType".to_string()) , "Missing key fieldType".to_string());
        };
        let field = match field_type.as_str() {
            "float" => NamedPrimitive::new(field_key.to_string(), Primitives::f64),
            "integer" | "timestamp" => NamedPrimitive::new(field_key.to_string(), Primitives::i64),
            "string" => NamedPrimitive::new(field_key.to_string(), Primitives::str),
            missing => {
                return Error::err_str(
                    ErrorType::UnsupportedType(missing.to_string()),
                    format!("Unsupported type returned {missing}"),
                );
            }
        };
        self.fields.push(field);
        Ok(())
    }

    pub(crate) fn merge_tag_key_map(
        &mut self,
        map: &BTreeMap<String, String>,
    ) -> Result<(), Error> {
        let Some(name) = map.get("name") else {
            return Error::err_str(ErrorType::MissingKeyError("name".to_string()), "Missing key name".to_string());
        };
        if !name.eq(&self.name) {
            return Error::err_str(
                ErrorType::NameKeyMismatch,
                format!("Name mismatch, found {name} expected {}", self.name),
            );
        }
        let Some(tag_key) = map.get("tagKey") else {
            return Error::err_str(ErrorType::MissingKeyError("tagKey".to_string()), "Missing key tagKey".to_string());
        };
        self.tags.push(tag_key.to_string());
        Ok(())
    }
}
