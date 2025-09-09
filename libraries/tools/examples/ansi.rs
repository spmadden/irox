// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_tools::ansi_colors::{get_termcap, get_textarea_size_pixels};
use irox_tools::hex::HexDump;

pub fn main() -> Result<(), std::io::Error> {
    let res = get_termcap("TN;")?;
    res.hexdump();
    let res = get_textarea_size_pixels()?;
    res.hexdump();
    Ok(())
}
