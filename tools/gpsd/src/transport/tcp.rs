//!
//! TCP Transport

use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::mpsc::Sender;

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
    sender_to_clients: Sender<Box<Frame>>,
}

impl TCPServer {
    pub fn start(settings: ListenSettings, close: Arc<AtomicBool>) -> Result<TCPServer, GPSdError> {
        let sockaddr = SocketAddr::new(settings.listen_ip, settings.listen_port);

        info!("GPSd server successfully started on {sockaddr}");
        Ok(TCPServer { conn_pool })
    }

    pub fn poll_commands(&mut self) {
        self.conn_pool.for_each_connected(|s| {
            let mut buf: [u8; 4096] = [0; 4096];
            let read = match s.read(&mut buf) {
                Ok(r) => r,
                Err(e) => {
                    error!("Error reading stream: {e:?}");
                    return false;
                }
            };
            let (read, _rem) = buf.split_at(read);
            let str = String::from_utf8_lossy(read);
            info!("{str}");
            true
        });
    }

    pub fn send(&mut self, frame: &Frame) -> Result<(), GPSdError> {
        let data = frame.to_json()?;
        let mut buf: Vec<u8> = Vec::new();
        buf.write_fmt(format_args!("{data}\r\n"))?;

        self.conn_pool.write_to_all_connected(&buf);
        Ok(())
    }
}
