//!
//! This module contains the configuration primitives
//!
use serde::{Deserialize, Serialize};

///
/// Configuration struct for the system
///
#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    ///
    /// Path to store the cached downloads on disk.
    /// Defaults to DEFAULT_CACHE_PATH
    #[serde(default = "default_cache_path")]
    pub cache_path: String,
}

fn default_cache_path() -> String {
    DEFAULT_CACHE_PATH.to_string()
}

/// The default location for the cache path
pub const DEFAULT_CACHE_PATH: &str = "~/.cache/rustproxy";
pub const DEFAULT_CRATESIO_UPSTREAM: &str = "https://github.com/rust-lang/crates.io-index";
