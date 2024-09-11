// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Macros to aid in the creation of crate-level error structs

crate::cfg_feature_alloc! {
    ///
    /// Macro to build a fairly standard error struct.
    /// ```ignore
    /// extern crate alloc;
    /// #[derive(Debug, Clone, Eq, PartialEq)]
    /// pub struct ErrorName {
    ///     error_type: ErrorType,
    ///     msg: alloc::string::String,
    /// }
    /// impl core::fmt::Display for ErrorName {
    ///     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    ///         f.write_fmt(format_args!("{:?}: {}", self.error_type, self.msg))
    ///     }
    /// }
    /// #[cfg(feature = "std")]
    /// impl std::error::Error for ErrorName {}
    /// ```
    #[macro_export]
    macro_rules! impl_error {
        ($ErrorName:ident, $ErrorType:ident) => {
            extern crate alloc;
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct $ErrorName {
                error_type: $ErrorType,
                msg: alloc::string::String,
            }
            impl core::fmt::Display for $ErrorName {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_fmt(format_args!("{:?}: {}", self.error_type, self.msg))
                }
            }
            #[cfg(feature = "std")]
            impl std::error::Error for $ErrorName {}
        };
    }
}
///
/// Macro to implement from converters for error types generated with [`impl_error!`]
/// ```ignore
/// # struct ErrorName;
/// impl From<source> for ErrorName {
///     fn from(value: source) -> Self {
///         ErrorName {
///             error_type: ty,
///             msg: value.to_string(),
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_from_error {
    ($ErrorName:ident,$source:ty,$ty:expr) => {
        impl From<$source> for $ErrorName {
            fn from(value: $source) -> Self {
                Error {
                    error_type: $ty,
                    msg: value.to_string(),
                }
            }
        }
    };
}

/// Macro to template out standard builder functions for types build with [`impl_error`]
/// ```ignore
/// impl ErrorName {
///     pub fn name(msg: String) -> Self {
///         Self {
///             error_type: ty,
///             msg,
///         }
///     }
///     pub fn name_err<T>(msg: String) -> Result<T, Self> {
///         Err(Self::name(msg))
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_err_fn {
    ($ErrorName:ident, $ty:expr, $name:ident, $name_err:ident) => {
        impl $ErrorName {
            pub fn $name(msg: String) -> Self {
                Self {
                    error_type: $ty,
                    msg,
                }
            }
            pub fn $name_err<T>(msg: String) -> Result<T, Self> {
                Err(Self::$name(msg))
            }
        }
    };
}
