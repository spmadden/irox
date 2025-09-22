// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_log::log;
use irox_log::log::Level;
use std::net::{IpAddr, Ipv4Addr};

fn main() -> Result<(), irox_bits::Error> {
    irox_log::init_console_level(Level::Info);
    let client = irox_gnss::tcp::TCPClient::new(IpAddr::V4(Ipv4Addr::new(10, 169, 0, 13)), 8089)?;
    while let Some(pkt) = client.recv() {
        log::info!("read packet: {pkt}");
    }
    Ok(())
}
