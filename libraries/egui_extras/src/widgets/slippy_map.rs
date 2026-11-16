// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

//! Raster tile maps using URL templates and egui's asynchronous image loaders.
//!
//! Register HTTP byte and image loaders, or a combined HTTP/image loader, on the
//! egui context. The native `slippy_map` example demonstrates the combined approach.
//! Loading and caching follow egui's loader API; this module adds no networking dependencies.
//! Pending or failed replacements preserve the previous complete layer; report
//! errors and implement retry/eviction in the application's loaders.
//! See the crate's `doc/slippy_map.md` for integration and troubleshooting notes.
//! Respect the tile provider's attribution, identification and usage requirements.
//!
//! ```no_run
//! # fn draw(ui: &mut egui::Ui, map: &mut irox_egui_extras::widgets::slippy_map::SlippyMap) {
//! ui.add(map);
//! # }
//! use irox_egui_extras::widgets::slippy_map::{SlippyMap, TileScheme};
//! // Keep this state between frames. Register HTTP and image loaders separately.
//! let mut map = SlippyMap::new("https://example.com/tiles/{z}/{x}/{y}.png")
//!     .scheme(TileScheme::Tms);
//! map.set_view(0.5, 0.5, 3.0);
//! ```

use std::collections::HashMap;

use egui::load::{SizeHint, SizedTexture, TexturePoll};
use egui::{pos2, vec2, Color32, Rect, Response, Sense, TextureOptions, Ui, Widget};

#[derive(Debug, Clone)]
struct TileLayer {
    zoom: u8,
    tiles: HashMap<(i64, i64), SizedTexture>,
}

/// Tile row numbering. Both schemes use Web Mercator square tiles.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TileScheme {
    /// Rows increase northward from the bottom of the map.
    #[default]
    Tms,
    /// Rows increase southward from the top of the map.
    Xyz,
}

/// Persistent slippy-map state. Center coordinates are normalized world coordinates:
/// x increases eastward, y increases southward; (0.5, 0.5) is longitude/latitude 0/0.
/// Zoom zero displays a world one configured tile wide (256 PNG pixels by default).
/// PNG pixels are converted to egui points using the current pixels-per-point scale.
/// Drag to pan; scroll or pinch to
/// zoom about the pointer. Only visible tiles are submitted to the image loaders.
/// The last complete tile layer stays visible, scaled to the current view, until
/// all visible replacement tiles are ready. Loader cache lifetime and eviction
/// remain the application's responsibility; keep displayed textures cached.
#[derive(Debug, Clone)]
pub struct SlippyMap {
    template: String,
    scheme: TileScheme,
    center: [f64; 2],
    zoom: f64,
    max_zoom: u8,
    tile_size: u32,
    displayed: Option<TileLayer>,
}

impl SlippyMap {
    /// A template containing `{x}`, `{y}`, and `{z}` placeholders.
    pub fn new<T: Into<String>>(template: T) -> Self {
        Self {
            template: template.into(),
            scheme: TileScheme::Tms,
            center: [0.5, 0.5],
            zoom: 0.0,
            max_zoom: 22,
            tile_size: 256,
            displayed: None,
        }
    }

    #[must_use]
    pub fn scheme(mut self, scheme: TileScheme) -> Self {
        self.scheme = scheme;
        self
    }

    /// Set the square tile width in PNG pixels (default: 256).
    /// At integer zoom levels one PNG pixel occupies one screen pixel. The widget
    /// divides by egui's current pixels-per-point scale for layout and input.
    /// Zero is clamped to one pixel.
    #[must_use]
    pub fn tile_size(mut self, size: u32) -> Self {
        self.set_tile_size(size);
        self
    }

    /// Change tile size in PNG pixels while keeping the current normalized center and zoom.
    /// Retained tiles are rescaled immediately while replacement tiles load.
    pub fn set_tile_size(&mut self, size: u32) {
        self.tile_size = size.max(1);
    }

    /// Limit tile requests and zoom, capped at 30 to keep tile indices in range.
    #[must_use]
    pub fn max_zoom(mut self, zoom: u8) -> Self {
        self.max_zoom = zoom.min(30);
        self.zoom = self.zoom.min(f64::from(self.max_zoom));
        self
    }

    /// Set normalized center and fractional zoom. Non-finite input is ignored.
    pub fn set_view(&mut self, x: f64, y: f64, zoom: f64) {
        if x.is_finite() && y.is_finite() && zoom.is_finite() {
            self.center = [x.rem_euclid(1.0), y.clamp(0.0, 1.0)];
            self.zoom = zoom.clamp(0.0, f64::from(self.max_zoom));
        }
    }

    pub fn view(&self) -> ([f64; 2], f64) {
        (self.center, self.zoom)
    }

    fn tile_url(&self, x: i64, y: i64, z: u8) -> String {
        let count = 1_i64 << z;
        let x = x.rem_euclid(count);
        let y = match self.scheme {
            TileScheme::Tms => count - 1 - y,
            TileScheme::Xyz => y,
        };
        self.template
            .replace("{x}", &x.to_string())
            .replace("{y}", &y.to_string())
            .replace("{z}", &z.to_string())
    }

    fn tile_rects(
        &self,
        z: u8,
        rect: Rect,
        visible: Rect,
        pixels_per_point: f32,
    ) -> Vec<(i64, i64, Rect)> {
        let count = 1_i64 << z;
        let tile_size = self.world_size(pixels_per_point) / (1_u64 << z) as f64;
        let left = self.center[0] * count as f64 - f64::from(rect.width()) / (2.0 * tile_size);
        let top = self.center[1] * count as f64 - f64::from(rect.height()) / (2.0 * tile_size);
        let first_x = left + f64::from(visible.left() - rect.left()) / tile_size;
        let first_y = top + f64::from(visible.top() - rect.top()) / tile_size;
        let end_x = left + f64::from(visible.right() - rect.left()) / tile_size;
        let end_y = top + f64::from(visible.bottom() - rect.top()) / tile_size;
        let mut tiles = Vec::new();
        for y in (first_y.floor() as i64).max(0)..(end_y.ceil() as i64).min(count) {
            for x in first_x.floor() as i64..end_x.ceil() as i64 {
                let min = pos2(
                    rect.left() + ((x as f64 - left) * tile_size) as f32,
                    rect.top() + ((y as f64 - top) * tile_size) as f32,
                );
                tiles.push((
                    x,
                    y,
                    Rect::from_min_size(min, vec2(tile_size as f32, tile_size as f32)),
                ));
            }
        }
        tiles
    }

    fn paint_layer(&self, ui: &Ui, layer: &TileLayer, rect: Rect, visible: Rect) {
        let count = (1_i64 << layer.zoom) as f64;
        let world = self.world_size(ui.pixels_per_point());
        let tile_size = world / count;
        // Iterate retained tiles, not the old zoom's viewport grid: a large
        // zoom-out can otherwise enumerate billions of unavailable tiles.
        for (&(x, y), texture) in &layer.tiles {
            let left = f64::from(rect.center().x) + (x as f64 / count - self.center[0]) * world;
            let top = f64::from(rect.center().y) + (y as f64 / count - self.center[1]) * world;
            if top + tile_size <= f64::from(visible.top()) || top >= f64::from(visible.bottom()) {
                continue;
            }
            let first = ((f64::from(visible.left()) - left - tile_size) / world).floor() as i64 + 1;
            let end = ((f64::from(visible.right()) - left) / world).ceil() as i64;
            for wrap in first..end {
                let tile_rect = Rect::from_min_size(
                    pos2((left + wrap as f64 * world) as f32, top as f32),
                    vec2(tile_size as f32, tile_size as f32),
                );
                ui.painter().image(
                    texture.id,
                    tile_rect,
                    Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
            }
        }
    }

    fn world_size(&self, pixels_per_point: f32) -> f64 {
        f64::from(self.tile_size) * self.zoom.exp2() / f64::from(pixels_per_point)
    }

    fn zoom_at(&mut self, delta: f64, offset: egui::Vec2, pixels_per_point: f32) {
        let old_world = self.world_size(pixels_per_point);
        self.zoom = (self.zoom + delta).clamp(0.0, f64::from(self.max_zoom));
        let new_world = self.world_size(pixels_per_point);
        self.center[0] += f64::from(offset.x) * (1.0 / old_world - 1.0 / new_world);
        self.center[1] += f64::from(offset.y) * (1.0 / old_world - 1.0 / new_world);
    }
}

impl Widget for &mut SlippyMap {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, mut response) = ui.allocate_exact_size(ui.available_size(), Sense::drag());
        if !rect.is_positive() {
            return response;
        }
        let before = self.view();
        if response.dragged() {
            let delta = ui.input(|i| i.pointer.delta());
            let world = self.world_size(ui.pixels_per_point());
            self.center[0] -= f64::from(delta.x) / world;
            self.center[1] -= f64::from(delta.y) / world;
        }
        if response.hovered() {
            let (delta, pointer) = ui.input(|i| {
                let pinch = f64::from(i.zoom_delta()).log2();
                let delta = if pinch.abs() > f64::EPSILON {
                    pinch
                } else {
                    f64::from(i.smooth_scroll_delta.y) / 256.0
                };
                (delta, i.pointer.hover_pos())
            });
            if let Some(pointer) = pointer {
                self.zoom_at(delta, pointer - rect.center(), ui.pixels_per_point());
            }
        }
        self.center[0] = self.center[0].rem_euclid(1.0);
        self.center[1] = self.center[1].clamp(0.0, 1.0);
        if self.view() != before {
            response.mark_changed();
        }

        let visible = rect.intersect(ui.clip_rect());
        if !visible.is_positive() {
            return response;
        }
        let z = self.zoom.floor() as u8;
        let mut replacement = TileLayer {
            zoom: z,
            tiles: HashMap::new(),
        };
        let mut complete = true;
        for (x, y, _) in self.tile_rects(z, rect, visible, ui.pixels_per_point()) {
            let key = (x.rem_euclid(1_i64 << z), y);
            if replacement.tiles.contains_key(&key) {
                continue;
            }
            match ui.ctx().try_load_texture(
                &self.tile_url(x, y, z),
                TextureOptions::default(),
                SizeHint::default(),
            ) {
                Ok(TexturePoll::Ready { texture }) => {
                    replacement.tiles.insert(key, texture);
                }
                _ => complete = false,
            }
        }
        let mut clipped = ui.new_child(egui::UiBuilder::new().max_rect(rect));
        clipped.set_clip_rect(visible);
        clipped
            .painter()
            .rect_filled(rect, 0.0, Color32::from_gray(40));
        // Promote an entire viewport together. Pending or failed loads must not
        // replace the previous level with placeholders or error icons.
        if complete {
            self.displayed = Some(replacement);
        } else if self.displayed.is_none() {
            // On first use there is no previous map; display available tiles.
            self.paint_layer(&clipped, &replacement, rect, visible);
        }
        if let Some(layer) = &self.displayed {
            self.paint_layer(&clipped, layer, rect, visible);
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_numbering_and_wrapping() {
        let map = SlippyMap::new("{z}/{x}/{y}");
        assert_eq!(map.tile_url(-1, 0, 2), "2/3/3");
        assert_eq!(map.tile_url(4, 3, 2), "2/0/0");
        assert_eq!(map.scheme(TileScheme::Xyz).tile_url(-1, 0, 2), "2/3/0");
    }

    #[test]
    fn zoom_preserves_world_point_under_pointer() {
        let mut map = SlippyMap::new("");
        map.set_view(0.4, 0.3, 4.0);
        let offset = vec2(100.0, -50.0);
        let point =
            map.center[0] + f64::from(offset.x) / (f64::from(map.tile_size) * map.zoom.exp2());
        map.zoom_at(1.5, offset, 1.0);
        let after =
            map.center[0] + f64::from(offset.x) / (f64::from(map.tile_size) * map.zoom.exp2());
        assert!((point - after).abs() < 1e-12);
    }

    #[test]
    fn view_is_bounded_and_ignores_nan() {
        let mut map = SlippyMap::new("").max_zoom(8);
        map.set_view(-0.25, 2.0, 50.0);
        assert_eq!(map.view(), ([0.75, 1.0], 8.0));
        map.set_view(f64::NAN, 0.0, 0.0);
        assert_eq!(map.view(), ([0.75, 1.0], 8.0));
    }

    #[test]
    fn tile_size_controls_layout_and_preserves_zoom_anchor() {
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(1024.0, 1024.0));
        for (size, pixels_per_point, expected_tiles) in [
            (256, 1.0, 16),
            (512, 1.0, 4),
            (512, 2.0, 16),
            (256, 2.0, 32),
            (512, 1.25, 16),
        ] {
            let mut map = SlippyMap::new("").tile_size(size);
            map.set_view(0.5, 0.5, 2.0);
            let tiles = map.tile_rects(2, rect, rect, pixels_per_point);
            assert_eq!(tiles.len(), expected_tiles);
            assert!(tiles
                .iter()
                .all(|(_, _, tile)| (tile.width() - size as f32 / pixels_per_point).abs() < 0.01));
            let offset = vec2(100.0, -50.0);
            let world = map.world_size(pixels_per_point);
            let point = [
                map.center[0] + f64::from(offset.x) / world,
                map.center[1] + f64::from(offset.y) / world,
            ];
            map.zoom_at(0.75, offset, pixels_per_point);
            let world = map.world_size(pixels_per_point);
            assert!((point[0] - map.center[0] - f64::from(offset.x) / world).abs() < 1e-12);
            assert!((point[1] - map.center[1] - f64::from(offset.y) / world).abs() < 1e-12);
            let view = map.view();
            map.set_tile_size(size * 2);
            assert_eq!(map.view(), view);
        }
        assert_eq!(SlippyMap::new("").tile_size(0).tile_size, 1);
    }

    #[test]
    fn clipping_and_world_edges_bound_tile_selection() {
        let mut map = SlippyMap::new("");
        map.set_view(0.5, 0.5, 3.0);
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(1024.0, 1024.0));
        let clip = Rect::from_min_size(pos2(512.0, 512.0), vec2(128.0, 128.0));
        let tiles = map.tile_rects(3, rect, clip, 1.0);
        assert_eq!(tiles.len(), 1);
        assert_eq!(tiles.first().map(|(x, y, _)| (*x, *y)), Some((4, 4)));
        map.set_view(0.0, 0.0, 0.0);
        let tiles = map.tile_rects(0, rect, rect, 1.0);
        assert!(tiles.iter().any(|(x, _, _)| *x < 0));
        assert!(tiles.iter().all(|(_, y, _)| *y == 0));
    }

    struct DelayedTextures {
        ready_zoom: std::sync::atomic::AtomicU8,
        blocked: std::sync::atomic::AtomicBool,
        fail: std::sync::atomic::AtomicBool,
    }

    impl egui::load::TextureLoader for DelayedTextures {
        fn id(&self) -> &str {
            "slippy-map-delayed-test"
        }
        fn load(
            &self,
            _: &egui::Context,
            uri: &str,
            _: TextureOptions,
            _: SizeHint,
        ) -> egui::load::TextureLoadResult {
            use std::sync::atomic::Ordering::Relaxed;
            let ready = self.ready_zoom.load(Relaxed).to_string();
            let middle = (1_u64 << self.ready_zoom.load(Relaxed)) / 2;
            let blocked_tile =
                self.blocked.load(Relaxed) && uri.ends_with(&format!("/{middle}/{middle}"));
            if blocked_tile && self.fail.load(Relaxed) {
                return Err(egui::load::LoadError::Loading(
                    "simulated tile failure".to_owned(),
                ));
            }
            if uri.starts_with(&format!("{ready}/")) && !blocked_tile {
                Ok(TexturePoll::Ready {
                    texture: SizedTexture::new(
                        egui::TextureId::User(u64::from(self.ready_zoom.load(Relaxed))),
                        vec2(256.0, 256.0),
                    ),
                })
            } else {
                Ok(TexturePoll::Pending { size: None })
            }
        }
        fn forget(&self, _: &str) {}
        fn forget_all(&self) {}
        fn byte_size(&self) -> usize {
            0
        }
    }

    #[test]
    fn retains_previous_layer_until_replacements_are_ready() {
        use std::sync::{atomic::Ordering::Relaxed, Arc};
        let ctx = egui::Context::default();
        let loader = Arc::new(DelayedTextures {
            ready_zoom: std::sync::atomic::AtomicU8::new(2),
            blocked: std::sync::atomic::AtomicBool::new(false),
            fail: std::sync::atomic::AtomicBool::new(false),
        });
        ctx.add_texture_loader(loader.clone());
        let mut map = SlippyMap::new("{z}/{x}/{y}");
        let frame = |map: &mut SlippyMap| {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(512.0, 512.0))),
                    ..Default::default()
                },
                |ui| {
                    egui::CentralPanel::default().show_inside(ui, |ui| {
                        ui.add(&mut *map);
                    });
                },
            )
        };
        map.set_view(0.5, 0.5, 2.0);
        let _ = frame(&mut map);
        assert_eq!(map.displayed.as_ref().map(|layer| layer.zoom), Some(2));
        for next in [3, 1] {
            map.set_view(0.5, 0.5, f64::from(next));
            loader.ready_zoom.store(next, Relaxed);
            loader.blocked.store(true, Relaxed);
            let output = frame(&mut map);
            let previous = map.displayed.as_ref().map(|layer| layer.zoom);
            assert_ne!(previous, Some(next));
            assert!(output.shapes.iter().any(|shape| matches!(&shape.shape,
                egui::epaint::Shape::Mesh(mesh) if mesh.texture_id == egui::TextureId::User(u64::from(previous.unwrap_or(0)))
            )), "previous tiles must still be painted while loading");
            // All other target tiles are ready, but a single failed tile must
            // still prevent promotion of the partial replacement layer.
            loader.fail.store(true, Relaxed);
            let _ = frame(&mut map);
            assert_eq!(map.displayed.as_ref().map(|layer| layer.zoom), previous);
            loader.fail.store(false, Relaxed);
            loader.blocked.store(false, Relaxed);
            let _ = frame(&mut map);
            assert_eq!(map.displayed.as_ref().map(|layer| layer.zoom), Some(next));
        }
    }
}
