use std::{io::{BufReader, BufRead, Read}, path::PathBuf, fmt::Debug, fs::File};
use flate2::read::ZlibDecoder;
use sha1::{Digest, Sha1};

use crate::error::Error;
use crate::sha1::SHA1;

#[derive(Clone, Debug)]
pub enum ObjectType{
    Blob(Blob),
    Tree(Tree),
    Commit(Commit)
}

pub trait Object {
    fn get_hash(&self) -> SHA1;
    fn get_type(&self) -> ObjectType;
    fn get_length(&self) -> usize;
    fn get_repository(&self) -> Repository;

    fn validate(&self) -> Result<(), Error>{
        let mut verifier = Sha1::new();
        let hash = self.get_hash();
        let mut read = open_object(&hash, &self.get_repository())?;
        loop {
            let buf = read.fill_buf() ?;
            let len = buf.len();
            if len <= 0 {
                break;
            }
            verifier.update(buf);
            read.consume(len);
        }
        let check_raw : [u8;20] = verifier.finalize().into();
        if !check_raw.eq(&hash.raw) {
            return Err(format!("Hash verification failed for {:?}", hash).into());
        }
        Ok(())
    }
}

impl Object for ObjectType {
    fn get_hash(&self) -> SHA1 {
        match self {
            ObjectType::Blob(b) => b.get_hash(),
            ObjectType::Tree(t) => todo!(),
            ObjectType::Commit(c) => todo!(),
        }
    }

    fn get_type(&self) -> ObjectType {
        match self {
            ObjectType::Blob(b) => b.get_type(),
            ObjectType::Tree(t) => todo!(),
            ObjectType::Commit(c) => todo!(),
        }
    }

    fn get_length(&self) -> usize {
        match self {
            ObjectType::Blob(b) => b.get_length(),
            ObjectType::Tree(t) => todo!(),
            ObjectType::Commit(c) => todo!(),
        }
    }

    fn get_repository(&self) -> Repository {
        match self {
            ObjectType::Blob(b) => b.get_repository(),
            ObjectType::Tree(t) => todo!(),
            ObjectType::Commit(c) => todo!(),
        }
    }
}


pub fn open_object(hash : &SHA1, repo : &Repository) -> Result<impl BufRead, Error> {
    let file = repo.path.join(&hash.dir).join(&hash.file);
    println!("Trying to open: {:?}", file);
    let dec = ZlibDecoder::new(File::open(file)?);
    Ok(BufReader::new(dec))
}

pub fn read_object(hash : &SHA1, repo : &Repository) -> Result<ObjectType, Error> {
    let mut verifier = Sha1::new();
    let mut read = open_object(hash, repo)?;

    let mut buf : Vec<u8> = Vec::new();
    let type_len = read.read_until(' ' as u8, &mut buf)?;
    verifier.update(&buf);
    let obj_type = std::str::from_utf8(&buf[..type_len-1])?;
    println!("Read {:?}", obj_type);
    
    let mut buf : Vec<u8> = Vec::new();
    let len_len = read.read_until(0, &mut buf)?;
    verifier.update(&buf);
    let length = std::str::from_utf8(&buf[..len_len-1])?.parse::<usize>()?;

    match obj_type {
        "blob" => Ok(ObjectType::Blob(Blob{hash:hash.clone(), length, repo:repo.clone()})),
        "tree" => todo!(),
        "commit" => {
            let res = <Commit>::try_from(&mut read)?;
            Ok(ObjectType::Commit(res))
        },
        _ => todo!()
    }
}

pub fn create_blob(data : &[u8], repo: &Repository) -> Result<Blob, Error> {
    let raw : [u8;20] = sha1::Sha1::digest(data).into();
    let hash = SHA1::new(raw);
    let length = data.len();
    return Ok(Blob { hash, length, repo: repo.clone()});
}

#[derive(Clone, Debug)]
pub struct Blob {
    hash : SHA1,
    length : usize,
    repo : Repository,
}

impl Object for Blob {
    fn get_hash(&self) -> SHA1 {
        self.hash.clone()
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Blob(self.clone())
    }

    fn get_length(&self) -> usize {
        self.length
    }

    fn get_repository(&self) -> Repository {
        self.repo.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Tree {
    hash : SHA1,
}

#[derive(Clone, Debug)]
pub struct Commit {
    hash : SHA1,
    length : usize,
    repo : Repository,
    fields : Vec<(String, String)>
}

impl Commit {
    fn try_from<T: BufRead> (reader: &mut T) -> Result<Commit, Error> {
        let fields : Vec<(String, String)> = Vec::new();
        loop {
            let mut key : Vec<u8> = Vec::new();
            let key_size = reader.read_until(' ' as u8, &mut key)?;
            if key_size <= 0 {
                break;
            }
            let mut val : Vec<u8> = Vec::new();
            let val_size = reader.read_until('\n' as u8, &mut val)?;
            if val_size <= 0 {
                break;
            }
            let strkey = std::str::from_utf8(&mut key)?;
            let strval = std::str::from_utf8(&mut val)?;
            println!("{:?} {:?}", strkey, strval);
        }

        todo!();
    }
}
impl Object for Commit {
    fn get_hash(&self) -> SHA1 {
        self.hash.clone()
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Commit(self.clone())
    }

    fn get_length(&self) -> usize {
        self.length
    }

    fn get_repository(&self) -> Repository {
        self.repo.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Repository {
    path : PathBuf
}

impl Repository {
    pub fn new(path : PathBuf) -> Repository {
        Repository { path }
    }
}
