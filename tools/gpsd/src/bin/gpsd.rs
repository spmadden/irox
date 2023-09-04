use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use human_panic::setup_panic;
use log::{error, info};

use irox_gpsd::config::{GPSdConfig, Transport};
use irox_gpsd::error::GPSdError;
use irox_gpsd::output::FrameGenerator;
use irox_gpsd::transport::serial::SerialConfig;
use irox_gpsd::transport::{ListenSettings, TCPServer};

fn main() -> Result<(), GPSdError> {
    setup_panic!();
    env_logger::Builder::from_env("GPSD_LOG").init();

    let term = Arc::new(AtomicBool::new(false));
    if let Err(e) = signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)) {
        error!("Unable to register ctrl+c handler: {e:?}");
    }

    let config = GPSdConfig::parse();

    let server = match TCPServer::start(
        ListenSettings {
            ..Default::default()
        },
        term.clone(),
    ) {
        Ok(s) => s,
        Err(e) => {
            error!("Error starting TCP Server: {e:?}");
            return Err(e);
        }
    };

    if let Err(e) = match config.source {
        Transport::Serial(e) => start_serial(server, term.clone(), e),

        #[cfg(target_os = "windows")]
        Transport::WindowsLocation => windows::start_windows(server, term.clone()),
    } {
        error!("Error starting transport: {e:?}");
        return Err(e);
    }

    while !term.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

pub fn start_serial(
    mut server: TCPServer,
    shouldquit: Arc<AtomicBool>,
    config: SerialConfig,
) -> Result<(), GPSdError> {
    let encoding = config.encoding;
    let port = match irox_gpsd::transport::serial::open(config) {
        Ok(p) => p,
        Err(e) => {
            error!("Unable to open serial port: {:?}", e.0);
            return Err(e.0);
        }
    };

    let mut framebuilder = FrameGenerator::new(encoding, port);
    while !shouldquit.load(Ordering::Relaxed) {
        let frame = framebuilder.build_from();
        let frame = match frame {
            Ok(f) => f,
            Err(e) => {
                error!("Error reading frame: {e:?}");
                continue;
            }
        };
        if let Ok(json) = frame.to_json() {
            info!("Generated frame {json}");
        }
        if let Err(e) = server.send(frame) {
            error!("Error sending frame: {e:?}");
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
mod windows {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    use log::{error, info};

    use irox_gpsd::error::GPSdError;
    use irox_gpsd::output::Frame;
    use irox_gpsd::transport::TCPServer;
    use irox_winlocation_api::WindowsLocationAPI;

    pub fn start_windows(mut server: TCPServer, running: Arc<AtomicBool>) -> Result<(), GPSdError> {
        let locator = WindowsLocationAPI::connect()?;
        info!("Connected to windows location api");

        let pos = match locator.get_location() {
            Ok(p) => p,
            Err(e) => {
                error!("Error getting first position: {e:?}");
                return Err(e.into());
            }
        };
        info!("First position: {pos}");
        let tpv: Frame = (&pos).into();
        if let Err(e) = server.send(tpv) {
            error!("Error sending initial TPV: {e:?}");
        }

        locator.on_location_changed(move |pos| {
            info!("{pos:?}");
            let frame: Frame = (&pos).into();
            if let Err(e) = server.send(frame) {
                error!("Error sending frame: {e:?}");
            }
        })?;

        while running.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}
