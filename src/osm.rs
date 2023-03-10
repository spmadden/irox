use std::{
    fmt::Display,
    fs::File,
    path::Path,
};

use log::info;
use rust_pbf::PBF;

use crate::pbf::{self, FieldID};

use self::error::OSMError;

#[derive(Debug, PBF)] 
pub struct BlobHeader {

    #[FieldID = 1]
    type_str: String,

    #[FieldID = 2]
    datasize: u32,

    #[FieldID = 3]
    indexdata: [u8;32],
}
impl Display for BlobHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indxdata = match &self.indexdata {
            Some(data) => format!("Some({})", data.len()),
            None => "NONE".to_string(),
        };
        f.write_str(
            format!(
                "BlobHeader {{ type_str: {}, datasize: {}, indexdata: {} }}",
                self.type_str, self.datasize, indxdata
            )
            .as_str(),
        )
    }
}

impl BlobHeader {
    pub fn new(data: Vec<u8>) -> Result<BlobHeader, OSMError> {
        let mut stream = pbf::PBFStream::from(data);

        let mut type_str: Option<String> = None;
        let mut datasize: Option<u32> = None;
        let mut indexdata: Option<Vec<u8>> = None;

        while stream.has_more()? {
            if let FieldID::Ok(id, wt) = stream.read_fieldid()? {
                match id {
                    1 => type_str = Some(stream.read_string()?),
                    2 => indexdata = Some(stream.read_delim_bytes()?),
                    3 => datasize = Some(stream.read_varint()? as u32),
                    e => return Err(OSMError::from(format!("Unknown field id {}", e))),
                }
            }
        }
        if type_str.is_none() {
            return Err(OSMError::from("Type is required"));
        }
        if datasize.is_none() {
            return Err(OSMError::from("Data size is required."));
        }

        Ok(BlobHeader {
            type_str: type_str.unwrap(),
            datasize: datasize.unwrap(),
            indexdata,
        })
    }
}

#[derive(Debug)]
pub enum BlobData {
    RAW(Vec<u8>),
    ZLIB(Vec<u8>),
    LZMA(Vec<u8>),
    BZ2(Vec<u8>),
    LZ4(Vec<u8>),
    ZSTD(Vec<u8>),
}

#[derive(Debug)]
pub struct Blob {
    raw_size: u32,
    data: BlobData,
}
impl Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let datastr = match &self.data {
            BlobData::RAW(v) => format!("RAW({})", v.len()),
            BlobData::ZLIB(v) => format!("ZLIB({})", v.len()),
            BlobData::LZMA(v) => format!("LZMA({})", v.len()),
            BlobData::BZ2(v) => format!("BZ2({})", v.len()),
            BlobData::LZ4(v) => format!("LZ4({})", v.len()),
            BlobData::ZSTD(v) => format!("ZSTD({})", v.len()),
        };
        f.write_str(format!("Blob {{ raw_size: {}, data: {} }}", self.raw_size, datastr).as_str())
    }
}

impl Blob {
    pub fn new(input: Vec<u8>) -> Result<Blob, OSMError> {
        let mut stream = pbf::PBFStream::from(input);

        let mut raw_size: Option<u32> = None;
        let mut data: Option<BlobData> = None;
        while stream.has_more()? {
            if let FieldID::Ok(id, wt) = stream.read_fieldid()? {
                match id {
                    2 => raw_size = Some(stream.read_varint()? as u32),
                    1 => data = Some(BlobData::RAW(stream.read_delim_bytes()?)),
                    3 => data = Some(BlobData::ZLIB(stream.read_delim_bytes()?)),
                    4 => data = Some(BlobData::LZMA(stream.read_delim_bytes()?)),
                    5 => data = Some(BlobData::BZ2(stream.read_delim_bytes()?)),
                    6 => data = Some(BlobData::LZ4(stream.read_delim_bytes()?)),
                    7 => data = Some(BlobData::ZSTD(stream.read_delim_bytes()?)),
                    els => return Err(OSMError::from(format!("Unknown field ID: {}", els))),
                }
            }
        }
        if raw_size.is_none() {
            return Err(OSMError::from("Raw Size is required"));
        }
        if data.is_none() {
            return Err(OSMError::from("Data is required."));
        }

        Ok(Blob {
            raw_size: raw_size.unwrap(),
            data: data.unwrap(),
        })
    }
}

pub struct OSMFile {}

impl OSMFile {
    pub fn scan(path: &Path) {
        let mut stream =
            pbf::PBFStream::<File>::try_from(path).expect("Unable to open input file.");

        let mut blocks = 0;
        while stream.has_more().unwrap() {
            let next_hdr_size = stream.read_fixed_u32().expect("Error.") as usize;

            let bytes = stream
                .read_bytes(next_hdr_size)
                .expect("Error reading bytes");

            let header = BlobHeader::new(bytes).expect("Error reading header");
            // info!("Header: {}", header);

            let bytes = stream
                .read_bytes(header.datasize.try_into().unwrap())
                .expect("msg");
            let blob = Blob::new(bytes).expect("Error parsing blob");
            // info!("Blob: {}", blob);

            blocks += 1;
            if blocks % 100 == 0 {
                info!("Blocks read: {}", blocks);
            }
        }
        info!("Final read: {}", blocks);
        return ();
    }
}

pub mod error {
    use std::{error::Error, fmt::Display};

    use crate::pbf::error::PBFError;

    #[derive(Debug)]
    pub struct OSMError {
        message: String,
    }
    impl Display for OSMError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("OSMError: {}", self.message))
        }
    }
    impl Error for OSMError {}

    impl From<PBFError> for OSMError {
        fn from(err: PBFError) -> Self {
            OSMError {
                message: err.to_string(),
            }
        }
    }
    impl From<&str> for OSMError {
        fn from(msg: &str) -> Self {
            OSMError {
                message: String::from(msg),
            }
        }
    }
    impl From<String> for OSMError {
        fn from(message: String) -> Self {
            OSMError { message }
        }
    }
}
