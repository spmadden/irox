use std::fs::File;

use irox_sqlite::header::Header;

#[test]
fn read_header() {
    let mut file = File::open("E:\\charts\\NOAA MBTiles\\ncds_06.mbtiles").expect("open");

    let header = Header::read_from(&mut file).expect("ugh.");

    println!("{:?}", header);
}