// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]
#![no_std]

use core::fmt::{Display, Formatter};

pub struct FixedU64 {
    data: u64,
}
impl Display for FixedU64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Into::<f64>::into(self))
    }
}
impl FixedU64 {
    pub fn from_raw_value(data: u64) -> Self {
        FixedU64 { data }
    }
}

impl From<u64> for FixedU64 {
    fn from(data: u64) -> Self {
        FixedU64 { data: data << 32 }
    }
}
macro_rules! impl_fromf64 {
    ($($typ:tt)+) => {
        impl From<$($typ)+> for f64 {
            fn from(value: $($typ)+) -> Self {
                let val = (value.data >> 32) as f64;
                val + ((value.data & 0xFFFFFFFF) as f64 / 0x100000000u64 as f64)
            }
        }
    };
}
impl_fromf64!(FixedU64);
impl_fromf64!(&FixedU64);
impl_fromf64!(&mut FixedU64);

macro_rules! impl_from {
    ($prim:ty, $($typ:tt)+) => {
        impl From<$($typ)+> for $prim {
            fn from(value: $($typ)+) -> Self {
                value.data >> 32
            }
        }
    };
}
impl_from!(u64, FixedU64);
impl_from!(u64, &FixedU64);
impl_from!(u64, &mut FixedU64);

macro_rules! impl_add {
    ($strukt:ty, $($typ:tt)+) => {
        impl core::ops::Add<$strukt> for $($typ)+ {
            type Output = $strukt;

            fn add(self, rhs: $strukt) -> Self::Output {
                <$strukt>::from_raw_value(self.data.saturating_add(rhs.data))
            }
        }
    };
}
impl_add!(FixedU64, FixedU64);
impl_add!(FixedU64, &FixedU64);
impl_add!(FixedU64, &mut FixedU64);

macro_rules! impl_sub {
    ($strukt:ty, $($typ:tt)+) => {
        impl core::ops::Sub<$strukt> for $($typ)+ {
            type Output = $strukt;

            fn sub(self, rhs: $strukt) -> Self::Output {
                <$strukt>::from_raw_value(self.data.saturating_sub(rhs.data))
            }
        }
    };
}
impl_sub!(FixedU64, FixedU64);
impl_sub!(FixedU64, &FixedU64);
impl_sub!(FixedU64, &mut FixedU64);

macro_rules! impl_addassign {
    ($strukt:ty, $($typ:tt)+) => {
        impl core::ops::AddAssign<$($typ)+> for $strukt {
            fn add_assign(&mut self, rhs: $($typ)+) {
                self.data = self.data.saturating_add(rhs.data)
            }
        }

        impl core::ops::AddAssign<$($typ)+> for &mut $strukt {
            fn add_assign(&mut self, rhs: $($typ)+) {
                self.data = self.data.saturating_add(rhs.data)
            }
        }
    }
}
impl_addassign!(FixedU64, FixedU64);
impl_addassign!(FixedU64, &FixedU64);
impl_addassign!(FixedU64, &mut FixedU64);

macro_rules! impl_subassign {
    ($strukt:ty, $($typ:tt)+) => {
        impl core::ops::SubAssign<$($typ)+> for $strukt {
            fn sub_assign(&mut self, rhs: $($typ)+) {
                self.data = self.data.saturating_sub(rhs.data)
            }
        }

        impl core::ops::SubAssign<$($typ)+> for &mut $strukt {
            fn sub_assign(&mut self, rhs: $($typ)+) {
                self.data = self.data.saturating_sub(rhs.data)
            }
        }
    }
}
impl_subassign!(FixedU64, FixedU64);
impl_subassign!(FixedU64, &FixedU64);
impl_subassign!(FixedU64, &mut FixedU64);

macro_rules! impl_mul {
    ($strukt:ty, $($typ:tt)+) => {
        impl core::ops::Mul<$($typ)+> for $strukt {
            type Output = $strukt;

            fn mul(self, rhs: $($typ)+) -> Self::Output {
                let o = (self.data as u128 * rhs.data as u128) >> 31;
                let add = o & 0x01;
                let o = (o >> 1) + add;
                <$strukt>::from_raw_value(o as u64)
            }
        }
    }
}

impl_mul!(FixedU64, FixedU64);
impl_mul!(FixedU64, &FixedU64);
impl_mul!(FixedU64, &mut FixedU64);

macro_rules! impl_div {
    ($strukt:ty, $($typ:tt)+) => {
        impl core::ops::Div<$($typ)+> for $strukt {
            type Output = $strukt;

            fn div(self, rhs: $($typ)+) -> Self::Output {
                let a = (self.data as u128) << 32;
                let b = (rhs.data as u128);
                let o = (a / b) as u64;
                <$strukt>::from_raw_value(o)
            }
        }
    }
}
impl_div!(FixedU64, FixedU64);
impl_div!(FixedU64, &FixedU64);
impl_div!(FixedU64, &mut FixedU64);
