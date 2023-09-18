IROX CSV Encoder/Decoder
=========================

Inspired by Python's [`csv`](https://docs.python.org/3/library/csv.html) module, a very basic csv reader & writer.

The primary use-case of this library is interacting with unstructured, variably structured, or dubiously structured data.  As such, you probably want the far more robust and far better implemented [`csv` crate](https://crates.io/crates/csv).

Goals: 
 * Provide a [`String`](https://doc.rust-lang.org/std/string/struct.String.html)-based mechanism to read and write CSV files in [`rfc4180`](doc/rfc4180.txt) format.
 * Handle mixed formats resiliently, such as mismatched and newlines within quotes.

Non-Goals:
 * Any interpretation of the contents of the CSV structure itself - everything is an owned [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
 * [`serde`](https://crates.io/crates/serde) support - if you need it, go use the [`csv` crate](https://crates.io/crates/csv).

Examples:
----------
* Straight Iteration:
```rust
use irox_csv::error::CSVError;

fn iter_example() -> Result<(), CSVError> {
    let mut input = irox_csv::CSVReader::new(std::io::stdin());
    loop {
        // iterate over each line of the input
        let line : Option<Vec<String>> = input.read_line()?;
        match line {
            Some(fields) => {
                // Use the individual fields of the CSV line
                println!("{:?}", fields); // fields : Vec<String>
            }
            None => {
                // EOF
                break;
            }
        }
    }
    Ok(())
}
```

* Map Iteration:
```rust
use irox_csv::error::CSVError;

fn map_example() -> Result<(), CSVError> {
    let mut maps = irox_csv::CSVMapReader::new(std::io::stdin());
    loop {
        // iterate over each line of the input
        let maybe_row : Option<Row> = maps.next_row()?;
        match maybe_row {
            Some(row) => {
                // Use the individual fields of the CSV line as a key-value map
                // The keys are the column headers found in the first row, the values are the matching row entry
                let map = row.into_map_lossy();
                println!("{:?}", map); // map : BTree<column:String, rowVal:String>
            }
            None => {
                // EOF
                break;
            }
        }
    }
    Ok(())
}
```

* Writing a CSV File using Maps:
```rust
fn map_writer_example() -> Result<(), CSVError> {
    let mut buf: Vec<u8> = Vec::new();
    let mut writer = CSVWriterBuilder::new()
        .with_columns(&["first", "second", "third"])
        .build(&mut buf);

    let mut map = BTreeMap::new();
    map.insert("first".to_string(), "firstColFirstRowVal".to_string());
    map.insert("second".to_string(), "secondColFirstRowVal".to_string());
    map.insert("third".to_string(), "thirdColFirstRowVal".to_string());
    writer.write_fields(&map)?;
    
    map.clear();
    map.insert("first".to_string(), "firstColSecondRowVal".to_string());
    map.insert("second".to_string(), "secondColSecondRowVal".to_string());
    map.insert("third".to_string(), "thirdColSecondRowVal".to_string());
    writer.write_fields(&map)?;
    
    Ok(())
}
```
will result in a buffer:
```csv
first,second,third
firstColFirstRowVal,secondColFirstRowVal,thirdColFirstRowVal
firstColSecondRowVal,secondColSecondRowVal,thirdColSecondRowVal
```