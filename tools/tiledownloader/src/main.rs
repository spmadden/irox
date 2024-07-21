#![allow(clippy::print_stderr)]
#![allow(clippy::print_stdout)]

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use irox_carto::coordinate::{EllipticalCoordinate, Latitude, Longitude};
use irox_carto::epsg3857::SphericalMercatorProjection;
use irox_mbtiles::{CreateOptions, OpenOptions};
use irox_tiledownloader::{config::Config, status::DownloadStatus, tile::TileData, url::URLParams};
use irox_units::units::angle::Angle;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};
use tokio::task::{self};

fn main() {
    let config = Config::parse();

    let mut default_headers = HeaderMap::new();
    let mut bldr = ClientBuilder::new()
        .brotli(true)
        .deflate(true)
        .gzip(true)
        .user_agent("TileDownloader v0.1.0");
    if let Some(referrer) = &config.referrer {
        let Ok(refhdr) = HeaderValue::from_str(referrer) else {
            eprintln!("Unable to convert referrer {referrer} into header type");
            return;
        };

        bldr = bldr.referer(false);
        default_headers.append("Referrer", refhdr);
    }
    bldr = bldr.default_headers(default_headers);

    let Ok(client) = bldr
        .build()
        .map_err(|e| eprintln!("Error building client: {e}"))
    else {
        return;
    };

    let options = CreateOptions {
        name: config.name.clone(),
        pragmas: OpenOptions::safe_performance().pragmas,
        ..Default::default()
    };
    let Ok(mut outfile) = irox_mbtiles::MBTiles::open_or_create_options(&config.out_file, &options)
        .map_err(|e| eprintln!("Error opening output file {e}"))
    else {
        return;
    };
    let bbox = &config.bbox;
    let lats = vec![bbox[0], bbox[2]];
    let lons = vec![bbox[1], bbox[3]];
    let (min_lat_deg, max_lat_deg) = irox_tools::f64::min_max(&lats);
    let (min_lon_deg, max_lon_deg) = irox_tools::f64::min_max(&lons);
    let (min_zoom, max_zoom) = irox_tools::u8::min_max(&config.zoom_levels);
    if let Err(e) = outfile.update_bounding_box(
        Latitude(Angle::new_degrees(min_lat_deg)),
        Latitude(Angle::new_degrees(max_lat_deg)),
        Longitude(Angle::new_degrees(min_lon_deg)),
        Longitude(Angle::new_degrees(max_lon_deg)),
        min_zoom,
        max_zoom,
    ) {
        eprintln!("Error updating bounding box: {e}")
    }

    let (tx, rx) = std::sync::mpsc::sync_channel(10);
    let joiner = std::thread::spawn(move || {
        let mut pb = create_progress_bar(0);
        let mut num_done = 0;
        loop {
            pb.tick();
            let Ok(cmd): Result<DownloadStatus, _> = rx.recv() else {
                eprintln!("consumer thread closed");
                break;
            };
            match cmd {
                DownloadStatus::TileDataAvailable(data) => {
                    if let Err(e) = outfile.insert_tile(&data.as_tile_data()) {
                        eprintln!("Error {e:?}");
                    };
                    num_done += 1;
                    pb.set_position(num_done);
                }
                DownloadStatus::TileComplete(_addr) => {
                    num_done += 1;
                    pb.set_position(num_done);
                }
                DownloadStatus::ZoomLevelStarted(_zl, num_tiles) => {
                    pb = create_progress_bar(num_tiles);
                }
                DownloadStatus::ZoomLevelComplete(zl) => {
                    if let Err(e) = outfile.update_min_max_zooms(zl) {
                        eprintln!("Error {e:?}");
                    }
                }
                DownloadStatus::Done => {
                    break;
                }
            };
        }
        println!("Staring database GC...");
        if let Err(e) = outfile.gc() {
            eprintln!("Error GC'ing db: {e}");
        } else {
            println!("Database GC done.");
        }
        pb.tick();
    });

    let upper_left = EllipticalCoordinate::new_degrees_wgs84(max_lat_deg, min_lon_deg);
    let upper_right = EllipticalCoordinate::new_degrees_wgs84(max_lat_deg, max_lon_deg);
    let lower_left = EllipticalCoordinate::new_degrees_wgs84(min_lat_deg, min_lon_deg);
    let lower_right = EllipticalCoordinate::new_degrees_wgs84(min_lat_deg, max_lon_deg);

    let coords = vec![upper_left, upper_right, lower_right, lower_left];

    let rt = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error building runtime: {e}");
            return;
        }
    };
    rt.block_on(async {
        for zoom_level in max_zoom..=min_zoom {
            let proj = SphericalMercatorProjection::new(zoom_level);
            let tile_x_indexes: Vec<u64> =
                coords.iter().map(|c| proj.tile_x_index(c) as u64).collect();
            let tile_y_indexes: Vec<u64> =
                coords.iter().map(|c| proj.tile_y_index(c) as u64).collect();

            let (min_x_idx, max_x_idx) = irox_tools::u64::min_max(&tile_x_indexes);
            let (min_y_idx, max_y_idx) = irox_tools::u64::min_max(&tile_y_indexes);
            let num_xs = max_x_idx - min_x_idx;
            let num_ys = max_y_idx - min_y_idx;
            let total_tiles = num_xs * num_ys;
            if let Err(e) = tx.send(DownloadStatus::ZoomLevelStarted(zoom_level, total_tiles)) {
                eprintln!("Error {e:?}");
            };

            let params = URLParams {
                max_x_idx,
                max_y_idx,
                min_x_idx,
                min_y_idx,
                server_parts: config.server_parts.clone(),
                url_template: config.url.clone(),
                zoom_level,
            };

            let (task_add, mut task_recv) = tokio::sync::mpsc::channel(10);

            rt.spawn(async move {
                loop {
                    let Some(task) = task_recv.recv().await else {
                        return;
                    };
                    if let Err(e) = task.await {
                        eprintln!("Error {e:?}");
                    }
                }
            });

            for address in params {
                let tx = tx.clone();
                let client = client.clone();
                if let Err(e) = task_add
                    .send(rt.spawn(async move {
                        let bldr = client.get(&address.url);
                        let req = match bldr.build() {
                            Ok(r) => r,
                            Err(e) => {
                                eprintln!("Error building client: {e}");
                                return;
                            }
                        };
                        let res = client.execute(req).await;
                        let Ok(response) = res.map_err(|e| eprintln!("{e:?}")) else {
                            return;
                        };

                        if response.status() != 200 {
                            return;
                        }

                        let data = match response.bytes().await {
                            Ok(d) => d,
                            Err(e) => {
                                eprintln!("Error getting data: {e}");
                                return;
                            }
                        };

                        if let Err(e) = tx.send(DownloadStatus::TileDataAvailable(TileData {
                            address,
                            data,
                        })) {
                            eprintln!("Error {e:?}");
                        };
                    }))
                    .await
                {
                    eprintln!("Error {e}");
                };
                task::yield_now().await;
            }
            if let Err(e) = tx.send(DownloadStatus::ZoomLevelComplete(zoom_level)) {
                eprintln!("Error {e:?}");
            };
        }
    });
    if let Err(e) = tx.send(DownloadStatus::Done) {
        eprintln!("Error {e:?}");
    }
    if joiner.join().is_err() {
        eprintln!("Error joining thread");
    };
}

fn create_progress_bar(total_tiles: u64) -> ProgressBar {
    let pb = ProgressBar::new(total_tiles);
    if let Ok(st) = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {per_sec} {human_pos}/{human_len} ({eta_precise})") {
        let st = st.progress_chars("#>-");
        pb.set_style(st);
    }
    pb
}
