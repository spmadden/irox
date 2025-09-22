// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;
use crate::{GNSSFrame, GNSSPacketStream};
use alloc::sync::Arc;
use core::net::{IpAddr, SocketAddr};
use core::sync::atomic::AtomicBool;
use irox_bits::BitsWrapper;
use irox_log::log;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

pub struct TCPClient {
    running: Arc<AtomicBool>,
    handl: Option<JoinHandle<()>>,
    rx_messages: Option<Receiver<GNSSFrame>>,
}
impl Drop for TCPClient {
    fn drop(&mut self) {
        self.running
            .store(false, core::sync::atomic::Ordering::Relaxed);
        if let Some(rx) = self.rx_messages.take() {
            drop(rx);
        }
        if let Some(handl) = self.handl.take() {
            let _ = handl.join();
        }
    }
}
impl TCPClient {
    pub fn new(host: IpAddr, port: u16) -> Result<TCPClient, irox_bits::Error> {
        let run = Arc::new(AtomicBool::new(true));
        let (tx, rx) = std::sync::mpsc::channel::<GNSSFrame>();
        let saddr = SocketAddr::new(host, port);
        let handl = {
            let run = run.clone();
            std::thread::spawn(move || {
                while run.load(core::sync::atomic::Ordering::Relaxed) {
                    let Ok(stream) = std::net::TcpStream::connect_timeout(
                        &saddr,
                        core::time::Duration::from_secs(1),
                    ) else {
                        log::info!("Failed to connect to {saddr}");
                        continue;
                    };
                    let mut stream = GNSSPacketStream::new(run.clone(), BitsWrapper::Owned(stream));
                    while run.load(core::sync::atomic::Ordering::Relaxed) {
                        let pkt = match stream.read_next() {
                            Ok(pkt) => pkt,
                            Err(e) => {
                                log::info!("Failed to read packet from {saddr}: {e:?}, retrying");
                                break;
                            }
                        };
                        if let Err(e) = tx.send(pkt) {
                            log::info!("Failed to send packet to receiver: {e:?}");
                        }
                    }
                    std::thread::sleep(core::time::Duration::from_secs(1));
                }
            })
        };
        Ok(TCPClient {
            handl: Some(handl),
            running: run,
            rx_messages: Some(rx),
        })
    }

    #[must_use]
    pub fn recv(&self) -> Option<GNSSFrame> {
        self.rx_messages.as_ref()?.recv().ok()
    }
}
