// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use core::iter::Sum;
use core::ops::{Add, Mul, Sub};

pub struct Matrix<const M: usize, const N: usize, T: Sized + Copy + Default> {
    values: [[T; N]; M],
}

impl<const M: usize, const N: usize, T: Sized + Copy + Default> Matrix<M, N, T> {
    pub fn new(values: [[T; N]; M]) -> Matrix<M, N, T> {
        Matrix { values }
    }
}
#[allow(clippy::indexing_slicing)]
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
#[allow(clippy::indexing_slicing)]
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
#[allow(clippy::indexing_slicing)]
impl<const M: usize, const N: usize, T: Sized + Copy + Default + Mul<f64, Output = T>> Mul<f64>
    for Matrix<M, N, T>
{
    type Output = Matrix<M, N, T>;
    fn mul(self, other: f64) -> Matrix<M, N, T> {
        let mut out = [[T::default(); N]; M];

        for (i, ith) in out.iter_mut().enumerate().take(M) {
            for (j, val) in ith.iter_mut().enumerate().take(N) {
                *val = self.values[i][j] * other;
            }
        }
        Matrix { values: out }
    }
}

#[allow(clippy::indexing_slicing)]
impl<
        const M: usize,
        const N: usize,
        T: Sized + Copy + Default + Add<Output = T> + Mul<f64, Output = T>,
    > Sub for Matrix<M, N, T>
{
    type Output = Matrix<M, N, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let v = rhs * -1.0;
        self + v
    }
}
