// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use log::{debug, error, info};

use irox_influxdb_v1::{InfluxConnectionBuilder, InfluxDB, InfluxDBConnectionParams};

#[derive(Debug, Parser)]
pub struct OptionalDB {
    /// Optional Database name
    #[arg(long)]
    db: Option<String>,
}

#[derive(Debug, Parser)]
pub struct QueryString {
    query: String,
}

#[derive(Subcommand, Debug)]
enum Operation {
    /// Ping the server to check aliveness
    PING,

    /// List the available databases
    ListDB,

    ListRetentionPolicies(OptionalDB),

    ShowTagKeys(OptionalDB),

    QueryCSV(QueryString),
    QueryJSON(QueryString),
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
        Operation::ListDB => list_db(&conn),
        Operation::ListRetentionPolicies(pol) => list_retention_policies(&conn, pol),
        Operation::ShowTagKeys(db) => show_tag_keys(&conn, db),
        Operation::QueryCSV(query) => query_csv(&conn, query.query),
        Operation::QueryJSON(query) => query_json(&conn, query.query),
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

fn list_db(db: &InfluxDB) -> ExitCode {
    match db.list_databases() {
        Ok(val) => {
            info!("{:?}", val);
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("{:?}", e);
            ExitCode::FAILURE
        }
    }
}

fn list_retention_policies(db: &InfluxDB, param: OptionalDB) -> ExitCode {
    match db.show_retention_policites(param.db) {
        Ok(val) => {
            info!("{:?}", val);
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("{:?}", e);
            ExitCode::FAILURE
        }
    }
}

fn show_tag_keys(db: &InfluxDB, param: OptionalDB) -> ExitCode {
    match db.show_tag_keys(param.db) {
        Ok(val) => {
            info!("{:?}", val);
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("{:?}", e);
            ExitCode::FAILURE
        }
    }
}

fn query_csv(db: &InfluxDB, query: String) -> ExitCode {
    match db.query_csv(query) {
        Ok(val) => {
            info!("{}", val);
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("{:?}", e);
            ExitCode::FAILURE
        },
    }
}

fn query_json(db: &InfluxDB, query: String) -> ExitCode {
    match db.query(query) {
        Ok(val) => {
            info!("{}", val);
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("{:?}", e);
            ExitCode::FAILURE
        },
    }
}
