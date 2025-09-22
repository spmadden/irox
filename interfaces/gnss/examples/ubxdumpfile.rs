// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::Error;
use irox_log::log::{info, Level};
use irox_tools::packetio::PacketBuilder;
use irox_ubx::ubx_dumptxtfile::DumpTxtFileParser;
use std::fs::OpenOptions;
use std::io::BufReader;

fn main() -> Result<(), Error> {
    irox_log::init_console_level(Level::Info);

    let file = OpenOptions::new()
        .read(true)
        .create(false)
        .open("interfaces/gnss/data/lea-F9T.txt")?;
    let mut file = BufReader::new(file);

    let parser = DumpTxtFileParser;
    while let Some(msg) = parser.build_from(&mut file)? {
        info!("{msg:#?}");
    }
    Ok(())
}
