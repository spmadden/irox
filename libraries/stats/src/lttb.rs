// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::points::Point2D;
use irox_tools::buf::{Buffer, RoundBuffer};

pub struct LTTBDownsampler<T: Point2D> {
    buf: RoundBuffer<3, T>,
}

impl<T: Point2D> LTTBDownsampler<T> {
    pub fn add(&mut self, val: T) -> Option<T> {
        if self.buf.len() < 3 {
            let _ = self.buf.push_back(val);
            return None;
        }
        let x = val.get_x();
        let y = val.get_y();
        let _z = T::new(x, y);
        None
    }
}
