// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

//! Run: cargo run -p irox-egui-extras --example slippy_map --features eframe,glow
//! Native example: ehttp handles HTTPS; irox-imagery decodes PNG tiles.
//! Only visible tiles are fetched, with a persistent seven-day cache.
//! Set IROX_MAP_TILE_SIZE=512 to specify 512-pixel PNG tiles (default: 256).
//! This does not change OSM's 256-pixel source images. See `doc/slippy_map.md`
//! for cache recovery, decoder limitations, and adapting the example to another server.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use egui::load::{ImageLoadResult, ImageLoader, ImagePoll, LoadError, SizeHint};
use egui::mutex::Mutex;
use egui::{ColorImage, Context, Ui};
use irox_egui_extras::widgets::slippy_map::{SlippyMap, TileScheme};
use irox_imagery::png::PNGFile;
use irox_tools::irox_bits::BitsWrapper;

const PREFIX: &str = "https://tile.openstreetmap.org/";
const USER_AGENT: &str = concat!(
    "irox-slippy-map-example/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/spmadden/irox)"
);
const CACHE_AGE: Duration = Duration::from_secs(7 * 24 * 60 * 60);
type TileCache = Arc<Mutex<HashMap<String, ImageLoadResult>>>;

/// Combines asynchronous HTTP fetching and PNG decoding in an egui image loader.
/// The default egui texture loader uploads the resulting ColorImage to the GPU.
struct OsmLoader {
    tiles: TileCache,
    directory: PathBuf,
    error: Arc<Mutex<Option<String>>>,
}

fn decode(bytes: &[u8]) -> Result<Arc<ColorImage>, String> {
    let mut input = bytes;
    let png =
        PNGFile::read_from(&mut BitsWrapper::Owned(&mut input)).map_err(|err| err.to_string())?;
    let image = png.to_image().map_err(|err| err.to_string())?;
    Ok(Arc::new((&image).into()))
}

/// Accept only OSM's z/x/y.png paths, also keeping disk paths inside the cache.
fn cache_path(directory: &std::path::Path, uri: &str) -> Option<PathBuf> {
    let path = uri.strip_prefix(PREFIX)?;
    let parts: Vec<_> = path.strip_suffix(".png")?.split('/').collect();
    if parts.len() != 3
        || parts
            .iter()
            .any(|part| part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return None;
    }
    Some(directory.join(path.replace('/', "_")))
}

impl ImageLoader for OsmLoader {
    fn id(&self) -> &str {
        egui::generate_loader_id!(OsmLoader)
    }

    fn load(&self, ctx: &Context, uri: &str, _: SizeHint) -> ImageLoadResult {
        let Some(path) = cache_path(&self.directory, uri) else {
            return Err(LoadError::NotSupported);
        };
        let mut tiles = self.tiles.lock();
        if let Some(tile) = tiles.get(uri) {
            return tile.clone();
        }
        tiles.insert(uri.to_owned(), Ok(ImagePoll::Pending { size: None }));
        drop(tiles);

        let uri = uri.to_owned();
        let tiles = Arc::clone(&self.tiles);
        let error = Arc::clone(&self.error);
        let ctx = ctx.clone();
        // Disk access and PNG decoding happen off the UI thread, including cache hits.
        std::thread::spawn(move || {
            let request_uri = uri.clone();
            let finish = move |result: Result<Arc<ColorImage>, String>| {
                if let Err(message) = &result {
                    *error.lock() = Some(message.clone());
                }
                let result = result
                    .map(|image| ImagePoll::Ready { image })
                    .map_err(LoadError::Loading);
                // Do not resurrect entries explicitly forgotten while loading.
                if let Some(entry) = tiles.lock().get_mut(&uri) {
                    *entry = result;
                }
                ctx.request_repaint();
            };
            let fresh = std::fs::metadata(&path)
                .and_then(|meta| meta.modified())
                .ok()
                .and_then(|time| time.elapsed().ok())
                .is_some_and(|age| age < CACHE_AGE);
            if fresh {
                if let Ok(bytes) = std::fs::read(&path) {
                    if let Ok(image) = decode(&bytes) {
                        finish(Ok(image));
                        return;
                    }
                }
            }
            let mut request = ehttp::Request::get(request_uri);
            request.headers.insert("User-Agent", USER_AGENT);
            ehttp::fetch(request, move |response| {
                let result = response.and_then(|response| {
                    if !response.ok {
                        return Err(format!(
                            "Tile HTTP error: {} {}",
                            response.status, response.status_text
                        ));
                    }
                    let image = decode(&response.bytes)?;
                    // Cache only valid PNGs. Failure is visible instead of silently
                    // causing repeated downloads on the next application launch.
                    std::fs::write(&path, &response.bytes)
                        .map_err(|err| format!("Tile cache: {err}"))?;
                    Ok(image)
                });
                finish(result);
            });
        });
        Ok(ImagePoll::Pending { size: None })
    }

    fn forget(&self, uri: &str) {
        self.tiles.lock().remove(uri);
    }
    fn forget_all(&self) {
        self.tiles.lock().clear();
    }
    fn byte_size(&self) -> usize {
        self.tiles
            .lock()
            .values()
            .map(|tile| match tile {
                Ok(ImagePoll::Ready { image }) => image.pixels.len() * 4,
                Err(error) => error.byte_size(),
                _ => 0,
            })
            .sum()
    }
    fn has_pending(&self) -> bool {
        self.tiles
            .lock()
            .values()
            .any(|tile| matches!(tile, Ok(ImagePoll::Pending { .. })))
    }
}

struct MapApp {
    map: SlippyMap,
    error: Arc<Mutex<Option<String>>>,
}

impl eframe::App for MapApp {
    fn ui(&mut self, ui: &mut Ui, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("Drag to pan. Scroll or pinch to zoom.");
            ui.horizontal(|ui| {
                ui.hyperlink_to(
                    "\u{a9} OpenStreetMap contributors",
                    "https://www.openstreetmap.org/copyright",
                );
                ui.hyperlink_to(
                    "Tile usage policy",
                    "https://operations.osmfoundation.org/policies/tiles/",
                );
            });
            if let Some(error) = self.error.lock().as_ref() {
                ui.colored_label(egui::Color32::RED, error);
            }
            ui.add(&mut self.map);
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tile_size =
        std::env::var("IROX_MAP_TILE_SIZE").map_or(Ok(256), |value| value.parse::<u32>())?;
    if tile_size == 0 {
        return Err("IROX_MAP_TILE_SIZE must be greater than zero".into());
    }
    let directory = std::env::var_os("IROX_OSM_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("irox-osm-tiles"));
    // Fail before making requests if the persistent cache cannot be created.
    std::fs::create_dir_all(&directory)?;
    eframe::run_native(
        "OpenStreetMap slippy map",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]),
            ..Default::default()
        },
        Box::new(move |cc| {
            let error = Arc::new(Mutex::new(None));
            // HTTP + PNG image loader; egui supplies the texture loader by default.
            cc.egui_ctx.add_image_loader(Arc::new(OsmLoader {
                tiles: Arc::default(),
                directory,
                error: Arc::clone(&error),
            }));
            let mut map = SlippyMap::new(format!("{PREFIX}{{z}}/{{x}}/{{y}}.png"))
                .scheme(TileScheme::Xyz)
                .tile_size(tile_size)
                .max_zoom(19);
            map.set_view(0.5, 0.5, 3.0);
            Ok(Box::new(MapApp { map, error }))
        }),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn png_loader_decodes_existing_asset() {
        // Indexed-color, 8-bit, 256x256 fixture: check content, not just parsing.
        let image = decode(include_bytes!("../../imagery/assets/0.png")).expect("PNG fixture");
        assert_eq!(image.size, [256, 256]);
        assert_eq!(image.pixels.len(), 256 * 256);
        assert_eq!(
            image.pixels.first().copied(),
            Some(egui::Color32::from_rgb(170, 211, 223))
        );
    }

    #[test]
    fn cache_paths_are_restricted() {
        let directory = PathBuf::from("cache");
        let uri = "https://tile.openstreetmap.org/3/2/1.png";
        let path = cache_path(&directory, uri).expect("valid tile URL");
        assert_eq!(path, directory.join("3_2_1.png"));
        assert!(cache_path(&directory, "https://tile.openstreetmap.org/../../1.png").is_none());
        assert!(cache_path(&directory, "https://example.com/3/2/1.png").is_none());
    }
}
