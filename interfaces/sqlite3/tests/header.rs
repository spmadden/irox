// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use irox_sqlite3::db::Database;

#[test]
fn read_header() {
    // let mut file = File::open().expect("open");

    let mut db = Database::open_db_path(&"./tests/test.db").expect("Ugh.");
    println!("{:#?}", db);

    let page = db.read_page(0).expect("Ugh");
    println!("{:#?}", page);
}
