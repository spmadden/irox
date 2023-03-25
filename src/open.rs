use pdf::file::{File, FileOptions, ObjectCache, StreamCache};
use pdf::object::ParseOptions;

pub type PDF = File<Vec<u8>, ObjectCache, StreamCache>;

pub fn open(input_file: &String) -> PDF {
    return FileOptions::cached()
        .parse_options(ParseOptions::tolerant())
        .open(input_file)
        .expect(format!("Error opening PDF file {}", input_file).as_str());

}