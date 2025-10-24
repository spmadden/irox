// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use eframe::emath::{Pos2, Rect};
use egui::emath::TSTransform;
use egui::{Align, Align2, Color32, Painter, Response, Sense, Shape, TextStyle, Ui};

#[derive(Default)]
pub struct DrawPanel {
    pub name: String,
    pub shapes: Vec<Shape>,
    pub transform: TSTransform,
}
impl DrawPanel {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self {
            name: name.as_ref().to_string(),
            ..Default::default()
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        profile_scope!("drawpanel.show", self.name.as_str());
        let size = ui.available_size();
        let (mut response, mut painter) = ui.allocate_painter(size, Sense::click_and_drag());
        let rect = response.rect;
        self.check_zoom(ui, &mut response);

        let mut transform = self.transform;
        transform.translation += rect.min.to_vec2();
        for sh in &self.shapes {
            let mut sh = sh.clone();
            sh.transform(transform);
            painter.add(sh);
        }

        self.draw_cursor(ui, &mut response, &mut painter, None, &rect);
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
            self.transform = TSTransform::IDENTITY;
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
        let galley = painter.layout_no_wrap(text.to_string(), font_id.clone(), color);
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
