// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use eframe::emath::Vec2;
use eframe::{App, CreationContext, Frame};
use egui::epaint::{PathShape, RectShape, TextShape, Vertex};
use egui::{
    CentralPanel, Color32, CornerRadius, FontFamily, FontId, Grid, Id, Mesh, Pos2, Rect, Sense,
    Shape, Slider, Stroke, StrokeKind, Ui, ViewportBuilder, Widget,
};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use irox_geometry::{Geometry, Point, Polygon, Vector, Vector2D};
use irox_imagery::colormaps::TABLEAU_10;
use irox_units::units::angle::Angle;
use log::{error, Level};
use std::sync::Arc;

pub struct Response {
    pub hovered: bool,
    pub clicked: bool,
}
impl Response {
    pub fn hovered(&self) -> bool {
        self.hovered
    }
    pub fn clicked(&self) -> bool {
        self.clicked
    }
}
pub struct ArcWedge {
    pub identifier: Id,
    pub start_angle: Angle,
    pub end_angle: Angle,
    pub pad_angle: Angle,
    pub inner_length: f32,
    pub outer_length: f32,
    pub pad_length: f32,
    pub size: f32,
    pub fill_color: Color32,
    pub hovered_fill_color: Color32,
}
impl ArcWedge {
    pub fn show(&self, ui: &mut Ui) -> Response {
        let hovered: bool = ui.memory(|mem| mem.data.get_temp(self.identifier).unwrap_or_default());
        let (id, rect) = ui.allocate_space(Vec2::splat(self.size));
        let response = ui.interact(rect, id, Sense::click());
        let painter = ui.painter_at(rect);
        let painter_space = response.rect;
        let ctr = painter_space.center();
        #[cfg(debug_assertions)]
        {
            painter.add(Shape::rect_stroke(
                rect,
                CornerRadius::default(),
                Stroke::new(2.0, Color32::RED),
                StrokeKind::Middle,
            ));
            painter.add(Shape::circle_filled(rect.center(), 2.0, Color32::RED));
        }
        let mut polygon_intersection = Polygon::<f32>::empty();
        let mut mesh = Mesh::default();
        let fill_color = if hovered {
            self.hovered_fill_color
        } else {
            self.fill_color
        };
        {
            // start line
            let pos = ctr
                + Vector::new(0.0, self.inner_length)
                    .rotate(self.start_angle + Angle::new_degrees(180.))
                    .into();
            mesh.vertices.push(Vertex::untextured(pos, fill_color));
            polygon_intersection.add_point(pos.into());
            let pos = ctr
                + Vector::new(0.0, self.outer_length)
                    .rotate(self.start_angle + Angle::new_degrees(180.))
                    .into();
            mesh.vertices.push(Vertex::untextured(pos, fill_color));
            polygon_intersection.add_point(pos.into());
        }
        let mut remaining_points = Vec::new();
        let mut idx = 0;
        {
            let mut angle = self.start_angle + Angle::new_degrees(180.);
            while angle < self.end_angle + Angle::new_degrees(180.) {
                let pos = ctr + Vector::new(0.0, self.inner_length).rotate(angle).into();
                mesh.vertices.push(Vertex::untextured(pos, fill_color));
                remaining_points.push(pos);
                let pos = ctr + Vector::new(0.0, self.outer_length).rotate(angle).into();
                mesh.vertices.push(Vertex::untextured(pos, fill_color));
                polygon_intersection.add_point(pos.into());
                angle += Angle::new_degrees(1.);

                mesh.add_triangle(idx, idx + 1, idx + 3);
                mesh.add_triangle(idx + 3, idx + 2, idx);
                idx += 2;
            }
        }
        while let Some(pos) = remaining_points.pop() {
            polygon_intersection.add_point(pos.into());
        }

        #[cfg(debug_assertions)]
        {
            let mut points = Vec::<Pos2>::new();
            for pnt in polygon_intersection.iter_points() {
                points.push((*pnt).into());
            }
            let shp = Shape::Path(PathShape::closed_line(
                points,
                Stroke::new(1.0, Color32::RED),
            ));
            painter.add(shp);
            let bbox = polygon_intersection.bounding_rectangle();
            let rect = Rect::from_min_max(bbox.min.into(), bbox.far_point().into());
            let shp = Shape::Rect(RectShape::stroke(
                rect,
                CornerRadius::default(),
                Stroke::new(1.0, Color32::RED),
                StrokeKind::Middle,
            ));
            painter.add(shp);
        }

        let shp = Shape::Mesh(Arc::new(mesh));
        painter.add(shp);
        let hovered = if let Some(hover_pos) = response.hover_pos() {
            if response.hovered() {
                #[cfg(debug_assertions)]
                {
                    let gallery = ui.fonts_mut(|font| {
                        font.layout(
                            hover_pos.to_string(),
                            FontId::new(10., FontFamily::default()),
                            Color32::RED,
                            100.0,
                        )
                    });
                    let shp = Shape::Text(TextShape::new(hover_pos, gallery, Color32::RED));
                    painter.add(shp);
                }
                let hover_pos: Point<f32> = hover_pos.into();
                polygon_intersection.contains(&hover_pos)
            } else {
                false
            }
        } else {
            false
        };
        let clicked = if hovered { response.clicked() } else { false };
        ui.memory_mut(|mem| {
            mem.data.insert_temp(self.identifier, hovered);
        });

        Response { hovered, clicked }
    }
}

pub fn main() {
    irox_log::init_console_level(Level::Info);
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "irox-egui-gallery-arcwedge",
        native_options,
        Box::new(|cc| {
            let comp = Box::new(ToolFrame::new(cc, Box::new(TestApp::new(cc))));
            Ok(comp)
        }),
    ) {
        error!("{e:?}");
    };
}

pub struct TestApp {
    pub start_angle: f32,
    pub end_angle: f32,
    pub pad_angle: f32,
    pub inner_length: f32,
    pub outer_length: f32,
    pub pad_length: f32,
    pub size: f32,
}
impl TestApp {
    pub fn new(_cc: &CreationContext) -> Self {
        TestApp {
            start_angle: 0.0,
            end_angle: 45.0,
            pad_angle: 1.0,
            inner_length: 150.0,
            outer_length: 200.0,
            pad_length: 1.0,
            size: 450.0,
        }
    }
}

impl App for TestApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            Grid::new("arcwedge_grid").show(ui, |ui| {
                Slider::new(&mut self.start_angle, 0.0..=360.0)
                    .text("Start Angle")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.end_angle, self.start_angle..=360.0)
                    .text("End Angle")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.pad_angle, 0.0..=5.0)
                    .text("Pad Angle")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.inner_length, 0.0..=self.size)
                    .text("Inner Length")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.outer_length, self.inner_length..=self.size)
                    .text("Outer Length")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.pad_length, 0.0..=5.0)
                    .text("Pad Length")
                    .ui(ui);
                ui.end_row();
                Slider::new(&mut self.size, 1.0..=1024.0)
                    .text("Size")
                    .ui(ui);
                ui.end_row();
            });
            let wedge = ArcWedge {
                identifier: Id::new("wedge1"),
                start_angle: Angle::new_degrees(self.start_angle as f64),
                end_angle: Angle::new_degrees(self.end_angle as f64),
                pad_angle: Angle::new_degrees(self.pad_angle as f64),
                inner_length: self.inner_length,
                outer_length: self.outer_length,
                pad_length: self.pad_length,
                size: self.size,
                fill_color: TABLEAU_10[0].into(),
                hovered_fill_color: TABLEAU_10[1].into(),
            };
            let resp = wedge.show(ui);
            if resp.hovered() {
                //todo
            }
        });
    }
}
impl ToolApp for TestApp {}
