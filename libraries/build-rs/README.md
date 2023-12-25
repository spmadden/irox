IROX-BUILD-RS
==============

*Compile-time build metadata injection inspired by shadow-rs*

Quick-start
------------
### Part 1: Generating the metadata during the build
Create a `build.rs` file in your module, and include the defaults: 
```rust
pub fn main() -> Result<(), irox_build_rs::Error> {
    println!("cargo:rerun-if-changed=build.rs");

    irox_build_rs::generate_module()?;

    Ok(())
}
```
Ensure your `Cargo.toml` has:
```toml
[build-dependencies]
irox-build-rs = "{latest-version}"
```

To include variables from Git, include the 'git' feature:
```toml
[build-dependencies]
irox-build-rs = { version = "{latest-version}", features = ["git"] }
```
### Part 2: Including the build metadata
Somwhere in your module, usually in your top level `lib.rs`/`main.rs`:
```rust
pub mod build {
    include!(concat!(env!("OUT_DIR"), "/builders.rs"));
}
```

### Part 3: Use them!
```rust
fn main() {
    println!("{}", build::CARGO_PKG_NAME);
    
    for (name, value) in build::get_ALL_ITEMS() {
        println!("{name} => {value}");
    }
}
```
Each of the sets of generated constants below are available in `BTreeMap<&str, &str>`'s as well:

```rust
use std::collections::BTreeMap;

fn main() {
    let cargo_items: &BTreeMap<&str, &str> = build::get_CARGO_ITEMS();
    let rustc_items: &BTreeMap<&str, &str> = build::get_RUSTC_ITEMS();
    let git_items: &BTreeMap<&str, &str> = build::get_GIT_ITEMS();
    
    // all_items is the union of all the sets
    let all_items: &BTreeMap<&str, &str> = build::get_ALL_ITEMS();
}
```

Generated Constants
--------------------
### From Cargo:
Consult [The Cargo Book](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates) for details:
```rust
pub const CARGO_CFG_TARGET_ARCH: &str = "x86_64";
pub const CARGO_CFG_TARGET_ENDIAN: &str = "little";
pub const CARGO_CFG_TARGET_ENV: &str = "gnu";
pub const CARGO_CFG_TARGET_FAMILY: &str = "windows";
pub const CARGO_CFG_TARGET_FEATURE: &str = "fxsr,sse,sse2";
pub const CARGO_CFG_TARGET_HAS_ATOMIC: &str = "16,32,64,8,ptr";
pub const CARGO_CFG_TARGET_OS: &str = "windows";
pub const CARGO_CFG_TARGET_POINTER_WIDTH: &str = "64";
pub const CARGO_CFG_TARGET_VENDOR: &str = "pc";
pub const CARGO_PKG_AUTHORS: &str = "Sean P. Madden <sean@seanmadden.net>";
pub const CARGO_PKG_BIN_NAME: &str = "";
pub const CARGO_PKG_CRATE_NAME: &str = "";
pub const CARGO_PKG_DESCRIPTION: &str = "Extras & tweaks for the egui framework";
pub const CARGO_PKG_HOMEPAGE: &str = "https://github.com/spmadden/irox";
pub const CARGO_PKG_LICENSE: &str = "MIT OR Apache-2.0";
pub const CARGO_PKG_LICENSE_FILE: &str = "";
pub const CARGO_PKG_NAME: &str = "irox-egui-extras";
pub const CARGO_PKG_README: &str = "README.md";
pub const CARGO_PKG_REPOSITORY: &str = "https://github.com/spmadden/irox";
pub const CARGO_PKG_RUST_VERSION: &str = "";
pub const CARGO_PKG_VERSION: &str = "0.3.5";
pub const CARGO_PKG_VERSION_MAJOR: &str = "0";
pub const CARGO_PKG_VERSION_MINOR: &str = "3";
pub const CARGO_PKG_VERSION_PATCH: &str = "5";
pub const CARGO_PKG_VERSION_PRE: &str = "";
pub const CARGO_PRIMARY_PACKAGE: &str = "";
```
### From Cargo, for Rustc
Consult [The Cargo Book](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts) for details:
```rust
pub const DEBUG: &str = "true"; // "true" 
pub const HOST: &str = "x86_64-pc-windows-gnu";
pub const OPT_LEVEL: &str = "0";
pub const PROCESSOR_ARCHITECTURE: &str = "AMD64";
pub const PROCESSOR_IDENTIFIER: &str = "AMD64 Family 23 Model 113 Stepping 0, AuthenticAMD";
pub const PROCESSOR_LEVEL: &str = "23";
pub const PROCESSOR_REVISION: &str = "7100";
pub const PROFILE: &str = "debug";
pub const RUSTUP_TOOLCHAIN: &str = "1.74-x86_64-pc-windows-gnu";
pub const TARGET: &str = "x86_64-pc-windows-gnu";
```
### From Git, with feature 'git'
```rust
pub const GIT_COMMIT_AUTHOR: &str = "user.name <user.email>";
pub const GIT_COMMIT_DATETIME: &str = "2023-11-09T04:07:06Z"; // ISO8601 UTC date-time from the commit timestamp
pub const GIT_COMMIT_FULLHASH: &str = "6a22a45109c9f8ab27971c8919d693bd995f0a16"; // Full hash of the current HEAD
pub const GIT_COMMIT_SHORTHASH: &str = "6a22a45"; // Short hash of the current HEAD
pub const GIT_COMMIT_TIMESTAMP_SECS: i64 = 1699502826; // Unix timestamp in UTC of the HEAD commit
pub const GIT_COMMIT_TZ_OFFSET_SECS: i64 = -18000; // Timezone Offset (seconds) from UTC of the HEAD commit
pub const GIT_DESCRIBE: &str = "irox-build-rs-g2c9c199-dirty";  // <module-name>-g<module-short-hash><-dirty>
pub const GIT_IS_CLEAN: bool = false; // true if `git status` reports clean 
```

