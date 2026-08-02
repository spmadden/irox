// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::widgets::arcwedge::{ArcWedge, ArcWedgeSet};
use eframe::emath::Align;
use egui::{Align2, Context, Id, Label, Layout, Order, Ui, Vec2, Window};
use irox_time::epoch::UnixTimestamp;
use irox_tools::static_init;
use irox_units::units::angle::Angle;
use irox_units::units::duration::Duration;

#[derive(Debug, Default, Clone)]
pub struct ToastIndex {
    bitfield: u64,
}
impl ToastIndex {
    pub fn claim_next(&mut self) -> Option<u64> {
        for idx in 0..63 {
            let bit = 1 << idx;
            if self.bitfield & bit == 0 {
                self.bitfield |= bit;
                return Some(idx);
            }
        }
        None
    }
    pub fn try_improve(&mut self, current: Option<u64>) -> Option<u64> {
        let check = current.unwrap_or(64);

        for idx in (0..check).rev() {
            let bit = 1 << idx;
            if self.bitfield & bit == 0 {
                self.bitfield |= bit;

                if let Some(current) = current {
                    self.free(current);
                }

                return Some(idx);
            }
        }
        current
    }
    pub fn free(&mut self, idx: u64) {
        let bit = 1 << idx;
        self.bitfield &= !bit;
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ToastState {
    #[default]
    Initializing,
    Opening,
    Pending,
    Closing,
    Done,
}

#[derive(Debug, Clone)]
struct ToastInnerState {
    opening: bool,
    state: ToastState,
    opened_time: Option<UnixTimestamp>,
    last_rect: Option<egui::Rect>,
    my_offset: Option<u64>,
}
impl Default for ToastInnerState {
    fn default() -> Self {
        Self {
            opening: false,
            state: ToastState::Initializing,
            opened_time: None,
            last_rect: None,
            my_offset: None,
        }
    }
}
static_init!(get_toast_count, Id, Id::new("TOAST_COUNT"));
pub struct Toast {
    pub id: Id,
    pub title: String,
    pub text: String,
    pub timeout: Duration,
}

impl Toast {
    pub fn new(id: Id, title: String, text: String, timeout: Duration, ui: &mut Ui) -> Self {
        ui.memory_mut(|mem| {
            let mut istate = ToastInnerState::default();

            let v = mem
                .data
                .get_temp_mut_or_default::<ToastIndex>(*get_toast_count());

            istate.my_offset = v.claim_next();
            mem.data.insert_temp(id.with("_state"), istate);
        });

        Self {
            id,
            title,
            text,
            timeout,
        }
    }
    fn clean_state(id: Id, state: &mut ToastInnerState, ui: &mut Ui) {
        state.opened_time = None;
        ui.memory_mut(|mem| mem.data.remove_temp::<ToastState>(id.with("_state")));
        if let Some(toast_idx) = state.my_offset.take() {
            ui.memory_mut(|mem| {
                let v = mem
                    .data
                    .get_temp_mut_or_default::<ToastIndex>(*get_toast_count());
                v.free(toast_idx);
            });
        }
    }
    pub fn show(&self, ctx: &Context, ui: &mut egui::Ui) -> ToastState {
        let Some(mut state) = ui
            .memory_mut::<Option<ToastInnerState>>(|mem| mem.data.get_temp(self.id.with("_state")))
        else {
            return ToastState::Done;
        };
        ui.memory_mut(|mem| {
            let v = mem
                .data
                .get_temp_mut_or_default::<ToastIndex>(*get_toast_count());
            state.my_offset = v.try_improve(state.my_offset);
        });
        let anim = ui.animate_bool_with_time(self.id.with("_anim"), state.opening, 0.5);
        let voff = state.last_rect.map(|rect| rect.height()).unwrap_or(500.);

        let toast_index = state.my_offset.unwrap_or_default() as f32;
        let anchor = Vec2::new(-5.0, -voff * anim - voff * toast_index + voff);
        match (state.state, anim) {
            (ToastState::Initializing, 0.0) => {
                state.opening = true;
                state.state = ToastState::Opening;
            }
            (ToastState::Opening, 1.0) => {
                state.state = ToastState::Pending;
                state.opened_time = Some(UnixTimestamp::now());
            }
            (ToastState::Pending, 1.0) => {
                if let Some(opened_time) = state.opened_time {
                    if opened_time.elapsed() >= self.timeout {
                        state.state = ToastState::Closing;
                        state.opening = false;
                    }
                }
            }
            (ToastState::Closing, 0.0) => {
                Self::clean_state(self.id, &mut state, ui);
                return ToastState::Done;
            }
            (_, _) => {}
        }
        let result = Window::new(&self.text)
            .id(self.id)
            .title_bar(false)
            .order(Order::Foreground)
            // .scroll(true)
            .movable(false)
            .resizable(false)
            .constrain(false)
            .collapsible(false)
            .min_width(250.0)
            .max_width(250.0)
            // .pivot(Align2::RIGHT_BOTTOM)
            .anchor(Align2::RIGHT_BOTTOM, anchor)
            .show(ctx, |ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(450., 25.),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.add_sized(Vec2::new(250., 25.), Label::new(&self.title));
                        if let Some(opened_time) = state.opened_time {
                            let elapsed = opened_time.elapsed();
                            let end_angle = Angle::new_degrees(360.) * (elapsed / self.timeout);
                            ArcWedgeSet {
                                identifier: self.id.with("_wedgeset"),
                                size: 24.,
                                wedges: vec![ArcWedge {
                                    identifier: self.id.with("_wedge"),
                                    start_angle: Angle::new_degrees(0.0),
                                    end_angle,
                                    pad_angle: Angle::new_degrees(0.0),
                                    inner_length: 7.0,
                                    outer_length: 8.0,
                                    pad_length: 0.0,
                                    stroke_color: ui.visuals().code_bg_color,
                                    fill_color: Default::default(),
                                    hovered_fill_color: Default::default(),
                                }],
                            }
                            .show(ui);
                        }
                        if ui.button("X").clicked() {
                            Self::clean_state(self.id, &mut state, ui);
                            state.state = ToastState::Done;
                        };
                    },
                );
                ui.label(&self.text);
            });
        let Some(result) = result else {
            return ToastState::Done;
        };
        let last_rect = result.response.rect;
        state.last_rect = Some(last_rect);
        let outstate = state.state;
        if outstate != ToastState::Done {
            ui.memory_mut(|mem| {
                mem.data.insert_temp(self.id.with("_state"), state);
            });
        }
        outstate
    }
}
