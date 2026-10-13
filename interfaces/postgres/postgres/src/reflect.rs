// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::connection::DbConn;
use crate::error::Error;
use irox_pgderive::PostgresImplGet;
use irox_types::{DynamicallySizedValue, Primitives, VariableValue};
use std::collections::BTreeMap;
use tokio_postgres::types::{FromSql, Kind, Type};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Nullable {
    #[default]
    Yes,
    No,
}
impl<'a> FromSql<'a> for Nullable {
    fn from_sql(
        _ty: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = String::from_utf8_lossy(raw);
        match s.as_ref() {
            "YES" => Ok(Nullable::Yes),
            "NO" => Ok(Nullable::No),
            _ => Err(format!("Invalid nullable value: {}", s).into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::new(
            "yes_or_no".to_string(),
            14164,
            Kind::Simple,
            "information_schema".to_owned(),
        ) || *ty == Type::VARCHAR
    }
}

#[derive(Debug, Default, Clone, PartialEq, PostgresImplGet)]
#[query = r#"
SELECT  c.*,
        td.description AS table_description,
        cd.description AS column_description
FROM information_schema.tables t
    LEFT JOIN pg_catalog.pg_description td
        ON (td.objoid = (quote_ident(t.table_schema)||'.'||quote_ident(t.table_name))::regclass AND td.objsubid = 0)
    NATURAL LEFT JOIN information_schema.columns c
    LEFT JOIN pg_catalog.pg_description cd
        ON (cd.objoid = (quote_ident(t.table_schema)||'.'||quote_ident(t.table_name))::regclass AND cd.objsubid = c.ordinal_position)
WHERE t.table_schema NOT IN('information_schema','pg_catalog')
ORDER BY t.table_name, c.ordinal_position;
"#]
pub struct Column {
    pub column_name: String,
    pub column_description: Option<String>,
    pub column_default: Option<String>,
    pub table_schema: String,
    pub table_name: String,
    pub table_description: Option<String>,
    pub is_nullable: Nullable,
    pub data_type: String,
    pub character_maximum_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub udt_name: String,
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Table {
    pub table_name: String,
    pub table_schema: String,
    pub table_description: Option<String>,
    pub columns: Vec<Column>,
}

impl Table {
    pub async fn list(conn: &mut DbConn) -> Result<Vec<Table>, Error> {
        let mut out = BTreeMap::<String, Table>::new();
        let columns = Column::list(conn).await?;

        for column in columns {
            let table_name = column.table_name.clone();
            let table = out.entry(table_name).or_insert_with_key(|name| Table {
                table_name: name.clone(),
                table_schema: column.table_schema.clone(),
                table_description: column.table_description.clone(),
                columns: Vec::new(),
            });
            table.columns.push(column);
        }

        Ok(out.values().cloned().collect())
    }
    pub async fn dump_data(&self, conn: &mut DbConn) -> Result<(), Error> {
        let data = conn
            .client
            .query(
                &format!(
                    "SELECT * FROM \"{}\".\"{}\"",
                    self.table_schema, self.table_name
                ),
                &[],
            )
            .await?;
        for row in &data {
            for idx in 0..row.columns().len() {
                let val = row.try_get::<_, Option<PType>>(idx);
                let v = match val {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Error: {}", e);
                        continue;
                    }
                };
                println!("{v:?}");
            }
        }

        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct PType(pub VariableValue);

// it's not actually useless.
#[allow(clippy::useless_asref)]
impl<'a> FromSql<'a> for PType {
    fn from_sql(
        ty: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn core::error::Error + Sync + Send>> {
        let raw = &mut raw.as_ref();
        if ty == &Type::INT4 {
            let val = Primitives::i32.read_be_from(raw)?;
            let val = VariableValue::Primitive(val);
            Ok(PType(val))
        } else if ty == &Type::INT8 {
            let val = Primitives::i64.read_be_from(raw)?;
            let val = VariableValue::Primitive(val);
            Ok(PType(val))
        } else if ty == &Type::BOOL {
            let val = Primitives::bool.read_be_from(raw)?;
            let val = VariableValue::Primitive(val);
            Ok(PType(val))
        } else if ty == &Type::VARCHAR {
            let s = String::from_utf8_lossy(raw).to_string();
            let val = DynamicallySizedValue::str(s);
            let val = VariableValue::DynamicallySized(val);
            Ok(PType(val))
        } else {
            todo!()
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::connection::{DbConn, DbConnBuilder};
    use crate::error::Error;
    use crate::reflect::Table;

    pub async fn test(conn: &mut DbConn) -> Result<(), Error> {
        let tables = Table::list(conn).await?;
        for table in &tables {
            println!("Table: {}.{}", table.table_schema, table.table_name);
            for column in &table.columns {
                println!(
                    "  Column: {} ({}/{})",
                    column.column_name, column.data_type, column.udt_name
                );
            }
            table.dump_data(conn).await?;
            break;
        }
        Ok(())
    }
    #[tokio::test]
    pub async fn test1() -> Result<(), Error> {
        let config = crate::connection::config_from_env();

        let (mut conn, worker) = DbConnBuilder::default()
            .try_load_environments()?
            .build_and_connect(config)
            .await?;
        let handl = tokio::spawn(worker);

        test(&mut conn).await?;
        drop(conn);

        if let Err(e) = handl.await {
            eprintln!("{e}");
        }

        Ok(())
    }
}
