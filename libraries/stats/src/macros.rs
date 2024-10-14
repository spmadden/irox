// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

/// Enables feature-specific code.
/// Use this macro instead of `cfg(feature = "miniz")` to generate docs properly.
#[macro_export]
macro_rules! cfg_feature_miniz {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), feature = "miniz"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "miniz")))]
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
