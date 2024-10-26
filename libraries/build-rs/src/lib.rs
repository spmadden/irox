// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Compile-time build metadata injection inspired by `shadow-rs`
//!

#![forbid(unsafe_code)]

use crate::cargo::load_buildhost_variables;
pub use crate::error::*;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::path::Path;

mod cargo;
mod error;
#[cfg(feature = "git")]
mod git;

pub enum ErrorType {
    IOError,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VariableSource {
    Environment,
    Cargo,
    Git,
    BuildHost,
    Other(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VariableType {
    String(String),
    Bool(bool),
    Integer(i64),
}
impl Display for VariableType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::String(s) => write!(f, "{s}"),
            VariableType::Bool(b) => write!(f, "{b}"),
            VariableType::Integer(i) => write!(f, "{i}"),
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct BuildEnvironment {
    pub(crate) variables: BTreeMap<String, BuildVariable>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BuildVariable {
    pub source: VariableSource,
    pub name: String,
    pub value: VariableType,
}

impl BuildVariable {
    pub fn new_str(name: &str, value: &str, source: VariableSource) -> BuildVariable {
        BuildVariable {
            name: name.to_string(),
            value: VariableType::String(value.to_string()),
            source,
        }
    }
    pub fn new_bool(name: &str, value: bool, source: VariableSource) -> BuildVariable {
        BuildVariable {
            name: name.to_string(),
            value: VariableType::Bool(value),
            source,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    include_cargo: bool,
    include_rustc: bool,
    include_buildhost: bool,
    #[cfg(feature = "git")]
    include_git: bool,

    extra_envs: Vec<String>,
    extra_varbls: Vec<BuildVariable>,
}
impl Default for Settings {
    fn default() -> Self {
        Settings {
            include_cargo: true,
            include_rustc: true,
            include_buildhost: true,
            #[cfg(feature = "git")]
            include_git: true,
            extra_envs: vec![],
            extra_varbls: vec![],
        }
    }
}
impl Settings {
    pub fn new() -> Self {
        Settings::default()
    }
    pub fn without_cargo(self) -> Self {
        Settings {
            include_cargo: false,
            ..self
        }
    }
    pub fn without_rustc(self) -> Self {
        Settings {
            include_rustc: false,
            ..self
        }
    }
    pub fn without_buildhost(self) -> Self {
        Settings {
            include_buildhost: false,
            ..self
        }
    }
    #[cfg(feature = "git")]
    pub fn without_git(self) -> Self {
        Settings {
            include_git: false,
            ..self
        }
    }
    pub fn with_extra_envs(self, envs: &[&str]) -> Self {
        let envs: Vec<String> = envs.iter().map(|v| v.to_string()).collect();
        Settings {
            extra_envs: envs,
            ..self
        }
    }
    pub fn with_extra_varbls(self, varbls: &[BuildVariable]) -> Self {
        let varbls: Vec<BuildVariable> = Vec::from(varbls);
        Settings {
            extra_varbls: varbls,
            ..self
        }
    }
}
pub fn generate_build_environment() -> Result<BuildEnvironment, Error> {
    generate_build_environment_settings(&mut Settings::default())
}

fn add_env(name: &str, envt: &mut BuildEnvironment) {
    let value = std::env::var(name).unwrap_or_default();
    envt.variables.insert(
        name.to_string(),
        BuildVariable {
            source: VariableSource::Environment,
            name: name.to_string(),
            value: VariableType::String(value),
        },
    );
}
pub fn generate_build_environment_settings(
    settings: &mut Settings,
) -> Result<BuildEnvironment, Error> {
    let mut envt = BuildEnvironment::default();
    if settings.include_cargo {
        for varbl in cargo::CARGO_ENV_VARIABLES {
            add_env(varbl, &mut envt);
        }
    }
    if settings.include_rustc {
        for varbl in cargo::RUSTC_ENV_VARIABLES {
            add_env(varbl, &mut envt);
        }
    }
    if settings.include_buildhost {
        load_buildhost_variables(&mut envt)?;
    }

    for extra_env in &settings.extra_envs {
        add_env(extra_env.as_str(), &mut envt);
    }
    for varbl in settings.extra_varbls.drain(..) {
        envt.variables.insert(varbl.name.clone(), varbl);
    }

    #[cfg(feature = "git")]
    {
        if settings.include_git {
            if let Err(e) = git::load_git_variables(&mut envt) {
                eprintln!("Warning: Unable to load git variables: {e:#?}");
            }
        }
    }

    Ok(envt)
}

pub fn generate_module() -> Result<(), Error> {
    generate_module_settings(Settings::default())
}
pub fn generate_module_settings(mut settings: Settings) -> Result<(), Error> {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir);
    std::fs::create_dir_all(dest_path)?;
    let dest_file = dest_path.join("builders.rs");
    let mut dest_file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(dest_file)?;
    dest_file.set_len(0)?;

    let env = generate_build_environment_settings(&mut settings)?;
    write_environment(&mut dest_file, &env, &settings)
}
pub fn write_environment<T: Write>(
    mut dest_file: &mut T,
    env: &BuildEnvironment,
    settings: &Settings,
) -> Result<(), Error> {
    let mut groups = BTreeMap::new();

    for varbl in env.variables.values() {
        let name = &varbl.name;
        match &varbl.value {
            VariableType::String(val) => {
                writeln!(dest_file, r##"pub const {name}: &str = r#"{val}"#;"##)?;
            }
            VariableType::Bool(val) => {
                writeln!(dest_file, "pub const {name}: bool = {val};")?;
            }
            VariableType::Integer(val) => {
                writeln!(dest_file, "pub const {name}: i64 = {val};")?;
            }
        }
    }

    if settings.include_cargo {
        filter_and_write(
            &mut dest_file,
            env,
            &mut groups,
            "CARGO_ITEMS",
            &cargo::CARGO_ENV_VARIABLES,
        )?;
    }
    if settings.include_rustc {
        filter_and_write(
            &mut dest_file,
            env,
            &mut groups,
            "RUSTC_ITEMS",
            &cargo::RUSTC_ENV_VARIABLES,
        )?;
    }

    filter_and_write(
        &mut dest_file,
        env,
        &mut groups,
        "BUILD_HOST",
        &cargo::BUILD_HOST_VARIABLES,
    )?;

    #[cfg(feature = "git")]
    {
        if settings.include_git {
            filter_and_write(
                &mut dest_file,
                env,
                &mut groups,
                "GIT_ITEMS",
                &git::GIT_VARIABLES,
            )?;
        }
    }

    write_aggregation_block(
        &mut dest_file,
        "ALL_ITEMS",
        env.variables
            .values()
            .cloned()
            .collect::<Vec<BuildVariable>>()
            .as_slice(),
    )?;
    write_grouped_block(&mut dest_file, groups)?;

    Ok(())
}

fn filter_and_write<T: Write>(
    dest_file: &mut T,
    env: &BuildEnvironment,
    groups: &mut BTreeMap<String, Vec<BuildVariable>>,
    name: &str,
    filter: &[&str],
) -> Result<(), Error> {
    let varbls: Vec<BuildVariable> = env
        .variables
        .values()
        .filter(|v| filter.contains(&v.name.as_str()))
        .cloned()
        .collect();
    groups.insert(name.to_string(), varbls.clone());
    write_aggregation_block(dest_file, name, varbls.as_slice())
}

fn write_aggregation_block<T: Write>(
    dest_file: &mut T,
    name: &str,
    items: &[BuildVariable],
) -> Result<(), Error> {
    writeln!(
        dest_file,
        "static {name}: std::sync::OnceLock<std::collections::BTreeMap<&'static str, &'static str>> = std::sync::OnceLock::new();"
    )?;
    writeln!(dest_file, "#[allow(non_snake_case)]")?;
    writeln!(
        dest_file,
        "pub fn get_{name}() -> &'static std::collections::BTreeMap<&'static str, &'static str> {{"
    )?;
    writeln!(
        dest_file,
        "\t{name}.get_or_init(|| std::collections::BTreeMap::from(["
    )?;
    for varbl in items {
        let name = &varbl.name;
        match varbl.value {
            VariableType::String(_) => writeln!(dest_file, "\t\t(\"{name}\", {name}),")?,
            VariableType::Bool(b) => writeln!(dest_file, "\t\t(\"{name}\", \"{b}\"),")?,
            VariableType::Integer(i) => writeln!(dest_file, "\t\t(\"{name}\", \"{i}\"),")?,
        }
    }
    writeln!(dest_file, "\t]))")?;
    writeln!(dest_file, "}}")?;
    Ok(())
}

fn write_grouped_block<T: Write>(
    dest_file: &mut T,
    items: BTreeMap<String, Vec<BuildVariable>>,
) -> Result<(), Error> {
    writeln!(
        dest_file,
        "static GROUPS: std::sync::OnceLock<std::collections::BTreeMap<&'static str, std::collections::BTreeMap<&'static str, &'static str>>> = std::sync::OnceLock::new();"
    )?;
    writeln!(dest_file, "#[allow(non_snake_case)]")?;
    writeln!(
        dest_file,
        "pub fn get_GROUPS() -> &'static std::collections::BTreeMap<&'static str, std::collections::BTreeMap<&'static str, &'static str>> {{"
    )?;
    writeln!(
        dest_file,
        "\tGROUPS.get_or_init(|| std::collections::BTreeMap::from(["
    )?;
    for (k, items) in items {
        writeln!(
            dest_file,
            "\t\t(\"{k}\", std::collections::BTreeMap::from(["
        )?;
        for varbl in items {
            let name = &varbl.name;
            match varbl.value {
                VariableType::String(_) => writeln!(dest_file, "\t\t\t(\"{name}\", {name}),")?,
                VariableType::Bool(b) => writeln!(dest_file, "\t\t\t(\"{name}\", \"{b}\"),")?,
                VariableType::Integer(i) => writeln!(dest_file, "\t\t\t(\"{name}\", \"{i}\"),")?,
            }
        }
        writeln!(dest_file, "\t\t])),")?;
    }
    writeln!(dest_file, "\t]))")?;
    writeln!(dest_file, "}}")?;
    Ok(())
}
