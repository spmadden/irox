// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

pub fn main() -> Result<(), irox_build_rs::Error> {
    println!("cargo:rerun-if-changed=build.rs");

    irox_build_rs::generate_module()?;

    Ok(())
}
