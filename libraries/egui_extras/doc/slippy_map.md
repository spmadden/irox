# Slippy-map integration and troubleshooting

`widgets::slippy_map::SlippyMap` lays out square raster tiles and handles navigation.
It uses egui loaders rather than adding a transport or decoder dependency to the
widget. Keep the map instance between frames and draw it with `ui.add(&mut map)`.

## Coordinates, units, and sizing

URL templates substitute `{z}`, `{x}`, and `{y}`. The default scheme is TMS,
whose Y increases northward from the bottom. Select `TileScheme::Xyz` for servers
whose Y increases southward from the top, including the standard OSM raster
endpoint. At zoom `z`, the row conversion is `tms_y = (1 << z) - 1 - xyz_y`.
X wraps horizontally; Y does not wrap.

`set_view(x, y, zoom)` takes normalized Web Mercator world coordinates, not
longitude and latitude in degrees. X increases eastward and Y southward;
`(0.5, 0.5)` is longitude/latitude `(0, 0)`. X wraps into `[0, 1)` and Y clamps
to `[0, 1]`. Non-finite inputs are ignored. Zoom is fractional and bounded by
zero and the configured maximum (default 22, hard cap 30). The requested tile
level is `floor(zoom)`; the remaining fraction scales those tiles continuously.

`tile_size(512)` and `set_tile_size(512)` specify square source PNG dimensions
in pixels. The default is 256; zero clamps to one. This is not an egui point
measurement or a request for a different image resolution from the server.

```text
tile width in points = tile_size_pixels * 2^(zoom - tile_level) / pixels_per_point
world width in points = tile_size_pixels * 2^zoom / pixels_per_point
```

At an integer zoom, a 512-pixel tile occupies 256 points on a 2x display. Layout,
panning, pointer-centered zoom, and retained layers use the current context's
pixels-per-point value, including egui UI zoom. Changing tile size keeps the
normalized center and map zoom unchanged. Decoded images are drawn into the
configured tile rectangle; their dimensions are not automatically used to infer
or validate tile size.

## Loader ownership and zoom transitions

Register HTTP byte and image loaders, or a combined HTTP/image loader, on the
same egui context. Egui supplies a default texture loader. Custom asynchronous
loaders must cache pending and completed results and request a repaint when they
finish. Do not hold cache locks while calling back into egui.

The widget requests tiles intersecting its clipped visible area. Once every
visible tile is ready as a GPU texture, it replaces the displayed layer together.
Until then, it draws the previous complete layer at the current view transform.
On first use, with no complete layer, available tiles appear progressively.
Pending or failed replacement tiles do not paint placeholder/error icons over a
usable old map. This avoids a flash at integer zoom transitions in both directions.

The retained layer contains texture IDs, not owning texture handles. Keep those
textures alive in the loader cache while displayed. Cache budgets, eviction,
error reporting, retry, and cancellation belong to the application. Load failures
can leave the old layer displayed indefinitely until the application retries.
Panning or zooming far outside retained coverage can expose the background; the
fallback only contains tiles from the previous complete viewport. It is not an
offline map or a guarantee of coverage for arbitrary jumps.

Retained rendering iterates cached tiles and their visible horizontal copies.
It must not enumerate the previous level's entire viewport grid after a large
zoom-out, when that grid could contain billions of tiny unavailable tiles.

## Native OpenStreetMap example

From the repository root:

```sh
cargo run -p irox-egui-extras --example slippy_map --features eframe,glow
```

The example registers a combined image loader: `ehttp` performs HTTPS requests,
`irox-imagery` decodes PNG bytes, and egui uploads textures. Disk reads and decoding
happen off the UI thread. A graphical desktop and internet access are required.

| Setting | Meaning |
| --- | --- |
| `IROX_MAP_TILE_SIZE` | Positive integer PNG tile size; default 256. |
| `IROX_OSM_CACHE_DIR` | Disk cache directory; default `irox-osm-tiles` under the system temporary directory. |

OSM's standard endpoint returns 256-pixel tiles. Setting the example size to 512
scales those same images; it does not obtain 512-pixel tiles. For another server,
configure its URL, scheme, size, and maximum zoom together. The example loader
intentionally accepts only numeric `z/x/y.png` URLs under its OSM prefix, so adapt
its URL validation, cache namespace, identification, and attribution as well.

The example reuses disk tiles younger than seven days based on modification time.
Invalid cache images are fetched again. HTTP failures, decoding failures, and disk
write failures are cached as errors in memory and reported above the map. Errors
are not automatically retried, and the last error message is not automatically
cleared. Restart after correcting connectivity or permissions; applications can
implement explicit retry by forgetting the affected URI through egui's loader API.
Keep displayed texture lifetime in mind when adding eviction or retry controls.
Forgetting does not cancel in-flight work in this example. A reload of the same
URI can race an older callback; production retry logic needs request identities
or cancellation to prevent stale completion from replacing newer results.

Choose a durable cache directory for repeated use: the system may clean temporary
files. The example does not implement bounded memory/disk eviction, conditional
HTTP revalidation, request concurrency limits, or robust multi-process cache writes.
Do not treat it as a production cache implementation. Follow the provider's
[current tile policy](https://operations.osmfoundation.org/policies/tiles/),
including attribution, identification, caching, and limits on automated access.

### Decoder limitations

The example uses the workspace PNG decoder, not a full general-purpose image
library. The regression fixture `libraries/imagery/assets/0.png` is an 8-bit
indexed-color 256x256 PNG. Its dimensions and first palette-derived pixel are
checked in the example tests. This establishes support for that fixture, not all
PNGs a tile server might emit.

In particular, the current `PNGFile::to_image` reads past scanline filter bytes
without reconstructing nonzero PNG filters. Do not assume arbitrary filtered or
interlaced tiles render correctly. When adopting another server, verify decoded
pixel content against representative known images and address decoder limitations
in `irox-imagery`, or install a suitable existing loader in the application.

## Verification

```sh
cargo test -p irox-egui-extras --lib --example slippy_map --features eframe,glow
cargo test -p irox-egui-extras --doc
```

Regression tests cover row conversion and wrapping, finite view bounds, 256/512
sizes at fractional and 2x scaling, zoom anchoring, clipping/world edges, and
retaining the old layer while replacement tiles are partially ready or fail.
The controlled texture loader tests both zoom directions without network access.
The example tests PNG fixture content and disk-cache path restrictions.

For manual verification, run the example with a writable persistent cache, pan
and cross integer zoom boundaries in both directions, change display/UI scale,
and check attribution and error visibility. A warm-cache run alone cannot verify
cold-load transitions. Do not automate public-server sweeps to test loading;
use a controlled loader or a local server. Record separately whether manual GUI
and live-network behavior were exercised.
