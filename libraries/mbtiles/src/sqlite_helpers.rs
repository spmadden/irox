// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::future::Future;
use std::fmt::Display;

use sqlite::{Connection, State, Statement};

use crate::{Error, Result};

#[derive(Debug, Clone, Copy, Default)]
pub enum JournalMode {
    /// Normal behavior.  The rollback journal is deleted after each transaction.
    #[default]
    Delete,

    /// Truncates journal rather than deleting it
    Truncate,

    /// Journal header is overwritten with zeros instead of deleting it
    Persist,

    /// Stores the journal in RAM
    Memory,

    /// Write-Ahead-Log
    WAL,

    /// Journaling & Protection off
    Off,
}

impl Display for JournalMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JournalMode::Delete => write!(f, "DELETE"),
            JournalMode::Truncate => write!(f, "TRUNCATE"),
            JournalMode::Persist => write!(f, "PERSIST"),
            JournalMode::Memory => write!(f, "MEMORY"),
            JournalMode::WAL => write!(f, "WAL"),
            JournalMode::Off => write!(f, "OFF"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum LockingMode {
    /// Unlocks the DB at the end of each transaction
    #[default]
    Normal,

    /// Maintains a lock over all transactions until the connection is closed.
    Exclusive,
}

impl Display for LockingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingMode::Normal => write!(f, "NORMAL"),
            LockingMode::Exclusive => write!(f, "EXCLUSIVE"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum SynchronousMode {
    /// fsync's after each transaction
    #[default]
    Full,

    /// full + more fsynchs
    Extra,

    /// less synchs (safe with journaling)
    Normal,

    /// no OS/FS synching
    Off,
}

impl Display for SynchronousMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SynchronousMode::Full => write!(f, "FULL"),
            SynchronousMode::Extra => write!(f, "EXTRA"),
            SynchronousMode::Normal => write!(f, "NORMAL"),
            SynchronousMode::Off => write!(f, "OFF"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pragma {
    ApplicationId(u32),

    /// Number of pages to keep in memory as the cache
    CacheSizePages(u32),

    /// Amount of memory (in bytes) to use as the cache
    CacheSizeBytes(u64),

    /// Page size in bytes for the new database
    PageSizeBytes(u32),

    /// Adjusts the journal mode
    JournalMode(JournalMode),

    JournalSizeLimitBytes(u64),

    LockingMode(LockingMode),

    SynchronousMode(SynchronousMode),
}

impl Pragma {
    pub fn name(&self) -> &'static str {
        match self {
            Pragma::ApplicationId(_) => "application_id",
            Pragma::CacheSizeBytes(_) => "cache_size",
            Pragma::PageSizeBytes(_) => "page_size",
            Pragma::CacheSizePages(_) => "cache_size_pages",
            Pragma::JournalMode(_) => "journal_mode",
            Pragma::JournalSizeLimitBytes(_) => "journal_size_limit_bytes",
            Pragma::LockingMode(_) => "locking_mode",
            Pragma::SynchronousMode(_) => "synchronous_mode",
        }
    }

    pub fn value(&self) -> String {
        match self {
            Pragma::ApplicationId(i) => format!("{i}"),
            Pragma::CacheSizePages(i) => format!("{i}"),
            Pragma::CacheSizeBytes(i) => format!("-{i}"),
            Pragma::PageSizeBytes(i) => format!("{i}"),
            Pragma::JournalMode(i) => format!("{i}"),
            Pragma::JournalSizeLimitBytes(i) => format!("{i}"),
            Pragma::LockingMode(i) => format!("{i}"),
            Pragma::SynchronousMode(i) => format!("{i}"),
        }
    }

    pub fn get(&self, conn: &Connection) -> Result<i64> {
        let name = self.name();
        let st = conn.prepare(format!("pragma {name} ;"))?;

        if let Some(row) = st.into_iter().next() {
            let row = row?;
            let res: i64 = row.try_read(0)?;
            return Ok(res);
        }

        Error::not_found("Pragma not found, returned 0 rows")
    }

    pub fn set(&self, conn: &Connection) -> Result<()> {
        let name = self.name();

        conn.execute(format!("pragma {name} = {};", self.value()))?;
        Ok(())
    }
}

pub trait Executable<'a> {
    fn execute(&mut self) -> Result<()>;
}

impl<'a> Executable<'a> for &mut Statement<'a> {
    fn execute(&mut self) -> Result<()> {
        while self.next()? != State::Done {
            // spin
        }
        Ok(())
    }
}

impl<'a> Executable<'a> for Statement<'a> {
    fn execute(&mut self) -> Result<()> {
        while self.next()? != State::Done {
            // spin
        }
        Ok(())
    }
}

// MPBX application id "Map Box"
pub const APPLICATION_ID: u32 = 0x4d504258;
