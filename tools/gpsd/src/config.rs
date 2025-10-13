use std::net::IpAddr;

use clap::*;
use clap_verbosity_flag::Verbosity;

#[derive(Debug, Default, Copy, Clone, ValueEnum)]
pub enum EncodingType {
    #[default]
    Nmea0183,
    SirfBinary,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Transport {
    #[cfg(not(target_arch = "wasm32"))]
    Serial(crate::transport::serial::SerialConfig),

    #[cfg(target_os = "windows")]
    WindowsLocation,
}

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct GPSdConfig {
    #[command(flatten)]
    pub verbose: Verbosity,

    #[command(subcommand)]
    pub source: Transport,

    /// TCP listen address
    #[arg(short = 'a', long, default_value = "127.0.0.1")]
    pub listen_address: Option<IpAddr>,

    /// TCP listen port
    #[arg(short='l', long, default_value_t=2947, value_parser=clap::value_parser!(u16).range(1..))]
    pub listen_port: u16,
}
