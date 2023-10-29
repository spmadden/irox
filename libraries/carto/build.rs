// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use std::collections::BTreeMap;
use std::env;
use std::io::Write;
use std::path::Path;

use irox_csv::{CSVError, CSVErrorType, CSVReader};

#[allow(clippy::unwrap_used)]

fn main() -> Result<(), irox_csv::CSVError> {
    println!("cargo:rerun-if-changed=data/iso-3166-1-country-codes.csv");
    println!("cargo:rerun-if-changed=build.rs");

    let datafile = std::fs::File::open("data/iso-3166-1-country-codes.csv")?;
    let mut reader = CSVReader::new(datafile);

    let mut country_data = BTreeMap::new();
    let _first = reader.read_line()?;
    while let Some(row) = reader.read_line()? {
        let Some(country) = row.get(0) else {
            return CSVError::err(
                CSVErrorType::MissingHeaderError,
                "Missing country".to_string(),
            );
        };
        let Some(code) = row.get(1) else {
            return CSVError::err(CSVErrorType::MissingHeaderError, "Missing code".to_string());
        };
        country_data.insert(country.replace('\"', "").clone(), code.clone());
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir);
    std::fs::create_dir_all(dest_path)?;
    let dest_file = dest_path.join("countries.rs");
    let mut dest_file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(dest_file)?;
    dest_file.set_len(0)?;

    dest_file.write_all("
        use irox_enums::{EnumName, EnumIterItem, EnumTryFromStr};
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, EnumName, EnumIterItem, EnumTryFromStr)]
        #[non_exhaustive]
        pub enum CountryCode {
    ".as_ref())?;

    for code in country_data.values() {
        dest_file.write_all(
            format!(
                "
            {code},"
            )
            .as_bytes(),
        )?;
    }
    dest_file.write_all(
        "
        }
        impl CountryCode {
            #[must_use]
            pub fn country_name(&self) -> &'static str {
                match self {
    "
        .as_ref(),
    )?;
    for (country, code) in &country_data {
        dest_file.write_all(
            format!(
                "
                    Self::{code} => \"{country}\","
            )
            .as_bytes(),
        )?;
    }
    dest_file.write_all(
        "
                }
            }
            #[must_use]
            pub fn country_code(&self) -> &'static str {
                self.name()
            }
        }
    "
        .as_ref(),
    )?;

    Ok(())
}
