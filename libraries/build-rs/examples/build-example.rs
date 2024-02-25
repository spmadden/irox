// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use std::collections::BTreeMap;
mod build {
    #![allow(dead_code)]
    pub const CARGO_CFG_TARGET_ARCH: &str = r#"x86_64"#;
    pub const CARGO_CFG_TARGET_ENDIAN: &str = r#"little"#;
    pub const CARGO_CFG_TARGET_ENV: &str = r#"gnu"#;
    pub const CARGO_CFG_TARGET_FAMILY: &str = r#"windows"#;
    pub const CARGO_CFG_TARGET_FEATURE: &str = r#"fxsr,sse,sse2"#;
    pub const CARGO_CFG_TARGET_HAS_ATOMIC: &str = r#"16,32,64,8,ptr"#;
    pub const CARGO_CFG_TARGET_OS: &str = r#"windows"#;
    pub const CARGO_CFG_TARGET_POINTER_WIDTH: &str = r#"64"#;
    pub const CARGO_CFG_TARGET_VENDOR: &str = r#"pc"#;
    pub const CARGO_PKG_AUTHORS: &str = r#"user.name <user.email>"#;
    pub const CARGO_PKG_BIN_NAME: &str = r#""#;
    pub const CARGO_PKG_CRATE_NAME: &str = r#""#;
    pub const CARGO_PKG_DESCRIPTION: &str =
        r#"Compile-time build metadata injection inspired by shadow-rs"#;
    pub const CARGO_PKG_HOMEPAGE: &str = r#"https://github.com/spmadden/irox"#;
    pub const CARGO_PKG_LICENSE: &str = r#"MIT OR Apache-2.0"#;
    pub const CARGO_PKG_LICENSE_FILE: &str = r#"LICENSE"#;
    pub const CARGO_PKG_NAME: &str = r#"irox-egui-extras"#;
    pub const CARGO_PKG_README: &str = r#"README.md"#;
    pub const CARGO_PKG_REPOSITORY: &str = r#"https://github.com/spmadden/irox"#;
    pub const CARGO_PKG_RUST_VERSION: &str = r#"1.74"#;
    pub const CARGO_PKG_VERSION: &str = r#"0.3.5"#;
    pub const CARGO_PKG_VERSION_MAJOR: &str = r#"0"#;
    pub const CARGO_PKG_VERSION_MINOR: &str = r#"3"#;
    pub const CARGO_PKG_VERSION_PATCH: &str = r#"5"#;
    pub const CARGO_PKG_VERSION_PRE: &str = r#""#;
    pub const CARGO_PRIMARY_PACKAGE: &str = r#"true"#;
    pub const DEBUG: &str = r#"true"#;
    pub const GIT_COMMIT_AUTHOR: &str = r#"user.name <user.email>"#;
    pub const GIT_COMMIT_DATETIME: &str = r#"2023-11-09T04:07:06Z"#;
    pub const GIT_COMMIT_FULLHASH: &str = r#"6a22a45109c9f8ab27971c8919d693bd995f0a16"#;
    pub const GIT_COMMIT_SHORTHASH: &str = r#"6a22a45"#;
    pub const GIT_COMMIT_TIMESTAMP_SECS: i64 = 1699502826;
    pub const GIT_COMMIT_TZ_OFFSET_SECS: i64 = -18000;
    pub const GIT_DESCRIBE: &str = r#"irox-build-rs-g2c9c199-dirty"#;
    pub const GIT_IS_CLEAN: bool = false;
    pub const HOST: &str = r#"x86_64-pc-windows-gnu"#;
    pub const OPT_LEVEL: &str = r#"0"#;
    pub const PROCESSOR_ARCHITECTURE: &str = r#"AMD64"#;
    pub const PROCESSOR_IDENTIFIER: &str = r#"AMD64 Family 23 Model 113 Stepping 0, AuthenticAMD"#;
    pub const PROCESSOR_LEVEL: &str = r#"23"#;
    pub const PROCESSOR_REVISION: &str = r#"7100"#;
    pub const PROFILE: &str = r#"debug"#;
    pub const RUSTUP_TOOLCHAIN: &str = r#"1.74-x86_64-pc-windows-gnu"#;
    pub const TARGET: &str = r#"x86_64-pc-windows-gnu"#;
    static CARGO_ITEMS: std::sync::OnceLock<
        std::collections::BTreeMap<&'static str, &'static str>,
    > = std::sync::OnceLock::new();
    #[allow(non_snake_case)]
    pub fn get_CARGO_ITEMS() -> &'static std::collections::BTreeMap<&'static str, &'static str> {
        CARGO_ITEMS.get_or_init(|| {
            std::collections::BTreeMap::from([
                ("CARGO_CFG_TARGET_ARCH", CARGO_CFG_TARGET_ARCH),
                ("CARGO_CFG_TARGET_ENDIAN", CARGO_CFG_TARGET_ENDIAN),
                ("CARGO_CFG_TARGET_ENV", CARGO_CFG_TARGET_ENV),
                ("CARGO_CFG_TARGET_FAMILY", CARGO_CFG_TARGET_FAMILY),
                ("CARGO_CFG_TARGET_FEATURE", CARGO_CFG_TARGET_FEATURE),
                ("CARGO_CFG_TARGET_HAS_ATOMIC", CARGO_CFG_TARGET_HAS_ATOMIC),
                ("CARGO_CFG_TARGET_OS", CARGO_CFG_TARGET_OS),
                (
                    "CARGO_CFG_TARGET_POINTER_WIDTH",
                    CARGO_CFG_TARGET_POINTER_WIDTH,
                ),
                ("CARGO_CFG_TARGET_VENDOR", CARGO_CFG_TARGET_VENDOR),
                ("CARGO_PKG_AUTHORS", CARGO_PKG_AUTHORS),
                ("CARGO_PKG_BIN_NAME", CARGO_PKG_BIN_NAME),
                ("CARGO_PKG_CRATE_NAME", CARGO_PKG_CRATE_NAME),
                ("CARGO_PKG_DESCRIPTION", CARGO_PKG_DESCRIPTION),
                ("CARGO_PKG_HOMEPAGE", CARGO_PKG_HOMEPAGE),
                ("CARGO_PKG_LICENSE", CARGO_PKG_LICENSE),
                ("CARGO_PKG_LICENSE_FILE", CARGO_PKG_LICENSE_FILE),
                ("CARGO_PKG_NAME", CARGO_PKG_NAME),
                ("CARGO_PKG_README", CARGO_PKG_README),
                ("CARGO_PKG_REPOSITORY", CARGO_PKG_REPOSITORY),
                ("CARGO_PKG_RUST_VERSION", CARGO_PKG_RUST_VERSION),
                ("CARGO_PKG_VERSION", CARGO_PKG_VERSION),
                ("CARGO_PKG_VERSION_MAJOR", CARGO_PKG_VERSION_MAJOR),
                ("CARGO_PKG_VERSION_MINOR", CARGO_PKG_VERSION_MINOR),
                ("CARGO_PKG_VERSION_PATCH", CARGO_PKG_VERSION_PATCH),
                ("CARGO_PKG_VERSION_PRE", CARGO_PKG_VERSION_PRE),
                ("CARGO_PRIMARY_PACKAGE", CARGO_PRIMARY_PACKAGE),
            ])
        })
    }
    static RUSTC_ITEMS: std::sync::OnceLock<
        std::collections::BTreeMap<&'static str, &'static str>,
    > = std::sync::OnceLock::new();
    #[allow(non_snake_case)]
    pub fn get_RUSTC_ITEMS() -> &'static std::collections::BTreeMap<&'static str, &'static str> {
        RUSTC_ITEMS.get_or_init(|| {
            std::collections::BTreeMap::from([
                ("DEBUG", DEBUG),
                ("HOST", HOST),
                ("OPT_LEVEL", OPT_LEVEL),
                ("PROCESSOR_ARCHITECTURE", PROCESSOR_ARCHITECTURE),
                ("PROCESSOR_IDENTIFIER", PROCESSOR_IDENTIFIER),
                ("PROCESSOR_LEVEL", PROCESSOR_LEVEL),
                ("PROCESSOR_REVISION", PROCESSOR_REVISION),
                ("PROFILE", PROFILE),
                ("RUSTUP_TOOLCHAIN", RUSTUP_TOOLCHAIN),
                ("TARGET", TARGET),
            ])
        })
    }
    static GIT_ITEMS: std::sync::OnceLock<std::collections::BTreeMap<&'static str, &'static str>> =
        std::sync::OnceLock::new();
    #[allow(non_snake_case)]
    pub fn get_GIT_ITEMS() -> &'static std::collections::BTreeMap<&'static str, &'static str> {
        GIT_ITEMS.get_or_init(|| {
            std::collections::BTreeMap::from([
                ("GIT_COMMIT_AUTHOR", GIT_COMMIT_AUTHOR),
                ("GIT_COMMIT_DATETIME", GIT_COMMIT_DATETIME),
                ("GIT_COMMIT_TIMESTAMP_SECS", "1699502826"),
                ("GIT_COMMIT_TZ_OFFSET_SECS", "-18000"),
                ("GIT_DESCRIBE", GIT_DESCRIBE),
                ("GIT_IS_CLEAN", "false"),
            ])
        })
    }
    static ALL_ITEMS: std::sync::OnceLock<std::collections::BTreeMap<&'static str, &'static str>> =
        std::sync::OnceLock::new();
    #[allow(non_snake_case)]
    pub fn get_ALL_ITEMS() -> &'static std::collections::BTreeMap<&'static str, &'static str> {
        ALL_ITEMS.get_or_init(|| {
            std::collections::BTreeMap::from([
                ("CARGO_CFG_TARGET_ARCH", CARGO_CFG_TARGET_ARCH),
                ("CARGO_CFG_TARGET_ENDIAN", CARGO_CFG_TARGET_ENDIAN),
                ("CARGO_CFG_TARGET_ENV", CARGO_CFG_TARGET_ENV),
                ("CARGO_CFG_TARGET_FAMILY", CARGO_CFG_TARGET_FAMILY),
                ("CARGO_CFG_TARGET_FEATURE", CARGO_CFG_TARGET_FEATURE),
                ("CARGO_CFG_TARGET_HAS_ATOMIC", CARGO_CFG_TARGET_HAS_ATOMIC),
                ("CARGO_CFG_TARGET_OS", CARGO_CFG_TARGET_OS),
                (
                    "CARGO_CFG_TARGET_POINTER_WIDTH",
                    CARGO_CFG_TARGET_POINTER_WIDTH,
                ),
                ("CARGO_CFG_TARGET_VENDOR", CARGO_CFG_TARGET_VENDOR),
                ("CARGO_PKG_AUTHORS", CARGO_PKG_AUTHORS),
                ("CARGO_PKG_BIN_NAME", CARGO_PKG_BIN_NAME),
                ("CARGO_PKG_CRATE_NAME", CARGO_PKG_CRATE_NAME),
                ("CARGO_PKG_DESCRIPTION", CARGO_PKG_DESCRIPTION),
                ("CARGO_PKG_HOMEPAGE", CARGO_PKG_HOMEPAGE),
                ("CARGO_PKG_LICENSE", CARGO_PKG_LICENSE),
                ("CARGO_PKG_LICENSE_FILE", CARGO_PKG_LICENSE_FILE),
                ("CARGO_PKG_NAME", CARGO_PKG_NAME),
                ("CARGO_PKG_README", CARGO_PKG_README),
                ("CARGO_PKG_REPOSITORY", CARGO_PKG_REPOSITORY),
                ("CARGO_PKG_RUST_VERSION", CARGO_PKG_RUST_VERSION),
                ("CARGO_PKG_VERSION", CARGO_PKG_VERSION),
                ("CARGO_PKG_VERSION_MAJOR", CARGO_PKG_VERSION_MAJOR),
                ("CARGO_PKG_VERSION_MINOR", CARGO_PKG_VERSION_MINOR),
                ("CARGO_PKG_VERSION_PATCH", CARGO_PKG_VERSION_PATCH),
                ("CARGO_PKG_VERSION_PRE", CARGO_PKG_VERSION_PRE),
                ("CARGO_PRIMARY_PACKAGE", CARGO_PRIMARY_PACKAGE),
                ("DEBUG", DEBUG),
                ("GIT_COMMIT_AUTHOR", GIT_COMMIT_AUTHOR),
                ("GIT_COMMIT_DATETIME", GIT_COMMIT_DATETIME),
                ("GIT_COMMIT_FULLHASH", GIT_COMMIT_FULLHASH),
                ("GIT_COMMIT_SHORTHASH", GIT_COMMIT_SHORTHASH),
                ("GIT_COMMIT_TIMESTAMP_SECS", "1699502826"),
                ("GIT_COMMIT_TZ_OFFSET_SECS", "-18000"),
                ("GIT_DESCRIBE", GIT_DESCRIBE),
                ("GIT_IS_CLEAN", "false"),
                ("HOST", HOST),
                ("OPT_LEVEL", OPT_LEVEL),
                ("PROCESSOR_ARCHITECTURE", PROCESSOR_ARCHITECTURE),
                ("PROCESSOR_IDENTIFIER", PROCESSOR_IDENTIFIER),
                ("PROCESSOR_LEVEL", PROCESSOR_LEVEL),
                ("PROCESSOR_REVISION", PROCESSOR_REVISION),
                ("PROFILE", PROFILE),
                ("RUSTUP_TOOLCHAIN", RUSTUP_TOOLCHAIN),
                ("TARGET", TARGET),
            ])
        })
    }
}

#[allow(clippy::print_stdout)]
fn main() {
    println!("{}", build::CARGO_PKG_NAME);

    for (name, value) in build::get_ALL_ITEMS() {
        println!("ALL: {name} => {value}");
    }

    let cargo_items: &BTreeMap<&str, &str> = build::get_CARGO_ITEMS();
    for (name, value) in cargo_items {
        println!("CARGO: {name} => {value}");
    }
    let rustc_items: &BTreeMap<&str, &str> = build::get_RUSTC_ITEMS();
    for (name, value) in rustc_items {
        println!("RUSTC: {name} => {value}");
    }
    let git_items: &BTreeMap<&str, &str> = build::get_GIT_ITEMS();
    for (name, value) in git_items {
        println!("GIT: {name} => {value}");
    }
}
