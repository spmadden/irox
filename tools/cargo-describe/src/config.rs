// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Contains configuration struct for the describe cmd

use cargo::core::Workspace;
use std::path::PathBuf;

use crate::fields::{FieldInfo, Fields};
use clap::{Parser, ValueEnum};
use irox_enums::EnumIterItem;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    /// Pretty human text
    #[default]
    HumanText,

    /// Comma separated values
    CSV,

    /// Markdown Table
    MDTable,
}

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub struct Config {
    /// Path to a non-standard 'Cargo.toml' package manifest, if not in current directory
    #[arg(short, long)]
    pub manifest: Option<String>,

    /// The output format of this tool
    #[arg(short, long, value_enum, default_value_t)]
    pub output_format: OutputFormat,

    /// An optional list of manifest fields to display
    #[arg(short, long, value_enum, default_values = ["name", "version"])]
    pub fields: Vec<Fields>,

    /// An optional list of crate packages members to display in this workspace
    #[arg(short, long, required = false)]
    pub crates: Vec<String>,
}

impl Config {
    pub fn get_manifest_path(&self) -> Result<PathBuf, std::io::Error> {
        if let Some(manifest) = &self.manifest {
            let buf = PathBuf::from(manifest);
            if buf.is_relative() {
                let mut cwd = std::env::current_dir()?;
                cwd.push(buf);
                return Ok(cwd);
            }
            Ok(buf)
        } else {
            let mut cwd = std::env::current_dir()?;
            cwd.push("Cargo.toml");
            Ok(cwd)
        }
    }

    pub fn get_fields(&self) -> Vec<Fields> {
        if self.fields.contains(&Fields::All) {
            Fields::iter_items().filter(|v| *v != Fields::All).collect()
        } else {
            self.fields.clone()
        }
    }

    pub fn get_field_info(&self) -> Vec<FieldInfo> {
        self.get_fields()
            .iter()
            .map(|f| FieldInfo {
                field: *f,
                value: None,
            })
            .collect()
    }

    pub fn get_context(&self, workspace: &Workspace) -> Context {
        if self.crates.is_empty() {
            let krates = workspace
                .members()
                .map(|p| Krate {
                    crate_name: p.name().to_string(),
                    fields: self.get_field_info(),
                })
                .collect();
            Context { crates: krates }
        } else {
            let krates = self
                .crates
                .iter()
                .map(|s| Krate {
                    crate_name: s.clone(),
                    fields: self.get_field_info(),
                })
                .collect();
            Context { crates: krates }
        }
    }
}

pub struct Krate {
    pub crate_name: String,
    pub fields: Vec<FieldInfo>,
}

pub struct Context {
    pub crates: Vec<Krate>,
}
