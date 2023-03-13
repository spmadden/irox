// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::future::Future;

use sqlite::{Connection, State, Statement};

use crate::{Error, Result};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pragma {
    ApplicationId,
    CacheSize,
    PageSize,
}

impl Pragma {
    pub fn name(&self) -> &'static str {
        match self {
            Pragma::ApplicationId => "application_id",
            Pragma::CacheSize => "cache_size",
            Pragma::PageSize => "page_size",
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

    pub fn set(&self, conn: &Connection, val: i64) -> Result<()> {
        let name = self.name();
        let mut st = conn.prepare(format!("pragma {name} = {val};"))?;

        st.execute()
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
pub const APPLICATION_ID: i64 = 0x4d504258;
