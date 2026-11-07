// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

pub use crate::connection::DbConn;
pub use irox_pgderive::*;

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

pub trait PostgresList {
    type Item;

    fn list(
        conn: &mut DbConn,
    ) -> impl std::future::Future<Output = Result<Vec<Self::Item>, error::Error>>;
}

pub trait PostgresGetByPrimaryKey {
    type PkType;
    type Item;

    fn get_by_pk(
        conn: &mut DbConn,
        primary_key: Self::PkType,
    ) -> impl std::future::Future<Output = Result<Option<Self::Item>, error::Error>>;
}
