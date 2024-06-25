// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Serde objects for reading & writing the output of `cargo metadata`
//!

#![forbid(unsafe_code)]

use error::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

pub use crate::lockfile::*;

mod error;
mod lockfile;

/// Cargo metadata
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Array of all packages in the workspace.
    //  It also includes all feature-enabled dependencies unless --no-deps is used.
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub packages: Vec<Package>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub workspace_members: Vec<String>,
    pub workspace_default_members: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub resolve: Option<Resolve>,
    pub target_directory: Option<String>,
    pub version: u32,
    pub workspace_root: Option<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub metadata: BTreeMap<String, Value>,
}
impl Metadata {
    pub fn read_from_path<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let child = std::process::Command::new("cargo")
            .current_dir(path)
            .args(["metadata", "--all-features", "--format-version=1"])
            .output()?;
        Ok(serde_json::from_slice::<Self>(&child.stdout)?)
    }
    pub fn read_current_dir() -> Result<Self, Error> {
        let cwd = std::env::current_dir()?;
        Self::read_from_path(cwd)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Package {
    /// The name of the package.
    pub name: String,
    /// The version of the package.
    pub version: String,
    pub id: Option<String>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub dependencies: Vec<Dependency>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub targets: Vec<Target>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub features: BTreeMap<String, Vec<String>>,
    pub manifest_path: Option<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub metadata: BTreeMap<String, Value>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub publish: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub authors: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub categories: Vec<String>,
    pub default_run: Option<String>,
    pub rust_version: Option<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub keywords: Vec<String>,
    pub readme: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub edition: Option<String>,
    pub links: Option<String>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: Option<String>,
    pub source: Option<String>,
    pub req: Option<String>,
    pub kind: Option<String>,
    pub rename: Option<String>,
    pub optional: Option<bool>,
    pub uses_default_features: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub features: Vec<String>,
    pub target: Option<String>,
    pub path: Option<String>,
    pub registry: Option<String>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Resolve {
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub nodes: Vec<Node>,
    pub root: Option<String>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub kind: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub crate_types: Vec<String>,
    pub name: Option<String>,
    pub src_path: Option<String>,
    pub edition: Option<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub required_features: Vec<String>,
    pub doc: Option<bool>,
    pub doctest: Option<bool>,
    pub test: Option<bool>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: Option<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub dependencies: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub deps: Vec<Dep>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub features: Vec<String>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Dep {
    pub name: Option<String>,
    pub pkg: Option<String>,
    #[serde(default, deserialize_with = "deserialize_if_null")]
    pub dep_kinds: Vec<DepKind>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DepKind {
    pub kind: Option<String>,
    pub target: Option<String>,
}

pub fn deserialize_if_null<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let val = Option::<T>::deserialize(deserializer)?;
    Ok(val.unwrap_or_default())
}
#[cfg(test)]
mod test {
    use crate::error::Error;

    #[test]
    pub fn test() -> Result<(), Error> {
        let out = super::Metadata::read_current_dir()?;
        println!("{out:#?}");
        Ok(())
    }
}
