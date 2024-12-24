// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Additional assert macros for tests
//!

///
/// Assert Equal with floating point epsilon/delta values.  `|left-right| <= eps`
#[macro_export]
macro_rules! assert_eq_eps {
    ($left:expr, $right:expr, $eps:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let delta = (*left_val - *right_val).abs();
                if !(delta <= $eps) {
                    panic!(
                        "Assertion failed, {} - {} = {} > {} (error: {})",
                        &*left_val,
                        &*right_val,
                        delta,
                        $eps,
                        delta - $eps
                    )
                }
            }
        }
    };
}
#[macro_export]
macro_rules! debug_assert_eq_eps {
    ($left:expr, $right:expr, $eps:expr) => {
        if cfg!(debug_assertions) {
            $crate::assert_eq_eps!($left, $right, $eps)
        }
    };
}
///
/// Assert equal w/ hex value slices, for slices of floating-point elements
#[macro_export]
macro_rules! assert_eq_eps_slice {
    ($left:expr, $right:expr, $eps:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_len = left_val.len();
                let right_len = right_val.len();
                if left_len != right_len {
                    panic!("Assertion failed, left len ({left_len}) != right len ({right_len})");
                }
                for idx in 0..left_len {
                    let lv = left_val[idx];
                    let rv = right_val[idx];
                    let delta = (lv - rv).abs();
                    if !(delta <= $eps) {
                        panic!(
                            "Assertion failed, {} - {} = {} > {} at index {idx}",
                            &lv, &rv, delta, $eps
                        )
                    }
                }
            }
        }
    };
}

///
/// Assert equal w/ hex value, for elements that implement [`core::fmt::UpperHex`]
#[macro_export]
macro_rules! assert_eq_hex {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val != *right_val {
                    panic!(
                        "Assertion failed, 0x{:0X} != 0x{:0X}",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    };
}

///
/// Assert equal w/ hex value slices, for slices of elements that implement [`core::fmt::UpperHex`]
#[macro_export]
macro_rules! assert_eq_hex_slice {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_len = left_val.len();
                let right_len = right_val.len();
                if left_len != right_len {
                    panic!("Assertion failed, left len (0x{left_len:0X}) != right len (0x{right_len:0X})");
                }
                for idx in 0..left_len {
                    let lv = left_val[idx];
                    let rv = right_val[idx];
                    if lv != rv {
                        panic!(
                            "Assertion failed, 0x{lv:0X} != 0x{rv:0X} at idx {idx}"
                        )
                    }
                }
            }
        }
    };
}
