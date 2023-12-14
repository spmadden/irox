// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]

use cargo::core::{Package, Workspace};
use clap::Parser;
use irox_csv::CSVError;
use irox_enums::EnumName;
use log::error;
use std::collections::BTreeMap;

use crate::config::{Config, Context, OutputFormat};
use crate::fields::Fields;
use crate::git::do_git_log;

mod config;
mod fields;
mod git;

pub fn main() {
    irox_log::init_console_from_env("RUST_LOG");

    let config = Config::parse();
    let path = match config.get_manifest_path() {
        Ok(p) => p,
        Err(e) => {
            error!("Error getting manifest path: {e}");
            return;
        }
    };

    let cargo_conf = match cargo::util::config::Config::default() {
        Ok(c) => c,
        Err(e) => {
            error!("Error loading cargo config: {e}");
            return;
        }
    };

    let wksp = match Workspace::new(path.as_path(), &cargo_conf) {
        Ok(w) => w,
        Err(e) => {
            error!("Error loading workspace from path {path:?}: {e}");
            return;
        }
    };

    let mut context = config.get_context(&wksp);

    let mut gitctx = Vec::new();
    if config.fields.contains(&Fields::GitVersion) {
        gitctx = do_git_log(&wksp);
    }

    for krate in &mut context.crates {
        for mem in wksp.members() {
            if mem.name() != krate.crate_name.as_str() {
                continue;
            }
            let gitmeminfo = gitctx.iter().find(|v| mem.name() == v.krate.as_str());
            for field in &mut krate.fields {
                field.value = Some(match field.field {
                    Fields::Name => mem.name().to_string(),
                    Fields::Version => mem.version().to_string(),
                    Fields::GitVersion => gitmeminfo
                        .map(|v| v.result.clone().unwrap_or_default())
                        .unwrap_or_default(),
                    Fields::ModuleRelativePath => mem.manifest_path().display().to_string(),
                    Fields::ModuleAbsolutePath => mem.manifest_path().display().to_string(),
                    Fields::ModuleRelativeManifestPath => mem.manifest_path().display().to_string(),
                    Fields::ModuleAbsoluteManifestPath => mem.manifest_path().display().to_string(),
                });
            }
        }
    }

    match &config.output_format {
        OutputFormat::HumanText => print_human_text(&context),
        OutputFormat::CSV => {}
        OutputFormat::MDTable => {}
    }

    return;
}

pub fn do_manifest_log_for_member(member: &Package) -> String {
    let name = member.name();
    let version = member.version();
    format!("{name}-{version}")
}

pub fn print_human_text(context: &Context) {
    for krate in &context.crates {
        println!("{}", krate.crate_name);
        for field in &krate.fields {
            if field.field == Fields::Name {
                continue;
            }
            println!(
                "\t{} : {}",
                field.field,
                field.value.clone().unwrap_or_default()
            );
        }
    }
}

pub fn print_csv(fields: &Vec<Fields>, context: &Context) -> Result<(), CSVError> {
    let headers: Vec<&str> = fields.iter().map(|v| v.name()).collect();
    let mut writer = irox_csv::CSVWriter::new(std::io::stdout()).with_column_names(&headers);
    writer.write_header()?;
    for krate in &context.crates {
        let mut map = BTreeMap::new();
        for field in &krate.fields {
            map.insert(field.field.name(), field.value.clone().unwrap_or_default());
        }
        writer.write_fields(&map)?;
    }

    Ok(())
}
