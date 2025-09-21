// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::Error;
use irox_chronyc::connect_unix_socket;
use irox_chronyc::msgs::ChronycRequest;

fn main() -> Result<(), Error> {
    let mut client = connect_unix_socket("/run/chrony/chronyd.sock")?;
    let mut req = ChronycRequest::new_num_sources_command();
    client.request_response(&mut req)?;
    Ok(())
}
