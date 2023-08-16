// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use log::{debug, error, info};

use irox_influxdb_v1::{InfluxConnectionBuilder, InfluxDB, InfluxDBConnectionParams};

#[derive(Subcommand, Debug)]
enum Operation {
    /// Ping the server to check aliveness
    PING,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about, propagate_version = true)]
struct Config {
    /// Command to execute
    #[command(subcommand)]
    command: Operation,

    /// Server (IP or Hostname) to connect to - defaults to 'localhost'
    #[arg(short = 's', long = "server")]
    server: Option<String>,

    /// Port to connect to - defaults to '8086'
    #[arg(short = 'p', long = "port")]
    port: Option<u16>,

    /// URL to connect to - defaults to 'http://localhost:8086'
    #[arg(short = 'u', long = "url")]
    url: Option<String>,
}

fn main() -> ExitCode {
    env_logger::init();

    let config = Config::parse();

    let conn = match match &config.url {
        None => InfluxConnectionBuilder::default()
            .maybe_host(config.server.clone())
            .maybe_port(config.port)
            .build(),
        Some(url) => InfluxDBConnectionParams::open_url(url),
    } {
        Ok(db) => db,
        Err(e) => {
            error!("{:?}", e);
            return ExitCode::FAILURE;
        }
    };

    debug!("{:?}", config);
    match config.command {
        Operation::PING => ping(&conn),
    }
}

fn ping(db: &InfluxDB) -> ExitCode {
    if let Err(e) = db.ping() {
        error!("{:?}", e);
        return ExitCode::FAILURE;
    }
    info!("PING SUCCESSFUL.");
    ExitCode::SUCCESS
}
