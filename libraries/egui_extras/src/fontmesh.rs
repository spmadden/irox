// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use egui::epaint::{Vertex, WHITE_UV};
use egui::{Color32, Mesh};
use std::sync::Arc;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
struct FontInfo {
    advance: u8,
    h_seg: usize,
    v_seg: usize,
}
impl FontInfo {
    const fn new(inp: [u8; 3]) -> Self {
        let [a, b, c] = inp;
        Self {
            advance: a,
            h_seg: b as usize,
            v_seg: c as usize,
        }
    }
}
static FONT_INFO: [FontInfo; 96] = [
    FontInfo::new([6, 0, 0]),
    FontInfo::new([3, 0, 0]),
    FontInfo::new([5, 1, 1]),
    FontInfo::new([7, 1, 4]),
    FontInfo::new([7, 3, 7]),
    FontInfo::new([7, 6, 12]),
    FontInfo::new([7, 8, 19]),
    FontInfo::new([4, 16, 21]),
    FontInfo::new([4, 17, 22]),
    FontInfo::new([4, 19, 23]),
    FontInfo::new([23, 21, 24]),
    FontInfo::new([23, 22, 31]),
    FontInfo::new([20, 23, 34]),
    FontInfo::new([22, 23, 36]),
    FontInfo::new([19, 24, 36]),
    FontInfo::new([21, 25, 36]),
    FontInfo::new([6, 25, 39]),
    FontInfo::new([6, 27, 43]),
    FontInfo::new([6, 28, 45]),
    FontInfo::new([6, 30, 49]),
    FontInfo::new([6, 33, 53]),
    FontInfo::new([6, 34, 57]),
    FontInfo::new([6, 40, 58]),
    FontInfo::new([6, 46, 59]),
    FontInfo::new([6, 47, 62]),
    FontInfo::new([6, 55, 64]),
    FontInfo::new([19, 57, 68]),
    FontInfo::new([20, 59, 68]),
    FontInfo::new([21, 61, 69]),
    FontInfo::new([22, 66, 69]),
    FontInfo::new([21, 68, 69]),
    FontInfo::new([7, 73, 69]),
    FontInfo::new([9, 75, 74]),
    FontInfo::new([6, 78, 81]),
    FontInfo::new([6, 80, 85]),
    FontInfo::new([6, 83, 90]),
    FontInfo::new([6, 85, 91]),
    FontInfo::new([6, 87, 95]),
    FontInfo::new([6, 90, 96]),
    FontInfo::new([7, 92, 97]),
    FontInfo::new([6, 96, 102]),
    FontInfo::new([5, 97, 106]),
    FontInfo::new([6, 99, 107]),
    FontInfo::new([6, 100, 110]),
    FontInfo::new([6, 100, 115]),
    FontInfo::new([7, 101, 116]),
    FontInfo::new([6, 101, 121]),
    FontInfo::new([6, 101, 125]),
    FontInfo::new([6, 102, 129]),
    FontInfo::new([7, 103, 133]),
    FontInfo::new([6, 104, 140]),
    FontInfo::new([6, 105, 145]),
    FontInfo::new([7, 107, 149]),
    FontInfo::new([6, 108, 151]),
    FontInfo::new([7, 109, 155]),
    FontInfo::new([7, 109, 160]),
    FontInfo::new([7, 109, 165]),
    FontInfo::new([7, 118, 167]),
    FontInfo::new([6, 118, 172]),
    FontInfo::new([4, 120, 176]),
    FontInfo::new([6, 122, 177]),
    FontInfo::new([4, 122, 181]),
    FontInfo::new([23, 124, 182]),
    FontInfo::new([22, 129, 182]),
    FontInfo::new([4, 130, 182]),
    FontInfo::new([22, 131, 183]),
    FontInfo::new([6, 133, 187]),
    FontInfo::new([22, 135, 191]),
    FontInfo::new([6, 137, 192]),
    FontInfo::new([22, 139, 196]),
    FontInfo::new([6, 144, 197]),
    FontInfo::new([22, 147, 198]),
    FontInfo::new([6, 150, 202]),
    FontInfo::new([19, 151, 206]),
    FontInfo::new([21, 152, 207]),
    FontInfo::new([6, 155, 209]),
    FontInfo::new([3, 160, 210]),
    FontInfo::new([23, 160, 211]),
    FontInfo::new([22, 164, 216]),
    FontInfo::new([22, 165, 220]),
    FontInfo::new([22, 167, 224]),
    FontInfo::new([22, 169, 228]),
    FontInfo::new([21, 171, 232]),
    FontInfo::new([21, 173, 233]),
    FontInfo::new([5, 178, 233]),
    FontInfo::new([22, 179, 234]),
    FontInfo::new([23, 180, 238]),
    FontInfo::new([23, 180, 243]),
    FontInfo::new([23, 180, 248]),
    FontInfo::new([22, 189, 248]),
    FontInfo::new([22, 191, 252]),
    FontInfo::new([5, 196, 252]),
    FontInfo::new([3, 203, 252]),
    FontInfo::new([5, 203, 253]),
    FontInfo::new([22, 210, 253]),
    FontInfo::new([0, 214, 253]),
];
static HSEGS: [u8; 214] = [
    97, 37, 69, 84, 28, 51, 2, 18, 10, 49, 98, 41, 65, 25, 81, 105, 33, 9, 97, 1, 97, 37, 37, 36,
    81, 10, 98, 107, 3, 100, 3, 99, 58, 51, 4, 99, 58, 8, 73, 81, 10, 50, 98, 8, 73, 81, 4, 10, 50,
    98, 8, 25, 33, 65, 81, 10, 50, 17, 65, 97, 25, 33, 25, 49, 9, 65, 20, 68, 1, 65, 25, 49, 41,
    11, 105, 13, 101, 76, 10, 50, 10, 50, 98, 11, 99, 10, 98, 11, 50, 99, 11, 50, 11, 99, 8, 57,
    58, 3, 99, 99, 107, 10, 10, 11, 10, 99, 11, 5, 100, 41, 65, 57, 41, 65, 9, 17, 81, 97, 3, 107,
    9, 97, 1, 97, 33, 25, 9, 25, 41, 100, 41, 26, 82, 42, 98, 27, 83, 42, 98, 26, 51, 82, 8, 41,
    35, 8, 10, 26, 82, 114, 42, 1, 114, 8, 9, 73, 57, 81, 41, 97, 18, 8, 8, 25, 26, 26, 82, 26, 82,
    26, 82, 41, 25, 33, 82, 26, 49, 73, 35, 90, 17, 81, 41, 65, 57, 41, 65, 25, 81, 90, 114, 20,
    84, 73, 57, 41, 49, 25, 33, 65, 81, 9, 97, 1, 97, 25, 33, 65, 81, 57, 33, 25, 41, 25,
];
static VSEGS: [u8; 253] = [
    4, 2, 8, 10, 15, 8, 15, 33, 8, 15, 8, 73, 82, 73, 57, 41, 82, 10, 82, 18, 66, 10, 21, 29, 1,
    65, 27, 8, 27, 9, 65, 8, 10, 50, 97, 74, 66, 42, 10, 21, 57, 41, 29, 25, 14, 81, 73, 57, 26, 8,
    8, 26, 66, 3, 8, 8, 15, 19, 21, 90, 58, 26, 18, 66, 18, 105, 89, 28, 74, 17, 8, 73, 57, 26, 21,
    8, 42, 41, 42, 8, 28, 22, 8, 8, 30, 7, 8, 8, 26, 66, 21, 7, 8, 8, 29, 7, 7, 21, 8, 8, 8, 59, 7,
    8, 8, 15, 29, 8, 8, 14, 7, 57, 43, 10, 82, 7, 7, 25, 42, 25, 15, 7, 25, 41, 15, 21, 105, 105,
    29, 7, 57, 57, 26, 21, 105, 73, 97, 89, 28, 97, 7, 57, 58, 26, 82, 18, 57, 57, 74, 8, 30, 6, 8,
    8, 14, 3, 58, 90, 58, 11, 7, 74, 43, 74, 15, 2, 82, 2, 42, 75, 42, 10, 67, 57, 41, 10, 7, 2,
    42, 74, 106, 15, 2, 35, 8, 8, 29, 7, 8, 8, 59, 35, 51, 8, 8, 15, 35, 30, 35, 8, 8, 30, 7, 8, 8,
    60, 36, 8, 45, 7, 7, 36, 8, 43, 8, 44, 21, 8, 8, 44, 35, 8, 8, 43, 23, 8, 8, 43, 35, 8, 8, 31,
    21, 15, 20, 8, 8, 28, 18, 58, 89, 58, 26, 21, 89, 73, 89, 29, 20, 8, 8, 30, 7,
];
#[inline]
fn add_hsegs(mesh: &mut Mesh, mut x: f32, y: f32, segs: &[u8], color: Color32) {
    for seg in segs {
        let len = (*seg & 7) as f32;
        let xoff = (*seg >> 3) & 1;
        let yoff = *seg >> 4;
        x += xoff as f32;
        let y0 = y + yoff as f32;

        mesh.vertices.push(Vertex {
            pos: [x, y0].into(),
            uv: WHITE_UV,
            color,
        });
        mesh.vertices.push(Vertex {
            pos: [x + len, y0].into(),
            uv: WHITE_UV,
            color,
        });
        mesh.vertices.push(Vertex {
            pos: [x + len, y0 + 1.0].into(),
            uv: WHITE_UV,
            color,
        });
        mesh.vertices.push(Vertex {
            pos: [x, y0 + 1.0].into(),
            uv: WHITE_UV,
            color,
        });
        let idx = mesh.vertices.len() as u32 - 4;
        mesh.add_triangle(idx, idx + 1, idx + 2);
        mesh.add_triangle(idx + 2, idx + 3, idx);
    }
}
#[inline]
fn add_vsegs(mesh: &mut Mesh, mut x: f32, y: f32, segs: &[u8], color: Color32) {
    for seg in segs {
        let len = (*seg & 7) as f32;
        let xoff = (*seg >> 3) & 1;
        let yoff = *seg >> 4;
        x += xoff as f32;
        let y0 = y + yoff as f32;

        mesh.vertices.push(Vertex {
            pos: [x, y0].into(),
            uv: WHITE_UV,
            color,
        });
        mesh.vertices.push(Vertex {
            pos: [x + 1.0, y0].into(),
            uv: WHITE_UV,
            color,
        });
        mesh.vertices.push(Vertex {
            pos: [x + 1.0, y0 + len].into(),
            uv: WHITE_UV,
            color,
        });
        mesh.vertices.push(Vertex {
            pos: [x, y0 + len].into(),
            uv: WHITE_UV,
            color,
        });
        let idx = mesh.vertices.len() as u32 - 4;
        mesh.add_triangle(idx, idx + 1, idx + 2);
        mesh.add_triangle(idx + 2, idx + 3, idx);
    }
}
#[derive(Debug, Default, Copy, Clone)]
pub struct SimpleFontParams {
    /// Start X position of the mesh
    pub x_pos: f32,
    /// Start Y position of the mesh
    pub y_pos: f32,
    /// Optional additional X spacing between characters
    pub extra_x_spacing: f32,
    /// Optional additional Y spacing between lines
    pub extra_y_spacing: f32,
    /// Color to render mesh in
    pub color: Color32,
}

#[derive(Debug, Default, Clone)]
pub struct SimpleFontResult {
    /// The resultant mesh
    pub mesh: Arc<Mesh>,
    /// The final width of the rendered text
    pub width: f32,
    /// The final height of the rendered text
    pub height: f32,
}
pub fn generate_mesh(params: SimpleFontParams, s: &str) -> SimpleFontResult {
    let mut out = Mesh::default();
    let SimpleFontParams {
        x_pos,
        y_pos,
        extra_x_spacing,
        extra_y_spacing,
        color,
    } = params;
    let mut x = x_pos;
    let mut y = y_pos;
    for b in s.as_bytes() {
        match *b {
            b'\n' => {
                x = x_pos;
                y += 10.0;
                y += extra_y_spacing;
            }
            b'\r' => {
                x = x_pos;
            }
            b'\t' => {
                x += 20.0;
                continue;
            }
            0..32 | 127.. => {
                continue;
            }
            _ => {
                let offset = b - 32;
                let offset = offset as usize;
                let Some(FontInfo {
                    advance,
                    h_seg,
                    v_seg,
                }) = FONT_INFO.get(offset).copied()
                else {
                    continue;
                };
                let Some(FontInfo {
                    h_seg: hseg_p1,
                    v_seg: vseg_p1,
                    ..
                }) = FONT_INFO.get(offset + 1).copied()
                else {
                    continue;
                };
                let y_ch = if advance & 16 > 0 { y + 1.0 } else { y };
                let Some(hseg) = HSEGS.get(h_seg..hseg_p1) else {
                    continue;
                };
                let Some(vseg) = VSEGS.get(v_seg..vseg_p1) else {
                    continue;
                };
                add_hsegs(&mut out, x, y_ch, hseg, color);
                add_vsegs(&mut out, x, y_ch, vseg, color);
                x += (advance & 15) as f32;
                x += extra_x_spacing;
            }
        }
    }
    SimpleFontResult {
        mesh: Arc::new(out),
        width: x,
        height: y,
    }
}
