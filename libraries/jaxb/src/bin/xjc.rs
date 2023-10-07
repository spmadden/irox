// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use clap::{Parser, ValueHint};
use clap_verbosity_flag::Verbosity;
use log::info;

use irox_jaxb::error::Error;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// Increase the logging verbosity
    #[command(flatten)]
    pub verbose: Verbosity,

    /// Output rust file
    #[arg(long, short)]
    pub out: String,

    /// Path to a single xsd file
    #[arg(value_hint = ValueHint::FilePath)]
    pub path: Vec<String>,
}

pub fn main() -> Result<(), Error> {
    env_logger::Builder::from_env("XJC_LOG").init();
    let config = Config::parse();
    info!("Using config: {config:?}");

    for path in &config.path {
        info!("Processing {path}");

        let file = std::fs::File::open(path)?;
        let file = irox_jaxb::schema::Schema::read_from(file)?;
    }

    Ok(())
}
