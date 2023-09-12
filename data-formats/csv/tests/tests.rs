// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_csv::error::CSVError;
use irox_csv::CSVWriterBuilder;
use std::collections::BTreeMap;

static INPUT_1: &str = "header1,header2,header3,header4
one,two,three,four\nfive,six,seven,eight\r\n1,2,3,4
5.1,6.2,7.3,8.4,\n\n\n\n\n\
\"a long, key\",\"a long key\nwith\r\nnewlines\"
\"quoted\"\"inner long\nnewlines\",second
";

#[test]
pub fn test_reader_1() -> Result<(), CSVError> {
    let mut reader = irox_csv::CSVReader::new(INPUT_1.as_bytes());

    let mut idx = 0;
    while let Some(line) = reader.read_line()? {
        match idx {
            0 => {
                assert_eq!(vec!["header1", "header2", "header3", "header4"], line);
            }
            1 => {
                assert_eq!(vec!["one", "two", "three", "four"], line);
            }
            2 => {
                assert_eq!(vec!["five", "six", "seven", "eight"], line);
            }
            3 => {
                assert_eq!(vec!["1", "2", "3", "4"], line);
            }
            4 => {
                assert_eq!(vec!["5.1", "6.2", "7.3", "8.4", ""], line);
            }
            5 => {
                assert_eq!(vec!["a long, key", "a long key\nwith\r\nnewlines"], line);
            }
            6 => {
                assert_eq!(vec!["quoted\"inner long\nnewlines", "second"], line);
            }
            e => {
                panic!("More lines than expected: {}", e);
            }
        }
        idx += 1;
    }
    assert_eq!(7, idx);
    Ok(())
}

static TEST_2: &str = "hdr1,hdr2,hdr3
elem1,elem2,elem3
elem4,elem5,elem6
elem7,elem8,elem9
";

#[test]
pub fn test2() -> Result<(), CSVError> {
    let mut reader = irox_csv::CSVMapReader::new(TEST_2.as_bytes())?;

    let mut idx = 0;
    while let Some(row) = reader.next_row()? {
        let map = row.into_map_lossy();
        match idx {
            0 => {
                assert_eq!(3, map.len());
                assert_eq!(Some(&"elem1".to_string()), map.get("hdr1"));
                assert_eq!(Some(&"elem2".to_string()), map.get("hdr2"));
                assert_eq!(Some(&"elem3".to_string()), map.get("hdr3"));
            }
            1 => {
                assert_eq!(3, map.len());
                assert_eq!(Some(&"elem4".to_string()), map.get("hdr1"));
                assert_eq!(Some(&"elem5".to_string()), map.get("hdr2"));
                assert_eq!(Some(&"elem6".to_string()), map.get("hdr3"));
            }
            2 => {
                assert_eq!(3, map.len());
                assert_eq!(Some(&"elem7".to_string()), map.get("hdr1"));
                assert_eq!(Some(&"elem8".to_string()), map.get("hdr2"));
                assert_eq!(Some(&"elem9".to_string()), map.get("hdr3"));
            }
            e => {
                panic!("More lines than expected: {}", e);
            }
        }
        idx += 1;
    }
    assert_eq!(3, idx);
    Ok(())
}

#[test]
pub fn writer_1() -> Result<(), CSVError> {
    let mut buf: Vec<u8> = Vec::new();
    {
        let mut writer = CSVWriterBuilder::new().build(&mut buf);

        writer.write_line(&["first", "second", "third"])?;
        writer.write_line(&["4th", "5th", "6th"])?;
    }
    assert_eq!("first,second,third\n4th,5th,6th\n".as_bytes(), buf);

    Ok(())
}

#[test]
pub fn writer_2() -> Result<(), CSVError> {
    let mut buf: Vec<u8> = Vec::new();
    {
        let mut writer = CSVWriterBuilder::new()
            .with_columns(&["first", "second", "third"])
            .build(&mut buf);

        let mut map = BTreeMap::new();
        map.insert("first".to_string(), "4th".to_string());
        map.insert("second".to_string(), "5th".to_string());
        map.insert("third".to_string(), "6th".to_string());
        writer.write_fields(&map)?;
    }
    assert_eq!("first,second,third\n4th,5th,6th\n".as_bytes(), buf);

    Ok(())
}
