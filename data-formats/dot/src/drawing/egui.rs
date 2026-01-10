// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::drawing::{Color, Draw, Edge, Ellipse, Font, Metanode, Point2, Points, Text};
use irox_egui_extras::eframe::epaint::{
    CubicBezierShape, EllipseShape, PathShape, RectShape, TextShape,
};
use irox_egui_extras::egui::{
    Color32, Context, CornerRadius, FontFamily, FontId, Pos2, Shape, Stroke, Vec2, Visuals,
};
use irox_egui_extras::fonts::LIGHT;
use irox_log::log::warn;

const DEF_FONT: FontId = FontId::new(14., FontFamily::Proportional);
#[derive(Debug, Default, Copy, Clone)]
pub struct StyleCtx {
    solid: bool,
    linewidth: Option<f32>,
}
#[derive(Debug, Clone)]
pub struct DrawContext<'a> {
    pub last_font: Option<Font>,
    pub last_color: Option<Color32>,
    pub last_stroke_color: Option<Color32>,
    pub last_style: Option<StyleCtx>,
    pub override_color: Option<Color32>,
    pub override_fill: Option<Color32>,
    pub visuals: Option<Visuals>,
    pub ctx: &'a Context,
}
impl<'a> DrawContext<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        Self {
            last_color: None,
            last_stroke_color: None,
            last_font: None,
            last_style: None,
            override_color: None,
            override_fill: None,
            visuals: None,
            ctx,
        }
    }
    pub fn get_text_color(&self, defl: Color32) -> Color32 {
        if let Some(c) = &self.override_color {
            return *c;
        }
        if let Some(c) = &self.last_color {
            return *c;
        };
        if let Some(v) = &self.visuals {
            return v.widgets.inactive.fg_stroke.color;
        }
        defl
    }
    pub fn get_stroke_color(&self, defl: Color32) -> Color32 {
        if let Some(c) = &self.override_color {
            return *c;
        }
        if let Some(c) = &self.last_stroke_color {
            return *c;
        };
        if let Some(c) = &self.last_color {
            return *c;
        };
        if let Some(v) = &self.visuals {
            return v.widgets.inactive.fg_stroke.color;
        }
        defl
    }
    pub fn get_fill_color(&self, defl: Color32) -> Color32 {
        if let Some(c) = &self.override_fill {
            return *c;
        }
        if let Some(s) = &self.last_style {
            if s.solid {
                return self.get_stroke_color(defl);
            }
        }
        defl
    }
    pub fn get_stroke(&self, defl: f32, col: Color32) -> Stroke {
        if let Some(s) = &self.last_style {
            if let Some(s) = s.linewidth {
                return Stroke::new(s, self.get_stroke_color(col));
            }
        }
        Stroke::new(defl, self.get_stroke_color(col))
    }
    pub fn get_font(&self, defl: FontId) -> FontId {
        if let Some(f) = &self.last_font {
            let size = f.size as f32;
            let family = FontFamily::Name(LIGHT.into());
            return FontId::new(size, family);
        };
        defl
    }
}
pub trait Drawable {
    fn get_shape(&self, ctx: &mut DrawContext) -> Option<Shape>;
}

impl Points {
    #[allow(unused_assignments, clippy::indexing_slicing)]
    pub fn bspline(&self, ctx: &mut DrawContext) -> Shape {
        let mut out = Vec::new();

        let mut a = invy(self.points[0]);
        let mut b = invy(self.points[1]);
        let mut c = invy(self.points[2]);
        let mut d = invy(self.points[0]);
        let mut i = 0;
        while i < self.points.len() - 3 {
            a = d;
            b = invy(self.points[i + 1]);
            c = invy(self.points[i + 2]);
            d = invy(self.points[i + 3]);
            out.push(Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                [a, b, c, d],
                false,
                Color32::TRANSPARENT,
                ctx.get_stroke(1.0, Color32::BLACK),
            )));
            i += 3;
        }
        Shape::Vec(out)
    }
    pub fn polyline(&self, ctx: &mut DrawContext) -> Shape {
        let mut points = Vec::new();
        for pnt in &self.points {
            points.push(invy(*pnt));
        }
        let stroke = ctx.get_stroke(1.0, Color32::BLACK);
        Shape::Path(PathShape::line(points, stroke))
    }
    pub fn polygon(&self, ctx: &mut DrawContext) -> Shape {
        let mut points = Vec::new();
        for pnt in &self.points {
            points.push(invy(*pnt));
        }
        let stroke = ctx.get_stroke(1.0, Color32::BLACK);
        Shape::Path(PathShape::convex_polygon(
            points,
            ctx.get_fill_color(Color32::TRANSPARENT),
            stroke,
        ))
    }
}

impl Color {
    pub fn as_color32(&self) -> Color32 {
        Color32::from_hex(&self.color).unwrap_or(Color32::BLACK)
    }
}

fn map_color(s: Option<&str>) -> Option<Color32> {
    let s = s?;
    match s {
        "blue" => Some(Color32::BLUE),
        "black" => Some(Color32::BLACK),
        "red" => Some(Color32::RED),
        v => {
            warn!("Unknown color: {v}");
            None
        }
    }
}
impl Metanode {
    #[allow(clippy::needless_pass_by_value)]
    pub fn get_shapes(&self, ectx: &Context, draw: Option<DrawContext>) -> Vec<Shape> {
        let mut out = Vec::new();
        let mut draw = draw.clone().unwrap_or(DrawContext::new(ectx));
        draw.last_stroke_color = map_color(self.color.as_deref());
        draw.last_color = map_color(self.textcolor.as_deref());

        let mut ctx = draw.clone();
        for d in &self._draw_ {
            if let Some(s) = d.get_shape(&mut ctx) {
                out.push(s);
            }
        }
        if let Some(ldraw) = &self._ldraw_ {
            ctx = draw.clone();
            for d in ldraw {
                if let Some(s) = d.get_shape(&mut ctx) {
                    out.push(s);
                }
            }
        }
        out
    }
}

macro_rules! maybevec {
    ($it:expr, $ectx:ident, $out:ident, $defl: ident) => {{
        let mut ctx = $defl.clone().unwrap_or(DrawContext::new($ectx));
        if let Some(draw) = $it {
            for d in draw {
                if let Some(s) = d.get_shape(&mut ctx) {
                    $out.push(s);
                }
            }
        }
    }};
}

impl Edge {
    #[allow(clippy::needless_pass_by_value)]
    pub fn get_shapes(&self, ectx: &Context, draw: Option<DrawContext>) -> Vec<Shape> {
        let mut out = Vec::new();
        maybevec!(&self._draw_, ectx, out, draw);
        maybevec!(&self._hdraw_, ectx, out, draw);
        maybevec!(&self._tdraw_, ectx, out, draw);
        maybevec!(&self._ldraw_, ectx, out, draw);
        maybevec!(&self._hldraw_, ectx, out, draw);
        maybevec!(&self._tldraw_, ectx, out, draw);
        out
    }
}
impl Drawable for Text {
    fn get_shape(&self, ctx: &mut DrawContext) -> Option<Shape> {
        let col = ctx.get_text_color(Color32::BLACK);
        let font = ctx.get_font(DEF_FONT);

        let shift = if self.align == "c" {
            Vec2::new(self.width as f32 / 2., font.size)
        } else {
            Vec2::ZERO
        };
        let text = Shape::Text(TextShape::new(
            Pos2::new(self.pt[0], -self.pt[1]) - shift,
            ctx.ctx
                .fonts(|f| f.layout_no_wrap(self.text.clone(), font, col)),
            col,
        ));
        Some(text)
    }
}
impl Drawable for Ellipse {
    fn get_shape(&self, ctx: &mut DrawContext) -> Option<Shape> {
        let center = Pos2::new(self.rect[0] as f32, -self.rect[1] as f32);
        let radius = Vec2::new(self.rect[2] as f32, self.rect[3] as f32);
        let stroke = ctx.get_stroke(1.0, Color32::BLACK);
        let ellipse = Shape::Ellipse(EllipseShape::stroke(center, radius, stroke));
        if let Some(fill) = ctx.override_fill {
            let rect = ellipse.visual_bounding_rect().expand(5.);
            let rect = Shape::Rect(RectShape::filled(rect, CornerRadius::default(), fill));
            return Some(Shape::Vec(vec![rect, ellipse]));
        }

        Some(ellipse)
    }
}
impl Drawable for Draw {
    fn get_shape(&self, ctx: &mut DrawContext) -> Option<Shape> {
        match self {
            Draw::Color(c) => {
                ctx.last_stroke_color = Some(c.as_color32());
            }
            Draw::FontStyle(_) => {}
            Draw::Font(f) => {
                ctx.last_font = Some(f.clone());
            }
            Draw::Style(s) => match s.style.as_str() {
                "solid" => {
                    ctx.last_style.get_or_insert_default().solid = true;
                }
                "setlinewidth(3)" => {
                    ctx.last_style.get_or_insert_default().linewidth = Some(3.);
                }
                _ => {}
            },

            Draw::Polygon(p) => {
                return Some(p.polygon(ctx));
            }
            Draw::Text(e) => {
                return Drawable::get_shape(e, ctx);
            }
            Draw::Ellipse(e) => {
                return Drawable::get_shape(e, ctx);
            }
            Draw::BSPLine(b) => return Some(b.bspline(ctx)),
            Draw::PolyLine(pl) => return Some(pl.polyline(ctx)),
        }
        None
    }
}

fn invy(p2: Point2) -> Pos2 {
    let [x, y] = p2;
    Pos2::new(x, -y)
}
