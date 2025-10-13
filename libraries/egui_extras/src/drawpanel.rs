// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::{Color32, CornerRadius};
use egui::{Painter, Response, Shape};

/// A tracking struct for a user interaction with a plot.
///
/// This tracks whether a drag has started, the end delta of the drag, and the
/// current "painter coordinates" of the highlighted area by the mouse.
#[derive(Default)]
pub struct Interaction {
    pub drag_started_pos: Option<Pos2>,
    pub drag_ended_delta: Option<Vec2>,
    pub zoom_area: Option<Rect>,
    pub infinite_x_area: bool,
    pub infinite_y_area: bool,
}
impl Interaction {
    /// Clear and reset the interaction state back to zero.
    pub fn clear(&mut self) {
        self.drag_ended_delta = None;
        self.drag_started_pos = None;
        self.zoom_area = None;
    }
    /// Call update on every frame to track the current interaction state.
    pub fn update(&mut self, response: &mut Response, painter: &mut Painter) {
        if response.drag_started() {
            // mark that a drag has started this frame.
            self.drag_started_pos = response.interact_pointer_pos();
        } else if response.drag_stopped() {
            // mark that a drag has stopped this frame.
            if let Some(start) = self.drag_started_pos {
                // ensure that the location the drag started was tracked.
                if let Some(delta) = self.drag_ended_delta {
                    // make sure the mouse moved between frames, and then update
                    // the zoom rectangle
                    let second = start + delta;
                    let min = Pos2::new(start.x.min(second.x), start.y.min(second.y));
                    let max = Pos2::new(start.x.max(second.x), start.y.max(second.y));
                    let overlay_rect = Rect { min, max };
                    self.clear();
                    self.zoom_area = Some(overlay_rect);
                }
            }
        } else if response.is_pointer_button_down_on() {
            // update the drag delta from the start location as the mouse moves.
            let new_delt = response.drag_delta();
            let delta = self.drag_ended_delta.get_or_insert(Vec2::default());
            delta.x += new_delt.x;
            delta.y += new_delt.y;
            if let Some(start) = self.drag_started_pos {
                let first = start;
                let second = start + *delta;
                let min = Pos2::new(first.x.min(second.x), first.y.min(second.y));
                let max = Pos2::new(first.x.max(second.x), first.y.max(second.y));
                let overlay_rect = Rect { min, max };
                // paint the horizontal drag rectangle.
                let _shp = painter.rect_filled(
                    overlay_rect,
                    CornerRadius::ZERO,
                    Color32::from_black_alpha(64),
                );
            }
            // println!("drag delta: {:#?}", self.interaction.drag_ended_delta);
        }
    }
}
#[derive(Default)]
pub struct DrawPanel {
    pub interaction: Interaction,
    pub shapes: Vec<Shape>,
}
impl DrawPanel {
    pub fn show(&mut self, _ui: &mut egui::Ui) {}
}
