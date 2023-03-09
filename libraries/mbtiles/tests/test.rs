use std::collections::HashMap;

use irox_mbtiles::{
    sqlite_helpers::Pragma, CreateOptions, ImageFormat, MBTiles, RasterFormat, Result,
    APPLICATION_ID,
};

#[test]
pub fn test() -> Result<()> {
    let mut pragmas = HashMap::new();
    pragmas.insert(Pragma::ApplicationId, APPLICATION_ID);
    pragmas.insert(Pragma::CacheSize, -1024 * 1024 * 5);
    pragmas.insert(Pragma::PageSize, 8192);

    let options = CreateOptions {
        name: "Test DB".to_string(),
        format: ImageFormat::Raster(RasterFormat::PNG),
        pragmas,
    };
    let db = MBTiles::open_or_create_options(&"test.mbtiles", &options)?;

    println!("Opened DB: {db:?}");

    let tile = db.get_tile(0, 0, 0)?;

    println!("Read data: {tile:?}");

    Ok(())
}
