// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Basic console and file logging.
//!

#![forbid(unsafe_code)]

pub use log::Level;
use std::str::FromStr;

pub mod console;

macro_rules! set_con_logger {
    ($name:ident) => {
        if let Err(e) = log::set_logger(&console::$name) {
            eprintln!("Error setting logger: {e:?}");
        };
    };
}

///
/// Initializes the console logger to [`Level::Warn`]
pub fn init_console() {
    init_console_level(Level::Warn);
}

///
/// Initializes the console logger to the specified [`Level`]
pub fn init_console_level(max_level: Level) {
    log::set_max_level(max_level.to_level_filter());
    match max_level {
        Level::Error => {
            set_con_logger!(ERROR_LOGGER);
        }
        Level::Warn => {
            set_con_logger!(WARN_LOGGER);
        }
        Level::Info => {
            set_con_logger!(INFO_LOGGER);
        }
        Level::Debug => {
            set_con_logger!(DEBUG_LOGGER);
        }
        Level::Trace => {
            set_con_logger!(TRACE_LOGGER);
        }
    }
}

///
/// Initializes the console logger from the configuration in the specified
/// environment variable.
pub fn init_console_from_env(varbl: &str) {
    if let Ok(level) = std::env::var(varbl) {
        let Ok(level) = Level::from_str(&level) else {
            return;
        };
        init_console_level(level);
    } else {
        init_console_level(Level::Warn);
    }
}
