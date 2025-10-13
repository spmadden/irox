// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::indexing_slicing)]

use crate::ToSigned;
use core::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Mul, Sub};

cfg_feature_std! {
    use crate::ToF64;
}

pub trait AsMatrix<const M: usize, const N: usize, T: Sized + Copy + Default> {
    fn as_matrix(&self) -> Matrix<M, N, T>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Matrix<const M: usize, const N: usize, T: Sized + Copy + Default> {
    pub values: [[T; N]; M],
}

impl<const M: usize, const N: usize, T: Sized + Copy + Default> Matrix<M, N, T> {
    #[must_use]
    pub const fn new(values: [[T; N]; M]) -> Matrix<M, N, T> {
        Matrix { values }
    }

    /// Returns only a subpart of the matrix
    #[must_use]
    pub fn submatrix(&self, rmin: usize, rmax: usize, cmin: usize, cmax: usize) -> Matrix<M, N, T> {
        let mut values = [[T::default(); N]; M];
        for i in rmin..rmax {
            for j in cmin..cmax {
                values[i - rmin][j - cmin] = self.values[i][j];
            }
        }
        Matrix { values }
    }
    pub fn augment<const O: usize, const R: usize>(
        &self,
        other: &Matrix<M, O, T>,
    ) -> Matrix<M, R, T> {
        augment(self, other)
    }

    ///
    /// # Panics
    pub fn swap_rows(&mut self, r1: usize, r2: usize) {
        assert!(r1 < M && r2 < M, "Values out of range");
        self.values.swap(r1, r2);
    }
}
fn augment<
    const M: usize,
    const N: usize,
    const O: usize,
    const R: usize,
    T: Sized + Copy + Default,
>(
    first: &Matrix<M, N, T>,
    second: &Matrix<M, O, T>,
) -> Matrix<M, R, T> {
    assert_eq!(R, N + O, "Values out of range");
    let mut values = [[T::default(); R]; M];
    for r in 0..M {
        for c in 0..N {
            values[r][c] = first[r][c];
        }
        for c in 0..O {
            values[r][c + N] = second[r][c];
        }
    }
    Matrix { values }
}

impl<const M: usize, const N: usize, T: Sized + Copy + Default> From<[[T; N]; M]>
    for Matrix<M, N, T>
{
    fn from(value: [[T; N]; M]) -> Self {
        Self { values: value }
    }
}
impl<const M: usize, const N: usize, T: Sized + Copy + Default> From<&[[T; N]; M]>
    for Matrix<M, N, T>
{
    fn from(value: &[[T; N]; M]) -> Self {
        Self { values: *value }
    }
}
impl<const M: usize, const N: usize, T: Sized + Copy + Default> AsMatrix<M, N, T> for [[T; N]; M] {
    fn as_matrix(&self) -> Matrix<M, N, T> {
        self.into()
    }
}
impl<const M: usize, const N: usize> Matrix<M, N, f64> {
    #[must_use]
    pub const fn mul<const P: usize>(&self, other: Matrix<N, P, f64>) -> Matrix<M, P, f64> {
        let mut out = [[0.0f64; P]; M];
        let mut m = 0;
        while m < M {
            let mut p = 0;
            while p < P {
                let mut n = 0;
                let mut sum = 0.0;
                while n < N {
                    sum += self.values[m][n] * other.values[n][p];
                    n += 1;
                }
                out[m][p] = sum;
                p += 1;
            }
            m += 1;
        }
        Matrix { values: out }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LUPDecomposition<const M: usize, const N: usize, T: Sized + Copy + Default> {
    pub lower: Matrix<M, N, T>,
    pub upper: Matrix<M, N, T>,
    pub permuted: Matrix<M, N, T>,
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

            #[must_use]
            pub fn transpose(&self) -> Self {
                let mut out = Self::empty();
                for i in 0..$N {
                    for j in 0..$N {
                        out[i][j] = self.values[j][i];
                    }
                }
                out
            }

            pub fn lup_decompose(&self) -> LUPDecomposition<$N, $N, f64> {
                let mut l = Self::identity();
                let mut u = self.clone();
                let mut p = Self::identity();

                for k in 0..$N {
                    let mut max = 0.0;
                    let mut pivot = k;
                    for i in k..$N {
                        if u[i][k].abs() > max {
                            max = u[i][k].abs();
                            pivot = i;
                        }
                    }

                    if pivot != k {
                        // rotate
                        for j in 0..$N {
                            let temp = u[k][j];
                            u[k][j] = u[pivot][j];
                            u[pivot][j] = temp;

                            let temp = p[k][j];
                            p[k][j] = p[pivot][j];
                            p[pivot][j] = temp;

                            if j < k {
                                let temp = l[k][j];
                                l[k][j] = l[pivot][j];
                                l[pivot][j] = temp;
                            }
                        }
                    }

                    for j in (k + 1)..$N {
                        l[j][k] = u[j][k] / u[k][k];
                        for i in k..$N {
                            u[j][i] -= l[j][k] * u[k][i];
                        }
                    }
                }

                LUPDecomposition {
                    lower: l,
                    upper: u,
                    permuted: p,
                }
            }
        }

        impl LUPDecomposition<$N, $N, f64> {
            pub fn solve_ax_eq_b(&self, b: &[f64; $N]) -> [f64; $N] {
                let mut b2 = [0usize; $N];
                for i in 0..$N {
                    for j in 0..$N {
                        if (self.permuted[i][j] - 1.).abs() < f64::EPSILON {
                            b2[i] = j;
                            break;
                        }
                    }
                }
                // println!("{b2:?}");
                let mut y: [f64; $N] = Default::default();
                for i in 1..$N {
                    let mut sum = 0.0;
                    for j in i..i {
                        sum += self.lower[i][j] * y[j];
                    }
                    y[i] = (b[b2[i]] - sum) / self.lower[i][i];
                }
                let mut x: [f64; $N] = Default::default();
                for i in (1..$N).rev() {
                    let mut sum = 0.0;
                    for j in i..$N {
                        sum += self.upper[i][j] * x[j];
                    }
                    x[i] = (y[i] - sum) / self.upper[i][i];
                }
                // let mut out = Matrix::<$N, $N, f64>::empty();
                // for i in 0..$N {
                //     for j in i..$N {
                //         out[i][j] = x[i];
                //     }
                // }
                // out
                x
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
    cfg_feature_std! {
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
        Matrix::mul(
            &Matrix::new([[1., 0., x], [0., 1., y], [0., 0., 1.]]),
            *self,
        )
    }
    cfg_feature_std! {
        #[must_use]
        pub fn rotate_x<T: ToF64 + Copy>(&self, angle: T) -> Self {
            Matrix::<3, 3, f64>::rotated_x(angle).mul(*self)
        }
        #[must_use]
        pub fn rotate_y<T: ToF64 + Copy>(&self, angle: T) -> Self {
            Matrix::<3, 3, f64>::rotated_y(angle).mul(*self)
        }
        #[must_use]
        pub fn rotate_z<T: ToF64 + Copy>(&self, angle: T) -> Self {
            Matrix::<3, 3, f64>::rotated_z(angle).mul(*self)
        }
        #[must_use]
        pub fn rotate_zyx<T: ToF64 + Copy>(&self, x_angle: T, y_angle: T, z_angle: T) -> Self {
            Matrix::<3, 3, f64>::rotated_zyx(x_angle, y_angle, z_angle).mul(*self)
        }
    }
}
impl Matrix<3, 3, f64> {
    cfg_feature_std! {
        #[must_use]
        pub fn rotated_x<T: ToF64 + Copy>(angle: T) -> Self {
        let angle = angle.to_f64();
            Self::new([
                [1., 0., 0.],
                [0., angle.cos(), -angle.sin()],
                [0., angle.sin(), angle.cos()],
            ])
        }
        #[must_use]
        pub fn rotate_x<T: ToF64 + Copy>(&self, angle: T) -> Self {
            self.mul(Self::rotated_x(angle))
        }
        #[must_use]
        pub fn rotated_y<T: ToF64 + Copy>(angle: T) -> Self {
            let angle = angle.to_f64();
                Self::new([
                    [angle.cos(), 0.0, angle.sin()],
                    [0., 1., 0.],
                    [-angle.sin(), 0., angle.cos()],
                ])
            }
            #[must_use]
        pub fn rotate_y<T: ToF64 + Copy>(&self, angle: T) -> Self {
            self.mul(Self::rotated_y(angle))
        }
        #[must_use]
        pub fn rotated_z<T: ToF64 + Copy>(angle: T) -> Self {
            let angle = angle.to_f64();
                Self::new([
                    [angle.cos(), angle.sin(), 0.],
                    [-angle.sin(), angle.cos(), 0.],
                    [0., 0., 1.],
                ])
            }
        #[must_use]
        pub fn rotate_z<T: ToF64 + Copy>(&self, angle: T) -> Self {
            self.mul(Self::rotated_z(angle))
        }

        #[must_use]
        pub fn rotated_zyx<T: ToF64 + Copy>(x_angle: T, y_angle: T, z_angle: T) -> Self {
            Self::rotated_z(z_angle).rotate_y(y_angle).rotate_x(x_angle)
        }
        #[must_use]
        pub fn rotate_zyx<T: ToF64 + Copy>(&self, x_angle: T, y_angle: T, z_angle: T) -> Self {
            self.rotate_z(z_angle).rotate_y(y_angle).rotate_x(x_angle)
        }
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
// matrix multiply
macro_rules! impl_mul {
    ($($ty:ty)+) => {
        impl<
            const M: usize,
            const N: usize,
            const P: usize,
            T: Sized + Copy + Default + Add + Mul + AddAssign<<T as Mul<T>>::Output>,
            > Mul<Matrix<N, P, T>> for $($ty)+
        {
            type Output = Matrix<M, P, T>;
            fn mul(self, other: Matrix<N, P, T>) -> Matrix<M, P, T> {
                let mut out = [[T::default(); P]; M];
                let mut m = 0;
                while m < M {
                    let mut p = 0;
                    while p < P {
                        let mut n = 0;
                        let mut sum = T::default();
                        while n < N {
                            sum += self.values[m][n] * other.values[n][p];
                            n += 1;
                        }
                        out[m][p] = sum;
                        p += 1;
                    }
                    m += 1;
                }
                Matrix { values: out }
            }
        }
    };
}
impl_mul!(Matrix<M, N, T>);
impl_mul!(&Matrix<M, N, T>);
impl_mul!(&mut Matrix<M, N, T>);

// matrix add
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
// scalar multiply
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
    use crate::math::{AsMatrix, LUPDecomposition, Matrix};
    use core::ops::Deref;

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

    #[cfg(feature = "std")]
    #[test]
    pub fn test_rotate1() {
        let m = [[3.], [7.], [4.]].as_matrix();
        let [[x], [y], [z]] = *m.rotate_x(core::f64::consts::FRAC_PI_2).deref();
        assert_eq_eps!(3., x, 2. * f64::EPSILON);
        assert_eq_eps!(-4., y, 2. * f64::EPSILON);
        assert_eq_eps!(7., z, 2. * f64::EPSILON);
    }

    #[cfg(feature = "std")]
    #[test]
    pub fn test_rotate2() {
        let m = [[3.], [7.], [4.]].as_matrix();
        let [[x], [y], [z]] = *m.rotate_y(core::f64::consts::FRAC_PI_2).deref();
        assert_eq_eps!(4., x, 2. * f64::EPSILON);
        assert_eq_eps!(7., y, 2. * f64::EPSILON);
        assert_eq_eps!(-3., z, 2. * f64::EPSILON);
    }

    #[cfg(feature = "std")]
    #[test]
    pub fn test_rotate3() {
        let m = [[3.], [7.], [4.]].as_matrix();
        let [[x], [y], [z]] = *m.rotate_z(core::f64::consts::FRAC_PI_2).deref();
        assert_eq_eps!(7., x, 2. * f64::EPSILON);
        assert_eq_eps!(-3., y, 2. * f64::EPSILON);
        assert_eq_eps!(4., z, 2. * f64::EPSILON);
    }

    #[cfg(feature = "std")]
    #[test]
    pub fn test_lup1() {
        let a = [
            [2., 0.0, 2., 0.6],
            [3., 3., 4., -2.],
            [5., 5., 4., 2.],
            [-1., -2., 3.4, -1.],
        ]
        .as_matrix();
        let LUPDecomposition {
            lower,
            upper,
            permuted,
        } = a.lup_decompose();
        assert_eq!(
            lower,
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.4, 1.0, 0.0, 0.0],
                [-0.2, 0.5, 1.0, 0.0],
                [0.6, -0.0, 0.4, 1.0]
            ]
            .as_matrix()
        );
        assert_eq!(
            upper,
            [
                [5.0, 5.0, 4.0, 2.0],
                [0.0, -2.0, 0.3999999999999999, -0.20000000000000007],
                [0.0, 0.0, 4.0, -0.49999999999999994],
                [0.0, 0.0, 0.0, -3.0]
            ]
            .as_matrix()
        );
        assert_eq!(
            permuted,
            [
                [0.0, 0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 0.0]
            ]
            .as_matrix()
        );

        let c1 = permuted * a;
        let c2 = lower * upper;
        assert_eq!(c1, c2);
    }

    #[cfg(feature = "std")]
    #[test]
    pub fn test_solve1() {
        let a = [[25., 5., 1.], [64., 8., 1.], [144., 12., 1.]].as_matrix();
        let lup = a.lup_decompose();
        println!("{:?}", lup);
        let c1 = lup.permuted * a;
        let c2 = lup.lower * lup.upper;
        assert_eq!(c1, c2);

        let b = [106.8, 177.2, 279.2];
        let c = [b].as_matrix() * lup.permuted;
        println!("{c:?}");
        let res = lup.solve_ax_eq_b(&b);
        println!("{:?}", res);
    }

    #[cfg(feature = "std")]
    #[test]
    pub fn test_lup2() {
        let a = [[3., 17., 10.], [2., 4., -2.], [6., 18., -12.]].as_matrix();
        let lup = a.lup_decompose();
        assert_eq!(
            lup.lower,
            [[1., 0., 0.], [0.5, 1., 0.], [1. / 3., -0.25, 1.]].as_matrix()
        );
        assert_eq!(
            lup.upper,
            [[6., 18., -12.], [0., 8., 16.], [0., 0., 6.]].as_matrix()
        );
    }

    #[cfg(feature = "std")]
    #[test]
    pub fn test_lup3() {
        let a = [[2., 1., -5.], [4., 4., -4.], [1., 3., 1.]].as_matrix();
        let lup = a.lup_decompose();
        println!("{:?}", lup);
        let c1 = lup.permuted * a;
        let c2 = lup.lower * lup.upper;
        assert_eq!(c1, c2);

        let b = [5., 0., 6.];
        let res = lup.solve_ax_eq_b(&b);
        println!("{:?}", res);

        let b2 = [res].as_matrix() * a;
        println!("{:?}", b2);
    }
}
