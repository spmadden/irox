// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

use std::io::{BufRead, BufReader, Lines, Read};
use std::path::Path;

use log::{error, info};
use rusqlite::Connection;

pub use error::*;
use irox_enums::{EnumIterItem, EnumName};
use irox_time::format::iso8601::BASIC_CALENDAR_DATE;
pub use registry::*;
pub use types::*;

mod error;
mod registry;
mod types;

pub struct RIRParser<T: Read> {
    lines: Lines<BufReader<T>>,
}

impl<T: Read> RIRParser<T> {
    pub fn new(path: T) -> Self {
        let lines = BufReader::new(path).lines();
        RIRParser { lines }
    }
}

impl<T: Read> Iterator for RIRParser<T> {
    type Item = RecordType;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next()?.ok()?;
            if line.starts_with('#') {
                continue;
            } else if line.starts_with('2') {
                // parse version line.
                let version: VersionLine = split(&line).try_into().ok()?;
                return Some(RecordType::VersionLine(version));
            } else if line.ends_with("summary") {
                // parse summary line
                let summary: SummaryLine = split(&line).try_into().ok()?;
                return Some(RecordType::SummaryLine(summary));
            }
            // assume record line
            let record: Record = split(&line).try_into().ok()?;
            return Some(RecordType::Record(record));
        }
    }
}

pub struct RIRExchangeDatabase {
    connection: Connection,
}
const PRAGMAS: &str = "
PRAGMA locking_mode = EXCLUSIVE;
PRAGMA journal_mode = OFF;
PRAGMA synchronous = OFF;
PRAGMA page_size = 4096;
PRAGMA cache_size = 1048576;
";
const CREATE: &str = "
CREATE TABLE IF NOT EXISTS records (registry text, country_code text, entry_type text, start_value text, value int, date text, opaque_id text);
CREATE INDEX IF NOT EXISTS idx_country ON records (country_code);
CREATE INDEX IF NOT EXISTS idx_org ON records (opaque_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_records ON records (registry, country_code, entry_type,start_value,value,opaque_id);
";
impl RIRExchangeDatabase {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<RIRExchangeDatabase, Error> {
        let connection = Connection::open(path)?;

        connection.execute_batch(PRAGMAS)?;
        connection.execute_batch(CREATE)?;

        Ok(RIRExchangeDatabase { connection })
    }

    pub fn insert_row(&self, row: &Record) -> Result<(), Error> {
        let mut cache = self.connection.prepare_cached(
            "INSERT OR IGNORE INTO records \
        (registry, country_code, entry_type, start_value, value, date, opaque_id) values \
        (:registry, :country_code, :entry_type, :start_value, :value, :date, :opaque_id);",
        )?;
        cache.execute(&[
            (":registry", row.get_registry().name()),
            (
                ":country_code",
                row.get_country_code()
                    .map_or_else(|| "", |e| e.country_code()),
            ),
            (":entry_type", row.get_entry_type().name()),
            (":start_value", row.get_start_value().value().as_str()),
            (":value", format!("{}", row.get_count()).as_str()),
            (
                ":date",
                row.get_date()
                    .map_or_else(String::new, |e| e.format(&BASIC_CALENDAR_DATE))
                    .as_str(),
            ),
            (
                ":opaque_id",
                row.get_opaque_id()
                    .map_or_else(String::new, ToString::to_string)
                    .as_str(),
            ),
        ])?;
        Ok(())
    }

    pub fn import_from<T: Read>(&self, reader: T) -> Result<(), Error> {
        let mut incr = 0;
        let _txn = Transaction::new(&self.connection)?;
        for record in RIRParser::new(reader) {
            let RecordType::Record(record) = record else {
                continue;
            };
            self.insert_row(&record)?;
            incr += 1;
        }
        info!("Imported {incr} records");
        Ok(())
    }

    pub fn get_unique_organizations(&self) -> Result<Vec<String>, Error> {
        let mut out: Vec<String> = Vec::new();
        let mut prep = self
            .connection
            .prepare("select opaque_id from records GROUP by opaque_id;")?;
        let mut rows = prep.query([])?;
        while let Some(row) = rows.next()? {
            out.push(row.get(0)?);
        }
        Ok(out)
    }

    #[cfg(feature = "download")]
    pub fn download_from(&self, rir: RegionalRegistry) -> Result<(), Error> {
        info!("Starting download from: {rir}");
        let req = ureq::get(rir.url());
        let Ok(resp) = req.call() else {
            return Ok(());
        };
        self.import_from(resp.into_reader())?;
        Ok(())
    }

    #[cfg(feature = "download")]
    pub fn download_from_all_registries(&self) -> Result<(), Error> {
        for rir in RegionalRegistry::iter_items() {
            self.download_from(rir)?;
        }
        info!("Download complete.");
        Ok(())
    }
}

struct Transaction<'a> {
    connection: &'a Connection,
}
impl<'a> Transaction<'a> {
    pub fn new(conn: &'a Connection) -> Result<Self, Error> {
        conn.execute_batch("BEGIN TRANSACTION;")?;
        Ok(Transaction { connection: conn })
    }
}
impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        let res = self.connection.execute_batch("COMMIT TRANSACTION;");
        if let Err(e) = res {
            error!("Error committing transaction: {e:?}")
        }
    }
}
fn split(value: &str) -> Vec<String> {
    value.splitn(8, '|').map(ToString::to_string).collect()
}

#[cfg(test)]
mod test {
    use crate::RIRExchangeDatabase;

    #[test]
    #[ignore]
    pub fn test() -> Result<(), crate::Error> {
        println!("{:?}", std::env::current_dir()?);
        let output = "test.db";

        let db = RIRExchangeDatabase::open(output)?;
        // db.import_from(std::io::BufReader::new(std::fs::File::open("tests/assets/delegated-apnic-extended-latest")?))?;
        // db.download_from(crate::RegionalRegistry::apnic)?;
        let orgs = db.get_unique_organizations()?;
        println!("{}", orgs.len());

        Ok(())
    }
}
