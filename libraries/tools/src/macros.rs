// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

//!
//! Macros for better documentation.

/// Enables Windows-specific code.
/// Use this macro instead of `cfg(windows)` to generate docs properly.
#[macro_export]
macro_rules! cfg_windows {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), windows))]
            #[cfg_attr(docsrs, doc(cfg(windows)))]
            $item
        )*
    }
}

/// Enables Unix-specific code.
/// Use this macro instead of `cfg(unix)` to generate docs properly.
#[macro_export]
macro_rules! cfg_unix {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), unix))]
            #[cfg_attr(docsrs, doc(cfg(unix)))]
            $item
        )*
    }
}

/// Enables feature-specific code.
/// Use this macro instead of `cfg(feature = "std")` to generate docs properly.
#[macro_export]
macro_rules! cfg_feature_std {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), feature = "std"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
            $item
        )*
    }
}

/// Enables feature-specific code.
/// Use this macro instead of `cfg(feature = "alloc")` to generate docs properly.
#[macro_export]
macro_rules! cfg_feature_alloc {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), feature = "alloc"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
            $item
        )*
    }
}

/// Enables feature-specific code.
/// Use this macro instead of `cfg(feature = "serde")` to generate docs properly.
#[macro_export]
macro_rules! cfg_feature_serde {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), feature = "serde"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
            $item
        )*
    }
}

/// Enables feature-specific code.
/// Use this macro instead of `cfg(feature = "bits")` to generate docs properly.
#[macro_export]
macro_rules! cfg_feature_bits {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), feature = "bits"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "bits")))]
            $item
        )*
    }
}

/// Enables feature-specific code.
/// Use this macro instead of `cfg(feature = "egui")` to generate docs properly.
#[macro_export]
macro_rules! cfg_feature_egui {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), feature = "egui"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "egui")))]
            $item
        )*
    }
}

/// Enables feature-specific code.
/// Use this macro instead of `cfg(feature = "plots")` to generate docs properly.
#[macro_export]
macro_rules! cfg_feature_plots {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), feature = "plots"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "plots")))]
            $item
        )*
    }
}

/// Enables feature-specific code.
/// Use this macro instead of `cfg(feature = "git")` to generate docs properly.
#[macro_export]
macro_rules! cfg_feature_git {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), feature = "git"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "git")))]
            $item
        )*
    }
}
