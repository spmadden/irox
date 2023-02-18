use pdf::file::File;
use pdf::object::ParseOptions;

pub fn open(input_file: &String) -> File<Vec<u8>> {
    let filedata = match std::fs::read(input_file) {
        Ok(data) => data,
        Err(err) => {
            panic!("Error opening file {}: {}", input_file, err);
        }
    };
    
    match File::from_data_with_options(filedata, ParseOptions::tolerant()) {
        Err(err) => {
            panic!("{}", format_args!("Error opening PDF file {:?}: {:?}", input_file, err));
        }
        Ok(pdf) => pdf
    }
}