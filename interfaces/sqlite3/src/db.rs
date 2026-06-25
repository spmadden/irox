use core::fmt::{Debug, Formatter};
use irox_bits::{Bits, BitsWrapper, Seek};

use crate::{
    error::Error,
    header::Header,
    page::{self, PageType},
};

pub struct Database<'a, T: Bits + Seek> {
    pub header: Header,
    pub file: BitsWrapper<'a, T>,
}
impl<'a, T: Bits + Seek> Debug for Database<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Database")
            .field("header", &self.header)
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "std")]
impl<'a> Database<'a, std::fs::File> {
    pub fn open_db_path<R: AsRef<std::path::Path>>(
        path: &'_ R,
    ) -> Result<Database<'_, std::fs::File>, Error> {
        open_db(path)
    }
}
impl<'a, T: Bits + Seek> Database<'a, T>
where
    BitsWrapper<'a, T>: Bits,
{
    pub fn read_page(&mut self, page_id: u32) -> Result<PageType, Error> {
        page::read_page(&mut self.file, page_id, &self.header)
    }

    pub fn open_db_bits(mut bits: BitsWrapper<'a, T>) -> Result<Database<'a, T>, Error> {
        Ok(Database {
            header: Header::read_from(&mut bits)?,
            file: bits,
        })
    }
}

#[cfg(feature = "std")]
pub fn open_db<R: AsRef<std::path::Path>>(
    path: &'_ R,
) -> Result<Database<'_, std::fs::File>, Error> {
    let file = std::fs::File::open(path)?;
    let mut file = BitsWrapper::Owned(file);
    let header = Header::read_from(&mut file)?;

    Ok(Database { header, file })
}
