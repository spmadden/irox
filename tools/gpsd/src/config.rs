use clap::*;
use clap_verbosity_flag::Verbosity;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Encoding {
    Nmea0183,
    SirfBinary,
}

#[derive(Debug, Copy, Clone, Subcommand, ValueEnum)]
pub enum Transport {
    // Serial(Encoding),
    WindowsLocation,
}

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct GPSdConfig {
    #[command(flatten)]
    pub verbose: Verbosity,

    #[command()]
    pub source: Transport,
}
