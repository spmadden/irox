use std::sync::Arc;

use clap::Parser;
use human_panic::setup_panic;
use log::error;
use simplelog::{ColorChoice, TerminalMode, TermLogger};

use irox_gpsd::config::{Encoding, GPSdConfig, Transport};
use irox_gpsd::error::GPSdError;
use irox_gpsd::transport::{ListenSettings, TCPServer};
use irox_tools::packetio::*;

fn main() -> Result<(), GPSdError> {
    setup_panic!();

    let config = GPSdConfig::parse();
    let loglevel = config.verbose.log_level_filter();
    let logconfig = simplelog::ConfigBuilder::new()
        .set_max_level(loglevel)
        .build();
    TermLogger::new(loglevel, logconfig, TerminalMode::Mixed, ColorChoice::Auto);

    let runtime = Arc::new(
        match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to create tokio runtime: {e:?}");
                return Err(e.into());
            }
        },
    );

    let server = TCPServer::start(
        runtime.clone(),
        ListenSettings {
            ..Default::default()
        },
    )?;

    if let Err(e) = match config.source {
        // Transport::Serial(e) => start_serial(&mut runtime, &e),
        Transport::WindowsLocation => start_windows(server),
    } {
        error!("Error starting transport: {e:?}");
        return Err(e);
    }

    runtime.block_on(async { tokio::signal::ctrl_c().await });

    Ok(())
}

pub fn start_serial(
    runtime: &mut tokio::runtime::Runtime,
    enc: &Encoding,
) -> Result<(), GPSdError> {
    let port = irox_gpsd::transport::serial::open();
    let Ok(mut port) = port else {
        return Ok(());
    };
    runtime.spawn(async move {
        let parser = irox_sirf::packet::PacketParser {};
        loop {
            let res = parser.build_from(&mut port);
            println!("{res:?}");
        }
    });
    Ok(())
}

pub fn start_windows(server: TCPServer) -> Result<(), GPSdError> {
    irox_gpsd::transport::winloc::open(server)
}
