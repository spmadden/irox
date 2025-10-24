// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::coordinate::{EllipticalCoordinate, Latitude, Longitude};
use crate::geo::EllipticalShape;
use core::ops::{Bound, RangeBounds};
use irox_units::units::angle::Angle;
use irox_units::units::length::Length;

#[derive(Debug, Copy, Clone)]
pub struct EllipticalRange {
    pub elliptical_shape: EllipticalShape,
    pub vertical_range: LatitudeRange,
    pub horizontal_range: LongitudeRange,
}
impl EllipticalRange {
    pub fn contains(&self, pnt: &EllipticalCoordinate) -> bool {
        &self.elliptical_shape == pnt.get_reference_frame()
            && self.vertical_range.contains(pnt.get_latitude())
            && self.horizontal_range.contains(pnt.get_longitude())
    }
}
impl RangeBounds<Latitude> for EllipticalRange {
    fn start_bound(&self) -> Bound<&Latitude> {
        RangeBounds::start_bound(&self.vertical_range)
    }

    fn end_bound(&self) -> Bound<&Latitude> {
        RangeBounds::end_bound(&self.vertical_range)
    }
}
impl RangeBounds<Longitude> for EllipticalRange {
    fn start_bound(&self) -> Bound<&Longitude> {
        RangeBounds::start_bound(&self.horizontal_range)
    }

    fn end_bound(&self) -> Bound<&Longitude> {
        RangeBounds::end_bound(&self.horizontal_range)
    }
}
macro_rules! range {
    ($range:ident, $ty:ident, $iter:ident, $lowerbound:expr) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $range {
            pub lower_bound: Bound<$ty>,
            pub upper_bound: Bound<$ty>,
        }
        impl RangeBounds<$ty> for $range {
            fn start_bound(&self) -> Bound<&$ty> {
                self.lower_bound.as_ref()
            }

            fn end_bound(&self) -> Bound<&$ty> {
                self.upper_bound.as_ref()
            }
        }
        impl $range {
            pub fn from<T: RangeBounds<$ty>>(value: T) -> Self {
                let lower_bound = value.start_bound().cloned();
                let upper_bound = value.end_bound().cloned();
                Self {
                    lower_bound,
                    upper_bound,
                }
            }
            pub fn iter_step(&self, step: $ty) -> $iter {
                let next = match self.lower_bound {
                    Bound::Unbounded => $lowerbound,
                    Bound::Included(v) => v - step,
                    Bound::Excluded(v) => v,
                };
                $iter {
                    range: *self,
                    next,
                    step,
                }
            }
        }
        pub struct $iter {
            range: $range,
            step: $ty,
            next: $ty,
        }
        impl Iterator for $iter {
            type Item = $ty;

            fn next(&mut self) -> Option<Self::Item> {
                let next = self.next + self.step;
                if self.range.contains(&next) {
                    self.next = next;
                    return Some(next);
                }
                None
            }
        }
    };
}
range!(LatitudeRange, Latitude, LatitudeIter, Latitude::min_value());
range!(
    LongitudeRange,
    Longitude,
    LongitudeIter,
    Longitude::min_value()
);
range!(AngleRange, Angle, AngleIter, Angle::min_value());

#[derive(Debug, Copy, Clone)]
pub struct CartesianRange {
    pub x_range: XAxisRange,
    pub y_range: YAxisRange,
    pub z_range: Option<ZAxisRange>,
}

range!(XAxisRange, Length, XAxisIter, Length::new_meters(-7e6));
range!(YAxisRange, Length, YAxisIter, Length::new_meters(-7e6));
range!(ZAxisRange, Length, ZAxisIter, Length::new_meters(-7e6));

#[cfg(test)]
mod test {
    use crate::coordinate::Latitude;
    use crate::range::LatitudeRange;
    use core::ops::RangeBounds;
    use irox_units::units::angle::Angle;

    #[test]
    pub fn test() {
        let start = Latitude(Angle::new_degrees(1.));
        let end = Latitude(Angle::new_degrees(10.));
        let r = LatitudeRange::from(start..=end);
        assert!(r.contains(&Latitude(Angle::new_degrees(5.0))));

        let mut i = r.iter_step(start);
        assert_eq!(Some(Latitude(Angle::new_degrees(1.0))), i.next());
        assert_eq!(Some(Latitude(Angle::new_degrees(2.0))), i.next());
        assert_eq!(Some(Latitude(Angle::new_degrees(3.0))), i.next());
        assert_eq!(Some(Latitude(Angle::new_degrees(4.0))), i.next());
        assert_eq!(Some(Latitude(Angle::new_degrees(5.0))), i.next());
        assert_eq!(Some(Latitude(Angle::new_degrees(6.0))), i.next());
        assert_eq!(Some(Latitude(Angle::new_degrees(7.0))), i.next());
        assert_eq!(Some(Latitude(Angle::new_degrees(8.0))), i.next());
        assert_eq!(Some(Latitude(Angle::new_degrees(9.0))), i.next());
        assert_eq!(Some(Latitude(Angle::new_degrees(10.0))), i.next());
        assert_eq!(None, i.next());

        ();
    }
}
