use clap::{Parser, ValueHint};
use clap_verbosity_flag::Verbosity;
use irox_raymarine_sonar::error::Error;
use irox_raymarine_sonar::SDFConnection;
use log::info;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// Increase the logging verbosity
    #[command(flatten)]
    pub verbose: Verbosity,

    /// Path to a single SDF file
    #[arg(value_hint = ValueHint::FilePath)]
    pub path: String,
}

pub fn main() -> Result<(), Error> {
    env_logger::Builder::from_env("SONAR_LOG").init();
    let config = Config::parse();
    info!("Using config: {config:?}");

    let conn = SDFConnection::open(&config.path)?;
    for row in conn.get_global_props()? {
        for (name, value) in row {
            info!("Found global prop: {name} => {value:?}");
        }
    }
    for track in conn.get_track_info()? {
        info!("Found track info: {track:?}");
    }
    Ok(())
}
