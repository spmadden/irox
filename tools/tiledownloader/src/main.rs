// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::print_stderr)]
#![allow(clippy::print_stdout)]
#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use clap::Parser;
use irox_carto::coordinate::{EllipticalCoordinate, Latitude, Longitude};
use irox_carto::epsg3857::SphericalMercatorProjection;
use irox_progress::console::ConsoleProgressPrinter;
use irox_progress::{ProgressPrinter, Task};
use irox_tiledownloader::{config::Config, status::DownloadStatus, tile::TileData, url::URLParams};
use irox_units::units::angle::Angle;
use irox_units::units::duration::Duration;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};
use tokio::task::{self};

#[cfg(not(target_arch = "wasm32"))]
pub fn builder() -> ClientBuilder {
    ClientBuilder::new()
        .brotli(true)
        .deflate(true)
        .gzip(true)
        .referer(false)
        .user_agent("TileDownloader v0.1.0")
}
#[cfg(target_arch = "wasm32")]
pub fn builder() -> ClientBuilder {
    ClientBuilder::new()
}
#[cfg(target_arch = "wasm32")]
fn main() {}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let config = Config::parse();

    let mut default_headers = HeaderMap::new();
    let mut bldr = builder();

    if let Some(referrer) = &config.referrer {
        let Ok(refhdr) = HeaderValue::from_str(referrer) else {
            eprintln!("Unable to convert referrer {referrer} into header type");
            return;
        };

        default_headers.append("Referrer", refhdr);
    }
    bldr = bldr.default_headers(default_headers);

    let Ok(client) = bldr
        .build()
        .map_err(|e| eprintln!("Error building client: {e}"))
    else {
        return;
    };

    let options = irox_mbtiles::CreateOptions {
        name: config.name.clone(),
        pragmas: irox_mbtiles::OpenOptions::safe_performance().pragmas,
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
    let nzoom = max_zoom - min_zoom;
    let (tx, rx) = std::sync::mpsc::sync_channel(10);
    let joiner = std::thread::spawn(move || {
        let pb = create_progress_bar();
        let overall_task = Task::new_named("Download Task".to_string(), nzoom as u64);
        pb.track_task_progress(&overall_task);
        let mut current_level_task: Option<Task> = None;
        loop {
            let Ok(cmd): Result<DownloadStatus, _> = rx.recv() else {
                eprintln!("consumer thread closed");
                break;
            };
            match cmd {
                DownloadStatus::TileDataAvailable(data) => {
                    if let Err(e) = outfile.insert_tile(&data.as_tile_data()) {
                        eprintln!("Error {e:?}");
                    };
                    if let Some(clt) = &current_level_task {
                        clt.mark_some_completed(1);
                    }
                }
                DownloadStatus::TileComplete(_addr) => {
                    if let Some(clt) = &current_level_task {
                        clt.mark_some_completed(1);
                    }
                }
                DownloadStatus::ZoomLevelStarted(zl, num_tiles) => {
                    if let Some(clt) = current_level_task.take() {
                        clt.mark_ended();
                        clt.mark_all_completed();
                        overall_task.mark_one_completed();
                    }
                    let newtask = overall_task.new_child_task(
                        zl as u64,
                        format!("Zoom Level {zl}"),
                        num_tiles,
                    );
                    newtask.mark_started();
                    current_level_task = Some(newtask);
                }
                DownloadStatus::ZoomLevelComplete(zl) => {
                    if let Err(e) = outfile.update_min_max_zooms(zl) {
                        eprintln!("Error {e:?}");
                    }
                }
                DownloadStatus::Done => {
                    overall_task.mark_all_completed();
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
    });

    let upper_left = EllipticalCoordinate::new_degrees_wgs84(max_lat_deg, min_lon_deg);
    let upper_right = EllipticalCoordinate::new_degrees_wgs84(max_lat_deg, max_lon_deg);
    let lower_left = EllipticalCoordinate::new_degrees_wgs84(min_lat_deg, min_lon_deg);
    let lower_right = EllipticalCoordinate::new_degrees_wgs84(min_lat_deg, max_lon_deg);

    let coords = vec![upper_left, upper_right, lower_right, lower_left];

    let rt = match tokio::runtime::Builder::new_current_thread()
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

pub fn create_progress_bar() -> Box<dyn ProgressPrinter> {
    Box::new(ConsoleProgressPrinter::new_update_rate(
        Duration::from_millis(250),
    ))
}
