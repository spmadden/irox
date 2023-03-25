use std::path::PathBuf;

use git::{Repository};

use crate::error::Error;
use crate::git::{Blob, Object};
use crate::sha1::SHA1;

mod git;
mod error;
mod sha1;
mod utils;

fn main() -> Result<(), Error>{
    let rep = Repository::new(PathBuf::from("C:\\proj\\rust-osm\\.git\\objects"));
    let hash : SHA1 = "79679b78e51a54d424e21378f3787f69b26b35f8".try_into()?;
    
    let read = git::read_object(&hash, &rep)?;
    println!("Read: {:?}", read);

    
    let raw : [u8;20] = [0x3a, 0x16, 0x09, 0xbd, 0x18, 0x47, 0xdd, 0xe9, 0x69, 0xd1, 0x21, 0x52, 0x8d, 0xe7, 0x1c, 0x50, 0x4d, 0x67, 0xb1, 0x9a];
    let hash = SHA1::new(raw);
    println!("SHA: {}", hash);
    let obj = git::read_object(&hash, &rep)?;
    println!("Result: {:?}", obj);
    
    let valid = obj.validate();
    println!("Valid: {:?}", valid);

    
    Ok(())
}
