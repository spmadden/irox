// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//!
//!

#![forbid(unsafe_code)]
#![allow(clippy::print_stdout)]

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use log::{debug, error};

use irox_influxdb_v1::{EncodingType, InfluxConnectionBuilder, InfluxDB, InfluxDBConnectionParams};

#[derive(Debug, Parser)]
pub struct OptionalDB {
    /// Optional Database name
    #[arg(long)]
    db: Option<String>,
}

#[derive(Debug, Parser)]
pub struct QueryString {
    /// Optional Database name
    #[arg(long)]
    db: Option<String>,

    query: String,
}

#[derive(Subcommand, Debug)]
enum Operation {
    /// Ping the server to check aliveness
    Ping,

    /// List the available databases
    ListDB,

    ListRetentionPolicies(OptionalDB),

    ShowTagKeys(OptionalDB),

    QueryCSV(QueryString),
    QueryJSON(QueryString),
    ShowDescriptors(OptionalDB),
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
        Operation::Ping => ping(&conn),
        Operation::ListDB => list_db(&conn),
        Operation::ListRetentionPolicies(pol) => list_retention_policies(&conn, pol),
        Operation::ShowTagKeys(db) => show_tag_keys(&conn, db),
        Operation::QueryCSV(query) => query_string(&conn, query, EncodingType::CSV),
        Operation::QueryJSON(query) => query_string(&conn, query, EncodingType::JSON),
        Operation::ShowDescriptors(db) => show_descriptors(&conn, &db),
    }
}

fn ping(db: &InfluxDB) -> ExitCode {
    if let Err(e) = db.ping() {
        error!("{:?}", e);
        return ExitCode::FAILURE;
    }
    println!("PING SUCCESSFUL.");
    ExitCode::SUCCESS
}

fn list_db(db: &InfluxDB) -> ExitCode {
    match db.list_databases() {
        Ok(val) => {
            println!("{val:?}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("{:?}", e);
            ExitCode::FAILURE
        }
    }
}

fn list_retention_policies(db: &InfluxDB, param: OptionalDB) -> ExitCode {
    match db.show_retention_policies(param.db) {
        Ok(val) => {
            println!("{val:?}");
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
            println!("{val:?}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("{:?}", e);
            ExitCode::FAILURE
        }
    }
}

fn query_string(db: &InfluxDB, query: QueryString, encoding: EncodingType) -> ExitCode {
    match db.query_string(query.query, encoding, query.db) {
        Ok(val) => {
            println!("{val}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("{e:?}");
            ExitCode::FAILURE
        }
    }
}

fn show_descriptors(db: &InfluxDB, param: &OptionalDB) -> ExitCode {
    let res = match db.get_descriptors(&param.db) {
        Ok(r) => r,
        Err(e) => {
            error!("{e:?}");
            return ExitCode::FAILURE;
        }
    };

    for meas in res {
        println!("Measurement ({}) {{", meas.name());
        for tag in meas.tags() {
            println!("\ttag {tag}");
        }
        for field in meas.fields() {
            println!("\tfield {} : {:?}", field.name(), field.variable_type());
        }
        println!("}}");
    }

    ExitCode::SUCCESS
}
