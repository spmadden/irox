// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Basic console log implementation.  Unconfigurable at the moment.
//!
//! Outputs in the format of: `[ThhMMssZ {LEVEL} {ThreadName} {Module}] {message}`

// Allow uninlined is needed because RustRover doesn't yet detect that these are actually *used*
// and will helpfully remove the imports during "optimization"
#![allow(clippy::uninlined_format_args)]

use std::io::Write;

use log::{Level, Metadata, Record};

use irox_time::datetime::UTCDateTime;
use irox_time::format::iso8601::BASIC_TIME_OF_DAY;
use irox_tools::ansi_colors::{
    FORMAT_COLOR_FG_BLUE, FORMAT_COLOR_FG_CYAN, FORMAT_COLOR_FG_MAGENTA, FORMAT_COLOR_FG_RED,
    FORMAT_COLOR_FG_YELLOW, FORMAT_RESET,
};

macro_rules! mk_log {
    ($name:ident, $level:expr) => {
        pub static $name: ConsoleLogger = ConsoleLogger { max_level: $level };
    };
}

mk_log!(ERROR_LOGGER, Level::Error);
mk_log!(WARN_LOGGER, Level::Warn);
mk_log!(INFO_LOGGER, Level::Info);
mk_log!(DEBUG_LOGGER, Level::Debug);
mk_log!(TRACE_LOGGER, Level::Trace);

///
/// Basic console logger with a static format.
pub struct ConsoleLogger {
    max_level: Level,
}

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.max_level >= metadata.level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let level = match record.level() {
            Level::Error => format!("{}ERROR{}", FORMAT_COLOR_FG_RED, FORMAT_RESET),
            Level::Warn => format!("{}WARN{FORMAT_RESET}", FORMAT_COLOR_FG_YELLOW),
            Level::Info => format!("{}INFO{FORMAT_RESET}", FORMAT_COLOR_FG_BLUE),
            Level::Debug => format!("{}DEBUG{FORMAT_RESET}", FORMAT_COLOR_FG_MAGENTA),
            Level::Trace => format!("{}TRACE{FORMAT_RESET}", FORMAT_COLOR_FG_CYAN),
        };
        let time = UTCDateTime::now().format(&BASIC_TIME_OF_DAY);
        let thread = std::thread::current();
        let thread = thread.name().unwrap_or("").split("::").last().unwrap_or("");
        let module = record
            .module_path()
            .unwrap_or("")
            .split("::")
            .last()
            .unwrap_or("");
        if let Err(_e) = writeln!(
            std::io::stderr(),
            "[{time} {level} {thread} {module}] {}",
            record.args()
        ) {
            // idk, lets eat it?
        }
    }

    fn flush(&self) {
        let _ign = std::io::stderr().flush();
    }
}

#[cfg(test)]
mod tests {
    use log::*;

    use crate::init_console_level;

    #[test]
    pub fn test() {
        init_console_level(Level::Trace);

        error!("Test Error!");
        warn!("Test Warn!");
        info!("Test Info!");
        debug!("Test Debug!");
        trace!("Test Trace!");
    }
}
