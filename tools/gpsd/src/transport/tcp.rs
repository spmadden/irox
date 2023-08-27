//!
//! TCP Transport

use std::cell::RefCell;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};

use log::{error, info};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

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
            listen_ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
        }
    }
}

pub struct TCPClient {
    sender: Sender<Arc<String>>,
}

pub struct TCPServer {
    listen_port: u16,
    listen_ip: IpAddr,

    sender: Sender<Frame>,
}

impl TCPServer {
    pub fn start(
        runtime: Arc<tokio::runtime::Runtime>,
        settings: ListenSettings,
    ) -> Result<TCPServer, GPSdError> {
        let sockaddr = SocketAddr::new(settings.listen_ip, settings.listen_port);
        let listener = match TcpListener::bind(sockaddr) {
            Ok(e) => e,
            Err(e) => {
                error!("Error opening TCP Listener: {e:?}");
                return Err(e.into());
            }
        };
        let clients = Arc::new(Mutex::new(RefCell::new(Vec::<TCPClient>::new())));
        let (sender, mut recv) = mpsc::channel(100);
        let rt2 = runtime.clone();
        let cl2 = clients.clone();
        runtime.spawn(async move {
            loop {
                let mut stream = match listener.accept() {
                    Ok((stream, addr)) => {
                        info!("Accepted connection from: {addr:?}");
                        stream
                    }
                    Err(e) => {
                        error!("Error accepting connection: {e:?}");
                        continue;
                    }
                };
                let (client_sender, mut client_recvr) = mpsc::channel(100);
                cl2.lock().unwrap().get_mut().push(TCPClient {
                    sender: client_sender,
                });
                rt2.spawn(async move {
                    loop {
                        let Some(msg) = client_recvr.recv().await else {
                            continue;
                        };
                        if let Err(e) = stream.write_fmt(format_args!("{}\n", msg)) {
                            error!("Unable to write msg to tcp stream {e:?}");
                            client_recvr.close();
                            return;
                        }
                    }
                });
            }
        });
        let clients = clients.clone();
        runtime.spawn(async move {
            loop {
                let val: Frame = match recv.recv().await {
                    Some(t) => t,
                    None => continue,
                };
                let res = serde_json::to_string(&val);
                let json = match val.to_json() {
                    Ok(j) => j,
                    Err(e) => {
                        error!("Error creating json output: {e:?}");
                        continue;
                    }
                };
                let strref = Arc::new(json);

                let senders: Vec<Sender<_>> = clients
                    .lock()
                    .unwrap()
                    .get_mut()
                    .iter()
                    .filter_map(|c| {
                        if !c.sender.is_closed() {
                            return Some(c.sender.clone());
                        }
                        None
                    })
                    .collect();

                for sender in senders {
                    if let Err(e) = sender.send(strref.clone()).await {
                        error!("Unable to queue message to client: {e:?}");
                    }
                }
            }
        });

        info!("GPSd server successfully started on {sockaddr}");
        Ok(TCPServer {
            listen_ip: settings.listen_ip,
            listen_port: settings.listen_port,
            sender,
        })
    }

    pub fn send(&self, frame: Frame) -> Result<(), GPSdError> {
        self.sender.blocking_send(frame).map_err(|e| {
            error!("Unable to queue message to clients");
            GPSdError::new_str(e.to_string())
        })
    }
}
