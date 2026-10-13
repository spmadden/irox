// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

#[cfg(not(target_arch = "wasm32"))]
pub mod connection;
pub mod dynamic;
pub mod error;
pub mod reflect;

#[cfg(target_arch = "wasm32")]
pub mod connection {
    pub struct DbConn {
        pub client: tokio_postgres::Client,
    }
}