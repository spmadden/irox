// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

#![allow(clippy::indexing_slicing)]

use crate::ToSigned;
use core::iter::Sum;
use core::ops::{Add, Deref, DerefMut, Index, IndexMut, Mul, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Matrix<const M: usize, const N: usize, T: Sized + Copy + Default> {
    values: [[T; N]; M],
}

impl<const M: usize, const N: usize, T: Sized + Copy + Default> Matrix<M, N, T> {
    #[must_use]
    pub const fn new(values: [[T; N]; M]) -> Matrix<M, N, T> {
        Matrix { values }
    }
}
impl<const M: usize, const N: usize> Matrix<M, N, f64> {
    #[must_use]
    pub const fn mul<const P: usize>(&self, other: Matrix<P, M, f64>) -> Matrix<P, N, f64> {
        let mut out = [[0.0; N]; P];
        let mut i = 0;
        while i < M {
            let mut j = 0;
            while j < P {
                let mut k = 0;
                let mut sum = 0.0;
                while k < N {
                    sum += self.values[i][k] * other.values[k][j];
                    k += 1;
                }
                out[i][j] = sum;
                j += 1;
            }
            i += 1;
        }
        Matrix { values: out }
    }
}

macro_rules! impl_square {
    ($N:literal) => {
        impl<T: Sized + Copy + Default> Matrix<$N, $N, T> {
            #[must_use]
            pub fn empty() -> Self {
                Self {
                    values: [<[T; $N]>::default(); $N],
                }
            }
        }
        impl Matrix<$N, $N, f64> {
            #[must_use]
            pub fn identity() -> Self {
                let mut out = Self::empty();
                for i in 0..$N {
                    out[i][i] = 1.0;
                }
                out
            }
        }
    };
}
impl_square!(2);
impl_square!(3);
impl_square!(4);
impl_square!(5);
impl_square!(6);
impl_square!(7);
impl_square!(8);
impl_square!(9);
impl_square!(10);

impl Matrix<2, 2, f64> {
    #[must_use]
    pub fn rotation_counterclockwise(angle: f64) -> Self {
        Self::new([[angle.cos(), -angle.sin()], [angle.sin(), angle.cos()]])
    }
    #[must_use]
    pub fn rotate_counterclockwise(&self, angle: f64) -> Self {
        self.mul(Self::rotation_counterclockwise(angle))
    }
    #[must_use]
    pub fn rotation_clockwise(angle: f64) -> Self {
        Self::new([[angle.cos(), angle.sin()], [-angle.sin(), angle.cos()]])
    }
    #[must_use]
    pub fn rotate_clockwise(&self, angle: f64) -> Self {
        self.mul(Self::rotation_clockwise(angle))
    }

    #[must_use]
    pub const fn sheered_x(factor: f64) -> Self {
        Self::new([[1., factor], [0., 1.]])
    }
    #[must_use]
    pub const fn sheer_x(&self, factor: f64) -> Self {
        self.mul(Self::sheered_x(factor))
    }
    #[must_use]
    pub const fn sheered_y(factor: f64) -> Self {
        Self::new([[1., 0.], [factor, 1.]])
    }
    #[must_use]
    pub const fn sheer_y(&self, factor: f64) -> Self {
        self.mul(Self::sheered_y(factor))
    }

    #[must_use]
    pub const fn scaled_x(factor: f64) -> Self {
        Self::new([[factor, 0.], [0., 1.]])
    }
    #[must_use]
    pub const fn scale_x(&self, factor: f64) -> Self {
        self.mul(Self::scaled_x(factor))
    }

    #[must_use]
    pub const fn scaled_y(factor: f64) -> Self {
        Self::new([[1., 0.], [0., factor]])
    }
    #[must_use]
    pub const fn scale_y(&self, factor: f64) -> Self {
        self.mul(Self::scaled_y(factor))
    }

    #[must_use]
    pub const fn scaled(factor: f64) -> Self {
        Self::new([[factor, 0.], [0., factor]])
    }
    #[must_use]
    pub const fn scale(&self, factor: f64) -> Self {
        self.mul(Self::scaled(factor))
    }
    #[must_use]
    pub const fn reflected() -> Self {
        Self::new([[-1., 0.], [0., -1.]])
    }
    #[must_use]
    pub const fn reflect(&self) -> Self {
        self.mul(Self::reflected())
    }

    #[must_use]
    pub const fn reflected_x() -> Self {
        Self::new([[1., 0.], [0., -1.]])
    }
    #[must_use]
    pub const fn reflect_x(&self) -> Self {
        self.mul(Self::reflected_x())
    }
    #[must_use]
    pub const fn reflected_y() -> Self {
        Self::new([[-1., 0.], [0., 1.]])
    }
    #[must_use]
    pub const fn reflect_y(&self) -> Self {
        self.mul(Self::reflected_y())
    }
}
impl Matrix<3, 1, f64> {
    #[must_use]
    pub const fn translate(&self, x: f64, y: f64) -> Self {
        Matrix::mul(self, Matrix::new([[1., 0., x], [0., 1., y], [0., 0., 1.]]))
    }
}

impl Matrix<3, 3, f64> {
    #[must_use]
    pub fn rotate_x(angle: f64) -> Self {
        Self::new([
            [1., 0., 0.],
            [0., angle.cos(), -angle.sin()],
            [0., angle.sin(), angle.cos()],
        ])
    }
    #[must_use]
    pub fn rotate_y(angle: f64) -> Self {
        Self::new([
            [angle.cos(), 0.0, angle.sin()],
            [0., 1., 0.],
            [-angle.sin(), 0., angle.cos()],
        ])
    }
    #[must_use]
    pub fn rotate_z(angle: f64) -> Self {
        Self::new([
            [angle.cos(), -angle.sin(), 0.],
            [angle.sin(), angle.cos(), 0.],
            [0., 0., 1.],
        ])
    }
}

impl<const M: usize, const N: usize, T: Sized + Copy + Default> Index<usize> for Matrix<M, N, T> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        self.values.index(index)
    }
}
impl<const M: usize, const N: usize, T: Sized + Copy + Default> IndexMut<usize>
    for Matrix<M, N, T>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.values.index_mut(index)
    }
}

impl<const M: usize, const N: usize, T: Sized + Copy + Default> Deref for Matrix<M, N, T> {
    type Target = [[T; N]; M];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
impl<const M: usize, const N: usize, T: Sized + Copy + Default> DerefMut for Matrix<M, N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
impl<
        const M: usize,
        const N: usize,
        const P: usize,
        T: Sized + Copy + Default + Add + Mul + Sum<<T as Mul<T>>::Output>,
    > Mul<Matrix<N, P, T>> for Matrix<M, N, T>
{
    type Output = Matrix<M, P, T>;
    fn mul(self, other: Matrix<N, P, T>) -> Matrix<M, P, T> {
        let mut out = [[T::default(); P]; M];
        for (i, ith) in out.iter_mut().enumerate().take(M) {
            for (j, val) in ith.iter_mut().enumerate().take(P) {
                *val = (0..N)
                    .map(|k| self.values[i][k] * other.values[k][j])
                    .sum::<T>();
            }
        }
        Matrix { values: out }
    }
}
impl<const M: usize, const N: usize, T: Sized + Copy + Default + Add<Output = T>> Add
    for Matrix<M, N, T>
{
    type Output = Matrix<M, N, T>;
    fn add(self, other: Matrix<M, N, T>) -> Matrix<M, N, T> {
        let mut out = [[T::default(); N]; M];

        for (i, ith) in out.iter_mut().enumerate().take(M) {
            for (j, val) in ith.iter_mut().enumerate().take(N) {
                *val = self.values[i][j] + other.values[i][j];
            }
        }
        Matrix { values: out }
    }
}
impl<const M: usize, const N: usize, T: Sized + Copy + Default + Mul<T, Output = T>> Mul<T>
    for Matrix<M, N, T>
{
    type Output = Matrix<M, N, T>;
    fn mul(self, other: T) -> Matrix<M, N, T> {
        let mut out = [[T::default(); N]; M];

        for (i, ith) in out.iter_mut().enumerate().take(M) {
            for (j, val) in ith.iter_mut().enumerate().take(N) {
                *val = self.values[i][j] * other;
            }
        }
        Matrix { values: out }
    }
}

impl<
        const M: usize,
        const N: usize,
        T: Sized + Copy + Default + Add<Output = T> + Mul<T, Output = T> + ToSigned<Output = T>,
    > Sub for Matrix<M, N, T>
{
    type Output = Matrix<M, N, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let v = rhs * <T as ToSigned>::negative_one();
        self + v
    }
}

#[cfg(test)]
mod test {
    use crate::math::Matrix;

    #[test]
    pub fn test_scalar() {
        let mat = Matrix::new([[4, 0], [1, -9]]);
        let res = mat * 2;
        assert_eq!(res, Matrix::new([[8, 0], [2, -18]]));
    }

    #[test]
    pub fn test_product() {
        let m1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let m2 = Matrix::new([[7, 8], [9, 10], [11, 12]]);
        let res = m1 * m2;
        assert_eq!(res, Matrix::new([[58, 64], [139, 154]]));
    }
}
