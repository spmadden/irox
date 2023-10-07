// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fs::File;
use std::io::BufWriter;

use clap::{Parser, ValueHint};
use clap_verbosity_flag::Verbosity;
use log::{debug, error, info};

use irox_carto::coordinate::CoordinateType;
use irox_gpx::{GPXWriter, Track, TrackSegment, GPX};
use irox_raymarine_sonar::error::Error;
use irox_raymarine_sonar::SDFConnection;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// Increase the logging verbosity
    #[command(flatten)]
    pub verbose: Verbosity,

    /// Output gpx file
    #[arg(long, short)]
    pub out: String,

    /// Path to a single SDF file
    #[arg(value_hint = ValueHint::FilePath)]
    pub path: Vec<String>,
}

pub fn main() -> Result<(), Error> {
    env_logger::Builder::from_env("SONAR_LOG").init();
    let config = Config::parse();
    info!("Using config: {config:?}");

    let mut gpx = GPX::new();

    for path in &config.path {
        info!("Processing {path}");
        let conn = SDFConnection::open(path)?;
        for row in conn.get_global_props()? {
            for (name, value) in row {
                debug!("Found global prop: {name} => {value:?}");
            }
        }

        let mut tr = Track::new();
        tr.name = Some(path.clone());
        for track in conn.get_tracks()? {
            info!("Adding track segment for {path}");
            let mut seg = TrackSegment::new();
            for point in track.iter()? {
                match point {
                    CoordinateType::Elliptical(e) => {
                        seg.track_point.push(e.into());
                        // println!("{e}")
                    }
                    CoordinateType::Cartesian(e) => {
                        // println!("{e}")
                    }
                    CoordinateType::Horizontal(_) => {}
                }
            }
            tr.trkseg.push(seg);
        }
        gpx.trk.push(tr);
    }

    info!("Writing output file: {}", config.out);
    let mut file = BufWriter::new(File::create(config.out)?);
    if let Err(e) = GPXWriter::new().write_to(&gpx, &mut file) {
        error!("Error writing gpx file: {e:?}");
    }

    Ok(())
}
