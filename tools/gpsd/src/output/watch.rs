//!
//! Structs around the watch command

/// Controls 'raw' mode
pub enum RawMode {
    /// In NMEA mode, returns the raw strings
    /// In binary mode, provides hex encoded strings
    AsciiDumped = 1,

    /// In binary mode, reports data verbatim without hex encoding
    RawBinary = 2,
}

/// This command sets watcher mode.
pub struct Watch {
    pub enable: Option<bool>,
    pub json: Option<bool>,
    pub nmea: Option<bool>,
    pub raw: Option<RawMode>,
    pub scaled: Option<bool>,
    pub split24: Option<bool>,
    pub pps: Option<bool>,
    pub remote: Option<String>,
}
