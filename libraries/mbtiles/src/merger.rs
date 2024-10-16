// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::path::Path;

use crate::{MBTiles, Result};

pub fn merge<T: AsRef<Path>>(inputs: &[T], output: &T) -> Result<()> {
    let options = crate::CreateOptions {
        name: "Combined".to_string(),
        ..Default::default()
    };
    let mut db = MBTiles::open_or_create_options(output, &options)?;

    for input in inputs {
        let mut indb = MBTiles::open(input)?;
        indb.foreach_tile(&mut |tile| db.insert_tile(tile))?;
    }
    Ok(())
}
