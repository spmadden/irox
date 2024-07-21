#![allow(clippy::all)]
#![allow(clippy::tests_outside_test_module)]
use irox_mbtiles::{CreateOptions, ImageFormat, MBTiles, OpenOptions, RasterFormat, Result};
use irox_tools::assert_eq_hex_slice;

const DATA: &[u8] = &[0xC0u8, 0xFF, 0xEE];

#[test]
pub fn test() -> Result<()> {
    let _ = std::fs::remove_file("test.mbtiles");
    {
        let perf = OpenOptions::safe_performance();

        let options = CreateOptions {
            name: "Test DB".to_string(),
            format: ImageFormat::Raster(RasterFormat::PNG),
            pragmas: perf.pragmas,
        };
        println!("{options:?}");
        let mut db = MBTiles::open_or_create_options(&"test.mbtiles", &options)?;

        println!("Opened DB: {db:?}");
        db.set_tile(0, 0, 0, 0, &DATA)?;

        let tile = db.get_tile(0, 0, 0)?;

        assert_eq_hex_slice!(&tile, DATA);
    }

    if let Err(e) = std::fs::remove_file("test.mbtiles") {
        eprintln!("{e:#?}");
    }

    Ok(())
}
