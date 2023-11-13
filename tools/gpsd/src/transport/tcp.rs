//!
//! TCP Transport

use std::io::{ErrorKind, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread::JoinHandle;

use log::{error, info};

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
            listen_port: 2947,
            listen_ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        }
    }
}
pub struct TCPServer {
    client_sender: Sender<String>,
    joiner: JoinHandle<()>,
    client_joiner: JoinHandle<()>,
    close: Arc<AtomicBool>,
}

impl TCPServer {
    pub fn start(settings: ListenSettings, close: Arc<AtomicBool>) -> Result<TCPServer, GPSdError> {
        let sockaddr = SocketAddr::new(settings.listen_ip, settings.listen_port);
        let listen = TcpListener::bind(sockaddr)?;
        listen.set_nonblocking(true)?;

        let (msg_sender, msg_receiver) = channel::<String>();
        let (client_sender, client_receiver) = channel();
        let clients_close = close.clone();
        let client_joiner = std::thread::spawn(move || {
            let mut clients: Vec<Client> = Vec::new();
            while !clients_close.load(Ordering::Relaxed) {
                match client_receiver.try_recv() {
                    Ok(c) => {
                        clients.push(c);
                    }
                    Err(e) => match e {
                        TryRecvError::Empty => {}
                        TryRecvError::Disconnected => {
                            break;
                        }
                    },
                }
                match msg_receiver.try_recv() {
                    Ok(e) => {
                        clients.retain_mut(move |c| {
                            if let Err(e) = c.get_sender().send(e.clone()) {
                                error!("Error sending msg to client: {e:?}");
                                return false;
                            }
                            true
                        });
                    }
                    Err(e) => match e {
                        TryRecvError::Empty => {
                            // eat it.
                        }
                        TryRecvError::Disconnected => break,
                    },
                };
                clients.retain_mut(|c| match c.do_some() {
                    Ok(cont) => {
                        return cont;
                    }
                    Err(e) => {
                        error!("Error doing work: {e:?}");
                        return false;
                    }
                });
            }
            info!("Exit clients thread.");
        });

        let accept_close = close.clone();
        let joiner = std::thread::spawn(move || {
            while !accept_close.load(Ordering::Relaxed) {
                match listen.accept() {
                    Ok((stream, remote)) => {
                        if let Err(e) = stream.set_nonblocking(true) {
                            error!("Error setting stream nonblocking: {e:?}");
                        }
                        info!("Accepted new client: {remote:?}");
                        let client = Client::new(stream, remote, accept_close.clone());
                        if let Err(e) = client_sender.send(client) {
                            error!("Error accepting client: {e:?}");
                        }
                    }
                    Err(e) => {
                        match e.kind() {
                            ErrorKind::WouldBlock => {
                                // eat it.
                            }
                            _ => {
                                error!("Error accepting new connection: {e:?}");
                            }
                        }
                    }
                }
            }
            info!("Exit accept thread.");
        });

        info!("GPSd server successfully started on {sockaddr}");
        Ok(TCPServer {
            joiner,
            client_sender: msg_sender,
            client_joiner,
            close,
        })
    }

    pub fn send(&mut self, frame: &Frame) -> Result<(), GPSdError> {
        let data = frame.to_json()?;
        if let Err(e) = self.client_sender.send(data) {
            error!("Error sending msg to clients: {e:?}");
        }
        Ok(())
    }
}

pub struct Client {
    remote: SocketAddr,
    sender: Sender<String>,
    receiver: Receiver<String>,
    stream: TcpStream,
    close: Arc<AtomicBool>,
    buffer: Vec<u8>,
}

impl Client {
    pub fn new(stream: TcpStream, remote: SocketAddr, close: Arc<AtomicBool>) -> Client {
        let (sender, receiver) = channel();
        Client {
            close: close.clone(),
            remote,
            sender,
            receiver,
            stream,
            buffer: Vec::new(),
        }
    }
    pub fn get_sender(&self) -> Sender<String> {
        self.sender.clone()
    }
    pub fn do_some(&mut self) -> Result<bool, GPSdError> {
        match self.receiver.try_recv() {
            Ok(json) => {
                write!(self.stream, "{json}\r\n")?;
            }
            Err(e) => match e {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => {
                    return Ok(false);
                }
            },
        };
        let mut readbuf: [u8; 4096] = [0; 4096];
        match self.stream.read(&mut readbuf) {
            Ok(read) => {
                if read == 0 {
                    return Ok(false);
                }
                self.buffer.extend_from_slice(readbuf.split_at(read).0);
                let str = String::from_utf8_lossy(&self.buffer);
                info!("{str}");
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::WouldBlock => {
                        // eat it.
                    }
                    _ => {
                        return Err(e.into());
                    }
                }
            }
        }

        Ok(true)
    }
}
