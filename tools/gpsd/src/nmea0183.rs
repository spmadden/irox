// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_nmea0183::{Frame, FramePayload};
use irox_tools::options::MaybeFrom;

#[allow(clippy::match_same_arms)]
impl MaybeFrom<Frame> for crate::output::Frame {
    fn maybe_from(value: Frame) -> Option<Self> {
        match value.payload {
            FramePayload::GGA(_) => None,
            FramePayload::GSA(_) => None,
            FramePayload::GSV(_) => None,
            FramePayload::Unknown(_) => None,
        }
    }
}
