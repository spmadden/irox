// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::pub_underscore_fields)]

irox_tools::cfg_feature_egui! {
    mod egui;
    pub use egui::*;
    pub mod app;

}

use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DotJson {
    pub name: String,
    pub directed: bool,
    pub strict: bool,
    #[serde(rename = "_draw_")]
    pub drawopts: Vec<Draw>,
    pub bb: String,
    pub fontname: Option<String>,
    pub fontsize: Option<String>,
    pub label: Option<String>,
    pub lheight: Option<String>,
    pub lp: Option<String>,
    pub lwidth: Option<String>,
    pub size: Option<String>,
    pub ssize: Option<String>,
    pub style: Option<String>,
    #[serde(default)]
    pub rankdir: String,
    #[serde(default)]
    pub xdotversion: String,
    pub _subgraph_cnt: u32,
    pub objects: Vec<Metanode>,
    pub edges: Vec<Edge>,
}
impl DotJson {
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    pub fn from_reader<R: std::io::Read>(r: R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(r)
    }
}
impl FromStr for DotJson {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum Draw {
    #[serde(alias = "c", alias = "C")]
    Color(Color),
    #[serde(alias = "S")]
    Style(Style),
    #[serde(alias = "t")]
    FontStyle(FontStyle),
    #[serde(alias = "p", alias = "P")]
    Polygon(Points),
    #[serde(alias = "T")]
    Text(Text),
    #[serde(alias = "e", alias = "E")]
    Ellipse(Ellipse),
    #[serde(alias = "b", alias = "B")]
    BSPLine(Points),
    #[serde(alias = "L")]
    PolyLine(Points),
    #[serde(alias = "F")]
    Font(Font),
}

pub type Point2 = [f32; 2];
pub type Point3 = [f64; 3];
pub type Rect = [f64; 4];
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
    pub size: f64,
    pub face: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ellipse {
    pub rect: Rect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub pt: Point2,
    pub width: f64,
    pub align: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Points {
    pub points: Vec<Point2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub style: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontStyle {
    pub fontchar: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub grad: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metanode {
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub _draw_: Vec<Draw>,
    pub _ldraw_: Option<Vec<Draw>>,
    pub _gvid: i32,
    pub subgraphs: Option<Vec<i32>>,
    pub height: Option<String>,
    pub label: String,
    pub pos: Option<String>,
    pub width: Option<String>,
    pub color: Option<String>,
    pub fontsize: Option<String>,
    pub textcolor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub _gvid: i32,
    pub head: i32,
    pub tail: i32,
    pub _hldraw_: Option<Vec<Draw>>,
    pub _tdraw_: Option<Vec<Draw>>,
    pub _ldraw_: Option<Vec<Draw>>,
    pub _draw_: Option<Vec<Draw>>,
    pub _tldraw_: Option<Vec<Draw>>,
    pub _hdraw_: Option<Vec<Draw>>,
    pub label: Option<String>,
    pub lp: Option<String>,
    pub pos: Option<String>,
    pub tailport: Option<String>,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
#[cfg(test)]
mod test {
    use crate::drawing::DotJson;
    use irox_egui_extras::egui::{Context, RawInput};
    use irox_egui_extras::fonts::FontSet;
    use std::io::{BufReader, Error};

    #[test]
    pub fn test_read() -> Result<(), Error> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open("data\\power.json")?;
        let mut f = BufReader::new(f);
        let obj: DotJson = serde_json::from_reader(&mut f)?;
        // println!("{obj:#?}");
        let ctx = Context::default();
        irox_egui_extras::fonts::load_fonts(FontSet::all(), &ctx);
        let _r = ctx.run(RawInput::default(), |_ui| {});
        for obj in &obj.objects {
            let shps = obj.get_shapes(&ctx, None);
            println!("{shps:#?}");
        }
        // println!("{:#?}", obj.objects);
        // println!("{:#?}", obj.edges);
        Ok(())
    }
}
