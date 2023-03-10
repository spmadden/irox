use std::fs::File;
use std::io::{BufRead, BufReader, Bytes, Read, Take, Cursor, ErrorKind, Seek};
use std::path::Path;

use log::info;

use self::error::{pbf_error, pbf_int_error, PBFError};

pub(crate) fn zz_decode(val: u64) -> f64 {
    return ((val << 1) ^ (val >> 63)) as f64;
}
pub(crate) fn zz_encode(val: f64) -> u64 {
    let i: u64 = val as u64;
    return (i >> 1) ^ (-((i & 1) as f64) as u64);
}

pub enum FieldID {
    Ok(u64, WireType),
    Err(String),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum WireType {
    VARINT,
    FIXED_64,
    LEN_DELIM,
    FIXED_32,
}
impl WireType {
    pub fn decode_field_id(val: u64) -> FieldID {
        let wt = val & 0x7;
        let fid = val >> 3;

        match wt {
            0 => return FieldID::Ok(fid, WireType::VARINT),
            1 => return FieldID::Ok(fid, WireType::FIXED_64),
            2 => return FieldID::Ok(fid, WireType::LEN_DELIM),
            5 => return FieldID::Ok(fid, WireType::FIXED_32),
            e => return FieldID::Err(format!("Unknown wire type {}", e)),
        }
    }
}

pub mod error {
    use std::error::Error;
    use std::fmt::Display;
    use std::num::TryFromIntError;
    use std::string::FromUtf8Error;

    #[derive(Debug)]
    pub struct PBFError {
        message: String,
    }
    impl Error for PBFError {}
    impl Display for PBFError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            return f.write_fmt(format_args!("PBFError({})", self.message));
        }
    }
    impl From<std::io::Error> for PBFError {
        fn from(err: std::io::Error) -> Self {
            PBFError {
                message: err.to_string(),
            }
        }
    }
    impl From<TryFromIntError> for PBFError {
        fn from(err: TryFromIntError) -> Self {
            PBFError {
                message: err.to_string(),
            }
        }
    }
    impl From<FromUtf8Error> for PBFError {
        fn from(err: FromUtf8Error) -> Self {
            PBFError {
                message: err.to_string(),
            }
        }
    }
    impl PBFError {
        pub fn new(msg: &str) -> PBFError {
            return PBFError {
                message: msg.into(),
            };
        }
    }
    pub fn pbf_error<T>(err: std::io::Error) -> Result<T, PBFError> {
        return Result::Err(PBFError::from(err));
    }
    pub fn pbf_int_error<T>(err: TryFromIntError) -> Result<T, PBFError> {
        return Result::Err(PBFError::from(err));
    }
}

pub struct PBFStream<R> {
    reader: BufReader<R>,
    length: u64
}

impl TryFrom<&Path> for PBFStream<File> {
    type Error = PBFError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let length = file.metadata()?.len();
        let reader = BufReader::new(file);
        Ok(PBFStream { reader, length })
    }
}

impl From<Vec<u8>> for PBFStream<Cursor<Vec<u8>>> {

    fn from(val: Vec<u8>) -> Self {
        let length = val.len() as u64;
        let reader = BufReader::new(Cursor::new(val));
        PBFStream { reader, length }
    }
}

impl<R:Read+Seek> PBFStream<R> {
    
    pub fn read_varint(&mut self) -> Result<u64, PBFError> {
        let mut last: u8 = 0x80;
        let mut val: u64 = 0;
        let mut bytes = self.reader.by_ref().bytes();

        let mut shift = 0;
        while last & 0x80 != 0 {
            if let Some(next) = bytes.next(){
                last = next?;
                val |= (last as u64 & 0x7F) << shift;
                shift += 7;
            }
        }
        return Ok(val);
    }

    pub fn read_fixed_u32(&mut self) -> Result<u32, PBFError> {
        let mut buf: [u8; 4] = [0; 4];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => Ok(u32::from_be_bytes(buf)),
            Err(err) => pbf_error(err),
        }
    }

    pub fn read_string(&mut self) -> Result<String, PBFError> {
        let buf = self.read_delim_bytes()?;
        Ok(String::from_utf8(buf.to_vec())?)
    }

    pub fn read_svarint(&mut self) -> Result<f64, PBFError> {
        Ok(zz_decode(self.read_varint()?))
    }

    pub fn read_bytes(&mut self, limit: usize) -> Result<Vec<u8>, PBFError> {
        let mut buf = vec![0 as u8;limit];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_delim_bytes(&mut self) -> Result<Vec<u8>, PBFError> {
        let len = self.read_varint()?;
        let size: usize = len.try_into()?;
        let buf = self.read_bytes(size)?;
        Ok(buf)
    }

    pub fn read_fieldid(&mut self) -> Result<FieldID, PBFError> {
        let enc = self.read_varint()?;
        Ok(WireType::decode_field_id(enc))
    }

    pub fn has_more(&mut self) -> Result<bool, PBFError> {
        return Ok(self.reader.stream_position()? < self.length);
    }

}

// trait PBF {
    
// }