use std::{fs::File, path::Path};
use std::fmt::{Debug, Formatter};
use irox_tools::bits::{Bits, BitsSeek, BitsWrapper, Seek};

use crate::{
    error::Error,
    header::Header,
    page::{self, PageType},
};


pub struct Database<'a> {
    pub header: Header,
    pub file: BitsWrapper<'a, dyn BitsSeek>,
}
impl<'a> Debug for Database<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("header", &self.header)
            .finish_non_exhaustive()
    }
}

impl<'a> Database<'a> {
    pub fn read_page(&mut self, page_id: u32) -> Result<PageType, Error> {
        page::read_page(&mut self.file, page_id, &self.header)
    }

    #[cfg(feature = "std")]
    pub fn open_db_path(path: &impl AsRef<Path>) -> Result<Database, Error> {
        open_db(path)
    }

    pub fn open_db_bits(bits: BitsWrapper<dyn BitsSeek>) -> Result<Database, Error> {
        Ok(Database {
            header: Header::read_from(bits.as_mut())?,
            file: bits
        })
    }
}

#[cfg(feature = "std")]
pub fn open_db(path: &impl AsRef<Path>) -> Result<Database, Error> {
    let mut file = File::open(path)?;
    let header = Header::read_from(&mut file)?;

    Ok(Database { header, file })
}
