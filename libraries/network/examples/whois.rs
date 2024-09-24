// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

#![allow(clippy::print_stdout)]
use irox_bits::BitsError;
use irox_networking::whois::WhoisOptions;
use log::Level;

fn main() -> Result<(), BitsError> {
    irox_log::init_console_level(Level::Debug);
    let results = WhoisOptions::default().query("rust-lang.org")?;
    println!("{results:#?}");
    Ok(())
}
