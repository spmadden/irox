// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
fn main() -> Result<(), irox_bits::Error> {
    let mut client = irox_chronyc::connect_unix_socket("/run/chrony/chronyd.sock")?;
    let mut req = irox_chronyc::msgs::ChronycRequest::new_num_sources_command();
    client.request_response(&mut req)?;
    Ok(())
}
#[cfg(any(target_arch = "wasm32", not(target_os = "linux")))]
fn main() {}
