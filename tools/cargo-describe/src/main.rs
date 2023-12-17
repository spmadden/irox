// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]
#![allow(clippy::print_stdout)]
#![allow(clippy::upper_case_acronyms)]

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
    let fields = config.get_fields();

    let mut gitctx = Vec::new();
    if fields.contains(&Fields::GitVersion) {
        gitctx = do_git_log(&wksp);
    }

    let wksp_root = format!("{}{}", wksp.root().display(), std::path::MAIN_SEPARATOR_STR);
    for krate in &mut context.crates {
        for mem in wksp.members() {
            if mem.name() != krate.crate_name.as_str() {
                continue;
            }
            let gitmeminfo = gitctx.iter().find(|v| mem.name() == v.krate.as_str());
            let root = mem
                .root()
                .display()
                .to_string()
                .strip_prefix(&wksp_root)
                .map(String::from)
                .unwrap_or_default();
            let manifestpath = mem.manifest_path().display().to_string();
            let modpath = manifestpath
                .strip_suffix("Cargo.toml")
                .map(String::from)
                .unwrap_or_default();
            let relmanifest = manifestpath
                .strip_prefix(&wksp_root)
                .map(String::from)
                .unwrap_or_default();
            // let relmod = relmanifest.strip_suffix("Cargo.toml").map(String::from).unwrap_or_default();
            for field in &mut krate.fields {
                field.value = Some(match field.field {
                    Fields::Name => mem.name().to_string(),
                    Fields::Version => mem.version().to_string(),
                    Fields::GitVersion => gitmeminfo
                        .map(|v| v.result.clone().unwrap_or_default())
                        .unwrap_or_default(),
                    Fields::ModuleRelativePath => root.clone(),
                    Fields::ModuleAbsolutePath => modpath.clone(),
                    Fields::ModuleRelativeManifestPath => relmanifest.clone(),
                    Fields::ModuleAbsoluteManifestPath => manifestpath.clone(),
                    Fields::All => String::new(),
                });
            }
        }
    }

    if config.dbg_versions {
        print_dbg(&context);
        return;
    }

    match &config.output_format {
        OutputFormat::HumanText => print_human_text(&context),
        OutputFormat::CSV => {
            let _ = print_csv(&fields, &context);
        }
        OutputFormat::MDTable => print_md(&fields, &context),
        OutputFormat::Plain => print_plain(&context),
    }
}

pub fn do_manifest_log_for_member(member: &Package) -> String {
    let name = member.name();
    let version = member.version();
    format!("{name}-{version}")
}

pub fn print_dbg(context: &Context) {
    for krate in &context.crates {
        let val = krate
            .fields
            .iter()
            .map(|v| v.value.clone().unwrap_or_default())
            .collect::<Vec<String>>()
            .join(":");
        println!("{val}");
    }
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

pub fn print_csv(fields: &[Fields], context: &Context) -> Result<(), CSVError> {
    let headers: Vec<&str> = fields.iter().map(EnumName::name).collect();
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

pub fn print_md(fields: &[Fields], context: &Context) {
    let mut max_field_lens: BTreeMap<Fields, usize> = BTreeMap::new();
    for krate in &context.crates {
        for field in &krate.fields {
            let flen = field.value.as_ref().map(String::len).unwrap_or(0);
            let len = field.field.name().len().max(flen);
            let val = max_field_lens.entry(field.field).or_default();
            *val = len.max(*val);
        }
    }
    let mut values = Vec::new();
    let mut widths = Vec::new();
    for field in fields {
        let name = field.name();
        let width = max_field_lens.get(field).copied().unwrap_or(name.len());
        widths.push(width);
        // let width = width - name.len();
        values.push(format!(" {name:<width$} "));
    }
    println!("|{}|", values.join("|"));
    println!(
        "|{}|",
        widths
            .iter()
            .map(|w| format!("{:-<1$}", "", *w + 2))
            .collect::<Vec<String>>()
            .join("|")
    );
    for krate in &context.crates {
        values.clear();
        for val in &krate.fields {
            let field = val.field;
            let value = val.value.clone().unwrap_or_default();
            let width = max_field_lens.get(&field).copied().unwrap_or(value.len());
            // let width = width - value.len();
            values.push(format!(" {value:<width$} "));
        }
        println!("|{}|", values.join("|"));
    }
}

pub fn print_plain(context: &Context) {
    for krate in &context.crates {
        for field in &krate.fields {
            println!("{}", field.value.clone().unwrap_or_default())
        }
    }
}
