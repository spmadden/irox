// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LinearRegression {
    pub slope: f64,
    pub mean_x: f64,
    pub mean_y: f64,
}

impl LinearRegression {
    pub fn from_data<
        'a,
        S: 'a,
        I: Iterator<Item = S> + Clone,
        X: Fn(&S) -> f64,
        Y: Fn(&S) -> f64,
    >(
        data: I,
        x_accessor: X,
        y_accessor: Y,
    ) -> Option<LinearRegression> {
        let mut mean_x = 0f64;
        let mut mean_y = 0f64;
        let mut count = 0f64;

        for s in data.clone() {
            let x = x_accessor(&s);
            let y = y_accessor(&s);
            mean_y += y;
            mean_x += x;
            count += 1.0;
        }

        if count <= 1. {
            return None;
        }
        mean_x /= count;
        mean_y /= count;

        let mut xsum = 0f64;
        let mut linear = 0f64;
        for s in data {
            let x = x_accessor(&s);
            let y = y_accessor(&s);

            let dy = y - mean_y;
            let dx = x - mean_x;

            xsum += dx * dx;
            linear += dx * dy;
        }
        if xsum <= 0.0 {
            return None;
        }

        Some(LinearRegression {
            slope: linear / xsum,
            mean_x,
            mean_y,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::fitting::LinearRegression;
    use crate::sampling::Sample;
    use irox_time::epoch::UnixTimestamp;

    #[test]
    pub fn test() {
        let data = &[
            Sample::new(0., UnixTimestamp::from_seconds(0)),
            Sample::new(0.5, UnixTimestamp::from_seconds_f64(0.5)),
            Sample::new(1., UnixTimestamp::from_seconds(1)),
        ];

        let reg = LinearRegression::from_data(
            data.iter(),
            |s| s.time.get_offset().as_seconds_f64(),
            |s| s.value,
        );
        assert!(reg.is_some());
        let Some(reg) = reg else {
            return;
        };
        assert_eq!(1.0, reg.slope);
        assert_eq!(0.5, reg.mean_x);
        assert_eq!(0.5, reg.mean_y);
    }
}
