// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Static Initialization Primitives
//!

///
/// Creates a static lazy initialization variable function.
///
/// ```no_run
/// # use irox_tools::static_init;
/// static_init!(get_data, String, {
///     "This could be an expensive operation!".to_string()
/// });
/// ```
/// expands to:
/// ```no_run
/// pub fn get_data() -> &'static String {
///     static VARBL: std::sync::OnceLock<String> = std::sync::OnceLock::new();
///     VARBL.get_or_init(|| {
///         "This could be an expensive operation!".to_string()
///     })
/// }
/// ```
#[macro_export]
macro_rules! static_init {
    ($name:ident,$out:ty,$($init:tt)+) => {
        pub fn $name() -> &'static $out {
            static VARBL: std::sync::OnceLock<$out> = std::sync::OnceLock::new();
            VARBL.get_or_init(|| {
                $($init)+
            })
        }
    };
}
