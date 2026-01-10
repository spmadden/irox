// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::drawing::{DotJson, DrawContext, Edge, Metanode};
use eframe::emath::{Rect, Vec2};
use eframe::epaint::Color32;
use egui::emath::TSTransform;
use egui::epaint::{RectShape, Stroke, StrokeKind};
use egui::{Align, Layout, PointerButton, Response, Shape, Ui};
use irox_egui_extras::drawpanel::{DrawPanel, LayerCommand};
use irox_egui_extras::eframe;
use irox_egui_extras::egui;
use irox_egui_extras::egui::{CentralPanel, Context, CornerRadius};
use irox_egui_extras::fonts::FontSet;
use irox_egui_extras::toolframe::ToolApp;
use irox_log::log::{error, info};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::mpsc::Sender;

pub struct InitData {
    pub dotjsondata: Vec<u8>,
    pub scale: f32,
    pub pos: [f32; 2],
}

pub struct App {
    dda: DrawPanel,
    nodes: BTreeMap<i32, Rc<RefCell<Node>>>,
    edges: BTreeMap<i32, Rc<AssocEdge>>,
    init: bool,
    search: String,
    found: Vec<Rc<RefCell<Node>>>,
    found_idx: Option<usize>,
    init_data: Option<InitData>,

    main_layer: Sender<LayerCommand>,
    hilite_layer: Sender<LayerCommand>,
}
impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, init: InitData) -> Self {
        irox_egui_extras::fonts::load_fonts(FontSet::all_as_defaults(), &cc.egui_ctx);

        let mut dda = DrawPanel::default();
        let main_layer = dda.add_layer("main".to_string(), true);
        let hilite_layer = dda.add_layer("hilites".to_string(), true);
        Self {
            dda,
            init: false,
            nodes: Default::default(),
            edges: Default::default(),
            search: Default::default(),
            found: Default::default(),
            found_idx: Default::default(),
            init_data: Some(init),
            main_layer,
            hilite_layer,
        }
    }
    fn init(&mut self, ctx: &Context) {
        if self.init {
            return;
        }
        self.init = true;
        let Some(init) = self.init_data.take() else {
            return;
        };

        // let px = ctx.native_pixels_per_point().unwrap_or(1.0);
        // ctx.set_pixels_per_point(1.0);
        // ctx.set_zoom_factor(1./px);
        ctx.tessellation_options_mut(|o| {
            o.feathering = true;
            o.parallel_tessellation = true;
            o.bezier_tolerance = 0.5;
        });
        let data = match DotJson::from_slice(&init.dotjsondata) {
            Ok(d) => d,
            Err(e) => {
                error!("Error loading dotjson data: {e:#?}");
                return;
            }
        };
        let mut draw = DrawContext::new(ctx);

        draw.visuals = Some(ctx.style().visuals.clone());
        let mut nodes = BTreeMap::<i32, Rc<RefCell<Node>>>::new();
        let mut edges = BTreeMap::<i32, Rc<AssocEdge>>::new();
        for obj in &data.objects {
            nodes.insert(
                obj._gvid,
                Rc::new(RefCell::new(Node {
                    meta: obj.clone(),
                    gvid: obj._gvid,
                    zone: Rect::ZERO,
                    searchable_text: obj.name.to_lowercase(),
                    shapes: obj.get_shapes(ctx, Some(draw.clone())),
                    upstream_nodes: vec![],
                    downstream_nodes: vec![],
                    highlit: false,
                })),
            );
        }
        for edge in &data.edges {
            let Some(head) = nodes.get(&edge.head) else {
                continue;
            };
            let Some(tail) = nodes.get(&edge.tail) else {
                continue;
            };
            let e = Rc::new(AssocEdge {
                meta: edge.clone(),
                gvid: edge._gvid,
                head: head.clone(),
                tail: tail.clone(),
                shapes: edge.get_shapes(ctx, Some(draw.clone())),
            });

            edges.insert(edge._gvid, e);
        }
        self.nodes = nodes;
        self.edges = edges;
        let [x, y] = init.pos;
        self.dda.transform = TSTransform::new(Vec2::new(x, y), init.scale);
        self.dda.initial_transform = self.dda.transform;
        self.update_shapes();
    }
    fn update_shapes(&mut self) {
        self.dda.world_area = Rect::NOTHING;
        let mut new_shapes = Vec::new();
        for n in self.nodes.values_mut() {
            let mut node = n.borrow_mut();
            let mut s = node.shapes.clone();
            let mut r = Rect::NOTHING;
            for updates in [&mut r, &mut self.dda.world_area] {
                for v in &s {
                    let bb = v.visual_bounding_rect();
                    updates.extend_with(bb.min);
                    updates.extend_with(bb.max);
                }
            }
            node.zone = r;

            new_shapes.append(&mut s);
        }
        for e in self.edges.values() {
            let mut s = e.shapes.clone();
            for v in &s {
                let bb = v.visual_bounding_rect();
                self.dda.world_area.extend_with(bb.min);
                self.dda.world_area.extend_with(bb.max);
            }
            new_shapes.append(&mut s);
        }
        let _ = self
            .main_layer
            .send(LayerCommand::ClearSetShapes(new_shapes));
        let _ = self.hilite_layer.send(LayerCommand::ClearShapes);
    }
    fn highlight(&mut self, ctx: &Context) {
        let search = self.search.trim().to_lowercase();
        self.found.clear();
        self.found_idx = None;
        let mut draw = DrawContext::new(ctx);
        draw.override_fill = Some(Color32::YELLOW);
        info!("Searching for {search} {}", self.nodes.len());
        let mut new_shapes = Vec::new();
        for node in self.nodes.values_mut() {
            let mut n = node.borrow_mut();
            if !search.is_empty() && n.searchable_text.contains(&search) {
                self.found.push(node.clone());
                n.highlight(true, &draw);
            } else {
                n.highlight(false, &draw);
            }
            new_shapes.append(&mut n.shapes.clone());
        }
        for e in self.edges.values() {
            new_shapes.append(&mut e.shapes.clone());
        }
        let _ = self
            .main_layer
            .send(LayerCommand::ClearSetShapes(new_shapes));
    }
    fn _find_menu_click(&self, _ui: &mut Ui, response: &mut Response) {
        let Some(draw_pos) = response.hover_pos() else {
            return;
        };
        if response.clicked_by(PointerButton::Secondary) {
            let needle = self.dda.transform.inverse().mul_pos(draw_pos);

            for haystack in self.nodes.values() {
                let haystack = haystack.borrow();
                if haystack.zone.contains(needle) {}
            }
        }
    }
    pub fn zoom_to(&mut self, node: &Rc<RefCell<Node>>) {
        let _ = self.hilite_layer.send(LayerCommand::ClearShapes);
        let mut rect = Option::<Rect>::None;
        for shp in &node.borrow().shapes {
            let bb = shp.visual_bounding_rect();
            if let Some(rect) = &mut rect {
                rect.extend_with(bb.min);
                rect.extend_with(bb.max);
            } else {
                rect = Some(bb);
            }
        }
        let Some(rect) = rect else {
            return;
        };
        if let Some(last) = self.dda.last_window_area {
            let _ = self
                .hilite_layer
                .send(LayerCommand::AppendShape(Shape::Rect(RectShape::stroke(
                    rect,
                    CornerRadius::default(),
                    Stroke::new(2.0, Color32::RED),
                    StrokeKind::Middle,
                ))));
            let scale = 1.0;
            let translate =
                -rect.min.to_vec2() + last.center().to_vec2() + (rect.center() - rect.min);
            let xfm = TSTransform::new(translate, scale);
            self.dda.transform = xfm;
        }
    }
}
impl ToolApp for App {}
pub struct AssocEdge {
    pub gvid: i32,
    pub meta: Edge,
    pub head: Rc<RefCell<Node>>,
    pub tail: Rc<RefCell<Node>>,
    pub shapes: Vec<Shape>,
}
pub struct Node {
    pub gvid: i32,
    pub meta: Metanode,
    pub zone: Rect,
    pub searchable_text: String,
    pub shapes: Vec<Shape>,
    pub upstream_nodes: Vec<Rc<RefCell<Node>>>,
    pub downstream_nodes: Vec<Rc<RefCell<Node>>>,
    pub highlit: bool,
}
impl Node {
    pub fn highlight(&mut self, highlit: bool, draw: &DrawContext) {
        if self.highlit != highlit {
            self.shapes.clear();
            if highlit {
                self.shapes = self.meta.get_shapes(draw.ctx, Some(draw.clone()));
            } else {
                self.shapes = self.meta.get_shapes(draw.ctx, None);
            }
            self.highlit = highlit;
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.init(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.label("Search:");
                if ui.text_edit_singleline(&mut self.search).changed() {
                    // if !self.search.is_empty() {
                    self.highlight(ctx);
                    // }
                }
                if !self.search.is_empty() {
                    let found = if self.found.is_empty() {
                        0
                    } else {
                        self.found_idx.unwrap_or_default()
                    };
                    ui.label(format!("found: {}/{}", found, self.found.len()));
                    if ui.button("next").clicked() {
                        let cur_idx = if let Some(f) = &mut self.found_idx {
                            *f += 1;
                            if *f > self.found.len() {
                                *f = 1;
                            }
                            *f
                        } else {
                            self.found_idx = Some(1);
                            1
                        };
                        let cur_idx = cur_idx.saturating_sub(1);
                        if let Some(v) = self.found.get_mut(cur_idx) {
                            let rc = v.clone();
                            self.zoom_to(&rc);
                        }
                    };
                }
            });

            self.dda.show(ui);
        });
    }
}
