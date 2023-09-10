// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_csv::error::CSVError;

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
    Ok(())
}
