// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use egui::emath::{Pos2, Rect, TSTransform};
use egui::{Align, Align2, Color32, Painter, Response, Sense, Shape, TextStyle, Ui, Vec2};
use std::sync::mpsc::{channel, Receiver, Sender};

pub enum LayerCommand {
    AppendShape(Shape),
    ClearShapes,
    ClearSetShapes(Vec<Shape>),
    UpdateOptions(LayerOpts),
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ScaleMode {
    #[default]
    ScaleEverything,
    ScaleOnlyPosition,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LayerOpts {
    pub visible: bool,
    pub scale_mode: ScaleMode,
}
impl Default for LayerOpts {
    fn default() -> Self {
        LayerOpts {
            visible: true,
            scale_mode: ScaleMode::ScaleEverything,
        }
    }
}
pub struct Layer {
    pub name: String,
    pub sender: Sender<LayerCommand>,
    pub visible: bool,
    pub shapes: Vec<Shape>,
    pub scale_mode: ScaleMode,
    receiver: Receiver<LayerCommand>,
}
impl Layer {
    pub fn new(name: String, opts: LayerOpts) -> Self {
        let (tx, rx) = channel::<LayerCommand>();

        let LayerOpts {
            visible,
            scale_mode,
        } = opts;
        Layer {
            name,
            sender: tx,
            receiver: rx,
            visible,
            scale_mode,
            shapes: Vec::default(),
        }
    }
    pub fn sender(&self) -> Sender<LayerCommand> {
        self.sender.clone()
    }
    fn process_commands(&mut self) {
        profile_scope!("drawpanel.layer.process_commands", self.name.as_str());
        while let Ok(shp) = self.receiver.try_recv() {
            match shp {
                LayerCommand::AppendShape(shp) => {
                    self.shapes.push(shp);
                }
                LayerCommand::ClearShapes => self.shapes.clear(),
                LayerCommand::ClearSetShapes(shps) => {
                    self.shapes = shps;
                }
                LayerCommand::UpdateOptions(opts) => {
                    self.visible = opts.visible;
                    self.scale_mode = opts.scale_mode;
                }
            }
        }
    }
    pub fn show(&mut self, transform: &TSTransform, painter: &Painter) {
        profile_scope!("drawpanel.layer.show", self.name.as_str());
        self.process_commands();
        if self.visible {
            for sh in &self.shapes {
                let mut sh = sh.clone();
                match self.scale_mode {
                    ScaleMode::ScaleEverything => {
                        sh.transform(*transform);
                    }
                    ScaleMode::ScaleOnlyPosition => Self::scale_position(&mut sh, *transform),
                }

                painter.add(sh);
            }
        }
    }

    fn scale_position(shape: &mut Shape, transform: TSTransform) {
        match shape {
            Shape::Text(txt) => {
                txt.pos = transform.mul_pos(txt.pos);
            }
            Shape::Circle(c) => {
                c.center = transform.mul_pos(c.center);
            }
            Shape::Ellipse(e) => {
                e.center = transform.mul_pos(e.center);
            }
            Shape::LineSegment { points, .. } => {
                for p in points {
                    *p = transform * *p;
                }
            }
            Shape::Path(p) => {
                for p in &mut p.points {
                    *p = transform * *p;
                }
            }
            Shape::Rect(r) => {
                let xlate = transform * r.rect.left_top();
                r.rect = Rect::from_min_size(xlate, r.rect.size());
            }
            Shape::QuadraticBezier(b) => {
                for p in &mut b.points {
                    *p = transform * *p;
                }
            }
            Shape::CubicBezier(b) => {
                for p in &mut b.points {
                    *p = transform * *p;
                }
            }
            Shape::Vec(v) => {
                for v in v {
                    Self::scale_position(v, transform);
                }
            }
            v => v.transform(transform),
        }
    }
}

#[derive()]
pub struct DrawPanel {
    pub name: String,
    pub layers: Vec<Layer>,
    pub transform: TSTransform,
    pub initial_transform: TSTransform,

    pub last_window_area: Option<Rect>,
    pub world_area: Rect,
    pub draw_cursor_crosshairs: bool,
}
impl Default for DrawPanel {
    fn default() -> Self {
        Self {
            name: Default::default(),
            layers: vec![],
            transform: Default::default(),
            initial_transform: Default::default(),
            last_window_area: None,
            world_area: Rect::ZERO,
            draw_cursor_crosshairs: true,
        }
    }
}
impl DrawPanel {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self {
            name: name.as_ref().to_string(),
            ..Default::default()
        }
    }
    pub fn add_layer(&mut self, name: String, opts: LayerOpts) -> Sender<LayerCommand> {
        let layer = Layer::new(name, opts);
        let sender = layer.sender();
        self.layers.push(layer);
        sender
    }
    pub fn show(&mut self, ui: &mut Ui) {
        profile_scope!("drawpanel.show", self.name.as_str());
        if ui.ctx().cumulative_pass_nr_for(ui.ctx().viewport_id()) == 0 {
            // skip first frame because we need to know where we are in the frame.
            return;
        };
        let size = ui.available_size();
        let (mut response, mut painter) = ui.allocate_painter(size, Sense::click_and_drag());
        self.check_zoom(ui, &mut response);
        let rect = response.rect;

        if self.last_window_area.is_none() {
            self.initial_transform.translation += Vec2::new(rect.width() / 2., rect.height() / 2.);
            self.transform = self.initial_transform;
        }
        self.last_window_area = Some(rect);

        let mut transform = self.transform;
        transform.translation += rect.min.to_vec2();
        for layer in &mut self.layers {
            layer.show(&transform, &painter);
        }
        if self.draw_cursor_crosshairs {
            self.draw_cursor(ui, &mut response, &mut painter, None, &rect);
        }
    }
    fn check_zoom(&mut self, ui: &mut Ui, response: &mut Response) {
        profile_scope!("check_zoom", self.name.as_str());
        self.transform.translation += response.drag_delta();
        let (zd, mousepos) = ui.input(|i| {
            let mut zd = i.zoom_delta();
            if (zd - 1.0).abs() < 1e-9 {
                zd = i.smooth_scroll_delta.y / 100. + 1.0;
            }
            (zd, i.pointer.latest_pos())
        });
        if (zd - 1.0).abs() != 0.0 {
            let Some(mousepos) = mousepos else {
                return;
            };
            if response.rect.contains(mousepos) {
                let m1 = mousepos;
                let s1 = self.transform.translation;
                let s2 = m1 - zd * (m1 - s1);
                self.transform.translation = s2;
                self.transform.scaling *= zd;
            }
        }
        if response.double_clicked() {
            self.transform = self.initial_transform;
        }
    }
    fn draw_cursor(
        &mut self,
        ui: &mut Ui,
        response: &mut Response,
        painter: &mut Painter,
        closest_point: Option<Pos2>,
        grid_bounds: &Rect,
    ) {
        // draw the hover cursors
        let draw_pos = if let Some(closest_point) = closest_point {
            closest_point
        } else if let Some(hover) = response.hover_pos() {
            hover
        } else {
            return;
        };
        if !grid_bounds.contains(draw_pos) {
            return;
        }
        let rect = response.rect;
        let xrng = rect.min.x..=rect.max.x;
        let yrng = rect.min.y..=rect.max.y;
        let color = ui.visuals().widgets.noninteractive.fg_stroke;
        // paint the crosshair lines
        painter.hline(xrng, draw_pos.y, color);
        painter.vline(draw_pos.x, yrng, color);

        // paint the text
        let mod_pos = self.transform.inverse().mul_pos(draw_pos);
        let mod_x = mod_pos.x;
        let mod_y = mod_pos.y;
        // let mod_x = self.x_axis.describe_screen_pos(draw_pos.x);
        // let mod_y = y_axis.describe_screen_pos(draw_pos.y);
        let text: String = format!("x: {mod_x}\ny: {mod_y}");
        let color = ui.visuals().text_cursor.stroke.color;
        let font_id = TextStyle::Monospace.resolve(ui.style());
        let mut align = Align2::LEFT_BOTTOM;

        // figure out if it extends out past the rectangle
        let galley = painter.layout_no_wrap(text.clone(), font_id.clone(), color);
        let txtrect = align.anchor_size(draw_pos, galley.size());

        if txtrect.max.x >= rect.max.x {
            // flip the x dimension
            let [_, v] = align.0;
            align.0 = [Align::Max, v];
        }
        if txtrect.min.y <= rect.min.y {
            // flip the y dimension
            let [h, _] = align.0;
            align.0 = [h, Align::Min];
        }
        let galley = painter.layout_no_wrap(text, font_id, color);
        let rect = align.anchor_size(draw_pos, galley.size());

        painter.rect_filled(rect, 0.0, Color32::from_white_alpha(32));
        painter.galley(rect.min, galley, color);
    }
}
