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
                // Use the individual fields of the CSV line
                row.
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

* Writing 