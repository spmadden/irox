//!
//! TCP Transport

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use log::{error, info};

use irox_networking::pool::TCPConnectionManager;

use crate::error::GPSdError;
use crate::output::Frame;

#[derive(Debug, Copy, Clone)]
pub struct ListenSettings {
    pub listen_port: u16,
    pub listen_ip: IpAddr,
}
impl Default for ListenSettings {
    fn default() -> Self {
        ListenSettings {
            listen_port: 2497,
            listen_ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        }
    }
}
pub struct TCPServer {
    conn_pool: TCPConnectionManager,
}

impl TCPServer {
    pub fn start(
        settings: ListenSettings,
        running: Arc<AtomicBool>,
    ) -> Result<TCPServer, GPSdError> {
        let sockaddr = SocketAddr::new(settings.listen_ip, settings.listen_port);

        let conn_pool = match TCPConnectionManager::start(sockaddr, running) {
            Ok(c) => c,
            Err(e) => {
                error!("Error starting TCPConnectionManager: {e:?}");
                return Err(e.into());
            }
        };

        info!("GPSd server successfully started on {sockaddr}");
        Ok(TCPServer { conn_pool })
    }

    pub fn send(&mut self, frame: Frame) -> Result<(), GPSdError> {
        let data = frame.to_json()?;
        self.conn_pool.write_to_all_connected(data.as_bytes());
        Ok(())
    }
}
