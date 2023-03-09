use std::{fs::File, path::Path};

use crate::{
    error::Error,
    header::Header,
    page::{self, PageType},
};

#[derive(Debug)]
pub struct Database {
    pub header: Header,
    pub file: File,
}

impl Database {
    pub fn read_page(&mut self, page_id: u32) -> Result<PageType, Error> {
        page::read_page(&mut self.file, page_id, &self.header)
    }

    pub fn open_db(path: &impl AsRef<Path>) -> Result<Database, Error> {
        open_db(path)
    }
}

pub fn open_db(path: &impl AsRef<Path>) -> Result<Database, Error> {
    let mut file = File::open(path)?;
    let header = Header::read_from(&mut file)?;

    Ok(Database { header, file })
}
