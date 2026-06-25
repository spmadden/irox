use irox_sqlite3::db::Database;

#[test]
fn read_header() {
    // let mut file = File::open().expect("open");

    let mut db = Database::open_db(&"C:\\chartdata\\NOAA MBTiles\\ncds_03.mbtiles").expect("Ugh.");
    println!("{:#?}", db);

    let page = db.read_page(0).expect("Ugh");
    println!("{:#?}", page);
}
