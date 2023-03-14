use irox_mbtiles::{
    sqlite_helpers::Pragma, CreateOptions, ImageFormat, MBTiles, OpenOptions, RasterFormat, Result,
    APPLICATION_ID,
};

#[test]
pub fn test() -> Result<()> {
    let mut perf = OpenOptions::safe_performance();
    perf.pragmas.append(&mut vec![
        Pragma::ApplicationId(APPLICATION_ID),
        Pragma::PageSizeBytes(16384),
    ]);

    let options = CreateOptions {
        name: "Test DB".to_string(),
        format: ImageFormat::Raster(RasterFormat::PNG),
        pragmas: perf.pragmas,
    };
    let db = MBTiles::open_or_create_options(&"test.mbtiles", &options)?;

    println!("Opened DB: {db:?}");

    let tile = db.get_tile(0, 0, 0)?;

    println!("Read data: {tile:?}");

    Ok(())
}
