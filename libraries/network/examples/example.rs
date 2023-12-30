// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_tools::hex::HexDump;
use std::io::{ErrorKind, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

pub fn main() -> Result<(), std::io::Error> {
    let mut con = TcpStream::connect(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(34, 193, 227, 35)),
        80,
    ))?;
    con.set_read_timeout(Some(Duration::from_secs(1)))?;

    write!(con, "GET /stream-bytes/10000?chunk_size=1 HTTP/1.1\r\n")?;
    write!(con, "Host: httpbin.org\r\n")?;
    write!(con, "\r\n\r\n")?;

    con.flush()?;

    let mut out: Vec<u8> = Vec::with_capacity(100_000);
    let mut buf: [u8; 8192] = [0; 8192];

    let mut reads: Vec<usize> = Vec::with_capacity(1000);
    loop {
        let read = match con.read(&mut buf) {
            Ok(r) => r,
            Err(e) => match e.kind() {
                ErrorKind::TimedOut | ErrorKind::WouldBlock => {
                    break;
                }
                _ => return Err(e),
            },
        };
        reads.push(read);
        if read == 0 {
            break;
        }
        out.extend_from_slice(buf.get(..read).unwrap_or_default());
        std::thread::sleep(Duration::from_millis(1));
    }
    // println!("{}: {reads:?}", reads.len());
    out.hexdump();
    Ok(())
}
