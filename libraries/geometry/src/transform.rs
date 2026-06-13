// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use crate::{Point, Vector};
use core::ops::{Deref, Mul};
use irox_tools::{cfg_feature_egui, FloatIsh, ToSigned};
use irox_units::systems::CoordinateTranslator;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LinearTransform<T: FloatIsh> {
    pub scale: T,
    pub translate: Vector<T>,
}
impl LinearTransform<f64> {
    pub fn new_model_point(&self, point: &Point<f64>) -> ModelPoint {
        ModelPoint {
            point: *point,
            transform: *self,
        }
    }
    pub fn new_screen_point(&self, point: &Point<f64>) -> ScreenPoint {
        ScreenPoint {
            point: *point,
            transform: *self,
        }
    }
}
impl<T: FloatIsh> Default for LinearTransform<T> {
    fn default() -> Self {
        Self {
            scale: T::ONE,
            translate: Vector::new(T::ZERO, T::ZERO),
        }
    }
}

impl<T: FloatIsh> LinearTransform<T>
where
    T: ToSigned<Output = T>,
{
    #[must_use]
    pub fn inverse(&self) -> LinearTransform<T> {
        let scale = T::ONE / self.scale;
        let translate = (self.translate * T::negative_one()) * scale;

        Self { scale, translate }
    }
}

impl<T: FloatIsh> Mul<&Point<T>> for LinearTransform<T>
where
    Point<T>: Mul<T, Output = Point<T>>,
{
    type Output = Point<T>;

    fn mul(self, rhs: &Point<T>) -> Self::Output {
        rhs * self.scale + self.translate
    }
}
impl<T: FloatIsh> Mul<&Point<T>> for &LinearTransform<T>
where
    Point<T>: Mul<T, Output = Point<T>>,
{
    type Output = Point<T>;

    fn mul(self, rhs: &Point<T>) -> Self::Output {
        rhs * self.scale + self.translate
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ModelPoint {
    point: Point<f64>,
    transform: LinearTransform<f64>,
}
impl ModelPoint {
    pub fn to_screen(&self) -> ScreenPoint {
        self.transform.to_screen(self)
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ScreenPoint {
    point: Point<f64>,
    transform: LinearTransform<f64>,
}
impl ScreenPoint {
    pub fn to_model(&self) -> ModelPoint {
        self.transform.to_model(self)
    }
}

impl Deref for ModelPoint {
    type Target = Point<f64>;

    fn deref(&self) -> &Self::Target {
        &self.point
    }
}
impl Deref for ScreenPoint {
    type Target = Point<f64>;

    fn deref(&self) -> &Self::Target {
        &self.point
    }
}

impl CoordinateTranslator<ModelPoint, ScreenPoint> for LinearTransform<f64> {
    fn to_model(&self, point: &ScreenPoint) -> ModelPoint {
        ModelPoint {
            point: self.inverse() * point.deref(),
            transform: *self,
        }
    }

    fn to_screen(&self, point: &ModelPoint) -> ScreenPoint {
        ScreenPoint {
            point: self * point.deref(),
            transform: *self,
        }
    }
}

cfg_feature_egui! {
    impl From<egui::emath::TSTransform> for LinearTransform<f64> {
        fn from(transform: egui::emath::TSTransform) -> Self {
            LinearTransform {
                scale: transform.scaling as f64,
                translate: transform.translation.into(),
            }
        }
    }
}
