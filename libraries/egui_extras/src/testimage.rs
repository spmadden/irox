// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use egui::epaint::RectShape;
use egui::{
    Color32, ColorImage, Context, CornerRadius, ImageData, Mesh, Pos2, Rect, Shape, TextureHandle,
    TextureId, TextureOptions, Vec2,
};
use std::sync::{Arc, OnceLock};

static LENA: &[u8] = include_bytes!("../data/lena.data");
static LENA_LOADED: OnceLock<TextureHandle> = OnceLock::new();
pub fn get_lena(ctx: &Context) -> TextureId {
    LENA_LOADED
        .get_or_init(|| {
            let img = ImageData::Color(Arc::new(ColorImage::from_gray([256, 256], LENA)));
            ctx.load_texture("LENA", img, TextureOptions::NEAREST)
        })
        .id()
}
static NUM_LOADED: OnceLock<[TextureHandle; 10]> = OnceLock::new();
pub fn get_img(ctx: &Context, num: usize) -> Option<TextureId> {
    let nums = NUM_LOADED.get_or_init(|| {
        let mut idx = 0;
        [0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39].map(|v| {
            let img = irox_imagery::bitpacked::nums::INFO
                .get_glyph(v)
                .unwrap_or_default();
            let img = ImageData::Color(Arc::new(img.into()));
            let name = idx.to_string();
            idx += 1;
            ctx.load_texture(name, img, TextureOptions::NEAREST)
        })
    });
    nums.get(num).map(TextureHandle::id)
}
static CHECKER_LOADED: OnceLock<TextureHandle> = OnceLock::new();
pub fn checker(ctx: &Context, draw: Rect, scale: f32, offset: f32) -> Shape {
    let tx = CHECKER_LOADED.get_or_init(|| {
        let img = ImageData::Color(Arc::new(ColorImage::from_gray(
            [2, 2],
            &[0x00, 0xFF, 0xFF, 0x00],
        )));
        ctx.load_texture("CHKR", img, TextureOptions::NEAREST_REPEAT)
    });
    let img = Rect::from_min_size(Pos2::new(offset, offset), Vec2::new(1., 1.) * scale);
    let mut mesh = Mesh::with_texture(tx.id());
    mesh.add_rect_with_uv(draw, img, Color32::WHITE);
    Shape::Mesh(Arc::new(mesh))
}

pub fn numshape(ctx: &Context, num: usize, pos: Pos2) -> Option<Shape> {
    let tx = get_img(ctx, num)?;
    Some(imgmeshshape(
        tx,
        Rect::from_min_size(pos, Vec2::new(16., 29.)),
    ))
}
pub fn lenashape(ctx: &Context, pos: Pos2) -> Shape {
    let tx = get_lena(ctx);
    imgmeshshape(tx, Rect::from_min_size(pos, Vec2::new(256., 256.)))
}
pub fn imgmeshshape(tx: TextureId, drawpos: Rect) -> Shape {
    let img = Rect::from_min_size(Pos2::ZERO, Vec2::new(1., 1.));
    let mut mesh = Mesh::with_texture(tx);
    mesh.add_rect_with_uv(drawpos, img, Color32::WHITE);
    Shape::Mesh(Arc::new(mesh))
}
pub fn walking(ctx: &Context, pat: u64, shr: bool) -> TextureHandle {
    let mut greys = [0u8; 4096];

    let mut pat = pat;
    let mut g = greys.iter_mut();
    for _ in 0..64 {
        for i in 0..64 {
            let idx = 63 - i;
            let v = (pat >> idx) & 0x1;
            if let Some(g) = g.next() {
                *g = 0xFF * v as u8;
            }
        }
        if shr {
            pat = pat.rotate_right(1);
        } else {
            pat = pat.rotate_left(1);
        }
    }

    let img = ImageData::Color(Arc::new(ColorImage::from_gray([64, 64], greys.as_ref())));
    ctx.load_texture(format!("walking-{pat:016X}"), img, TextureOptions::NEAREST)
}

pub struct TestImage {
    pub handles: Vec<TextureHandle>,
    pub shapes: Vec<Shape>,
    pub width: f32,
    pub height: f32,
}

impl TestImage {
    pub fn new(ctx: &Context) -> Self {
        let mut handles = Vec::new();
        let mut shapes = Vec::new();
        // background fill
        shapes.push(Shape::Rect(RectShape::filled(
            Rect {
                min: Pos2::new(0.0, 0.0),
                max: Pos2::new(1024.0, 1024.0),
            },
            CornerRadius::ZERO,
            Color32::from_rgb(0x80, 0x80, 0x80),
        )));
        // main boxes
        let spacing = 20.0f32;
        let side = 180.;
        let colorincr = 0x11u8;
        {
            let initpos = Pos2::new(22., 22.);
            let mut color = 0;
            let mut curpos = initpos;
            for i in 0..5 {
                // walk y down
                let col = Color32::from_rgb(color, color, color);
                color += colorincr;
                let min = curpos;
                let max = min + Vec2::new(side, side);
                shapes.push(Shape::Rect(RectShape::filled(
                    Rect { min, max },
                    CornerRadius::ZERO,
                    col,
                )));
                let min = min + Vec2::new(4., 147.);
                if let Some(n) = numshape(ctx, i + 1, min) {
                    shapes.push(n);
                }
                curpos.y += side + spacing;
            }
            curpos.y -= side + spacing;
            curpos.x += side + spacing;
            for i in 0..4 {
                // walk x right
                let col = Color32::from_rgb(color, color, color);
                color += colorincr;
                let min = curpos;
                let max = min + Vec2::new(side, side);
                shapes.push(Shape::Rect(RectShape::filled(
                    Rect { min, max },
                    CornerRadius::ZERO,
                    col,
                )));
                let min = min + Vec2::new(4., 147.);
                if let Some(n) = numshape(ctx, i + 6, min) {
                    shapes.push(n);
                }
                curpos.x += side + spacing;
            }
            curpos.y -= side + spacing;
            curpos.x -= side + spacing;
            for i in 0..4 {
                // walk y up
                let col = Color32::from_rgb(color, color, color);
                color += colorincr;
                let min = curpos;
                let max = min + Vec2::new(side, side);
                shapes.push(Shape::Rect(RectShape::filled(
                    Rect { min, max },
                    CornerRadius::ZERO,
                    col,
                )));
                let min = min + Vec2::new(4., 147.);
                if let Some(n) = numshape(ctx, 1, min) {
                    shapes.push(n);
                }
                let min = min + Vec2::new(16., 0.0);
                if let Some(n) = numshape(ctx, i, min) {
                    shapes.push(n);
                }
                curpos.y -= side + spacing;
            }
            curpos.y += side + spacing;
            curpos.x -= side + spacing;
            for i in 0..3 {
                // walk x left
                let col = Color32::from_rgb(color, color, color);
                color = color.saturating_add(colorincr);
                let min = curpos;
                let max = min + Vec2::new(side, side);
                shapes.push(Shape::Rect(RectShape::filled(
                    Rect { min, max },
                    CornerRadius::ZERO,
                    col,
                )));
                let min = min + Vec2::new(4., 147.);
                if let Some(n) = numshape(ctx, 1, min) {
                    shapes.push(n);
                }
                let min = min + Vec2::new(16., 0.0);
                if let Some(n) = numshape(ctx, i + 4, min) {
                    shapes.push(n);
                }
                curpos.x -= side + spacing;
            }
        }
        // gradients
        {
            let basepos = Pos2::new(256., 256.);
            for i in 0..=255 {
                let brtening = Color32::from_rgb(i, i, i);
                let d = 0xFF - i;
                let darkening = Color32::from_rgb(d, d, d);
                let min = basepos + Vec2::new(2. * i as f32, 0.0);
                let max = min + Vec2::new(2., 32.);
                shapes.push(Shape::Rect(RectShape::filled(
                    Rect { min, max },
                    CornerRadius::ZERO,
                    brtening,
                )));

                let min = min + Vec2::new(0.0, 384. + 96.);
                let max = min + Vec2::new(2., 32.);
                shapes.push(Shape::Rect(RectShape::filled(
                    Rect { min, max },
                    CornerRadius::ZERO,
                    darkening,
                )));
            }
            let bp2 = basepos + Vec2::new(0.0, 448.);
            let basepos = basepos + Vec2::new(512. - 32., 32.);
            for i in 0..16 {
                let c = (0x11 * i) as u8;
                let c = Color32::from_gray(c);
                let min = basepos - Vec2::new(32.0, 0.0) * i as f32;
                let max = min + Vec2::new(32., 32.);
                shapes.push(Shape::Rect(RectShape::filled(
                    Rect { min, max },
                    CornerRadius::ZERO,
                    c,
                )));
                let min = bp2 + Vec2::new(32., 0.) * i as f32;
                let max = min + Vec2::new(32., 32.);
                shapes.push(Shape::Rect(RectShape::filled(
                    Rect { min, max },
                    CornerRadius::ZERO,
                    c,
                )));
            }
        }
        // bars
        {
            let basepos = Pos2::new(320., 320.);
            for i in 0..384 {
                let min = basepos + Vec2::new(i as f32, 0.0);
                let max = min + Vec2::new(1.0, 32.);
                let col = if i & 0x1 == 0x1 {
                    Color32::WHITE
                } else {
                    Color32::BLACK
                };
                // 1px horizontal top
                shapes.push(Shape::Rect(RectShape::filled(
                    Rect { min, max },
                    CornerRadius::ZERO,
                    col,
                )));
                if i & 0x1 == 0x0 {
                    let col = if i & 0x3 == 0x0 {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    };
                    let min = min + Vec2::new(0.0, 352.0);
                    let max = min + Vec2::new(2., 32.);
                    // 2px bar 4
                    shapes.push(Shape::Rect(RectShape::filled(
                        Rect { min, max },
                        CornerRadius::ZERO,
                        col,
                    )));
                }
                if i & 0x3 == 0x0 {
                    // 4px vertical bars
                    let col = if i & 0x7 == 0x0 {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    };
                    let min = min + Vec2::new(0.0, 32.0);
                    let max = min + Vec2::new(4., 32.);
                    // 4px bar 2
                    shapes.push(Shape::Rect(RectShape::filled(
                        Rect { min, max },
                        CornerRadius::ZERO,
                        col,
                    )));
                }
                if i & 0x7 == 0x0 {
                    // 8px bar 3
                    let col = if i & 0xF == 0x0 {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    };
                    let min = min + Vec2::new(0.0, 320.0);
                    let max = min + Vec2::new(8.0, 32.0);
                    shapes.push(Shape::Rect(RectShape::filled(
                        Rect { min, max },
                        CornerRadius::ZERO,
                        col,
                    )));
                }
            }
            let basepos = basepos + Vec2::new(0.0, 64.);
            for i in 0..=255 {
                let min = basepos + Vec2::new(0.0, i as f32);
                let max = min + Vec2::new(32.0, 1.0);
                let col = if i & 0x1 == 0x0 {
                    Color32::WHITE
                } else {
                    Color32::BLACK
                };
                shapes.push(Shape::Rect(RectShape::filled(
                    Rect { min, max },
                    CornerRadius::ZERO,
                    col,
                )));
                if i & 0x1 == 0x0 {
                    let col = if i & 0x3 == 0x0 {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    };
                    let min = min + Vec2::new(352., 0.0);
                    let max = min + Vec2::new(32., 4.);
                    shapes.push(Shape::Rect(RectShape::filled(
                        Rect { min, max },
                        CornerRadius::ZERO,
                        col,
                    )));
                }
                if i & 0x3 == 0x0 {
                    let col = if i & 0x5 == 0x0 {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    };
                    let min = min + Vec2::new(32.0, 0.0);
                    let max = min + Vec2::new(32., 4.);
                    shapes.push(Shape::Rect(RectShape::filled(
                        Rect { min, max },
                        CornerRadius::ZERO,
                        col,
                    )));
                }
                if i & 0x7 == 0x0 {
                    let col = if i & 0xF == 0x0 {
                        Color32::WHITE
                    } else {
                        Color32::BLACK
                    };
                    let min = min + Vec2::new(320.0, 0.0);
                    let max = min + Vec2::new(32.0, 8.0);
                    shapes.push(Shape::Rect(RectShape::filled(
                        Rect { min, max },
                        CornerRadius::ZERO,
                        col,
                    )));
                }
            }
        }
        {
            // checkers
            let pos = Pos2::new(256., 320.);
            let end = pos + Vec2::new(64., 64.);
            shapes.push(checker(ctx, Rect::from_min_max(pos, end), 8., 0.));
            let pos = Pos2::new(256., 384.);
            let end = pos + Vec2::new(64., 64.);
            shapes.push(checker(ctx, Rect::from_min_max(pos, end), 16., 0.25));
            let pos = Pos2::new(256., 448.);
            let end = pos + Vec2::new(64., 64.);
            shapes.push(checker(ctx, Rect::from_min_max(pos, end), 32., 0.));
        }
        {
            // walkings
            let pos = Pos2::new(256., 512.);
            let hndl = walking(ctx, 0x7777777777777777, true);
            let tx = hndl.id();
            handles.push(hndl);
            shapes.push(imgmeshshape(
                tx,
                Rect::from_min_size(pos, Vec2::new(64., 64.)),
            ));

            let pos = Pos2::new(256., 576.);
            let hndl = walking(ctx, 0xCCCCCCCCCCCCCCCC, false);
            let tx = hndl.id();
            handles.push(hndl);
            shapes.push(imgmeshshape(
                tx,
                Rect::from_min_size(pos, Vec2::new(64., 64.)),
            ));

            let pos = Pos2::new(256., 640.);
            let hndl = walking(ctx, 0x8888888888888888, true);
            let tx = hndl.id();
            handles.push(hndl);
            shapes.push(imgmeshshape(
                tx,
                Rect::from_min_size(pos, Vec2::new(64., 64.)),
            ));
        }
        {
            // lena
            let tx = get_lena(ctx);
            let min = Pos2::new(384., 384.);
            let max = min + Vec2::new(256., 256.);
            let pos = Rect::from_min_max(min, max);
            shapes.push(imgmeshshape(tx, pos));
        }
        Self {
            handles,
            shapes,
            width: 1024.,
            height: 1024.,
        }
    }
}
