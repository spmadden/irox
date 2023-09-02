// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::Debug;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use log::{error, info};

pub type OnConnectionCallback = Box<dyn FnMut(&TcpStream, &SocketAddr)>;
pub type ConnectionWorker = Box<dyn Fn(&TcpStream)>;

pub struct TCPConnectionManager {
    active_connections: Arc<Mutex<Vec<TcpStream>>>,
    running_thread: JoinHandle<()>,
}

impl TCPConnectionManager {
    pub fn start<A: ToSocketAddrs + Debug>(
        addr: A,
        running: Arc<AtomicBool>,
    ) -> Result<TCPConnectionManager, std::io::Error> {
        let mut addr: Vec<SocketAddr> = match addr.to_socket_addrs() {
            Ok(a) => a.collect(),
            Err(e) => {
                error!("Error converting {addr:?} to socketaddr");
                return Err(e);
            }
        };
        info!("Collected SocketAddrs: {addr:?}");
        let Some(addr) = addr.pop() else {
            return Err(std::io::ErrorKind::InvalidInput.into());
        };

        let sock = match TcpListener::bind(addr) {
            Ok(s) => s,
            Err(e) => {
                error!("Error binding to address {:?}: {e:?}", &addr);
                return Err(e);
            }
        };

        let active_connections = Arc::new(Mutex::new(Vec::new()));

        let conns = active_connections.clone();
        let handle = thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let client = match sock.accept() {
                    Ok(c) => c,
                    Err(e) => {
                        error!("SocketAccept error: {e:?}");
                        continue;
                    }
                };

                let Ok(ref mut conns) = conns.lock() else {
                    continue;
                };
                conns.push(client.0);
            }
        });

        Ok(TCPConnectionManager {
            active_connections,
            running_thread: handle,
        })
    }

    pub fn join(self) -> thread::Result<()> {
        self.running_thread.join()
    }

    pub fn write_to_all_connected(&mut self, data: &[u8]) {
        let Ok(ref mut conns) = self.active_connections.lock() else {
            return;
        };
        conns.retain_mut(|x| {
            let Ok(()) = x.write_all(data) else {
                // remove and close the TCP stream if there was an error writing to it.
                return false;
            };
            true
        });
    }

    pub fn for_each_connected<T: FnMut(&mut TcpStream) -> bool>(&mut self, func: T) {
        let Ok(ref mut conns) = self.active_connections.lock() else {
            return;
        };
        conns.retain_mut(func);
    }
}
