use std::{path::Path, fs::File};

use crate::header::Header;

pub struct Database {
    pub header : Header,
    pub file : File
}

pub fn open(path: &impl AsRef<Path>) -> Result<Database, ()> {

    todo!()
}