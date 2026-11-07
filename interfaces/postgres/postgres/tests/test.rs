// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use irox_postgres::connection::{config_from_env, DbConnBuilder};
use irox_postgres::error::Error;
use irox_postgres::*;

#[derive(Debug, Clone, PostgresList, PostgresGetByPrimaryKey)]
#[table_name = "test"]
pub struct TestStruct {
    #[pk]
    pub id: i32,
}

#[tokio::test]
#[ignore]
pub async fn test() -> Result<(), Error> {
    let (mut conn, connection) = DbConnBuilder::default()
        .try_load_environments()?
        .with_identity_cert_env()?
        .build_and_connect(config_from_env())
        .await?;
    let handl = tokio::spawn(connection);

    let res = TestStruct::list(&mut conn).await?;
    println!("{:#?}", res);

    handl.await.unwrap()?;
    Ok(())
}
